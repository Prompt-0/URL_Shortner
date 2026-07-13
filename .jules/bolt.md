## 2024-07-01 - Avoid cargo fix / cargo clippy --fix
**Learning:** Running `cargo fix` and `cargo clippy --fix` globally can introduce unstable features (like `let_chains`) or modify files outside the scope of a targeted optimization, violating the directive to make ONE small improvement.
**Action:** Only make targeted manual changes for performance improvements. Don't run automated fixers that modify unrelated files. Avoid adding new dependencies unless absolutely necessary.

## 2024-05-18 - Avoid chained string allocations in HTML templates
**Learning:** The previous implementation used chained `.replace()` calls on static template strings. This forced the application to allocate multiple intermediate strings for every HTML response, creating an unnecessary performance bottleneck and memory overhead specific to this simplistic rendering pattern.
**Action:** Replace chained string replacement with a single-pass string builder (like `render_template` in `src/utils.rs`) that pre-allocates enough capacity and iterates over the template once, inserting variables inline. This yields an immediate >1.5x performance boost on HTML rendering and reduces memory allocations.
