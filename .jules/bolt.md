## 2024-07-01 - Avoid cargo fix / cargo clippy --fix
**Learning:** Running `cargo fix` and `cargo clippy --fix` globally can introduce unstable features (like `let_chains`) or modify files outside the scope of a targeted optimization, violating the directive to make ONE small improvement.
**Action:** Only make targeted manual changes for performance improvements. Don't run automated fixers that modify unrelated files. Avoid adding new dependencies unless absolutely necessary.
## 2024-07-12 - Single-pass HTML Template Rendering
**Learning:** Chained `.replace()` calls on strings require allocating a completely new `String` and fully scanning the entire text on every single call. For 4-6 variables, this results in 4-6 unnecessary heap allocations per request.
**Action:** Replace chained `.replace()` calls with a single-pass `render_template` function that pre-allocates a single `String` buffer and performs replacement in one linear scan. This yields a >2x performance improvement.
