## 2024-05-24 - [Avoid intermediate UUID string allocations]
**Learning:** `Uuid::new_v4().simple().to_string()` performs a heap allocation for the entire 32-character hex string. When only a substring is needed (e.g. for generating short codes), this is wasteful.
**Action:** Use `Uuid::encode_buffer()` to allocate a buffer on the stack, and render the UUID into it via `.encode_lower(&mut buf)`. This avoids the intermediate String allocation entirely.
