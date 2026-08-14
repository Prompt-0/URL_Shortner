
## 2024-05-24 - [Information Leakage via Unmasked Internal Errors]
**Vulnerability:** Internal system errors, specifically SQLite database errors from `sqlx` and QR generation errors from `qrcode`, were formatted into the HTTP response error messages and exposed to the end user.
**Learning:** This repo has a custom error type `AppError::internal` that accepts a string which it includes in the response body. If the application forwards the exact internal error text (like `format!("Database error: {e}")`) to `AppError::internal`, it will leak sensitive infrastructure details.
**Prevention:** Always log the internal `e` string using `tracing::error!` on the server side and return a generic generic user-friendly string (e.g., "Internal server error") when returning an internal error back to the client.

## 2024-05-24 - [DoS via Unbounded QR Code Generation]
**Vulnerability:** A Denial of Service (DoS) vulnerability existed in the `/qr/:code` endpoint (`src/handlers.rs`). The `code` path parameter was passed directly to the QR code generation library without any length or format validation. An attacker could send excessively large strings, causing CPU and memory exhaustion during QR code rendering.
**Learning:** In axum, path parameters like `Path(code): Path<String>` are conceptually unbounded unless explicitly validated in the handler. Operations whose computational complexity scales with input size (like rendering a QR code or hashing) must validate inputs before processing.
**Prevention:** Always validate path and query parameters before using them in expensive operations. Reused existing custom validation logic (`validate_custom_code`) to limit the short code to a maximum of 32 characters, matching the backend constraints used during creation.

## 2024-05-24 - [Information Leakage & DoS via API Endpoints]
**Vulnerability:** Similar to the UI handlers, the programmatic API endpoints in `src/api.rs` (`create_link` and `get_link`) were returning unmasked internal database error strings to the client, leading to information leakage. Additionally, the `get_link` endpoint lacked validation on the unbounded `code` path parameter before querying the database, creating a DoS risk.
**Learning:** Security gaps often duplicate across boundaries. A vulnerability found in a web UI handler is highly likely to exist in the corresponding API endpoint if they don't share underlying middleware or validation logic.
**Prevention:** Unified validation logic (`utils::validate_custom_code`) was applied to the `get_link` API endpoint. Internal database errors in API responses were replaced with generic errors while preserving server-side logging (`tracing::error!`), adhering to the "Fail securely" principle.
