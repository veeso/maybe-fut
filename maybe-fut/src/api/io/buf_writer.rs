use super::Write;

/// Wraps a writer and buffers its output.
#[derive(Debug)]
pub struct BufWriter<W: ?Sized + Write> {
    buf: Vec<u8>,
    filled: usize,
    pos: usize,
    inner: W,
}

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

impl<W> BufWriter<W>
where
    W: Write,
{
    /// Creates a new [`BufWriter`] with the default buffer size.
    pub fn new(inner: W) -> Self {
        Self::with_capacity(DEFAULT_BUF_SIZE, inner)
    }

    /// Creates a new [`BufWriter`] with the specified buffer size.
    pub fn with_capacity(capacity: usize, inner: W) -> Self {
        Self {
            buf: vec![0; capacity],
            filled: 0,
            pos: 0,
            inner,
        }
    }

    /// Returns a reference to the internal buffer.
    pub fn buffer(&self) -> &[u8] {
        &self.buf[self.pos..self.filled]
    }

    /// Returns the number of bytes the internal buffer can hold.
    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    /// Returns a reference to the underlying writer.
    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    /// Returns a mutable reference to the underlying writer.
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }

    /// Returns the underlying writer.
    pub fn into_inner(self) -> W {
        self.inner
    }

    /// Disassembles this BufWriter<W>, returning the underlying writer, and any buffered but unwritten data.
    pub fn into_parts(self) -> (W, Vec<u8>) {
        let buf = self.buf;
        let inner = self.inner;
        (inner, buf)
    }
}

impl<W> Write for BufWriter<W>
where
    W: Write,
{
    async fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if buf.len() < self.capacity() {
            self.buf[self.filled..self.filled + buf.len()].copy_from_slice(buf);
            self.filled += buf.len();
            Ok(buf.len())
        } else {
            let n = self.inner.write(buf).await?;
            self.filled += n;
            Ok(n)
        }
    }

    async fn flush(&mut self) -> std::io::Result<()> {
        if self.filled > 0 {
            self.inner.write(&self.buf[..self.filled]).await?;
            self.filled = 0;
        }
        self.inner.flush().await
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn test_buf_writer() {
        let data = vec![0; 1024];
        let mut buf_writer = BufWriter::new(Buffer::new(data));

        let input = b"Hello, world!";
        let n = buf_writer.write(input).await.unwrap();
        assert_eq!(n, input.len());

        buf_writer.flush().await.unwrap();
    }

    #[tokio::test]
    async fn test_buf_writer_with_capacity() {
        let data = vec![0; 2048];
        let mut buf_writer = BufWriter::with_capacity(1024, Buffer::new(data));

        let input = b"Hello, world!";
        let n = buf_writer.write(input).await.unwrap();
        assert_eq!(n, input.len());

        buf_writer.flush().await.unwrap();
    }

    #[tokio::test]
    async fn test_buf_writer_into_inner() {
        let data = vec![0; 1024];
        let buf_writer = BufWriter::new(Buffer::new(data));

        let inner = buf_writer.into_inner();
        assert_eq!(inner.pos, 0);
    }

    #[tokio::test]
    async fn test_buf_writer_into_parts() {
        let data = vec![0; 1024];
        let buf_writer = BufWriter::new(Buffer::new(data));

        let (inner, buf) = buf_writer.into_parts();
        assert_eq!(inner.pos, 0);
        assert_eq!(buf.len(), DEFAULT_BUF_SIZE);
    }

    #[tokio::test]
    async fn test_buf_writer_buffer() {
        let data = vec![0; 1024];
        let buf_writer = BufWriter::new(Buffer::new(data));

        let buffer = buf_writer.buffer();
        assert_eq!(buffer.len(), 0);
    }

    #[tokio::test]
    async fn test_buf_writer_capacity() {
        let data = vec![0; 1024];
        let buf_writer = BufWriter::new(Buffer::new(data));

        let capacity = buf_writer.capacity();
        assert_eq!(capacity, DEFAULT_BUF_SIZE);
    }

    #[tokio::test]
    async fn test_buf_writer_get_ref() {
        let data = vec![0; 1024];
        let buf_writer = BufWriter::new(Buffer::new(data));

        let inner = buf_writer.get_ref();
        assert_eq!(inner.pos, 0);
    }

    #[tokio::test]
    async fn test_buf_writer_get_mut() {
        let data = vec![0; 1024];
        let mut buf_writer = BufWriter::new(Buffer::new(data));

        let inner = buf_writer.get_mut();
        assert_eq!(inner.pos, 0);
    }

    struct Buffer {
        data: Vec<u8>,
        pos: usize,
    }

    impl Buffer {
        fn new(data: Vec<u8>) -> Self {
            Self { data, pos: 0 }
        }
    }

    impl Write for Buffer {
        async fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            if self.pos >= self.data.len() {
                return Ok(0);
            }
            let n = std::cmp::min(buf.len(), self.data.len() - self.pos);
            self.data[self.pos..self.pos + n].copy_from_slice(buf);
            self.pos += n;
            Ok(n)
        }

        async fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
}
