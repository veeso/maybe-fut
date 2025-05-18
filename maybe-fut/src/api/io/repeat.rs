use super::Read;

/// A reader which yields one byte over and over and over and over and over and…
///
/// This struct is generally created by calling [`repeat`]. Please see the documentation of [`repeat`] for more details.
#[derive(Debug, Clone, Copy, Default)]
pub struct Repeat {
    byte: u8,
}

impl Read for Repeat {
    async fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // Fill the buffer with the byte.
        for b in buf.iter_mut() {
            *b = self.byte;
        }
        Ok(buf.len())
    }
}

/// Creates a new [`Repeat`] instance with the specified byte to repeat.
pub const fn repeat(byte: u8) -> Repeat {
    Repeat { byte }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::io::Read;

    #[tokio::test]
    async fn test_repeat() {
        let mut repeat = repeat(b'A');
        let mut buf = [0; 10];
        let n = repeat.read(&mut buf).await.unwrap();
        assert_eq!(n, buf.len());
        assert_eq!(buf, [b'A'; 10]);
    }
}
