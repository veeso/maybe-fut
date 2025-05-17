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
