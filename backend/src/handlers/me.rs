use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header, jwk::JwkSet};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{bootstrap::state::AppState, handlers::common::response::ApiResponse};

#[derive(Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    role: String,
}

#[derive(Serialize)]
pub struct Account {
    id: Uuid,
}

type MeResponse = (StatusCode, Json<ApiResponse<Account>>);

fn error(status: StatusCode, code: &'static str, message: &str) -> MeResponse {
    (
        status,
        Json(ApiResponse {
            success: false,
            data: None,
            error: Some(crate::handlers::common::response::ApiErrorBody {
                code,
                message: message.to_owned(),
            }),
        }),
    )
}

pub async fn me(State(state): State<AppState>, headers: HeaderMap) -> MeResponse {
    let Some(token) = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
    else {
        return error(
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "A bearer token is required",
        );
    };
    let Ok(header) = decode_header(token) else {
        return error(
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "Invalid bearer token",
        );
    };
    if !matches!(header.alg, Algorithm::RS256 | Algorithm::ES256) {
        return error(
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "Unsupported token algorithm",
        );
    }
    let mut identity = None;
    let client = reqwest::Client::new();
    for issuer in &state.config.auth_issuers {
        let response = client
            .get(format!("{issuer}/.well-known/jwks.json"))
            .send()
            .await;
        let Ok(response) = response else { continue };
        let Ok(keys) = response.json::<JwkSet>().await else {
            continue;
        };
        for key in keys.keys {
            if key.common.key_id.as_deref() != header.kid.as_deref() {
                continue;
            }
            let Ok(decoding_key) = DecodingKey::from_jwk(&key) else {
                continue;
            };
            let mut validation = Validation::new(header.alg);
            validation.set_audience(&["authenticated"]);
            validation.set_issuer(&[issuer.as_str()]);
            let Ok(data) = decode::<Claims>(token, &decoding_key, &validation) else {
                continue;
            };
            if data.claims.role == "authenticated"
                && data.claims.iss == *issuer
                && !data.claims.sub.is_empty()
            {
                identity = Some((issuer.as_str(), data.claims.sub));
                break;
            }
        }
        if identity.is_some() {
            break;
        }
    }
    let Some((issuer, subject)) = identity else {
        return error(
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "Invalid or expired session",
        );
    };
    let mut tx = match state.pool.begin().await {
        Ok(tx) => tx,
        Err(_) => {
            return error(
                StatusCode::SERVICE_UNAVAILABLE,
                "database_unavailable",
                "Account service unavailable",
            );
        }
    };
    let id = Uuid::new_v4();
    // The identity row is the only account lookup key. A duplicate login never creates
    // another account, even when both requests arrive concurrently.
    let account_id = sqlx::query_scalar::<_, Uuid>(
        "WITH inserted_account AS (INSERT INTO accounts (id) VALUES ($1) RETURNING id), \
         inserted_identity AS (INSERT INTO account_identities (issuer, subject, account_id) \
         SELECT $2, $3, id FROM inserted_account ON CONFLICT (issuer, subject) DO NOTHING RETURNING account_id) \
         SELECT account_id FROM inserted_identity"
    ).bind(id).bind(issuer).bind(&subject).fetch_optional(&mut *tx).await;
    let account_id = match account_id {
        Ok(Some(id)) => id,
        Ok(None) => {
            if tx.rollback().await.is_err() {
                return error(
                    StatusCode::SERVICE_UNAVAILABLE,
                    "database_unavailable",
                    "Account service unavailable",
                );
            }
            match sqlx::query_scalar::<_, Uuid>(
                "SELECT account_id FROM account_identities WHERE issuer = $1 AND subject = $2",
            )
            .bind(issuer)
            .bind(&subject)
            .fetch_one(&state.pool)
            .await
            {
                Ok(id) => return (StatusCode::OK, Json(ApiResponse::ok(Account { id }))),
                Err(_) => {
                    return error(
                        StatusCode::SERVICE_UNAVAILABLE,
                        "database_unavailable",
                        "Account service unavailable",
                    );
                }
            }
        }
        Err(_) => {
            return error(
                StatusCode::SERVICE_UNAVAILABLE,
                "database_unavailable",
                "Account service unavailable",
            );
        }
    };
    if tx.commit().await.is_err() {
        return error(
            StatusCode::SERVICE_UNAVAILABLE,
            "database_unavailable",
            "Account service unavailable",
        );
    }
    (
        StatusCode::OK,
        Json(ApiResponse::ok(Account { id: account_id })),
    )
}
