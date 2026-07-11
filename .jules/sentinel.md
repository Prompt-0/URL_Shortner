## 2024-05-24 - [Information Disclosure via Error Messages]
**Vulnerability:** Raw database and internal application errors are being returned directly to clients via `AppError::internal(format!("Database error: {db_err}"))` or similar constructs.
**Learning:** This exposes internal database structure, connection strings, or system paths, which can aid attackers in reconnaissance and exploitation. We must ensure internal errors are logged securely and generic messages are presented to the user.
**Prevention:** Always use `tracing::error!` (or a similar server-side logging mechanism) to record the detailed error for debugging, and return a generic error message (like "Internal server error") to the client. Never map underlying system errors directly to HTTP response bodies.
