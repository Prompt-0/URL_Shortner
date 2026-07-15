## 2024-05-18 - Fixed IP Spoofing Vulnerability
**Vulnerability:** IP spoofing via `X-Forwarded-For` header in `src/handlers.rs`.
**Learning:** The application was trusting the client-provided `X-Forwarded-For` HTTP header instead of using the verifiable TCP connection peer socket address, allowing a malicious actor to spoof IP addresses.
**Prevention:** Never trust user-provided headers for IP logging without using a trusted proxy configuration. Directly use `addr.ip().to_string()` for strict verification.
