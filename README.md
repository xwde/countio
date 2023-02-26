### xwde: countio

> **Warning** : The library is in active development. Expect breaking changes.

The wrapper struct to enable byte counting for common `Reader` & `Writer`
implementations.

Following features available:

- `std` to enable `std::io::Read` and `std::io::Write`.
- `futures` to enable `futures_io::AsyncRead` and `futures_io::AsyncWrite`.
- `tokio` to enable `tokio::io::AsyncRead` and `tokio::io::AsyncWrite`.

#### `std::io::Read`

```rust
use std::io::prelude::*;
use std::io::BufReader;
use countio::Counter;

fn main() -> std::io::Result<()> {
    let reader = "hello world".as_bytes();

    let mut reader = Counter::new(reader);
    let mut reader = BufReader::new(reader);

    let mut line = String::new();
    let len = reader.read_line(&mut line)?;
    assert_eq!(len, reader.get_ref().bytes());

    Ok(())
}
```

#### `std::io::Write`

```rust
use std::io::prelude::*;                                      
use countio::Counter;                                         
                                                              
fn main() -> std::io::Result<()> {                            
    let mut writer = Vec::new();                              
    let mut writer = Counter::new(&mut writer);               
                                                              
    let len = writer.write("hello world".as_bytes())?;        
    assert_eq!(len, writer.bytes());                          
                                                              
    Ok(())                                                    
}
```
