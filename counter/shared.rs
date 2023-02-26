pub struct Counter<D> {
    pub(crate) inner: D,
    pub(crate) reader_bytes: usize,
    pub(crate) writer_bytes: usize,
}

impl<D> Counter<D> {
    /// Creates a new `Counter<D>` with a zero read/written bytes.
    pub fn new(inner: D) -> Self {
        Self::with_bytes(0, 0, inner)
    }

    /// Creates a new `Counter<D>` with the specified read and written bytes.
    pub fn with_bytes(read_bytes: usize, written_bytes: usize, inner: D) -> Self {
        Self {
            inner,
            reader_bytes: read_bytes,
            writer_bytes: written_bytes,
        }
    }
}

impl<D> Counter<D> {
    /// Returns the sum of read and written bytes by the internal reader/writer.
    pub fn bytes(&self) -> usize {
        self.reader_bytes + self.writer_bytes
    }

    /// Returns the count of read bytes by the internal reader.
    pub fn read_bytes(&self) -> usize {
        self.reader_bytes
    }

    /// Returns the count of written bytes by the internal writer.
    pub fn written_bytes(&self) -> usize {
        self.writer_bytes
    }
}

impl<D> Counter<D> {
    /// Consumes `Counter` returning the underlying reader/writer.
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
