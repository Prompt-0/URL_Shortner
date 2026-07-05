## 2024-07-01 - Avoid cargo fix / cargo clippy --fix
**Learning:** Running `cargo fix` and `cargo clippy --fix` globally can introduce unstable features (like `let_chains`) or modify files outside the scope of a targeted optimization, violating the directive to make ONE small improvement.
**Action:** Only make targeted manual changes for performance improvements. Don't run automated fixers that modify unrelated files. Avoid adding new dependencies unless absolutely necessary.

## 2024-07-05 - SQLite Write Optimization
**Learning:** In Rust projects using sqlx with SQLite, if WAL mode is enabled, `synchronous = NORMAL` should be set to significantly reduce fsync() calls and improve write performance without risking database corruption on application crash.
**Action:** When configuring SqliteConnectOptions, always pair `journal_mode(SqliteJournalMode::Wal)` with `synchronous(SqliteSynchronous::Normal)` for maximum performance.
