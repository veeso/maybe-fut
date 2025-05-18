use super::Write;

/// A writer which will move data into the void.
///
/// This struct is generally created by calling [`sink`].
#[derive(Debug, Clone, Copy, Default)]
pub struct Sink;

impl Write for Sink {
    async fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // This is a no-op, so we just return the length of the buffer.
        Ok(buf.len())
    }

    async fn flush(&mut self) -> std::io::Result<()> {
        // This is a no-op, so we just return Ok.
        Ok(())
    }
}

/// Creates a new [`Sink`] instance.
pub const fn sink() -> Sink {
    Sink
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::io::Write;

    #[tokio::test]
    async fn test_sink() {
        let mut sink = sink();
        let buf = b"Hello, world!";
        let n = sink.write(buf).await.unwrap();
        assert_eq!(n, buf.len());
        assert!(sink.flush().await.is_ok());
    }
}
