# DBML parser for Rust

[![crate](https://img.shields.io/crates/v/dbml-rs.svg)](https://crates.io/crates/dbml-rs)
![MSRV](https://img.shields.io/badge/rustc-1.61+-ab6000.svg)
![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/dbml-rs.svg)
![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)

DBML parser implemented in Rust programming language.

## How to use it?

```rust
use dbml_rs::*;
use std::fs;

fn main() {
  let input = fs::read_to_string("path/to/your/file.dbml");
  let result = parse_dbml(&input);
}
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Always welcome you to participate, contribute and together.
