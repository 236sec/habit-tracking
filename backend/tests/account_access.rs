use std::sync::Arc;

use habit_tracking_backend::bootstrap::{app::create_app, config::AppConfig, state::AppState};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;

#[tokio::test]
async fn me_rejects_requests_without_a_verified_bearer_token() {
    let pool = PgPoolOptions::new()
        .connect_lazy("postgres://localhost/unused")
        .unwrap();
    let state = AppState {
        config: Arc::new(AppConfig {
            database_url: "postgres://localhost/unused".to_owned(),
            bind_addr: "127.0.0.1:0".to_owned(),
            auth_issuers: [
                "http://127.0.0.1:1/auth/v1".to_owned(),
                "http://127.0.0.1:2/auth/v1".to_owned(),
            ],
        }),
        pool,
    };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, create_app(state)).await.unwrap() });
    let client = reqwest::Client::new();

    let missing = client
        .get(format!("http://{address}/api/me"))
        .send()
        .await
        .unwrap();
    assert_eq!(missing.status(), reqwest::StatusCode::UNAUTHORIZED);
    assert_eq!(missing.headers()["cache-control"], "private, no-store");

    let forged = client
        .get(format!("http://{address}/api/me"))
        .bearer_auth("forged.token.signature")
        .send()
        .await
        .unwrap();
    assert_eq!(forged.status(), reqwest::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn me_accepts_only_signed_tokens_with_the_expected_issuer_audience_and_expiry() {
    let jwks = json!({ "keys": [{
        "kty": "RSA", "use": "sig", "alg": "RS256", "kid": "test-key", "e": "AQAB",
        "n": "s82ZAWaHSUDNt7uY4vhv3x8HwQeDFB1pVNPs38fTLD9vrwipg6RtFknp3otGsu1qF7_zr7_lkqvMa-uC1kGpy2Nd4I1Y_HV5hh2g03Pr18LMuUpUszwysdkDxxEqaHiYCFQ7Y_YBL0FpLBZ4sJPKBWmzds-i0QxdAbMT6Uv7m-MHPkPmaBmXmZWI8fkdUfQ_XvQcUhoTEsd1sd-PsuISKltK7w959bAF4DLcBpHKqPXbZqifT5mD2eHwEMzNgSfntNgcJGlH0SGcABs-flZjCskwvrsh6IJPhrBIJ0rZdhk0wtAhvgpJaOdUK1FAQ1kfrdP8nIE9jM8nGFyBM0yqTQ"
    }] });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let issuer = format!("http://{}/auth/v1", listener.local_addr().unwrap());
    let jwks_router = axum::Router::new().route(
        "/auth/v1/.well-known/jwks.json",
        axum::routing::get(move || {
            let keys = jwks.clone();
            async move { axum::Json(keys) }
        }),
    );
    tokio::spawn(async move { axum::serve(listener, jwks_router).await.unwrap() });
    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_millis(500))
        .connect_lazy("postgres://127.0.0.1:1/unused")
        .unwrap();
    let state = AppState {
        config: Arc::new(AppConfig {
            database_url: "postgres://127.0.0.1:1/unused".to_owned(),
            bind_addr: "127.0.0.1:0".to_owned(),
            auth_issuers: [issuer.clone(), "http://127.0.0.1:2/auth/v1".to_owned()],
        }),
        pool,
    };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, create_app(state)).await.unwrap() });

    let key =
        jsonwebtoken::EncodingKey::from_rsa_pem(include_bytes!("fixtures/auth-test.pem")).unwrap();
    let mut header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);
    header.kid = Some("test-key".to_owned());
    let client = reqwest::Client::new();
    for (claim_issuer, audience, expiry, expected_status) in [
        (
            issuer.as_str(),
            "authenticated",
            chrono::Utc::now().timestamp() + 3600,
            reqwest::StatusCode::SERVICE_UNAVAILABLE,
        ),
        (
            "http://untrusted.example/auth/v1",
            "authenticated",
            chrono::Utc::now().timestamp() + 3600,
            reqwest::StatusCode::UNAUTHORIZED,
        ),
        (
            issuer.as_str(),
            "other",
            chrono::Utc::now().timestamp() + 3600,
            reqwest::StatusCode::UNAUTHORIZED,
        ),
        (
            issuer.as_str(),
            "authenticated",
            chrono::Utc::now().timestamp() - 3600,
            reqwest::StatusCode::UNAUTHORIZED,
        ),
    ] {
        let token = jsonwebtoken::encode(
            &header,
            &json!({
                "iss": claim_issuer, "sub": "test-subject", "aud": audience,
                "role": "authenticated", "exp": expiry
            }),
            &key,
        )
        .unwrap();
        let response = client
            .get(format!("http://{address}/api/me"))
            .bearer_auth(token)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), expected_status);
        assert_eq!(response.headers()["cache-control"], "private, no-store");
    }
}

