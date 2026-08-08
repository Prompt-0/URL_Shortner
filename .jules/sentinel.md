
## 2024-05-24 - [Information Leakage via Unmasked Internal Errors]
**Vulnerability:** Internal system errors, specifically SQLite database errors from `sqlx` and QR generation errors from `qrcode`, were formatted into the HTTP response error messages and exposed to the end user.
**Learning:** This repo has a custom error type `AppError::internal` that accepts a string which it includes in the response body. If the application forwards the exact internal error text (like `format!("Database error: {e}")`) to `AppError::internal`, it will leak sensitive infrastructure details.
**Prevention:** Always log the internal `e` string using `tracing::error!` on the server side and return a generic generic user-friendly string (e.g., "Internal server error") when returning an internal error back to the client.

## 2024-05-24 - [DoS via Unbounded QR Code Generation]
**Vulnerability:** A Denial of Service (DoS) vulnerability existed in the `/qr/:code` endpoint (`src/handlers.rs`). The `code` path parameter was passed directly to the QR code generation library without any length or format validation. An attacker could send excessively large strings, causing CPU and memory exhaustion during QR code rendering.
**Learning:** In axum, path parameters like `Path(code): Path<String>` are conceptually unbounded unless explicitly validated in the handler. Operations whose computational complexity scales with input size (like rendering a QR code or hashing) must validate inputs before processing.
**Prevention:** Always validate path and query parameters before using them in expensive operations. Reused existing custom validation logic (`validate_custom_code`) to limit the short code to a maximum of 32 characters, matching the backend constraints used during creation.

## 2024-05-24 - [Authorization Bypass via Caching]
**Vulnerability:** The Moka cache implementation (`Cache<String, String>`) was caching only the original URL string. On cache hit, this allowed users to bypass security and business logic checks (like `expires_at` checks and pending password protection) since those checks were executed before the item was cached or completely skipped on cache hits.
**Learning:** Caching raw primitives (like strings) instead of the actual data structures (like `LinkRecord`) can inadvertently bypass security checks if those checks depend on properties of the data structure (like expiration times or password requirements) that are not stored in the cache. This creates a "surprising security gap" in the architecture.
**Prevention:** Always cache the complete domain objects (e.g., `LinkRecord`) and perform business/security logic checks *after* retrieving the object from the cache (or ensure the cache eviction matches all business logic perfectly, which is harder).
