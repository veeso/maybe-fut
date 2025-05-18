use std::io::SeekFrom;

use super::{Read, Seek, Write};

/// Empty ignores any data written via [`Write`], and will always be empty (returning zero bytes) when read via [`Read`].
///
/// This struct is generally created by calling [`empty`]. Please see the documentation of [`empty`] for more details.
#[derive(Debug, Clone, Copy, Default)]
pub struct Empty;

impl Write for Empty {
    async fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // This is a no-op, so we just return the length of the buffer.
        Ok(buf.len())
    }

    async fn flush(&mut self) -> std::io::Result<()> {
        // This is a no-op, so we just return Ok.
        Ok(())
    }
}

impl Seek for Empty {
    async fn seek(&mut self, _pos: SeekFrom) -> std::io::Result<u64> {
        // This is a no-op, so we just return Ok(0).
        Ok(0)
    }
}

impl Read for Empty {
    async fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        // This is a no-op, so we just return Ok(0).
        Ok(0)
    }
}

/// Creates a new [`Empty`] instance.
pub fn empty() -> Empty {
    Empty
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::io::{Read, Write};

    #[tokio::test]
    async fn test_empty() {
        let mut empty = empty();
        let buf = b"Hello, world!";
        let n = empty.write(buf).await.unwrap();
        assert_eq!(n, buf.len());
        assert!(empty.flush().await.is_ok());

        let mut read_buf = [0; 13];
        let n = empty.read(&mut read_buf).await.unwrap();
        assert_eq!(n, 0);
    }
}
