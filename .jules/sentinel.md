## 2024-07-12 - Information Disclosure in Error Handling
**Vulnerability:** The application was exposing raw database errors (e.g. `format!("Database error: {e}")`) directly to the clients in HTTP responses in both frontend handlers and API routes.
**Learning:** This is a common pattern in Rust applications when developers use `map_err` directly mapping errors to user-facing formats, leaking schema details or internal query information, which can assist an attacker with subsequent attacks.
**Prevention:** Always log the unmasked/raw internal error to the backend server logs (e.g., using `tracing::error!`), and map the user-facing response to a generic, non-descriptive error message like "Internal server error".
