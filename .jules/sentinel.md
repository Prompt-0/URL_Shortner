
## 2024-05-24 - [Information Leakage via Unmasked Internal Errors]
**Vulnerability:** Internal system errors, specifically SQLite database errors from `sqlx` and QR generation errors from `qrcode`, were formatted into the HTTP response error messages and exposed to the end user.
**Learning:** This repo has a custom error type `AppError::internal` that accepts a string which it includes in the response body. If the application forwards the exact internal error text (like `format!("Database error: {e}")`) to `AppError::internal`, it will leak sensitive infrastructure details.
**Prevention:** Always log the internal `e` string using `tracing::error!` on the server side and return a generic generic user-friendly string (e.g., "Internal server error") when returning an internal error back to the client.

## 2024-05-24 - [DoS via Unbounded QR Code Generation]
**Vulnerability:** A Denial of Service (DoS) vulnerability existed in the `/qr/:code` endpoint (`src/handlers.rs`). The `code` path parameter was passed directly to the QR code generation library without any length or format validation. An attacker could send excessively large strings, causing CPU and memory exhaustion during QR code rendering.
**Learning:** In axum, path parameters like `Path(code): Path<String>` are conceptually unbounded unless explicitly validated in the handler. Operations whose computational complexity scales with input size (like rendering a QR code or hashing) must validate inputs before processing.
**Prevention:** Always validate path and query parameters before using them in expensive operations. Reused existing custom validation logic (`validate_custom_code`) to limit the short code to a maximum of 32 characters, matching the backend constraints used during creation.

## 2024-05-24 - [DoS via Unbounded Path Parameters in Database/Cache Lookups]
**Vulnerability:** A Denial of Service (DoS) vulnerability existed in `redirect_short_link`, `stats` (`src/handlers.rs`), and `get_link` (`src/api.rs`) endpoints. The `code` path parameter was passed directly to the cache or database lookup without length validation. An attacker could send large, arbitrary strings to these endpoints, causing unnecessary cache misses, database queries, and potential performance degradation.
**Learning:** Operations whose computational complexity scales with input size, or stateful operations like database/cache lookups, must validate inputs before processing to prevent resource exhaustion and unnecessary load.
**Prevention:** Always validate path parameters before using them in lookups. Reused existing custom validation logic (`validate_custom_code`) to limit the short code to a maximum of 32 characters, matching the backend constraints used during creation.
