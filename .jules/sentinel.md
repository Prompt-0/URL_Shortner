## 2024-03-24 - [Information Leakage via AppError::internal]
**Vulnerability:** The application was exposing sensitive internal details (e.g., database connection strings, error messages, QR generation errors) to clients by formatting the actual `Err` object into the `AppError::internal` message.
**Learning:** The `AppError::internal` method directly displays its argument to users. It was used incorrectly by wrapping low-level errors into the public message string.
**Prevention:** Always log the internal `db_err` or `e` to standard server logs (using `tracing::error!`) and return a generic "Internal server error" string to the end user for 500 status codes.
