//! This module contains the test for the `maybe_fut` macro.

use maybe_fut_derive::maybe_fut;

#[derive(Debug, Clone, Copy)]
struct TestStruct {
    value: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestError {
    TooSmall,
    NoLifeMeaning,
}

#[crate::maybe_fut(
    sync = SyncTestStruct,
    tokio = TokioTestStruct,
    tokio_feature = "tokio",
)]
impl TestStruct {
    /// Creates a new [`TestStruct`] instance.
    ///
    /// # Panics
    ///
    /// - Panics if `value` is less than 10.\
    /// - Panics if `value` is 42.
    pub fn new(value: u64) -> Self {
        Self::try_new(value).expect("Failed to create TestStruct")
    }

    /// Creates a new [`TestStruct`] instance.
    ///
    /// # Errors
    ///
    /// - Returns [`TestError::TooSmall`] if `value` is less than 10.
    /// - Returns [`TestError::NoLifeMeaning`] if `value` is 42.
    pub fn try_new(value: u64) -> Result<Self, TestError> {
        if value < 10 {
            return Err(TestError::TooSmall);
        }

        if value == 42 {
            return Err(TestError::NoLifeMeaning);
        }

        Ok(TestStruct { value })
    }

    /// Creates a new [`TestStruct`] instance.
    ///
    /// Returns `None` if the value is less than 10 or equal to 42.
    pub fn try_new_opt(value: u64) -> Option<Self> {
        Self::try_new(value).ok()
    }

    /// Create a tempfile
    pub async fn create_tempfile() -> Result<(), std::io::Error> {
        let tempdir = tempfile::tempdir()?;

        let path = tempdir.path().join("test.txt");
        maybe_fut::fs::File::create(&path).await?;

        Ok(())
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    #[inline]
    const fn life_meaning() -> u64 {
        42
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn test_should_proc_derive_async() {
        let result = TokioTestStruct::try_new(96).expect("Failed to create TestStruct");
        assert_eq!(result.value(), 96);

        let result = TokioTestStruct::create_tempfile().await;
        assert!(result.is_ok());

        assert_eq!(SyncTestStruct::life_meaning(), 42);
    }

    #[test]
    fn test_should_proc_derive_sync() {
        let result = SyncTestStruct::try_new(96).expect("Failed to create TestStruct");
        assert_eq!(result.value(), 96);

        let result = SyncTestStruct::create_tempfile();
        assert!(result.is_ok());

        assert_eq!(SyncTestStruct::life_meaning(), 42);
    }
}
