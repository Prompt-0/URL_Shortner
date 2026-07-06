## 2024-07-01 - Avoid cargo fix / cargo clippy --fix
**Learning:** Running `cargo fix` and `cargo clippy --fix` globally can introduce unstable features (like `let_chains`) or modify files outside the scope of a targeted optimization, violating the directive to make ONE small improvement.
**Action:** Only make targeted manual changes for performance improvements. Don't run automated fixers that modify unrelated files. Avoid adding new dependencies unless absolutely necessary.

## 2024-05-18 - SQLite Performance Optimization
**Learning:** The project uses a SQLite database and it is configured to use Write-Ahead Logging (WAL) mode. `sqlx` supports a `synchronous = NORMAL` pragma, which is extremely well suited for WAL mode. It can dramatically improve write performance at a negligible risk of corruption compared to `synchronous = FULL`.
**Action:** Always configure SQLite connections with `synchronous = NORMAL` when using WAL mode for maximum performance.
