use std::io::{Result as IoResult, Write};

/// The `CountWriter<W>` struct adds byte counting to any writer.
///
/// # Examples
///
/// ```rust
/// use std::io::prelude::*;
/// use countio::CountWriter;
///
/// fn main() -> std::io::Result<()> {
///     let mut writer = Vec::new();
///     let mut writer = CountWriter::new(&mut writer);
///
///     let len = writer.write("hello world".as_bytes())?;
///     assert_eq!(len, writer.bytes());
///
///     Ok(())
/// }
/// ```
pub struct CountWriter<W: Write> {
    inner: W,
    bytes: usize,
}

impl<W: Write> CountWriter<W> {
    /// Creates a new `CountWriter<W>` with a zero written bytes.
    pub fn new(inner: W) -> Self {
        Self::with_bytes(0, inner)
    }

    /// Creates a new `CountWriter<W>` with the specified written bytes.
    pub fn with_bytes(bytes: usize, inner: W) -> Self {
        Self { inner, bytes }
    }

    /// Returns the number of written bytes by the internal writer.
    pub fn bytes(&self) -> usize {
        self.bytes
    }

    /// Consumes `CountWriter` returning the underlying writer.
    pub fn into_inner(self) -> W {
        self.inner
    }

    /// Gets a reference to the underlying writer.
    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    /// Gets a mutable reference to the underlying writer.
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }
}

impl<W: Write> Write for CountWriter<W> {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        let bytes = self.inner.write(buf)?;
        self.bytes += bytes;
        Ok(bytes)
    }

    fn flush(&mut self) -> IoResult<()> {
        self.inner.flush()
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
