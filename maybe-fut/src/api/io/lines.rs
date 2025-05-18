use super::BufRead;

#[derive(Debug)]
pub struct Lines<B> {
    pub(crate) buf: B,
}

impl<B: BufRead> Lines<B> {
    /// Returns next line from the buffer.
    pub async fn next(&mut self) -> Option<std::io::Result<String>> {
        let mut buf = String::new();
        match self.buf.read_line(&mut buf).await {
            Ok(0) => None,
            Ok(_n) => {
                if buf.ends_with('\n') {
                    buf.pop();
                    if buf.ends_with('\r') {
                        buf.pop();
                    }
                }
                Some(Ok(buf))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::io::{BufReader, Read};

    #[tokio::test]
    async fn test_should_return_lines() {
        let data = b"line1\nline2\r\nline3\n";
        let buf = BufReader::new(Buffer::new(data.to_vec()));
        let mut lines = Lines { buf };

        assert_eq!(lines.next().await.unwrap().unwrap(), "line1");
        assert_eq!(lines.next().await.unwrap().unwrap(), "line2");
        assert_eq!(lines.next().await.unwrap().unwrap(), "line3");
        assert!(lines.next().await.is_none());
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

    impl Read for Buffer {
        async fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            if self.pos >= self.data.len() {
                return Ok(0);
            }
            let n = std::cmp::min(buf.len(), self.data.len() - self.pos);
            buf[..n].copy_from_slice(&self.data[self.pos..self.pos + n]);
            self.pos += n;
            Ok(n)
        }
    }
}
