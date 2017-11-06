# sna

An implementation of **Serial Number Arithmetic** as defined by
[RFC 1982][rfc-1982] for Rust.

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
