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
    inner: R,
}

const DEFAULT_BUF_SIZE: usize = 4096;

impl<R: Read> BufReader<R> {
    /// Creates a new BufReader with the default buffer size.
    pub fn new(inner: R) -> Self {
        Self::with_capacity(DEFAULT_BUF_SIZE, inner)
    }

    /// Creates a new BufReader with the specified buffer size.
    pub fn with_capacity(capacity: usize, inner: R) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
            inner,
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
        &self.buf
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
        if self.buf.is_empty() {
            let n = self.inner.read(&mut self.buf).await?;
            if n == 0 {
                return Ok(&self.buf);
            }
        }
        Ok(&self.buf)
    }

    async fn consume(&mut self, amount: usize) {
        self.buf.drain(..amount);
    }
}
