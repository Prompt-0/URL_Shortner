## 2024-07-14 - Information Disclosure via Database Errors
**Vulnerability:** The application returned raw database error messages to the client (e.g., `AppError::internal(format!("Database error: {e}"))`).
**Learning:** This exposes internal database structure and potential queries, which is a significant security risk for data leakage. The `tracing` crate is already available but was not used for these internal errors.
**Prevention:** Internal application and database errors must be logged securely on the server side using `tracing::error!` and a generic error message (e.g., 'Internal server error') should be returned to the client. Avoid returning unmasked output.
