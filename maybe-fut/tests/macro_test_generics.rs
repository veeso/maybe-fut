//! This module contains the test for the `maybe_fut` macro for generics.

use std::fmt::Display;

use maybe_fut_derive::maybe_fut;

#[derive(Debug, Clone, Copy)]
struct TestStruct<T: Sized + Copy + Display> {
    value: T,
}

#[crate::maybe_fut(
    sync = SyncTestStruct,
    tokio = TokioTestStruct,
    tokio_feature = "tokio",
)]
impl<T> TestStruct<T>
where
    T: Sized + Copy + Display,
{
    /// Creates a new [`TestStruct`] instance.
    pub fn new(value: T) -> Self {
        Self { value }
    }

    /// Create a tempfile
    pub async fn create_tempfile() -> Result<(), std::io::Error> {
        let tempdir = tempfile::tempdir()?;

        let path = tempdir.path().join("test.txt");
        maybe_fut::fs::File::create(&path).await?;

        Ok(())
    }

    pub fn value(&self) -> T {
        self.value
    }

    #[inline]
    const fn life_meaning() -> u64 {
        42
    }
}

/// A trait to greet the user.
pub trait Greet {
    /// Greets the user with a message.
    fn greet(&self) -> String;

    // Greets the user with a message asynchronously.
    fn greet_async(&self) -> impl Future<Output = String>;
}

#[crate::maybe_fut(
    sync = SyncTestStruct,
    tokio = TokioTestStruct,
    tokio_feature = "tokio",
)]
impl<T> Greet for TestStruct<T>
where
    T: Sized + Copy + Display,
{
    fn greet(&self) -> String {
        format!("Hello, I'm {}", self.value)
    }

    async fn greet_async(&self) -> String {
        format!("Hello, I'm {}", self.value)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn test_should_proc_derive_async() {
        let test_struct: TokioTestStruct<u64> = TokioTestStruct::new(96);
        assert_eq!(test_struct.value(), 96);

        let result = TokioTestStruct::<u64>::create_tempfile().await;
        assert!(result.is_ok());

        assert_eq!(SyncTestStruct::<u64>::life_meaning(), 42);

        test_struct.greet();
        test_struct.greet_async().await;
    }

    #[test]
    fn test_should_proc_derive_sync() {
        let test_struct: SyncTestStruct<u64> = SyncTestStruct::new(96);
        assert_eq!(test_struct.value(), 96);

        let result = SyncTestStruct::<u64>::create_tempfile();
        assert!(result.is_ok());

        assert_eq!(SyncTestStruct::<u64>::life_meaning(), 42);

        test_struct.greet();
    }
}
