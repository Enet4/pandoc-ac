# Pandoc acronym

[![Build Status](https://travis-ci.org/Enet4/pandoc-ac.svg?branch=master)](https://travis-ci.org/Enet4/pandoc-ac)
![Minimum Rust Version 1.31](https://img.shields.io/badge/Minimum%20Rust%20Version-1.31-yellow.svg)

This filter provides a slightly cleaner syntax for LaTeX acronyms.

What it does:

| Code    | Description               | Translates to       |
|---------|---------------------------|---------------------|
| `(+x)`  | basic acronym instruction | `\ac{x}`  |
| `(+~x)` | full form of the acronym  | `\acf{x}`  |
| `(+-x)` | always short form | `\acs{x}`  |
| `(+x)`  | always expand acronym | `\acs{x}` |
| `(+*x)`, `(+.*x)`, `(+-*x)`, `(+~*x)` | plural form of the above | `\acp{x}`, `\acsp{x}`, `\aclp{x}`, `\acfp{x}` respectively |
| `(+^x)`, `(+.^x)`, `(+-^x)`, `(+~^x)` | plural form, alternate syntax | `\acp{x}`, `\acsp{x}`, `\aclp{x}`, `\acfp{x}` |

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
