use sqlx::Error;
use url::Url;
use uuid::Uuid;

pub fn generate_code() -> String {
    let raw = Uuid::new_v4().simple().to_string();
    raw[..12].to_string()
}

pub fn normalize_url(input: &str) -> Result<String, &'static str> {
    let url = Url::parse(input).map_err(|_| "the URL format is invalid")?;

    match url.scheme() {
        "http" | "https" => Ok(url.to_string()),
        _ => Err("only http:// and https:// URLs are allowed"),
    }
}

pub fn validate_custom_code(code: &str) -> Result<(), &'static str> {
    if code.len() < 3 || code.len() > 32 {
        return Err("custom code must be between 3 and 32 characters");
    }

    if !code
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err("custom code may only contain letters, numbers, hyphens, and underscores");
    }

    Ok(())
}

pub fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub fn is_unique_violation(err: &Error) -> bool {
    matches!(
        err,
        Error::Database(db_err) if db_err.message().contains("UNIQUE constraint failed")
    )
}
