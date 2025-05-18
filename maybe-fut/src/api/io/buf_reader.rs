use super::{Lines, Read, Split};

pub trait BufRead: Read {
    /// Returns the contents of the internal buffer, filling it with more data, via Read methods, if empty.
    fn fill_buf(&mut self) -> impl Future<Output = std::io::Result<&[u8]>>;

    /// Consumes the specified amount of data from the internal buffer.
    fn consume(&mut self, amount: usize) -> impl Future<Output = ()>;

    /// Reads bytes from the internal buffer until the specified byte is found.
    ///
    /// This function will read until the specified byte is found, including the byte itself.
    /// If the byte is not found, it will read until EOF.
    /// The read bytes will be appended to the provided buffer.
    /// Returns the number of bytes read.
    fn read_until(
        &mut self,
        byte: u8,
        buf: &mut Vec<u8>,
    ) -> impl Future<Output = std::io::Result<usize>> {
        async move {
            let mut read = 0;
            loop {
                let (done, used) = {
                    let available = match self.fill_buf().await {
                        Ok(n) => n,
                        Err(e) => return Err(e),
                    };
                    match memchr::memchr(byte, available) {
                        Some(i) => {
                            buf.extend_from_slice(&available[..=i]);
                            (true, i + 1)
                        }
                        None => {
                            buf.extend_from_slice(available);
                            (false, available.len())
                        }
                    }
                };
                self.consume(used).await;
                read += used;
                if done || used == 0 {
                    return Ok(read);
                }
            }
        }
    }

    /// Reads bytes from the internal buffer until the specified byte is found.
    ///
    /// This function will read until the specified byte is found, including the byte itself.
    fn skip_until(&mut self, byte: u8) -> impl Future<Output = std::io::Result<usize>> {
        async move {
            let mut read = 0;
            loop {
                let (done, used) = {
                    let available = match self.fill_buf().await {
                        Ok(n) => n,
                        Err(e) => return Err(e),
                    };
                    match memchr::memchr(byte, available) {
                        Some(i) => (true, i + 1),
                        None => (false, available.len()),
                    }
                };
                self.consume(used).await;
                read += used;
                if done || used == 0 {
                    return Ok(read);
                }
            }
        }
    }

    /// Reads a line from the internal buffer, appending it to the provided buffer.
    fn read_line(&mut self, buf: &mut String) -> impl Future<Output = std::io::Result<usize>> {
        async move {
            let mut read = 0;
            loop {
                let (done, used) = {
                    let available = match self.fill_buf().await {
                        Ok(n) => n,
                        Err(e) => return Err(e),
                    };
                    match memchr::memchr(b'\n', available) {
                        Some(i) => {
                            buf.push_str(std::str::from_utf8(&available[..=i]).unwrap());
                            (true, i + 1)
                        }
                        None => {
                            buf.push_str(std::str::from_utf8(available).unwrap());
                            (false, available.len())
                        }
                    }
                };
                self.consume(used).await;
                read += used;
                if done || used == 0 {
                    return Ok(read);
                }
            }
        }
    }

    /// Returns an iterator over the tokens of this reader, separated by the specified delimiter.
    fn split(self, delim: u8) -> Split<Self>
    where
        Self: Sized,
    {
        Split { buf: self, delim }
    }

    /// Returns an iterator over the lines of this reader.
    fn lines(self) -> Lines<Self>
    where
        Self: Sized,
    {
        Lines { buf: self }
    }
}

/// The BufReader<R> struct adds buffering to any reader.
///
/// It can be excessively inefficient to work directly with a [`Read`] instance.
/// For example, every call to read on TcpStream results in a system call. A BufReader<R> performs large, infrequent reads on the underlying Read and maintains an in-memory buffer of the results.
pub struct BufReader<R: ?Sized> {
    buf: Vec<u8>,
    filled: usize,
    pos: usize,
    inner: R,
}

const DEFAULT_BUF_SIZE: usize = 8192;

impl<R: Read> BufReader<R> {
    /// Creates a new BufReader with the default buffer size.
    pub fn new(inner: R) -> Self {
        Self::with_capacity(DEFAULT_BUF_SIZE, inner)
    }

    /// Creates a new BufReader with the specified buffer size.
    pub fn with_capacity(capacity: usize, inner: R) -> Self {
        Self {
            buf: vec![0; capacity],
            inner,
            filled: 0,
            pos: 0,
        }
    }

    /// Returns a reference to the inner reader.
    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    /// Returns a mutable reference to the inner reader.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    /// Returns a reference to the internal buffer.
    pub fn buffer(&self) -> &[u8] {
        &self.buf[self.pos..self.filled]
    }

    /// Returns the number of bytes the internal buffer can hold.
    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    /// Returns the underlying reader.
    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R: Read> Read for BufReader<R>
where
    R: ?Sized,
{
    async fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.buf.len() >= self.buf.capacity() {
            self.buf.clear();
            return self.inner.read(buf).await;
        }
        let rem = self.fill_buf().await?;
        let nread = rem.len();
        buf.copy_from_slice(rem);
        self.consume(nread).await;
        Ok(nread)
    }
}

