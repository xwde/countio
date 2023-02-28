//! The wrapper struct to enable byte counting for `std::io::Read` and
//! `std::io::Write` and its asynchronous variants from `futures` and `tokio`
//! crates.
//!
//! ## Features
//!
//! - `std` to enable `std::io::Read` and `std::io::Write`. **Enabled by default**.                                                                                    
//! - `futures` to enable `futures_io::AsyncRead` and `futures_io::AsyncWrite`.                                                                                        
//! - `tokio` to enable `tokio::io::AsyncRead` and `tokio::io::AsyncWrite`.                                                                                            
//!
//! ## Examples
//!
//! - `std::io::Read`:
//!
//! ```rust
//! use std::io::prelude::*;
//! use std::io::BufReader;
//! use countio::Counter;
//!
//! let mut reader = "Hello World!".as_bytes();
//! let mut reader = Counter::new(&mut reader);
//! let mut reader = BufReader::new(&mut reader);
//!
//! let mut buf = String::new();
//! let len = reader.read_line(&mut buf).unwrap();
//!
//! assert_eq!(len, reader.get_ref().reader_bytes());
//! ```
//!
//! - `std::io::Write`:
//!
//! ```rust
//! use std::io::prelude::*;
//! use std::io::BufWriter;
//! use countio::Counter;
//!
//! let mut writer = Vec::new();
//! let mut writer = Counter::new(&mut writer);
//! let mut writer = BufWriter::new(&mut writer);
//!
//! let buf = "Hello World!".as_bytes();
//! let len = writer.write(buf).unwrap();
//! writer.flush().unwrap();
//!
//! assert_eq!(len, writer.get_ref().writer_bytes());
//! ```

mod counter;
pub use counter::*;
