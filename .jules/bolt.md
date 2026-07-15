## 2024-07-01 - Avoid cargo fix / cargo clippy --fix
**Learning:** Running `cargo fix` and `cargo clippy --fix` globally can introduce unstable features (like `let_chains`) or modify files outside the scope of a targeted optimization, violating the directive to make ONE small improvement.
**Action:** Only make targeted manual changes for performance improvements. Don't run automated fixers that modify unrelated files. Avoid adding new dependencies unless absolutely necessary.

## 2024-11-20 - Single-pass HTML Template Rendering
**Learning:** Chaining multiple `.replace()` calls on a string (e.g., when rendering HTML templates) results in multiple intermediate String allocations, causing a performance overhead. In a Rust web server handling template strings manually, this can be a bottleneck.
**Action:** Use a single-pass rendering approach instead of chained `.replace()` calls. A custom function (`render_template` using `String::with_capacity` and a `while` loop) replaces variables in one pass, effectively reducing memory allocations and dramatically improving performance (benchmarked ~2x speedup).
