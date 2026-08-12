
## 2024-05-24 - [Information Leakage via Unmasked Internal Errors]
**Vulnerability:** Internal system errors, specifically SQLite database errors from `sqlx` and QR generation errors from `qrcode`, were formatted into the HTTP response error messages and exposed to the end user.
**Learning:** This repo has a custom error type `AppError::internal` that accepts a string which it includes in the response body. If the application forwards the exact internal error text (like `format!("Database error: {e}")`) to `AppError::internal`, it will leak sensitive infrastructure details.
**Prevention:** Always log the internal `e` string using `tracing::error!` on the server side and return a generic generic user-friendly string (e.g., "Internal server error") when returning an internal error back to the client.

## 2024-05-24 - [DoS via Unbounded QR Code Generation]
**Vulnerability:** A Denial of Service (DoS) vulnerability existed in the `/qr/:code` endpoint (`src/handlers.rs`). The `code` path parameter was passed directly to the QR code generation library without any length or format validation. An attacker could send excessively large strings, causing CPU and memory exhaustion during QR code rendering.
**Learning:** In axum, path parameters like `Path(code): Path<String>` are conceptually unbounded unless explicitly validated in the handler. Operations whose computational complexity scales with input size (like rendering a QR code or hashing) must validate inputs before processing.
**Prevention:** Always validate path and query parameters before using them in expensive operations. Reused existing custom validation logic (`validate_custom_code`) to limit the short code to a maximum of 32 characters, matching the backend constraints used during creation.

## 2024-08-12 - Information Leakage in API
**Vulnerability:** Internal database errors were being returned directly to the client in JSON responses (e.g., `internal(&format!("database error: {e}"))`).
**Learning:** While the UI handlers correctly masked database errors and used `tracing::error!` to log them, the API endpoints directly embedded the raw SQLx error details into the client response payload. This exposes internal table schemas and database structures to external users.
**Prevention:** Always mask underlying implementation errors with generic messages for external clients and use server-side logging for the actual error context.
