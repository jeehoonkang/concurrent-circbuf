# Concurrent channels based on circular buffer

[![Build Status](https://travis-ci.org/jeehoonkang/concurrent-circbuf.svg?branch=master)](https://travis-ci.org/jeehoonkang/concurrent-circbuf)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/jeehoonkang/concurrent-circbuf)
<!-- [![Cargo](https://img.shields.io/crates/v/crossbeam-epoch.svg)](https://crates.io/crates/crossbeam-epoch) -->
<!-- [![Documentation](https://docs.rs/crossbeam-epoch/badge.svg)](https://docs.rs/crossbeam-epoch) -->

**CAVEAT: This crate is WIP**, and is not yet available in [crates.io](https://crates.io).

This crate provides concurrent channels based on circular buffer.

## Usage

Add this to your `Cargo.toml`:

<!-- ```toml -->
<!-- [dependencies] -->
<!-- concurrent-circbuf = "0.1" -->
<!-- ``` -->

```toml
[dependencies]
concurrent-circbuf = { git = "https://github.com/jeehoonkang/concurrent-circbuf.git" }
```

Next, add this to your crate:

```rust
extern crate concurrent_circbuf as circbuf;
```

## License

Licensed under the terms of MIT license and the Apache License (Version 2.0).

See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.
