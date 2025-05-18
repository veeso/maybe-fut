//! Traits, helpers, and type definitions for core I/O functionality.
//!
//! The `io` module contains a number of common things you'll need when doing input and output.
//!
//! Reference:
//!
//! - std: <https://doc.rust-lang.org/std/io/index.html>
//! - tokio: <https://docs.rs/tokio/latest/tokio/io/index.html>

mod buf_reader;
mod buf_writer;
mod empty;
mod lines;
mod read;
mod repeat;
mod seek;
mod sink;
mod split;
mod stderr;
mod stdin;
mod stdout;
mod write;

pub use self::buf_reader::{BufRead, BufReader};
pub use self::buf_writer::BufWriter;
pub use self::empty::{Empty, empty};
pub use self::lines::Lines;
pub use self::read::Read;
pub use self::repeat::{Repeat, repeat};
pub use self::seek::Seek;
pub use self::sink::{Sink, sink};
pub use self::split::Split;
pub use self::stderr::{Stderr, stderr};
pub use self::stdin::{Stdin, stdin};
pub use self::stdout::{Stdout, stdout};
pub use self::write::Write;

/// Copies the entire contents of a reader into a writer.
///
/// This function will continuously read data from reader and then write it into writer in a streaming fashion until reader returns EOF.
///
/// On success, the total number of bytes that were copied from reader to writer is returned.
pub async fn copy<R, W>(reader: &mut R, writer: &mut W) -> std::io::Result<u64>
where
    R: Read + ?Sized,
    W: Write + ?Sized,
{
    let mut total = 0;
    let mut buf = [0; 8192];
    loop {
        let n = reader.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        writer.write_all(&buf[..n]).await?;
        total += n as u64;
    }
    Ok(total)
}

/// Reads all bytes from a reader into a new [`String`].
///
/// This is a convenience function for [`Read::read_to_string`].
///
/// Using this function avoids having to create a variable first and
/// provides more type safety since you can only get the buffer out if there were no errors
pub async fn read_to_string<R>(reader: &mut R) -> std::io::Result<String>
where
    R: Read + ?Sized,
{
    reader.read_to_string().await
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_copy() {
        let mut reader = Buffer::new(vec![b'A'; 8192]);
        let mut writer = sink();
        let total = copy(&mut reader, &mut writer).await.unwrap();
        assert_eq!(total, 8192);
    }

    #[tokio::test]
    async fn test_read_to_string() {
        let mut reader = Buffer::new(vec![b'A'; 8192]);
        let result = read_to_string(&mut reader).await.unwrap();
        assert_eq!(result, "A".repeat(8192));
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
