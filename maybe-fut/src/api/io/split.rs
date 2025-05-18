use super::BufRead;

#[derive(Debug)]
pub struct Split<B> {
    pub(crate) buf: B,
    pub(crate) delim: u8,
}

impl<B: BufRead> Split<B> {
    /// Returns next token from the buffer.
    pub async fn next(&mut self) -> Option<std::io::Result<Vec<u8>>> {
        let mut buf = Vec::new();
        match self.buf.read_until(self.delim, &mut buf).await {
            Ok(0) => None,
            Ok(_n) => {
                if buf[buf.len() - 1] == self.delim {
                    buf.pop();
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
    async fn test_should_return_tokens() {
        let data = b"line1|line2|line3";
        let buf = BufReader::new(Buffer::new(data.to_vec()));
        let mut tokens = Split { buf, delim: b'|' };

        assert_eq!(tokens.next().await.unwrap().unwrap(), b"line1");
        assert_eq!(tokens.next().await.unwrap().unwrap(), b"line2");
        assert_eq!(tokens.next().await.unwrap().unwrap(), b"line3");
        assert!(tokens.next().await.is_none());
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
