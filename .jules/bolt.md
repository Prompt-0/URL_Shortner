## 2024-07-01 - Avoid cargo fix / cargo clippy --fix
**Learning:** Running `cargo fix` and `cargo clippy --fix` globally can introduce unstable features (like `let_chains`) or modify files outside the scope of a targeted optimization, violating the directive to make ONE small improvement.
**Action:** Only make targeted manual changes for performance improvements. Don't run automated fixers that modify unrelated files. Avoid adding new dependencies unless absolutely necessary.

## 2024-11-20 - Single-pass HTML Template Rendering
**Learning:** Chaining multiple `.replace()` calls on a string (e.g., when rendering HTML templates) results in multiple intermediate String allocations, causing a performance overhead. In a Rust web server handling template strings manually, this can be a bottleneck.
**Action:** Use a single-pass rendering approach instead of chained `.replace()` calls. A custom function (`render_template` using `String::with_capacity` and a `while` loop) replaces variables in one pass, effectively reducing memory allocations and dramatically improving performance (benchmarked ~2x speedup).

## 2024-11-20 - Zero-allocation UUID Formatting
**Learning:** Calling `.to_string()` on UUIDs (like `Uuid::new_v4().simple().to_string()`) causes an unnecessary heap allocation, especially if you only need a substring.
**Action:** Use `Uuid::encode_buffer()` and `uuid.encode_lower(&mut buf)` to write the formatted UUID directly to a stack-allocated buffer (a `[u8; 32]`). This eliminates the intermediate `String` allocation, resulting in faster and more memory-efficient string generation.

## 2024-11-20 - Avoid Cow allocations when unnecessary
**Learning:** Returning a `Cow<'_, str>` instead of a `String` from functions like `escape_html` can avoid unnecessary memory allocations when the string doesn't actually need escaping. However, changing the return type of a public utility function is a breaking change that requires updating all call sites that expect a strict `String` (e.g., using `escape_html(...).into_owned()` where necessary). Since we use `render_template` which expects `&str` values in the replacement array, passing `&escape_html(...)` works automatically via deref coercion when `Cow` is returned.
**Action:** When implementing `Cow` optimizations, ensure it does not break call sites. Verify that deref coercion allows `&Cow<'_, str>` to be passed to functions expecting `&str`, like `render_template`.
