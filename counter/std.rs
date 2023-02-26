use std::io::{Read, Result as IoResult, Write};

use crate::Counter;

impl<R: Read> Read for Counter<R> {
    /// The `Counter<R>` struct adds byte counting to any `Read`er.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::prelude::*;
    /// use std::io::BufReader;
    /// use countio::Counter;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let reader = "hello world".as_bytes();
    ///
    ///     let mut reader = Counter::new(reader);
    ///     let mut reader = BufReader::new(reader);
    ///
    ///     let mut line = String::new();
    ///     let len = reader.read_line(&mut line)?;
    ///     assert_eq!(len, reader.get_ref().bytes());
    ///
    ///     Ok(())
    /// }
    /// ```
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let bytes = self.inner.read(buf)?;
        self.reader_bytes += bytes;
        Ok(bytes)
    }
}

impl<W: Write> Write for Counter<W> {
    /// The `Counter<W>` struct adds byte counting to any `Write`r.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::prelude::*;
    /// use countio::Counter;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let mut writer = Vec::new();
    ///     let mut writer = Counter::new(&mut writer);
    ///
    ///     let len = writer.write("hello world".as_bytes())?;
    ///     assert_eq!(len, writer.bytes());
    ///
    ///     Ok(())
    /// }
    /// ```
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        let bytes = self.inner.write(buf)?;
        self.writer_bytes += bytes;
        Ok(bytes)
    }

    fn flush(&mut self) -> IoResult<()> {
        self.inner.flush()
    }
}

#[cfg(test)]
mod tests {
    use crate::Counter;
    use std::io::{BufRead, BufReader, Write};

    #[test]
    fn reader() {
        let reader = "hello world".as_bytes();

        let mut reader = Counter::new(reader);
        let mut reader = BufReader::new(reader);

        let mut line = String::new();
        let len = reader.read_line(&mut line)?;
        assert_eq!(len, reader.get_ref().bytes());
    }

    #[test]
    fn writer() {
        let mut writer = Vec::new();
        let mut writer = Counter::new(&mut writer);

        let len = writer.write("hello world".as_bytes())?;
        assert_eq!(len, writer.bytes());
    }
}
