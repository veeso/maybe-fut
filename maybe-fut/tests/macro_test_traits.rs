//! This module contains the test for the `maybe_fut` macro for traits.

use maybe_fut_derive::maybe_fut;

#[derive(Debug, Clone, Copy)]
struct TestStruct {
    value: u64,
}

#[crate::maybe_fut(
    sync = SyncTestStruct,
    tokio = TokioTestStruct,
    tokio_feature = "tokio",
)]
impl TestStruct {
    /// Creates a new [`TestStruct`] instance.
    pub fn new(value: u64) -> Self {
        Self { value }
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
impl Greet for TestStruct {
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
        let result = TokioTestStruct::new(96);

        println!("{}", result.greet());
    }

    #[test]
    fn test_should_proc_derive_sync() {
        let result = SyncTestStruct::new(96);

        println!("{}", result.greet());
    }
}