#[tokio::test]
#[ignore = "requires TEST_DATABASE_URL"]
async fn same_email_in_distinct_verified_issuers_keeps_separate_accounts() {
    let database_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let jwks = json!({ "keys": [{
        "kty": "RSA", "use": "sig", "alg": "RS256", "kid": "test-key", "e": "AQAB",
        "n": "s82ZAWaHSUDNt7uY4vhv3x8HwQeDFB1pVNPs38fTLD9vrwipg6RtFknp3otGsu1qF7_zr7_lkqvMa-uC1kGpy2Nd4I1Y_HV5hh2g03Pr18LMuUpUszwysdkDxxEqaHiYCFQ7Y_YBL0FpLBZ4sJPKBWmzds-i0QxdAbMT6Uv7m-MHPkPmaBmXmZWI8fkdUfQ_XvQcUhoTEsd1sd-PsuISKltK7w959bAF4DLcBpHKqPXbZqifT5mD2eHwEMzNgSfntNgcJGlH0SGcABs-flZjCskwvrsh6IJPhrBIJ0rZdhk0wtAhvgpJaOdUK1FAQ1kfrdP8nIE9jM8nGFyBM0yqTQ"
    }] });
    let mut issuers = Vec::new();
    for _ in 0..2 {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let issuer = format!("http://{}/auth/v1", listener.local_addr().unwrap());
        let keys = jwks.clone();
        let router = axum::Router::new().route(
            "/auth/v1/.well-known/jwks.json",
            axum::routing::get(move || {
                let keys = keys.clone();
                async move { axum::Json(keys) }
            }),
        );
        tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
        issuers.push(issuer);
    }

    let state = AppState {
        config: Arc::new(AppConfig {
            database_url,
            bind_addr: "127.0.0.1:0".to_owned(),
            auth_issuers: [issuers[0].clone(), issuers[1].clone()],
        }),
        pool,
    };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, create_app(state)).await.unwrap() });

    let key =
        jsonwebtoken::EncodingKey::from_rsa_pem(include_bytes!("fixtures/auth-test.pem")).unwrap();
    let mut header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);
    header.kid = Some("test-key".to_owned());
    let subject = uuid::Uuid::new_v4().to_string();
    let sign = |issuer: &str| {
        jsonwebtoken::encode(
            &header,
            &json!({
                "iss": issuer,
                "sub": subject,
                "aud": "authenticated",
                "role": "authenticated",
                "email": "same@example.test",
                "exp": chrono::Utc::now().timestamp() + 3600
            }),
            &key,
        )
        .unwrap()
    };
    let client = reqwest::Client::new();
    let get_id = |token: String| {
        let client = client.clone();
        async move {
            let response = client
                .get(format!("http://{address}/api/me"))
                .bearer_auth(token)
                .send()
                .await
                .unwrap();
            assert_eq!(response.status(), reqwest::StatusCode::OK);
            assert_eq!(response.headers()["cache-control"], "private, no-store");
            response.json::<serde_json::Value>().await.unwrap()["data"]["id"]
                .as_str()
                .unwrap()
                .to_owned()
        }
    };
    let google = sign(&issuers[0]);
    let github = sign(&issuers[1]);
    let google_id = get_id(google.clone()).await;
    let github_id = get_id(github).await;
    assert_ne!(google_id, github_id);
    assert_eq!(google_id, get_id(google).await);

    for claims in [
        json!({"iss": "http://untrusted.example/auth/v1", "sub": subject, "aud": "authenticated", "role": "authenticated", "exp": chrono::Utc::now().timestamp() + 3600}),
        json!({"iss": issuers[0], "sub": subject, "aud": "authenticated", "role": "authenticated", "exp": chrono::Utc::now().timestamp() - 3600}),
        json!({"iss": issuers[0], "sub": subject, "aud": "other", "role": "authenticated", "exp": chrono::Utc::now().timestamp() + 3600}),
    ] {
        let token = jsonwebtoken::encode(&header, &claims, &key).unwrap();
        let response = client
            .get(format!("http://{address}/api/me"))
            .bearer_auth(token)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);
    }
}
