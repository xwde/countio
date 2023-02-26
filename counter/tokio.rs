use std::io::Result as IoResult;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

use crate::Counter;

impl<R: AsyncRead> AsyncRead for Counter<R> {
    /// The `Counter<R>` struct adds byte counting to any `AsyncRead`er.
    ///
    /// TODO example
    fn poll_read(
        self: Pin<&mut Self>,
        ctx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<IoResult<()>> {
        todo!()
    }
}

impl<W: AsyncWrite> AsyncWrite for Counter<W> {
    /// The `Counter<R>` struct adds byte counting to any `AsyncWrite`r.
    ///
    /// TODO example
    fn poll_write(
        self: Pin<&mut Self>,
        ctx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<IoResult<usize>> {
        todo!()
    }

    fn poll_flush(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<IoResult<()>> {
        todo!()
    }

    fn poll_shutdown(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<IoResult<()>> {
        todo!()
    }
}
