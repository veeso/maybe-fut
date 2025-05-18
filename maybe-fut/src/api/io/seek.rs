use std::io::SeekFrom;

/// The [`Seek`] trait provides a cursor which can be moved within a stream of bytes.
/// The stream typically has a fixed size, allowing seeking relative to either end or the current offset.
pub trait Seek {
    /// Moves the cursor to a new position within the stream.
    fn seek(&mut self, pos: SeekFrom) -> impl Future<Output = std::io::Result<u64>>;

    /// Rewind to the beginning of a stream.
    ///
    /// This is a convenience method, equivalent to `self.seek(SeekFrom::Start(0))`.
    fn rewind(&mut self) -> impl Future<Output = std::io::Result<u64>> {
        self.seek(SeekFrom::Start(0))
    }

    /// Returns the current seek position from the start of the stream.
    ///
    /// This is equivalent to `self.seek(SeekFrom::Current(0))`.
    fn stream_position(&mut self) -> impl Future<Output = std::io::Result<u64>> {
        self.seek(SeekFrom::Current(0))
    }

    /// Seeks relative to the current position.
    ///
    /// This is equivalent to `self.seek(SeekFrom::Current(offset))` but doesn’t return the new position which can allow some implementations such as [`std::io::BufReader`] to perform more efficient seeks.
    fn seek_relative(&mut self, offset: i64) -> impl Future<Output = std::io::Result<u64>> {
        self.seek(SeekFrom::Current(offset))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct MockSeek {
        position: u64,
        max_size: i64,
    }

    impl MockSeek {
        fn new(max_size: i64) -> Self {
            Self {
                position: 0,
                max_size,
            }
        }
    }

    impl Seek for MockSeek {
        fn seek(&mut self, pos: SeekFrom) -> impl Future<Output = std::io::Result<u64>> {
            async move {
                match pos {
                    SeekFrom::Start(offset) => {
                        self.position = offset;
                    }
                    SeekFrom::Current(offset) => {
                        self.position = self.position.saturating_add(offset as u64);
                    }
                    SeekFrom::End(offset) => {
                        self.position = self.max_size.saturating_add(offset) as u64;
                    }
                }
                Ok(self.position)
            }
        }
    }

    #[tokio::test]
    async fn test_seek() {
        let mut seek = MockSeek::new(50);
        assert_eq!(seek.seek(SeekFrom::Start(10)).await.unwrap(), 10);
        assert_eq!(seek.seek(SeekFrom::Current(5)).await.unwrap(), 15);
        assert_eq!(seek.seek(SeekFrom::End(-5)).await.unwrap(), 45);
    }
}
