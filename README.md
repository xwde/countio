## xwde: countio

[![Build Status][action-badge]][action-url]
[![Crate Docs][docs-badge]][docs-url]
[![Crate Version][crates-badge]][crates-url]

[action-badge]: https://img.shields.io/github/actions/workflow/status/xwde/countio/build.yaml?branch=main&label=build&logo=github&style=for-the-badge
[action-url]: https://github.com/xwde/countio/actions/workflows/build.yaml
[crates-badge]: https://img.shields.io/crates/v/countio.svg?logo=rust&style=for-the-badge
[crates-url]: https://crates.io/crates/countio
[docs-badge]: https://img.shields.io/docsrs/countio?logo=Docs.rs&style=for-the-badge
[docs-url]: http://docs.rs/countio

The wrapper struct to enable byte counting for `std::io::Read`,
`std::io::Write`, `std::io::Seek` and its asynchronous variants from `futures`
and `tokio` crates.

### Features

- `std` to enable `std::io::{Read, Write, Seek}`. **Enabled by default**.
- `futures` to enable `futures_io::{AsyncRead, AsyncWrite, AsyncSeek}`.
- `tokio` to enable `tokio::io::{AsyncRead, AsyncWrite, AsyncSeek}`.

### Examples

- `std::io::Read`:

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

    assert_eq!(len, reader.get_ref().reader_bytes());
}
```

- `std::io::Write`:

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

    assert_eq!(len, writer.get_ref().writer_bytes());
}
```
