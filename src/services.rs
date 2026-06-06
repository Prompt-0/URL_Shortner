use crate::{
    db,
    models::LinkRecord,
    utils::{generate_code, is_unique_violation},
};
use chrono::Utc;
use sqlx::SqlitePool;
use std::{error::Error, fmt};

const MAX_AUTO_ATTEMPTS: usize = 32;

#[derive(Debug)]
pub enum CreateLinkError {
    DuplicateCode,
    Exhausted,
    Database(sqlx::Error),
}

impl fmt::Display for CreateLinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateCode => write!(f, "that custom code is already taken"),
            Self::Exhausted => write!(f, "could not allocate a unique short code"),
            Self::Database(err) => write!(f, "{err}"),
        }
    }
}

impl Error for CreateLinkError {}

pub async fn create_link(
    pool: &SqlitePool,
    original_url: &str,
    custom_code: Option<&str>,
    expires_at: Option<&str>,
    password: Option<&str>,
) -> Result<LinkRecord, CreateLinkError> {
    let created_at = Utc::now().to_rfc3339();

    if let Some(code) = custom_code {
        return match db::insert_link(pool, code, original_url, &created_at, expires_at, password).await {
            Ok(()) => Ok(LinkRecord {
                code: code.to_string(),
                original_url: original_url.to_string(),
                created_at,
                clicks: 0,
                expires_at: expires_at.map(String::from),
                password: password.map(String::from),
            }),
            Err(err) if is_unique_violation(&err) => Err(CreateLinkError::DuplicateCode),
            Err(err) => Err(CreateLinkError::Database(err)),
        };
    }

    for _ in 0..MAX_AUTO_ATTEMPTS {
        let code = generate_code();

        match db::insert_link(pool, &code, original_url, &created_at, expires_at, password).await {
            Ok(()) => {
                return Ok(LinkRecord {
                    code,
                    original_url: original_url.to_string(),
                    created_at,
                    clicks: 0,
                    expires_at: expires_at.map(String::from),
                    password: password.map(String::from),
                });
            }
            Err(err) if is_unique_violation(&err) => continue,
            Err(err) => return Err(CreateLinkError::Database(err)),
        }
    }

    Err(CreateLinkError::Exhausted)
}
