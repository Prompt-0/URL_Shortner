
## 2024-05-24 - [Information Leakage via Unmasked Internal Errors]
**Vulnerability:** Internal system errors, specifically SQLite database errors from `sqlx` and QR generation errors from `qrcode`, were formatted into the HTTP response error messages and exposed to the end user.
**Learning:** This repo has a custom error type `AppError::internal` that accepts a string which it includes in the response body. If the application forwards the exact internal error text (like `format!("Database error: {e}")`) to `AppError::internal`, it will leak sensitive infrastructure details.
**Prevention:** Always log the internal `e` string using `tracing::error!` on the server side and return a generic generic user-friendly string (e.g., "Internal server error") when returning an internal error back to the client.

## 2024-05-27 - [Information Leakage via Unmasked Internal Errors in API]
**Vulnerability:** Internal system errors, specifically SQLite database errors from `sqlx`, were formatted into the HTTP JSON response error messages in the API (`src/api.rs`) and exposed to the end user.
**Learning:** This repo has a `ApiErrorResponse` returned by the API endpoints. If the application forwards the exact internal error text (like `format!("database error: {e}")`) to `internal(...)`, it will leak sensitive infrastructure details via the API, similar to the UI endpoints.
**Prevention:** Always log the internal `e` string using `tracing::error!` on the server side and return a generic user-friendly string (e.g., "internal server error") when returning an internal error back to the client via the API endpoints.
