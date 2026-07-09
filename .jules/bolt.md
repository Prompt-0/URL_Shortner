## 2024-07-01 - Avoid cargo fix / cargo clippy --fix
**Learning:** Running `cargo fix` and `cargo clippy --fix` globally can introduce unstable features (like `let_chains`) or modify files outside the scope of a targeted optimization, violating the directive to make ONE small improvement.
**Action:** Only make targeted manual changes for performance improvements. Don't run automated fixers that modify unrelated files. Avoid adding new dependencies unless absolutely necessary.
## 2026-07-09 - Single-pass HTML Template Rendering
**Learning:** Chaining `.replace()` calls on embedded HTML strings causes unnecessary heap allocations per substitution. Rust's string building in single passes using pre-allocated capacity significantly cuts memory overhead for templating.
**Action:** Use a single-pass rendering function, like `render_template`, to avoid intermediate allocations when interpolating variables into large template strings.
