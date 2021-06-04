# Pandoc acronym

[![Latest Version](https://img.shields.io/crates/v/pandoc-ac.svg)](https://crates.io/crates/pandoc-ac)
![Minimum Rust Version 1.31](https://img.shields.io/badge/Minimum%20Rust%20Version-1.31-green.svg)
![Minimum Pandoc Version 2.13](https://img.shields.io/badge/Minimum%20Pandoc%20Version-2.12-green.svg)
[![Build Status](https://travis-ci.org/Enet4/pandoc-ac.svg?branch=master)](https://travis-ci.org/Enet4/pandoc-ac)
[![dependency status](https://deps.rs/repo/github/Enet4/pandoc-ac/status.svg)](https://deps.rs/repo/github/Enet4/pandoc-ac)

This filter provides a slightly cleaner syntax for LaTeX acronyms.

What it does:

| Code    | Description               | Translates to       |
|---------|---------------------------|---------------------|
| `(+x)`  | basic acronym instruction | `\ac{x}`  |
| `(+~x)` | full form of the acronym  | `\acf{x}`  |
| `(+.x)` | always short form | `\acs{x}`  |
| `(+-x)`  | always expand acronym | `\acl{x}` |
| `(+*x)`, `(+.*x)`, `(+-*x)`, `(+~*x)` | plural form of the above | `\acp{x}`, `\acsp{x}`, `\aclp{x}`, `\acfp{x}` respectively |
| `(+^x)`, `(+.^x)`, `(+-^x)`, `(+~^x)` | plural form, alternate syntax | `\acp{x}`, `\acsp{x}`, `\aclp{x}`, `\acfp{x}` |

## Using

The binary `pandoc-ac` is a standard [pandoc filter](http://pandoc.org/filters.html).
It is currently compatible with the Pandoc AST API v1.22.

```none
pandoc mytext.md -F pandoc-ac -o out.pdf
```

Moreover, the filter is accessible programmatically from Rust with the `pandoc_ac` library.

```rust
use pandoc_ast::Pandoc;
use pandoc_ac::make_acronym_formatting;
use serde_json::from_reader;
use std::fs::File;

let input_file = "resources/test.md";

let text_json: Pandoc = from_reader(File::open(input_file)?)?;
let result: Pandoc = make_acronym_formatting(text_json);
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
