use sqlx::PgPool;
use uuid::Uuid;

use crate::infrastructure::auth::AuthIdentity;

pub async fn get_or_create_account(
    pool: &PgPool,
    identity: &AuthIdentity,
) -> Result<Uuid, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let candidate = Uuid::new_v4();
    // Keep the new account and identity atomic. If the identity already exists,
    // rolling back also discards the unused candidate account.
    let inserted = sqlx::query_scalar::<_, Uuid>(
        "WITH inserted_account AS (INSERT INTO accounts (id) VALUES ($1) RETURNING id), \
         inserted_identity AS (INSERT INTO account_identities (issuer, subject, account_id) \
         SELECT $2, $3, id FROM inserted_account ON CONFLICT (issuer, subject) DO NOTHING RETURNING account_id) \
         SELECT account_id FROM inserted_identity"
    ).bind(candidate).bind(&identity.issuer).bind(&identity.subject)
        .fetch_optional(&mut *tx).await?;
    if let Some(id) = inserted {
        tx.commit().await?;
        return Ok(id);
    }
    tx.rollback().await?;
    sqlx::query_scalar::<_, Uuid>(
        "SELECT account_id FROM account_identities WHERE issuer = $1 AND subject = $2",
    )
    .bind(&identity.issuer)
    .bind(&identity.subject)
    .fetch_one(pool)
    .await
}
