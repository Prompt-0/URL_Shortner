use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Conflict(String),
    NotFound(String),
    ServiceUnavailable(String),
    Internal(String),
    Forbidden(String),
}

impl AppError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict(message.into())
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }

    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::ServiceUnavailable(message.into())
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::Forbidden(message.into())
    }

    fn status(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
        }
    }

    fn title(&self) -> &'static str {
        match self {
            Self::BadRequest(_) => "Bad request",
            Self::Conflict(_) => "Conflict",
            Self::NotFound(_) => "Not found",
            Self::ServiceUnavailable(_) => "Service unavailable",
            Self::Internal(_) => "Internal server error",
            Self::Forbidden(_) => "Forbidden",
        }
    }

    fn message(&self) -> &str {
        match self {
            Self::BadRequest(message)
            | Self::Conflict(message)
            | Self::NotFound(message)
            | Self::ServiceUnavailable(message)
            | Self::Internal(message)
            | Self::Forbidden(message) => message,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status();
        let html = error_page(self.title(), self.message());
        (status, Html(html)).into_response()
    }
}

fn error_page(title: &str, message: &str) -> String {
    format!(
        r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{}</title>
  <style>
    body {{ font-family: system-ui, sans-serif; max-width: 760px; margin: 40px auto; padding: 0 16px; line-height: 1.5; }}
    .card {{ border: 1px solid #ddd; border-radius: 16px; padding: 20px; }}
    code {{ background: #f5f5f5; padding: 2px 6px; border-radius: 6px; }}
  </style>
</head>
<body>
  <h1>{}</h1>
  <div class="card">
    <p>{}</p>
  </div>
  <p><a href="/">Back home</a></p>
</body>
</html>
"#,
        crate::utils::escape_html(title),
        crate::utils::escape_html(title),
        crate::utils::escape_html(message)
    )
}

pub type AppResult<T> = Result<T, AppError>;
