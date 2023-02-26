use std::io::Result as IoResult;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

use crate::Counter;

impl<R: AsyncRead + Unpin> AsyncRead for Counter<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        ctx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<IoResult<()>> {
        let this = unsafe { self.get_unchecked_mut() };

        let pin = Pin::new(&mut this.inner);
        let bytes = buf.filled().len();
        let poll = pin.poll_read(ctx, buf);
        let bytes = buf.filled().len() - bytes;

        if let Poll::Ready(Ok(())) = poll {
            this.reader_bytes += bytes
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
        let this = unsafe { self.get_unchecked_mut() };

        let pin = Pin::new(&mut this.inner);
        let poll = pin.poll_write(ctx, buf);
        if let Poll::Ready(Ok(bytes)) = poll {
            this.writer_bytes += bytes
        }

        poll
    }

    fn poll_flush(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<IoResult<()>> {
        unsafe { self.map_unchecked_mut(|c| &mut c.inner) }.poll_flush(ctx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<IoResult<()>> {
        unsafe { self.map_unchecked_mut(|c| &mut c.inner) }.poll_shutdown(ctx)
    }
}
