
## 2024-05-24 - [Information Leakage via Unmasked Internal Errors]
**Vulnerability:** Internal system errors, specifically SQLite database errors from `sqlx` and QR generation errors from `qrcode`, were formatted into the HTTP response error messages and exposed to the end user.
**Learning:** This repo has a custom error type `AppError::internal` that accepts a string which it includes in the response body. If the application forwards the exact internal error text (like `format!("Database error: {e}")`) to `AppError::internal`, it will leak sensitive infrastructure details.
**Prevention:** Always log the internal `e` string using `tracing::error!` on the server side and return a generic generic user-friendly string (e.g., "Internal server error") when returning an internal error back to the client.

## 2024-07-28 - Database Error Information Disclosure in API endpoints
**Vulnerability:** Internal database error messages (including SQL execution details) were being returned verbatim to the client in JSON responses (e.g., `internal(&format!("database error: {db_err}"))`) within `src/api.rs`.
**Learning:** Returning unmasked output stringified directly from standard errors poses an information disclosure risk, as it could reveal schema structure, backend configuration details, and system states to an attacker.
**Prevention:** Avoid formatting server-side errors into user-facing HTTP responses. Instead, log the detailed error internally (`tracing::error!`) and provide a sanitized, generic error string (e.g., `"internal server error"`) to the client.
