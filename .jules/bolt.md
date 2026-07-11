## 2024-07-01 - Avoid cargo fix / cargo clippy --fix
**Learning:** Running `cargo fix` and `cargo clippy --fix` globally can introduce unstable features (like `let_chains`) or modify files outside the scope of a targeted optimization, violating the directive to make ONE small improvement.
**Action:** Only make targeted manual changes for performance improvements. Don't run automated fixers that modify unrelated files. Avoid adding new dependencies unless absolutely necessary.
## 2024-07-02 - Avoid chained .replace() on strings
**Learning:** Chained `.replace()` calls on strings result in multiple intermediate string allocations which can severely degrade performance, especially on hot paths like HTML template rendering.
**Action:** Use a single-pass rendering approach like `utils::render_template` to pre-allocate memory and substitute placeholders efficiently. Avoid committing temporary benchmark or test files.
