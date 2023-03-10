use std::io::Result as IoResult;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_io::{AsyncRead, AsyncWrite};

use crate::Counter;

impl<R: AsyncRead + Unpin> AsyncRead for Counter<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        ctx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<IoResult<usize>> {
        let counter = self.get_mut();

        let pin = Pin::new(&mut counter.inner);
        let poll = pin.poll_read(ctx, buf);
        if let Poll::Ready(Ok(bytes)) = poll {
            counter.reader_bytes += bytes
        }

        poll
    }
}

impl<W: AsyncWrite + Unpin> AsyncWrite for Counter<W> {
    fn poll_write(
        self: Pin<&mut Self>,
        ctx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<IoResult<usize>> {
        let counter = self.get_mut();
        let pin = Pin::new(&mut counter.inner);
        let poll = pin.poll_write(ctx, buf);

        if let Poll::Ready(Ok(bytes)) = poll {
            counter.writer_bytes += bytes
        }

        poll
    }

    fn poll_flush(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<IoResult<()>> {
        let counter = self.get_mut();
        let pin = Pin::new(&mut counter.inner);
        pin.poll_flush(ctx)
    }

    fn poll_close(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<IoResult<()>> {
        let counter = self.get_mut();
        let pin = Pin::new(&mut counter.inner);
        pin.poll_close(ctx)
    }
}

#[cfg(test)]
mod test {
    use futures_util::io::{BufReader, BufWriter};
    use futures_util::{AsyncBufReadExt, AsyncWriteExt};

    use crate::Counter;

    #[futures_test::test]
    async fn reader() {
        let mut reader = "Hello World!".as_bytes();
        let mut reader = Counter::new(&mut reader);
        let mut reader = BufReader::new(&mut reader);

        let mut buf = String::new();
        let len = reader.read_line(&mut buf).await.unwrap();

        assert_eq!(len, reader.get_ref().reader_bytes());
    }

    #[futures_test::test]
    async fn writer() {
        let mut writer = Vec::new();
        let mut writer = Counter::new(&mut writer);
        let mut writer = BufWriter::new(&mut writer);

        let buf = "Hello World!".as_bytes();
        let len = writer.write(buf).await.unwrap();
        writer.flush().await.unwrap();

        assert_eq!(len, writer.get_ref().writer_bytes());
    }
}
