use std::io::{Read, Result as IoResult};

/// The `CountReader<R>` struct adds byte counting to any reader.
///
/// # Examples
///
/// ```rust
/// use std::io::prelude::*;
/// use std::io::BufReader;
/// use countio::CountReader;
///
/// fn main() -> std::io::Result<()> {
///     let reader = "hello world".as_bytes();
///
///     let mut reader = CountReader::new(reader);
///     let mut reader = BufReader::new(reader);
///
///     let mut line = String::new();
///     let len = reader.read_line(&mut line)?;
///     assert_eq!(len, reader.get_ref().bytes());
///
///     Ok(())
/// }
/// ```
pub struct CountReader<R: Read> {
    inner: R,
    bytes: usize,
}

impl<R: Read> CountReader<R> {
    /// Creates a new `CountReader<R>` with zero read bytes.
    pub fn new(inner: R) -> Self {
        Self::with_bytes(0, inner)
    }

    /// Creates a new `CountReader<R>` with the specified read bytes.
    pub fn with_bytes(bytes: usize, inner: R) -> Self {
        Self { inner, bytes }
    }

    /// Returns the number of read bytes by the internal reader.
    pub fn bytes(&self) -> usize {
        self.bytes
    }

    /// Consumes `CountReader` returning the underlying reader.
    pub fn into_inner(self) -> R {
        self.inner
    }

    /// Gets a reference to the underlying reader.
    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    /// Gets a mutable reference to the underlying reader.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }
}

impl<R: Read> Read for CountReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let bytes = self.inner.read(buf)?;
        self.bytes += bytes;
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn foo() {
        assert_eq!(2 + 2, 4);
    }
}
