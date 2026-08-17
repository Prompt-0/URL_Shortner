
## 2024-05-24 - [Information Leakage via Unmasked Internal Errors]
**Vulnerability:** Internal system errors, specifically SQLite database errors from `sqlx` and QR generation errors from `qrcode`, were formatted into the HTTP response error messages and exposed to the end user.
**Learning:** This repo has a custom error type `AppError::internal` that accepts a string which it includes in the response body. If the application forwards the exact internal error text (like `format!("Database error: {e}")`) to `AppError::internal`, it will leak sensitive infrastructure details.
**Prevention:** Always log the internal `e` string using `tracing::error!` on the server side and return a generic generic user-friendly string (e.g., "Internal server error") when returning an internal error back to the client.

## 2024-05-24 - [DoS via Unbounded QR Code Generation]
**Vulnerability:** A Denial of Service (DoS) vulnerability existed in the `/qr/:code` endpoint (`src/handlers.rs`). The `code` path parameter was passed directly to the QR code generation library without any length or format validation. An attacker could send excessively large strings, causing CPU and memory exhaustion during QR code rendering.
**Learning:** In axum, path parameters like `Path(code): Path<String>` are conceptually unbounded unless explicitly validated in the handler. Operations whose computational complexity scales with input size (like rendering a QR code or hashing) must validate inputs before processing.
**Prevention:** Always validate path and query parameters before using them in expensive operations. Reused existing custom validation logic (`validate_custom_code`) to limit the short code to a maximum of 32 characters, matching the backend constraints used during creation.
## 2024-08-17 - Authorization Bypass in Cache
**Vulnerability:** The application was caching the raw `original_url` string instead of the entire `LinkRecord`. As a result, when resolving short links via cache hits, the backend bypassed `expires_at` checks and `password` protection validations, exposing private and expired links.
**Learning:** Caching raw attributes instead of domain objects can easily lead to authorization and validation bypasses because the intermediate checks normally enforced before the attribute is extracted are completely skipped on cache hits.
**Prevention:** Always cache complete domain objects (`LinkRecord`) and ensure authorization/expiration validations execute uniformly on both cache hits and database fetches before extracting raw values to return to clients.
