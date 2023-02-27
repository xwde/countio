use std::io::{Read, Result as IoResult, Write};

use crate::Counter;

impl<R: Read> Read for Counter<R> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let bytes = self.inner.read(buf)?;
        self.reader_bytes += bytes;
        Ok(bytes)
    }
}

impl<W: Write> Write for Counter<W> {
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
mod test {
    use std::io::{BufRead, BufReader, Write};

    use crate::Counter;

    #[test]
    fn reader() {
        let reader = "hello world".as_bytes();

        let reader = Counter::new(reader);
        let mut reader = BufReader::new(reader);

        let mut buf = String::new();
        let len = reader.read_line(&mut buf).unwrap();
        assert_eq!(len, reader.get_ref().bytes());
    }

    #[test]
    fn writer() {
        let mut writer = Vec::new();
        let mut writer = Counter::new(&mut writer);

        let buf = "hello world".as_bytes();
        let len = writer.write(buf).unwrap();
        assert_eq!(len, writer.bytes());
    }
}
