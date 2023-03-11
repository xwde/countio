use std::fmt::{Debug, Formatter, Result as FmtResult};

/// The `Counter<D>` struct adds byte counting to any reader or writer.
pub struct Counter<D> {
    pub(crate) inner: D,
    pub(crate) reader_bytes: usize,
    pub(crate) writer_bytes: usize,
}

impl<D> Counter<D> {
    /// Creates a new `Counter<D>` with zero read/written bytes.
    pub fn new(inner: D) -> Self {
        Self::with_bytes(0, 0, inner)
    }

    /// Creates a new `Counter<D>` with the specified read/written bytes.
    pub fn with_bytes(reader_bytes: usize, writer_bytes: usize, inner: D) -> Self {
        Self {
            inner,
            reader_bytes,
            writer_bytes,
        }
    }

    /// Returns the sum of read and written bytes by the underlying reader/writer.
    pub fn total_bytes(&self) -> usize {
        self.reader_bytes + self.writer_bytes
    }

    /// Returns the count of read bytes by the underlying reader.
    pub fn reader_bytes(&self) -> usize {
        self.reader_bytes
    }

    /// Returns the count of written bytes by the underlying writer.
    pub fn writer_bytes(&self) -> usize {
        self.writer_bytes
    }

    /// Consumes `Counter<D>` returning the underlying reader/writer.
    pub fn into_inner(self) -> D {
        self.inner
    }

    /// Gets a reference to the underlying reader/writer.
    pub fn get_ref(&self) -> &D {
        &self.inner
    }

    /// Gets a mutable reference to the underlying reader/writer.
    pub fn get_mut(&mut self) -> &mut D {
        &mut self.inner
    }
}

impl<D> From<D> for Counter<D> {
    fn from(inner: D) -> Self {
        Self::new(inner)
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

#[cfg(feature = "futures")]
mod futures;
#[cfg(feature = "std")]
mod stdlib;
#[cfg(feature = "tokio")]
mod tokio;
