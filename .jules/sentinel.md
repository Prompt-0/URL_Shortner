
## 2024-05-24 - [Information Leakage via Unmasked Internal Errors]
**Vulnerability:** Internal system errors, specifically SQLite database errors from `sqlx` and QR generation errors from `qrcode`, were formatted into the HTTP response error messages and exposed to the end user.
**Learning:** This repo has a custom error type `AppError::internal` that accepts a string which it includes in the response body. If the application forwards the exact internal error text (like `format!("Database error: {e}")`) to `AppError::internal`, it will leak sensitive infrastructure details.
**Prevention:** Always log the internal `e` string using `tracing::error!` on the server side and return a generic generic user-friendly string (e.g., "Internal server error") when returning an internal error back to the client.

## 2024-07-24 - Information Disclosure in API Error Responses
**Vulnerability:** API endpoints returned raw database errors directly to the client via `format!("database error: {e}")`, leaking database structure and internal state.
**Learning:** The UI handlers correctly masked database errors, but the API layer did not. This reveals an architectural inconsistency where different boundaries (UI vs API) handle internal errors differently, increasing the risk of information disclosure.
**Prevention:** Always use generic error responses for external clients (e.g., `AppError::internal("Internal server error")`) and rely on `tracing::error!` for secure, server-side logging of raw exception details. Implement unified error mapping logic across both API and UI bounds.
