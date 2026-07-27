
## 2024-05-24 - [Information Leakage via Unmasked Internal Errors]
**Vulnerability:** Internal system errors, specifically SQLite database errors from `sqlx` and QR generation errors from `qrcode`, were formatted into the HTTP response error messages and exposed to the end user.
**Learning:** This repo has a custom error type `AppError::internal` that accepts a string which it includes in the response body. If the application forwards the exact internal error text (like `format!("Database error: {e}")`) to `AppError::internal`, it will leak sensitive infrastructure details.
**Prevention:** Always log the internal `e` string using `tracing::error!` on the server side and return a generic generic user-friendly string (e.g., "Internal server error") when returning an internal error back to the client.

## 2024-05-24 - [Information Leakage via Unmasked Internal Errors in API]
**Vulnerability:** Similar to the UI handlers, the JSON API endpoints in `src/api.rs` (`create_link` and `get_link`) were returning raw database errors in the JSON response using `internal(&format!("database error: {e}"))`.
**Learning:** The pattern of leaking sensitive infrastructure details was present across both the UI and JSON API layers. It's critical to ensure uniform error handling practices (log internal errors with `tracing::error!` and return a generic error message to the client) across all application endpoints.
**Prevention:** Apply the established error handling pattern universally and review all endpoint response generation for similar leaks.
