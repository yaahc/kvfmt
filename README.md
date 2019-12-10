kvfmt="valuable"
======================

[![Latest Version](https://img.shields.io/crates/v/kvfmt.svg)](https://crates.io/crates/kvfmt)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/kvfmt)

This library provides a convenient macro generating structured logging-esque strings with a minimalistic syntax.

```toml
[dependencies]
kvfmt = "0.1"
```

<br>

## Example

```rust
use kvfmt::kvfmt;

fn main() {
    let dir = "/var/log";
    let paths = vec!["dmesg", "syslog"];

    assert_eq!(
        "dir=/var/log paths=[\"dmesg\", \"syslog\"]",
        kvfmt!(dir, ?paths)
    );
}
```

<br>

## Details

- This macro supports any number of identifiers, optionally prefixed with a `?`
  to indicate that it should be formatted with the `Debug` trait rather than
  the `Display` trait which is the default.
  - `kvfmt!(dir)` ⟶ `format!("dir={}", dir)`
  - `kvfmt!(?dir)` ⟶ `format!("dir={:?}", dir)`

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

<br>

#### Credits

A special thanks to [Nika](https://github.com/mystor) for helping me write the
macro and [David](https://github.com/dtolnay/) whose wonderful README style I
continously steal for all of my projects.
