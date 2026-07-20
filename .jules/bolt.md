## 2024-07-01 - Avoid cargo fix / cargo clippy --fix
**Learning:** Running `cargo fix` and `cargo clippy --fix` globally can introduce unstable features (like `let_chains`) or modify files outside the scope of a targeted optimization, violating the directive to make ONE small improvement.
**Action:** Only make targeted manual changes for performance improvements. Don't run automated fixers that modify unrelated files. Avoid adding new dependencies unless absolutely necessary.

## 2024-11-20 - Single-pass HTML Template Rendering
**Learning:** Chaining multiple `.replace()` calls on a string (e.g., when rendering HTML templates) results in multiple intermediate String allocations, causing a performance overhead. In a Rust web server handling template strings manually, this can be a bottleneck.
**Action:** Use a single-pass rendering approach instead of chained `.replace()` calls. A custom function (`render_template` using `String::with_capacity` and a `while` loop) replaces variables in one pass, effectively reducing memory allocations and dramatically improving performance (benchmarked ~2x speedup).

## 2024-11-20 - Uuid to_string() allocation overhead
**Learning:** Calling `Uuid::new_v4().simple().to_string()` followed by string slicing `raw[..12].to_string()` causes double heap allocation of a string, first 32 characters, then 12 characters.
**Action:** Use `Uuid::encode_buffer()` which allocates a fixed 32-byte array on the stack, call `encode_lower(&mut buf)`, and slice the string to do a single 12 character string allocation. This provides around ~2.5x performance boost in tight loops for formatting.
