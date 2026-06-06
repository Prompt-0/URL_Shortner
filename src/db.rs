use crate::models::LinkRecord;
use sqlx::{Row, SqlitePool};

pub async fn insert_link(
    pool: &SqlitePool,
    code: &str,
    original_url: &str,
    created_at: &str,
    expires_at: Option<&str>,
    password: Option<&str>,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"
        INSERT INTO links (code, original_url, created_at, clicks, expires_at, password)
        VALUES (?1, ?2, ?3, 0, ?4, ?5)
        "#,
    )
    .bind(code)
    .bind(original_url)
    .bind(created_at)
    .bind(expires_at)
    .bind(password)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn fetch_link(pool: &SqlitePool, code: &str) -> Result<Option<LinkRecord>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT code, original_url, created_at, clicks, expires_at, password
        FROM links
        WHERE code = ?1
        "#,
    )
    .bind(code)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|row| LinkRecord {
        code: row.get("code"),
        original_url: row.get("original_url"),
        created_at: row.get("created_at"),
        clicks: row.get("clicks"),
        expires_at: row.get("expires_at"),
        password: row.get("password"),
    }))
}

pub async fn log_click(
    pool: &SqlitePool,
    code: &str,
    user_agent: Option<&str>,
    referer: Option<&str>,
    ip_address: Option<&str>,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"
        UPDATE links
        SET clicks = clicks + 1
        WHERE code = ?1
        "#,
    )
    .bind(code)
    .execute(&mut *tx)
    .await?;

    let clicked_at = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO clicks (code, user_agent, referer, ip_address, clicked_at)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
    )
    .bind(code)
    .bind(user_agent)
    .bind(referer)
    .bind(ip_address)
    .bind(clicked_at)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}
