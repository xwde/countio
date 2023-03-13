use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::io::{Read, Result as IoResult, Seek, SeekFrom, Write};

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

impl<D: Seek> Seek for Counter<D> {
    fn seek(&mut self, pos: SeekFrom) -> IoResult<u64> {
        self.inner.seek(pos)
    }
}

impl<D: Debug> Debug for Counter<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Counter")
            .field("inner", &self.inner)
            .field("read", &self.reader_bytes)
            .field("written", &self.writer_bytes)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use std::io::{BufRead, BufReader};
    use std::io::{BufWriter, Write};

    use crate::Counter;

    #[test]
    fn reader() {
        let mut reader = "Hello World!".as_bytes();
        let mut reader = Counter::new(&mut reader);
        let mut reader = BufReader::new(&mut reader);

        let mut buf = String::new();
        let len = reader.read_line(&mut buf).unwrap();

        assert_eq!(len, reader.get_ref().reader_bytes());
        assert_eq!(len, reader.get_ref().total_bytes());
    }

    #[test]
    fn writer() {
        let mut writer = Vec::new();
        let mut writer = Counter::new(&mut writer);
        let mut writer = BufWriter::new(&mut writer);

        let buf = "Hello World!".as_bytes();
        let len = writer.write(buf).unwrap();
        writer.flush().unwrap();

        assert_eq!(len, writer.get_ref().writer_bytes());
        assert_eq!(len, writer.get_ref().total_bytes());
    }

    #[test]
    fn debug() {
        let writer = Vec::<u8>::new();
        let writer = Counter::new(writer);

        let fmt = "Counter { inner: [], read: 0, written: 0 }";
        assert_eq!(format!("{writer:?}"), fmt);
    }
}
