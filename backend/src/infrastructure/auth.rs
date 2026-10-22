use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header, jwk::JwkSet};
use serde::Deserialize;

#[derive(Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    role: String,
}

pub struct AuthIdentity {
    pub issuer: String,
    pub subject: String,
}

pub async fn verify_bearer(token: &str, issuers: &[String; 2]) -> Option<AuthIdentity> {
    let header = decode_header(token).ok()?;
    if !matches!(header.alg, Algorithm::RS256 | Algorithm::ES256) {
        return None;
    }
    let client = reqwest::Client::new();
    for issuer in issuers {
        let Ok(response) = client
            .get(format!("{issuer}/.well-known/jwks.json"))
            .send()
            .await
        else {
            continue;
        };
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
                return Some(AuthIdentity {
                    issuer: issuer.clone(),
                    subject: data.claims.sub,
                });
            }
        }
    }
    None
}
