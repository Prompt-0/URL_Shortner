## 2024-07-01 - Avoid cargo fix / cargo clippy --fix
**Learning:** Running `cargo fix` and `cargo clippy --fix` globally can introduce unstable features (like `let_chains`) or modify files outside the scope of a targeted optimization, violating the directive to make ONE small improvement.
**Action:** Only make targeted manual changes for performance improvements. Don't run automated fixers that modify unrelated files. Avoid adding new dependencies unless absolutely necessary.

## 2024-11-20 - Single-pass HTML Template Rendering
**Learning:** Chaining multiple `.replace()` calls on a string (e.g., when rendering HTML templates) results in multiple intermediate String allocations, causing a performance overhead. In a Rust web server handling template strings manually, this can be a bottleneck.
**Action:** Use a single-pass rendering approach instead of chained `.replace()` calls. A custom function (`render_template` using `String::with_capacity` and a `while` loop) replaces variables in one pass, effectively reducing memory allocations and dramatically improving performance (benchmarked ~2x speedup).

## 2024-11-20 - Zero-allocation UUID Formatting
**Learning:** Calling `.to_string()` on UUIDs (like `Uuid::new_v4().simple().to_string()`) causes an unnecessary heap allocation, especially if you only need a substring.
**Action:** Use `Uuid::encode_buffer()` and `uuid.encode_lower(&mut buf)` to write the formatted UUID directly to a stack-allocated buffer (a `[u8; 32]`). This eliminates the intermediate `String` allocation, resulting in faster and more memory-efficient string generation.

## 2025-01-16 - Avoid allocation during HTML escaping using Cow
**Learning:** Returning `String` from `escape_html` causes unnecessary heap allocations for the vast majority of strings that don't contain characters needing escaping (like shortcodes, standard URLs).
**Action:** Return `std::borrow::Cow<'_, str>` instead of `String`. Scan the string for characters needing escaping first. If none are found, return `Cow::Borrowed(input)`. If characters are found, perform the escaping and return `Cow::Owned(out)`. This drastically reduces memory allocations and improves performance when escaping is not needed.
