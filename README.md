## xwde: countio

[![Build Status][action-badge]][action-url]
[![Docs][docs-badge]][docs-url]
[![Crate Version][crates-badge]][crates-url]

[action-badge]: https://img.shields.io/github/actions/workflow/status/xwde/countio/build.yaml?branch=main&label=build&logo=github&style=for-the-badge
[action-url]: https://github.com/xwde/countio/actions/workflows/build.yaml
[crates-badge]: https://img.shields.io/crates/v/countio.svg?logo=rust&style=for-the-badge
[crates-url]: https://crates.io/crates/countio
[docs-badge]: https://img.shields.io/docsrs/countio?logo=rust&style=for-the-badge
[docs-url]: http://docs.rs/countio

The wrapper struct to enable byte counting for `std::io::Read` and
`std::io::Write` and its asynchronous variants from `futures` and `tokio`
crates.

> **Note** : The library contains unsafe code. See implementation of `Async*`
> traits for more details.

Following features available:

- `std` to enable `std::io::Read` and `std::io::Write`. **Enabled by default**.
- `futures` to enable `futures_io::AsyncRead` and `futures_io::AsyncWrite`.
- `tokio` to enable `tokio::io::AsyncRead` and `tokio::io::AsyncWrite`.

### `std::io::Read`:

```rust
use std::io::prelude::*;
use std::io::BufReader;
use countio::Counter;

fn main() {
    let mut reader = "Hello World!".as_bytes();
    let mut reader = Counter::new(&mut reader);
    let mut reader = BufReader::new(&mut reader);

    let mut buf = String::new();
    let len = reader.read_line(&mut buf).unwrap();

    assert_eq!(len, reader.get_ref().read_bytes());
}
```

### `std::io::Write`:

```rust
use std::io::prelude::*;
use std::io::BufWriter;
use countio::Counter;

fn main() {
    let mut writer = Vec::new();
    let mut writer = Counter::new(&mut writer);
    let mut writer = BufWriter::new(&mut writer);

    let buf = "Hello World!".as_bytes();
    let len = writer.write(buf).unwrap();
    writer.flush().unwrap();

    assert_eq!(len, writer.get_ref().written_bytes());
}
```
