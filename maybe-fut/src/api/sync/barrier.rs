use crate::maybe_fut_constructor_sync;

/// A barrier enables multiple threads to synchronize the beginning of some computation.
#[derive(Debug, Unwrap)]
#[unwrap_types(
    std(std::sync::Barrier),
    tokio(tokio::sync::Barrier),
    tokio_gated("tokio-sync")
)]
pub struct Barrier(BarrierInner);

/// Inner wrapper for [`Barrier`].
#[derive(Debug)]
enum BarrierInner {
    /// Std barrier.
    Std(std::sync::Barrier),
    /// Tokio barrier.
    #[cfg(tokio_sync)]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio-sync")))]
    Tokio(tokio::sync::Barrier),
}

impl From<std::sync::Barrier> for Barrier {
    fn from(barrier: std::sync::Barrier) -> Self {
        Self(BarrierInner::Std(barrier))
    }
}

#[cfg(tokio_sync)]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-sync")))]
impl From<tokio::sync::Barrier> for Barrier {
    fn from(barrier: tokio::sync::Barrier) -> Self {
        Self(BarrierInner::Tokio(barrier))
    }
}

impl Barrier {
    maybe_fut_constructor_sync!(
        /// Creates a new barrier that can block a given number of threads.
        ///
        /// A barrier will block n-1 threads which call [`Self::wait`] and then wake up all threads at once when the `n`th thread calls [`Self::wait`].
        new(n: usize) -> Self,
        std::sync::Barrier::new,
        tokio::sync::Barrier::new,
        tokio_sync
    );

    /// Blocks the current thread until all threads have rendezvoused here.
    ///
    /// Barriers are re-usable after all threads have rendezvoused once, and can be used continuously.
    pub async fn wait(&self) -> BarrierWaitResult {
        match &self.0 {
            BarrierInner::Std(barrier) => barrier.wait().into(),
            #[cfg(tokio_sync)]
            BarrierInner::Tokio(barrier) => barrier.wait().await.into(),
        }
    }
}

/// Result of a [`Barrier`] [`Barrier::wait`] operation.
#[derive(Debug)]
pub struct BarrierWaitResult(InnerBarrierWaitResult);

/// Inner wrapper for [`BarrierWaitResult`].
#[derive(Debug)]
enum InnerBarrierWaitResult {
    /// Std barrier wait result.
    Std(std::sync::BarrierWaitResult),
    /// Tokio barrier wait result.
    #[cfg(tokio_sync)]
    Tokio(tokio::sync::BarrierWaitResult),
}

impl From<std::sync::BarrierWaitResult> for BarrierWaitResult {
    fn from(result: std::sync::BarrierWaitResult) -> Self {
        Self(InnerBarrierWaitResult::Std(result))
    }
}

#[cfg(tokio_sync)]
impl From<tokio::sync::BarrierWaitResult> for BarrierWaitResult {
    fn from(result: tokio::sync::BarrierWaitResult) -> Self {
        Self(InnerBarrierWaitResult::Tokio(result))
    }
}

impl BarrierWaitResult {
    /// Returns `true` if this thread is the "leader thread" for the call to [`Barrier::wait`].
    ///
    /// Only one thread will have `true` returned from their result, all other threads will have `false` returned.
    pub fn is_leader(&self) -> bool {
        match &self.0 {
            InnerBarrierWaitResult::Std(result) => result.is_leader(),
            #[cfg(tokio_sync)]
            InnerBarrierWaitResult::Tokio(result) => result.is_leader(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_should_create_barrier_sync() {
        let barrier = Barrier::new(1);
        assert!(matches!(barrier.0, BarrierInner::Std(_)));
    }

    #[cfg(tokio_sync)]
    #[tokio::test]
    async fn test_should_create_barrier_async() {
        let barrier = Barrier::new(1);
        assert!(matches!(barrier.0, BarrierInner::Tokio(_)));
    }

    #[test]
    fn test_should_create_barrier_wait_result_sync() {
        let barrier = Barrier::new(1);
        let result = crate::SyncRuntime::block_on(barrier.wait());
        assert!(matches!(result.0, InnerBarrierWaitResult::Std(_)));
    }

    #[cfg(tokio_sync)]
    #[tokio::test]
    async fn test_should_create_barrier_wait_result_async() {
        let barrier = Barrier::new(1);
        let result = barrier.wait().await;
        assert!(matches!(result.0, InnerBarrierWaitResult::Tokio(_)));
    }
}