impl<R> BufRead for BufReader<R>
where
    R: Read + ?Sized,
{
    async fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        if self.pos >= self.filled {
            self.pos = 0;
            self.filled = self.inner.read(&mut self.buf).await?;
        }

        Ok(&self.buf[self.pos..self.filled])
    }

    async fn consume(&mut self, amount: usize) {
        self.pos = std::cmp::min(self.pos + amount, self.filled);
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::io::Read;

    #[tokio::test]
    async fn test_should_fill_buf() {
        let data = b"line1\nline2\r\nline3\n";
        let mut buf = BufReader::new(Buffer::new(data.to_vec()));

        let buffer = buf.fill_buf().await.unwrap();
        assert_eq!(buffer, b"line1\nline2\r\nline3\n");
        assert_eq!(buf.capacity(), 8192);
        assert_eq!(buf.buffer(), b"line1\nline2\r\nline3\n");
    }

    #[tokio::test]
    async fn test_should_consume() {
        let data = b"line1\nline2\r\nline3\n";
        let mut buf = BufReader::new(Buffer::new(data.to_vec()));

        buf.consume(6).await;
        assert_eq!(buf.buffer(), []);
    }

    #[tokio::test]
    async fn test_should_read_until() {
        let data = b"line1|line2|line3";
        let mut buf = BufReader::new(Buffer::new(data.to_vec()));
        let mut result = vec![];

        let n = buf.read_until(b'|', &mut result).await.unwrap();
        assert_eq!(n, 6);
        assert_eq!(result, b"line1|");
        assert_eq!(buf.buffer(), b"line2|line3");
    }

    #[tokio::test]
    async fn test_should_skip_until() {
        let data = b"line1|line2|line3";
        let mut buf = BufReader::new(Buffer::new(data.to_vec()));

        let n = buf.skip_until(b'|').await.unwrap();
        assert_eq!(n, 6);
        assert_eq!(buf.buffer(), b"line2|line3");
    }

    #[tokio::test]
    async fn test_should_read_line() {
        let data = b"line1\nline2\r\nline3\n";
        let mut buf = BufReader::new(Buffer::new(data.to_vec()));
        let mut result = String::new();

        let n = buf.read_line(&mut result).await.unwrap();
        assert_eq!(n, 6);
        assert_eq!(result, "line1\n");
    }

    #[tokio::test]
    async fn test_should_split() {
        let data = b"line1|line2|line3";
        let buf = BufReader::new(Buffer::new(data.to_vec()));
        let mut tokens = buf.split(b'|');

        assert_eq!(tokens.next().await.unwrap().unwrap(), b"line1");
        assert_eq!(tokens.next().await.unwrap().unwrap(), b"line2");
        assert_eq!(tokens.next().await.unwrap().unwrap(), b"line3");
        assert!(tokens.next().await.is_none());
    }

    #[tokio::test]
    async fn test_should_lines() {
        let data = b"line1\nline2\r\nline3\n";
        let buf = BufReader::new(Buffer::new(data.to_vec()));
        let mut lines = buf.lines();

        assert_eq!(lines.next().await.unwrap().unwrap(), "line1");
        assert_eq!(lines.next().await.unwrap().unwrap(), "line2");
        assert_eq!(lines.next().await.unwrap().unwrap(), "line3");
        assert!(lines.next().await.is_none());
    }

    #[tokio::test]
    async fn test_should_read_bytes() {
        let data = b"line1\nline2\r\nline3\n";
        let mut buf = BufReader::new(Buffer::new(data.to_vec()));
        let mut result = vec![0; 13];

        let n = buf.read(&mut result).await.unwrap();
        assert_eq!(n, 13);
        assert_eq!(result, b"line1\nline2\r\n");
    }

    #[tokio::test]
    async fn test_should_into_inner() {
        let data = b"line1\nline2\r\nline3\n";
        let buf = BufReader::new(Buffer::new(data.to_vec()));
        let mut inner = buf.into_inner();

        assert_eq!(inner.read(&mut [0; 14]).await.unwrap(), 14);
    }

    #[tokio::test]
    async fn test_should_get_ref() {
        let data = b"line1\nline2\r\nline3\n";
        let buf = BufReader::new(Buffer::new(data.to_vec()));
        let inner = buf.get_ref();
        assert_eq!(inner.pos, 0);
    }

    #[tokio::test]
    async fn test_should_get_mut() {
        let data = b"line1\nline2\r\nline3\n";
        let mut buf = BufReader::new(Buffer::new(data.to_vec()));
        let inner = buf.get_mut();

        assert_eq!(inner.read(&mut [0; 14]).await.unwrap(), 14);
    }

    #[tokio::test]
    async fn test_should_capacity() {
        let data = b"line1\nline2\r\nline3\n";
        let buf = BufReader::new(Buffer::new(data.to_vec()));
        assert_eq!(buf.capacity(), 8192);
    }

    #[tokio::test]
    async fn test_should_buffer() {
        let data = b"line1\nline2\r\nline3\n";
        let buf = BufReader::new(Buffer::new(data.to_vec()));
        assert_eq!(buf.buffer(), []);
    }

    #[tokio::test]
    async fn test_should_with_capacity() {
        let data = b"line1\nline2\r\nline3\n";
        let buf = BufReader::with_capacity(1024, Buffer::new(data.to_vec()));
        assert_eq!(buf.capacity(), 1024);
    }

    #[tokio::test]
    async fn test_should_new() {
        let data = b"line1\nline2\r\nline3\n";
        let buf = BufReader::new(Buffer::new(data.to_vec()));
        assert_eq!(buf.capacity(), 8192);
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
