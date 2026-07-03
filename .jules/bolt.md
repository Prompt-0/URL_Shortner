## 2024-07-01 - Avoid cargo fix / cargo clippy --fix
**Learning:** Running `cargo fix` and `cargo clippy --fix` globally can introduce unstable features (like `let_chains`) or modify files outside the scope of a targeted optimization, violating the directive to make ONE small improvement.
**Action:** Only make targeted manual changes for performance improvements. Don't run automated fixers that modify unrelated files. Avoid adding new dependencies unless absolutely necessary.

## 2024-11-20 - Avoid chained String.replace() for template rendering
**Learning:** In Rust, calling `.replace()` on a `String` repeatedly (e.g. `s.replace(A).replace(B)...`) results in multiple allocations of intermediate `String`s. This is highly inefficient in hot paths like HTTP response generation.
**Action:** For simple template replacements, use a single-pass loop over the string, building up a single pre-allocated `String`. This reduces memory allocations and is significantly faster (>2x speedup).
