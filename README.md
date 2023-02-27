### xwde: countio

> **Warning** : The library is in active development. Expect breaking changes.

> **Note** : The library contains unsafe code. See implementations of
> `AsyncRead` and `AsyncWrite` traits for more details.

The wrapper struct to enable byte counting for `std::io::Read` and
`std::io::Write` and its asynchronous variants from `futures` and `tokio`.

Following features available:

- `std` to enable `std::io::Read` and `std::io::Write`. **Enabled by default**.
- `futures` to enable `futures_io::AsyncRead` and `futures_io::AsyncWrite`.
- `tokio` to enable `tokio::io::AsyncRead` and `tokio::io::AsyncWrite`.

#### `std::io::Read`:

```rust
use std::io::prelude::*;
use std::io::BufReader;
use countio::Counter;

fn main() {
    let reader = "Hello World!".as_bytes();

    let reader = Counter::new(reader);
    let mut reader = BufReader::new(reader);

    let mut buf = String::new();
    let len = reader.read_line(&mut buf).unwrap();
    assert_eq!(len, reader.get_ref().bytes());
}
```

#### `std::io::Write`:

```rust
use std::io::prelude::*;
use countio::Counter;

fn main() {
    let mut writer = Vec::new();
    let mut writer = Counter::new(&mut writer);

    let buf = "Hello World!".as_bytes();
    let len = writer.write(buf).unwrap();
    assert_eq!(len, writer.bytes());
}
```
