
## 2024-05-24 - [Information Leakage via Unmasked Internal Errors]
**Vulnerability:** Internal system errors, specifically SQLite database errors from `sqlx` and QR generation errors from `qrcode`, were formatted into the HTTP response error messages and exposed to the end user.
**Learning:** This repo has a custom error type `AppError::internal` that accepts a string which it includes in the response body. If the application forwards the exact internal error text (like `format!("Database error: {e}")`) to `AppError::internal`, it will leak sensitive infrastructure details.
**Prevention:** Always log the internal `e` string using `tracing::error!` on the server side and return a generic generic user-friendly string (e.g., "Internal server error") when returning an internal error back to the client.

## 2024-05-24 - [IP Spoofing via Untrusted X-Forwarded-For Header]
**Vulnerability:** The application was reading the `X-Forwarded-For` header to determine the client's IP address for click logging without any validation that the request was coming from a trusted reverse proxy. This allowed attackers to easily spoof their IP address by supplying a custom `X-Forwarded-For` header.
**Learning:** Blindly trusting `X-Forwarded-For` is a common vulnerability pattern when applications are developed without assuming a trusted proxy topology. This codebase needs to strictly rely on the physical TCP socket address unless specifically configured to trust a reverse proxy.
**Prevention:** Always use the physical socket address (`addr.ip()`) to identify the client's IP address. If deploying behind a reverse proxy, configure a secure IP extractor middleware that explicitly validates the proxy's IP before trusting any forwarded headers.
