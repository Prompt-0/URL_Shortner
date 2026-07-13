use crate::{
    db,
    error::{AppError, AppResult},
    models::ShortenForm,
    services,
    state::AppState,
    ui,
    utils::{escape_html, normalize_url, validate_custom_code},
};
use axum::{
    extract::{ConnectInfo, Form, Path, State},
    http::HeaderMap,
    response::{Html, Redirect},
};
use std::net::SocketAddr;

pub async fn healthz() -> &'static str {
    "ok"
}

pub async fn home() -> Html<&'static str> {
    Html(ui::HOME_HTML)
}

pub async fn shorten(
    State(state): State<AppState>,
    Form(input): Form<ShortenForm>,
) -> AppResult<Html<String>> {
    let original_url = normalize_url(&input.url)
        .map_err(|e| AppError::bad_request(format!("Invalid URL: {e}")))?;

    if let Some(code) = input.custom_code.as_deref() {
        validate_custom_code(code).map_err(|e| AppError::bad_request(e))?;
    }

    let record = services::create_link(
        &state.pool, 
        &original_url, 
        input.custom_code.as_deref(),
        input.expires_at.as_deref(),
        input.password.as_deref()
    )
        .await
        .map_err(|err| match err {
            services::CreateLinkError::DuplicateCode => {
                AppError::conflict("That custom code is already taken.")
            }
            services::CreateLinkError::Exhausted => AppError::service_unavailable(
                "Could not allocate a unique short code. Please try again.",
            ),
            services::CreateLinkError::Database(db_err) => {
                tracing::error!("Database error: {}", db_err);
                AppError::internal("Internal server error")
            }
        })?;

    let short_url = format!("{}/{}", state.base_url.trim_end_matches('/'), record.code);
    let stats_url = format!("/stats/{}", record.code);

    let body = ui::SUCCESS_HTML_TEMPLATE
        .replace("{code}", &record.code)
        .replace("{short_url}", &short_url)
        .replace("{stats_url}", &stats_url)
        .replace("{original_url}", &escape_html(&record.original_url));

    Ok(Html(body))
}

pub async fn redirect_short_link(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Path(code): Path<String>,
) -> AppResult<Redirect> {
    let original_url = if let Some(url) = state.cache.get(&code).await {
        url
    } else {
        let Some(link) = db::fetch_link(&state.pool, &code)
            .await
            .map_err(|e| {
                tracing::error!("Database error: {}", e);
                AppError::internal("Internal server error")
            })?
        else {
            return Err(AppError::not_found("Short link not found"));
        };

        if let Some(expires) = &link.expires_at {
            if let Ok(exp_time) = chrono::DateTime::parse_from_rfc3339(expires) {
                if chrono::Utc::now() > exp_time {
                    return Err(AppError::not_found("This link has expired"));
                }
            }
        }
        
        if link.password.is_some() {
            // Password protection UI is pending. 
            // For now, prevent caching and redirecting to original url silently.
            return Err(AppError::bad_request("This link is password protected. UI pending."));
        }
        
        state.cache.insert(code.clone(), link.original_url.clone()).await;
        link.original_url
    };

    let pool = state.pool.clone();
    let code_clone = code.clone();
    
    let user_agent = headers.get(axum::http::header::USER_AGENT).and_then(|v| v.to_str().ok()).map(|s| s.to_string());
    let referer = headers.get(axum::http::header::REFERER).and_then(|v| v.to_str().ok()).map(|s| s.to_string());
    let ip = headers.get("X-Forwarded-For").and_then(|v| v.to_str().ok()).map(|s| s.to_string()).unwrap_or_else(|| addr.ip().to_string());

    tokio::spawn(async move {
        let _ = db::log_click(&pool, &code_clone, user_agent.as_deref(), referer.as_deref(), Some(&ip)).await;
    });

    Ok(Redirect::temporary(&original_url))
}

pub async fn stats(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> AppResult<Html<String>> {
    let Some(link) = db::fetch_link(&state.pool, &code)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            AppError::internal("Internal server error")
        })?
    else {
        return Err(AppError::not_found("Short link not found"));
    };

    let short_url = format!("{}/{}", state.base_url.trim_end_matches('/'), link.code);
    let stats_url = format!("/stats/{}", link.code);

    let html = ui::STATS_HTML_TEMPLATE
        .replace("{code}", &escape_html(&link.code))
        .replace("{short_url}", &escape_html(&short_url))
        .replace("{stats_url}", &escape_html(&stats_url))
        .replace("{original_url}", &escape_html(&link.original_url))
        .replace("{created_at}", &escape_html(&link.created_at))
        .replace("{clicks}", &link.clicks.to_string());

    Ok(Html(html))
}

pub async fn qr_code(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> AppResult<impl axum::response::IntoResponse> {
    use qrcode::QrCode;
    use qrcode::render::svg;

    let short_url = format!("{}/{}", state.base_url.trim_end_matches('/'), code);
    
    let qr = QrCode::new(short_url.as_bytes())
        .map_err(|e| {
            tracing::error!("QR generation error: {}", e);
            AppError::internal("Internal server error")
        })?;
        
    let image = qr.render::<svg::Color>()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#0b0f19"))
        .light_color(svg::Color("#f8fafc"))
        .build();
        
    Ok((
        [(axum::http::header::CONTENT_TYPE, "image/svg+xml")],
        image
    ))
}
