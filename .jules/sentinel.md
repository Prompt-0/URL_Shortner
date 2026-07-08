## 2024-07-08 - Information Disclosure via Error Messages
**Vulnerability:** The application was directly returning raw database errors (e.g., `format!("Database error: {e}")`) and QR generation errors to the client in HTTP responses, causing information disclosure of backend system details and internal states.
**Learning:** Returning unmasked raw internal errors to users reveals sensitive infrastructure details, which can be useful to an attacker mapping the system for further exploitation.
**Prevention:** Always mask internal errors by logging the actual error string on the server side (e.g., via `tracing::error!`) and returning a generic, safe error message like "Internal server error" to the client.
