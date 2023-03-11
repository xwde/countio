use std::io::{Result as IoResult, SeekFrom};
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::io::{AsyncRead, AsyncSeek, AsyncWrite, ReadBuf};

use crate::Counter;

impl<R: AsyncRead + Unpin> AsyncRead for Counter<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        ctx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<IoResult<()>> {
        let counter = self.get_mut();

        let pin = Pin::new(&mut counter.inner);
        let bytes = buf.filled().len();
        let poll = pin.poll_read(ctx, buf);
        let bytes = buf.filled().len() - bytes;

        if let Poll::Ready(Ok(())) = poll {
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

    fn poll_shutdown(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<IoResult<()>> {
        let counter = self.get_mut();
        let pin = Pin::new(&mut counter.inner);
        pin.poll_shutdown(ctx)
    }
}

impl<D: AsyncSeek + Unpin> AsyncSeek for Counter<D> {
    fn start_seek(self: Pin<&mut Self>, position: SeekFrom) -> IoResult<()> {
        let counter = self.get_mut();
        let pin = Pin::new(&mut counter.inner);
        pin.start_seek(position)
    }

    fn poll_complete(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<IoResult<u64>> {
        let counter = self.get_mut();
        let pin = Pin::new(&mut counter.inner);
        pin.poll_complete(ctx)
    }
}

#[cfg(test)]
mod tests {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
    use tokio::io::{BufReader, BufWriter};

    use crate::Counter;

    #[tokio::test]
    async fn reader() {
        let mut reader = "Hello World!".as_bytes();
        let mut reader = Counter::new(&mut reader);
        let mut reader = BufReader::new(&mut reader);

        let mut buf = String::new();
        let len = reader.read_line(&mut buf).await.unwrap();

        assert_eq!(len, reader.get_ref().reader_bytes());
    }

    #[tokio::test]
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
