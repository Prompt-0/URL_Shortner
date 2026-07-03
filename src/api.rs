use crate::{
    error::AppError,
    models::{ApiErrorResponse, CreateLinkJsonRequest},
    services::{self, CreateLinkError},
    state::AppState,
    utils::normalize_url,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

pub async fn create_link(
    State(state): State<AppState>,
    Json(payload): Json<CreateLinkJsonRequest>,
) -> Result<Json<crate::models::LinkResponse>, (StatusCode, Json<ApiErrorResponse>)> {
    let original_url = normalize_url(&payload.url).map_err(|e| bad_request(e))?;

    if let Some(code) = payload.custom_code.as_deref() {
        crate::utils::validate_custom_code(code).map_err(|e| bad_request(e))?;
    }

    let record = services::create_link(
        &state.pool, 
        &original_url, 
        payload.custom_code.as_deref(),
        payload.expires_at.as_deref(),
        payload.password.as_deref()
    )
        .await
        .map_err(|err| match err {
            CreateLinkError::DuplicateCode => conflict("that custom code is already taken"),
            CreateLinkError::Exhausted => {
                service_unavailable("could not allocate a unique short code")
            }
            CreateLinkError::Database(db_err) => internal(&format!("database error: {db_err}")),
        })?;

    Ok(Json(record.to_response(&state.base_url)))
}

pub async fn get_link(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<crate::models::LinkResponse>, (StatusCode, Json<ApiErrorResponse>)> {
    let Some(link) = crate::db::fetch_link(&state.pool, &code)
        .await
        .map_err(|e| internal(&format!("database error: {e}")))?
    else {
        return Err(not_found("short link not found"));
    };

    if link.password.is_some() {
        return Err(unauthorized("This link is password protected."));
    }

    Ok(Json(link.to_response(&state.base_url)))
}

fn unauthorized(message: impl Into<String>) -> (StatusCode, Json<ApiErrorResponse>) {
    error(StatusCode::UNAUTHORIZED, "unauthorized", message)
}

fn bad_request(message: impl Into<String>) -> (StatusCode, Json<ApiErrorResponse>) {
    error(StatusCode::BAD_REQUEST, "bad_request", message)
}

fn conflict(message: impl Into<String>) -> (StatusCode, Json<ApiErrorResponse>) {
    error(StatusCode::CONFLICT, "conflict", message)
}

fn not_found(message: impl Into<String>) -> (StatusCode, Json<ApiErrorResponse>) {
    error(StatusCode::NOT_FOUND, "not_found", message)
}

fn service_unavailable(message: impl Into<String>) -> (StatusCode, Json<ApiErrorResponse>) {
    error(
        StatusCode::SERVICE_UNAVAILABLE,
        "service_unavailable",
        message,
    )
}

fn internal(message: impl Into<String>) -> (StatusCode, Json<ApiErrorResponse>) {
    error(
        StatusCode::INTERNAL_SERVER_ERROR,
        "internal_server_error",
        message,
    )
}

fn error(
    status: StatusCode,
    error: &'static str,
    message: impl Into<String>,
) -> (StatusCode, Json<ApiErrorResponse>) {
    (
        status,
        Json(ApiErrorResponse {
            error: error.to_string(),
            message: message.into(),
        }),
    )
}
