//! Sync contains the runtime to execute async code when working in sync context.

use std::pin::Pin;
use std::task::{Context, Poll, Waker};

/// A runtime to execute sync code without async context.
///
/// This type should be used only when exporting the sync api of a library using
/// maybe-fut to create an interoperable async/sync api.
pub struct SyncRuntime;

impl SyncRuntime {
    pub fn block_on<F>(mut f: F) -> F::Output
    where
        F: Future,
    {
        let mut f = unsafe { Pin::new_unchecked(&mut f) };

        let mut ctx = Context::from_waker(Waker::noop());

        let Poll::Ready(val) = f.as_mut().poll(&mut ctx) else {
            unreachable!("Future should not be pending in sync context");
        };

        val
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_execute_async_code_in_sync_context() {
        let result = SyncRuntime::block_on(async_fn());
        assert_eq!(result, 42);
    }

    async fn async_fn() -> i32 {
        42
    }
}
