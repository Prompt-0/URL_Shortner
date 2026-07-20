## 2024-05-24 - API Information Disclosure
**Vulnerability:** Detailed database error messages were being leaked directly to the client in API responses for `/api/links` endpoints.
**Learning:** Returning unmasked output like `format!("Database error: {e}")` via API endpoints exposes internal database structures or schema information, posing an information disclosure risk.
**Prevention:** Ensure internal application and database errors are mapped to generic response messages (e.g., 'Internal server error') while being logged securely on the server side using tools like `tracing::error!`.
