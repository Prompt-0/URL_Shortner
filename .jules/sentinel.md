## 2024-07-09 - Information Disclosure via Unhandled Error Responses
**Vulnerability:** The application was leaking raw database error messages and stack traces to end-users in error responses (e.g., `format!("Database error: {e}")`).
**Learning:** This is a common pattern when quickly mapping internal errors (like `sqlx::Error`) to HTTP responses. Returning unmasked internal errors can reveal sensitive database structure, dependency versions, or query details to potential attackers.
**Prevention:** Always mask internal errors by logging them securely on the server side using `tracing::error!` and returning a generic error message (e.g., "Internal server error") to the client.
