## 2024-05-18 - Information Disclosure via Application Errors
**Vulnerability:** Application endpoints (handlers & api) previously returned detailed database and internal server errors formatted into HTTP response strings.
**Learning:** Returning unmasked error messages such as `format!("Database error: {e}")` to the client can leak sensitive infrastructure details, table structure, or internal application states.
**Prevention:** Avoid formatting the underlying error details into the client response string. Instead, securely log the full details server-side (e.g., using `tracing::error!`) and return a generic "Internal server error" string to the user.
