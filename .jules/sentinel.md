
## 2024-05-24 - [Information Leakage via Unmasked Internal Errors]
**Vulnerability:** Internal system errors, specifically SQLite database errors from `sqlx` and QR generation errors from `qrcode`, were formatted into the HTTP response error messages and exposed to the end user.
**Learning:** This repo has a custom error type `AppError::internal` that accepts a string which it includes in the response body. If the application forwards the exact internal error text (like `format!("Database error: {e}")`) to `AppError::internal`, it will leak sensitive infrastructure details.
**Prevention:** Always log the internal `e` string using `tracing::error!` on the server side and return a generic generic user-friendly string (e.g., "Internal server error") when returning an internal error back to the client.
## 2024-07-30 - [Inconsistent Error Handling Exposing Internal Database Details]
**Vulnerability:** API routes (`src/api.rs`) returned raw SQLite database errors to clients upon database failure, whereas UI handlers correctly masked these.
**Learning:** Found a vulnerability pattern of inconsistent error handling across web and API boundaries. While UI error responses were secure, the JSON API boundary leaked internal data via generic formatting (`format!("database error: {e}")`).
**Prevention:** Apply consistent secure error handling across all boundaries (both UI and API). Always log the raw internal error server-side via `tracing::error!` and send a generic "internal server error" message to clients, regardless of the response format (HTML or JSON).
