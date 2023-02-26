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
        let this = unsafe { self.get_unchecked_mut() };

        let pin = Pin::new(&mut this.inner);
        let poll = pin.poll_read(ctx, buf);
        if let Poll::Ready(Ok(bytes)) = poll {
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

    fn poll_close(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<IoResult<()>> {
        unsafe { self.map_unchecked_mut(|c| &mut c.inner) }.poll_close(ctx)
    }
}
