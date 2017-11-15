# sna

An implementation of **Serial Number Arithmetic** as defined by
[RFC 1982][rfc-1982] for Rust.

![Version][version-badge]
[![Travis][travis-badge]][travis-url]
[![AppVeyor][appveyor-badge]][appveyor-url]
[![Codecov][codecov-badge]][codecov-url]

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

### Linting

To run clippy lints, compile the library with `--features clippy` on a nightly
compiler:

    $ cargo build --features clippy

If `nightly` is not your default compiler:

    $ rustup run nightly cargo build --features clippy


[rfc-1982]: https://tools.ietf.org/html/rfc1982
[version-badge]: https://img.shields.io/crates/v/sna.svg
[travis-badge]: https://img.shields.io/travis/lgrahl/sna.svg
[travis-url]: https://travis-ci.org/lgrahl/sna
[appveyor-badge]: https://img.shields.io/appveyor/ci/lgrahl/sna.svg
[appveyor-url]: https://ci.appveyor.com/project/lgrahl/sna
[codecov-badge]: https://img.shields.io/codecov/c/github/lgrahl/sna.svg
[codecov-url]: https://codecov.io/gh/lgrahl/sna
