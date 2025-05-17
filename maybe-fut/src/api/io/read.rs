use std::io::IoSliceMut;

/// The [`Read`] trait provides an asynchronous interface for reading bytes from a source.
///
/// Implementors of the `Read` trait are called 'readers'.
pub trait Read {
    /// Reads data from the stream into the provided buffer.
    fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = std::io::Result<usize>>;

    fn read_vectored(
        &mut self,
        bufs: &mut [IoSliceMut<'_>],
    ) -> impl Future<Output = std::io::Result<usize>> {
        async move {
            let mut total = 0;
            for buf in bufs.iter_mut() {
                let n = self.read(buf).await?;
                total += n;
            }
            Ok(total)
        }
    }

    fn is_read_vectored(&self) -> bool {
        false
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> impl Future<Output = std::io::Result<usize>> {
        let mut probe = [0u8; 32];

        async move {
            let mut total = 0;
            loop {
                let n = self.read(&mut probe).await?;
                if n == 0 {
                    break;
                }
                buf.extend_from_slice(&probe[..n]);
                total += n;
            }
            Ok(total)
        }
    }

    fn read_to_string(&mut self) -> impl Future<Output = std::io::Result<String>> {
        let mut buf = Vec::new();
        async move {
            self.read_to_end(&mut buf).await?;
            String::from_utf8(buf)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        }
    }

    fn read_exact(&mut self, mut buf: &mut [u8]) -> impl Future<Output = std::io::Result<()>> {
        async move {
            while !buf.is_empty() {
                match self.read(buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        buf = &mut buf[n..];
                    }
                    Err(e) => return Err(e),
                }
            }
            if !buf.is_empty() {
                Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "failed to fill whole buffer",
                ))
            } else {
                Ok(())
            }
        }
    }
}
