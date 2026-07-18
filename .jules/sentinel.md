
## 2024-05-24 - [Information Leakage via Unmasked Internal Errors]
**Vulnerability:** Internal system errors, specifically SQLite database errors from `sqlx` and QR generation errors from `qrcode`, were formatted into the HTTP response error messages and exposed to the end user.
**Learning:** This repo has a custom error type `AppError::internal` that accepts a string which it includes in the response body. If the application forwards the exact internal error text (like `format!("Database error: {e}")`) to `AppError::internal`, it will leak sensitive infrastructure details.
**Prevention:** Always log the internal `e` string using `tracing::error!` on the server side and return a generic generic user-friendly string (e.g., "Internal server error") when returning an internal error back to the client.

## 2024-05-25 - [IP Spoofing via Untrusted X-Forwarded-For Header]
**Vulnerability:** The application was using the `X-Forwarded-For` HTTP header to determine the client's IP address for click logging without verifying if the request came from a trusted proxy. Malicious actors could easily spoof this header to log fake IP addresses.
**Learning:** In a direct-to-internet deployment or when relying on axum's `ConnectInfo`, untrusted proxy headers like `X-Forwarded-For` should not be used as the source of truth for the client IP unless the application is explicitly configured to trust the immediate upstream proxy.
**Prevention:** Always use the physical TCP connection socket address (`addr.ip().to_string()`) obtained via `axum::extract::ConnectInfo<SocketAddr>` to retrieve the actual client IP, preventing IP spoofing.
