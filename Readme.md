# sna

An implementation of **Serial Number Arithmetic** as defined by
[RFC 1982][rfc-1982] for Rust.

![Version](https://img.shields.io/crates/v/sna.svg)
[![Travis](https://img.shields.io/travis/lgrahl/sna.svg)](https://travis-ci.org/lgrahl/sna)
[![AppVeyor](https://img.shields.io/appveyor/ci/lgrahl/sna.svg)](https://ci.appveyor.com/project/lgrahl/sna)
[![Codecov](https://img.shields.io/codecov/c/github/lgrahl/sna.svg)](https://codecov.io/gh/lgrahl/sna)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
sna = "0.1"
```

Add this to your crate:

```rust
extern crate sna;
```

## Examples

```rust
use sna::SerialNumber;

let zero = SerialNumber(0u8);
let one = SerialNumber(1u8);

assert_eq!(0u8, one + 255u8);
assert!(zero > 255u8);
```


[rfc-1982]: https://tools.ietf.org/html/rfc1982
