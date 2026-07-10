## 2024-07-01 - Avoid cargo fix / cargo clippy --fix
**Learning:** Running `cargo fix` and `cargo clippy --fix` globally can introduce unstable features (like `let_chains`) or modify files outside the scope of a targeted optimization, violating the directive to make ONE small improvement.
**Action:** Only make targeted manual changes for performance improvements. Don't run automated fixers that modify unrelated files. Avoid adding new dependencies unless absolutely necessary.
## 2024-07-10 - HTML Templating Optimization
**Learning:** Chained `.replace()` calls on large strings in Rust perform unnecessary intermediate allocations and copy the string repeatedly. This codebase relies heavily on dynamic string templating using placeholders like `{code}`. Replacing chained `.replace()` calls with a single-pass `render_template` custom builder yields a significant micro-optimization.
**Action:** When updating HTML embedded in strings or doing templating, always use single-pass algorithms (like `render_template`) instead of chained `.replace()` calls.
