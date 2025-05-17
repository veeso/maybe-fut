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
