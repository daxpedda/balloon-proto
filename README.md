# balloon-proto

[![Crates.io](https://img.shields.io/crates/v/balloon-proto.svg)](https://crates.io/crates/balloon-proto)
[![CI](https://github.com/daxpedda/balloon-proto/actions/workflows/ci.yml/badge.svg)](https://github.com/daxpedda/balloon-proto/actions/workflows/ci.yml)
[![Docs](https://docs.rs/balloon-proto/badge.svg)](https://docs.rs/balloon-proto)

[![Libraries.io](https://img.shields.io/librariesio/release/cargo/balloon-proto.svg)](https://libraries.io/cargo/balloon-proto)
[![Commits since](https://img.shields.io/github/commits-since/daxpedda/balloon-proto/latest)](https://github.com/daxpedda/balloon-proto/releases/latest)
[![License](https://img.shields.io/crates/l/balloon-proto)](https://github.com/daxpedda/balloon-proto/blob/master/LICENSE)
[![LoC](https://tokei.rs/b1/github/daxpedda/balloon-proto)](https://github.com/daxpedda/balloon-proto)

## Description

Rust bindings to the official prototype implementation of the Balloon hashing
algorithm. See https://github.com/henrycg/balloon.

This crate only supports Linux and is created to expand the testing capabilities
of the Balloon implementation of RustCrypto. See
https://github.com/RustCrypto/password-hashes/tree/master/balloon.

## Build

Balloon requires an OpenSSL installation. This is handles in the build scripts.
If cashing the OpenSSL build or providing a external installation is desired,
the `BALLOON_OPENSSL_INSTALL` environment variable can be used to point to the
desired path.

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](https://github.com/daxpedda/balloon-proto/blob/rust/LICENSE-APACHE)
  or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](https://github.com/daxpedda/balloon-proto/blob/rust/LICENSE-MIT)
  or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
