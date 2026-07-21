
## 2024-05-24 - [Information Leakage via Unmasked Internal Errors]
**Vulnerability:** Internal system errors, specifically SQLite database errors from `sqlx` and QR generation errors from `qrcode`, were formatted into the HTTP response error messages and exposed to the end user.
**Learning:** This repo has a custom error type `AppError::internal` that accepts a string which it includes in the response body. If the application forwards the exact internal error text (like `format!("Database error: {e}")`) to `AppError::internal`, it will leak sensitive infrastructure details.
**Prevention:** Always log the internal `e` string using `tracing::error!` on the server side and return a generic generic user-friendly string (e.g., "Internal server error") when returning an internal error back to the client.

## 2024-05-18 - Information Disclosure via Database Errors
**Vulnerability:** Raw database error strings (e.g., SQLite constraint violations or SQL syntax errors) were being exposed directly to the client in the `/api/v1/links` JSON error responses (using `internal(&format!("database error: {db_err}"))`).
**Learning:** Returning unmasked database errors provides an attacker with insights into database schema, execution state, or backend logic, facilitating further attacks like SQL injection.
**Prevention:** Catch and log all internal database errors on the server side using the `tracing` crate, and return a generic error message (e.g., "internal server error") to the client.
