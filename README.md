## xwde: countio

[![Build Status][action-badge]][action-url]
[![Docs][docs-badge]][docs-url]
[![Crate Version][crates-badge]][crates-url]

[action-badge]: https://img.shields.io/github/actions/workflow/status/xwde/countio/build.yaml?branch=main&label=build&logo=github&style=for-the-badge
[action-url]: https://github.com/xwde/countio/actions/workflows/build.yaml
[crates-badge]: https://img.shields.io/crates/v/countio.svg?logo=rust&style=for-the-badge
[crates-url]: https://crates.io/crates/countio
[docs-badge]: https://img.shields.io/docsrs/countio?&style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
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
