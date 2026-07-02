## 2024-07-01 - Avoid cargo fix / cargo clippy --fix
**Learning:** Running `cargo fix` and `cargo clippy --fix` globally can introduce unstable features (like `let_chains`) or modify files outside the scope of a targeted optimization, violating the directive to make ONE small improvement.
**Action:** Only make targeted manual changes for performance improvements. Don't run automated fixers that modify unrelated files. Avoid adding new dependencies unless absolutely necessary.
## 2024-07-02 - Chained `.replace()` memory allocations
**Learning:** In Rust, chained `.replace()` calls on strings result in a new memory allocation for each step. For HTML templates with multiple substitutions, this is an O(n * m) allocation bottleneck.
**Action:** Replaced chained `.replace()` calls with a single-pass `render_template` utility function that writes to a pre-allocated `String` buffer. This prevents multiple memory allocations and was benchmarked to perform ~4-5x faster. Added the utility to `src/utils.rs` and updated `src/handlers.rs`. Also discovered that `cargo fmt` should be run to ensure formatting remains consistent.
