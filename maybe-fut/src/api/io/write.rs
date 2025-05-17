use std::io::IoSlice;

/// A trait for objects which are byte-oriented sinks.
///
/// Implementors of the [`Write`] trait are called 'writers'.
///
/// Writers are defined by two required methods, `write` and `flush`:
///
/// - The `write` method will attempt to write some data into the object, returning how many bytes were successfully written.
/// - The `flush` method is useful for adapters and explicit buffers themselves for ensuring that all buffered data has been pushed out to the ‘true sink’.
pub trait Write {
    /// Writes a buffer into this writer, returning how many bytes were successfully written.
    fn write(&mut self, buf: &[u8]) -> impl Future<Output = std::io::Result<usize>>;

    /// Flushes the output streamer, ensuring that all intermediately buffered contents reach their destination.
    fn flush(&mut self) -> impl Future<Output = std::io::Result<()>>;

    /// Like `write`, except that it writes from a slice of buffers.
    fn write_vectored(
        &mut self,
        bufs: &[IoSlice<'_>],
    ) -> impl Future<Output = std::io::Result<usize>> {
        async move {
            let mut total = 0;
            for buf in bufs.iter() {
                let n = self.write(buf).await?;
                total += n;
            }
            Ok(total)
        }
    }

    /// Attempts to write an entire buffer into this writer.
    fn write_all(&mut self, mut buf: &[u8]) -> impl Future<Output = std::io::Result<()>> {
        async move {
            while !buf.is_empty() {
                let n = self.write(buf).await?;
                if n == 0 {
                    break;
                } else {
                    buf = &buf[n..];
                }
            }
            Ok(())
        }
    }
}
