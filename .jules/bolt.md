## 2024-07-01 - Avoid cargo fix / cargo clippy --fix
**Learning:** Running `cargo fix` and `cargo clippy --fix` globally can introduce unstable features (like `let_chains`) or modify files outside the scope of a targeted optimization, violating the directive to make ONE small improvement.
**Action:** Only make targeted manual changes for performance improvements. Don't run automated fixers that modify unrelated files. Avoid adding new dependencies unless absolutely necessary.
## 2024-07-08 - Single-pass HTML Template Rendering
**Learning:** Chained `.replace()` calls on large HTML template strings cause multiple unnecessary memory allocations and redundant string traversals. This was identified as a performance bottleneck in `src/handlers.rs`.
**Action:** Replaced chained `.replace()` calls with a custom `render_template` utility function in `src/utils.rs` that performs single-pass string building with pre-allocated capacity. Ensure future string template substitutions avoid chained replaces.
