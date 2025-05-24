mod read_guard;
mod write_guard;

pub use self::read_guard::RwLockReadGuard;
pub use self::write_guard::RwLockWriteGuard;
use crate::maybe_fut_constructor_sync;

/// A reader-writer lock.
///
/// This type of lock allows a number of readers or at most one writer at any point in time.
/// The write portion of this lock typically allows modification of the underlying data (exclusive access)
/// and the read portion of this lock typically allows for read-only access (shared access).
pub struct RwLock<T: Sized>(RwLockInner<T>);

enum RwLockInner<T: Sized> {
    Std(std::sync::RwLock<T>),
    #[cfg(tokio_sync)]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio-sync")))]
    Tokio(tokio::sync::RwLock<T>),
}

impl<T> From<std::sync::RwLock<T>> for RwLock<T>
where
    T: Sized,
{
    fn from(rwlock: std::sync::RwLock<T>) -> Self {
        RwLock(RwLockInner::Std(rwlock))
    }
}

#[cfg(tokio_sync)]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-sync")))]
impl<T> From<tokio::sync::RwLock<T>> for RwLock<T> {
    fn from(rwlock: tokio::sync::RwLock<T>) -> Self {
        RwLock(RwLockInner::Tokio(rwlock))
    }
}

impl<T> RwLock<T>
where
    T: Sized,
{
    maybe_fut_constructor_sync!(
        /// Creates a new instance of an [`RwLock`] which is unlocked.
        new(t: T) -> Self,
        std::sync::RwLock::new,
        tokio::sync::RwLock::new,
        tokio_sync
    );

    /// Clear the poisoned state from a read-write lock.
    ///
    /// If the lock is poisoned, it will remain poisoned until this function is called.
    /// This allows recovering from a poisoned state and marking that it has recovered.
    /// For example, if the value is overwritten by a known-good value, then the lock can be marked as un-poisoned.
    ///
    /// If the inner lock is a Tokio lock, this function will do nothing.
    pub fn clear_poison(&self) {
        if let RwLockInner::Std(lock) = &self.0 {
            lock.clear_poison();
        }
    }

    /// Returns `true` if the lock is poisoned.
    pub fn is_poisoned(&self) -> bool {
        match &self.0 {
            RwLockInner::Std(lock) => lock.is_poisoned(),
            #[cfg(tokio_sync)]
            RwLockInner::Tokio(_) => false, // Tokio locks are not poisoned
        }
    }

    /// Locks this RwLock with shared read access, blocking the current thread until it can be acquired.
    pub async fn read(
        &self,
    ) -> Result<RwLockReadGuard<'_, T>, std::sync::PoisonError<std::sync::RwLockReadGuard<'_, T>>>
    {
        match &self.0 {
            RwLockInner::Std(lock) => Ok(RwLockReadGuard::from(lock.read()?)),
            #[cfg(tokio_sync)]
            RwLockInner::Tokio(lock) => Ok(RwLockReadGuard::from(lock.read().await)),
        }
    }

    /// Attempts to lock this RwLock with shared read access, returning immediately if it cannot be acquired.
    pub async fn try_read(
        &self,
    ) -> Result<RwLockReadGuard<'_, T>, std::sync::TryLockError<std::sync::RwLockReadGuard<'_, T>>>
    {
        match &self.0 {
            RwLockInner::Std(lock) => Ok(RwLockReadGuard::from(lock.try_read()?)),
            #[cfg(tokio_sync)]
            RwLockInner::Tokio(lock) => Ok(RwLockReadGuard::from(
                lock.try_read()
                    .map_err(|_| std::sync::TryLockError::WouldBlock)?,
            )),
        }
    }

    /// Locks this RwLock with exclusive write access, blocking the current thread until it can be acquired.
    pub async fn write(
        &self,
    ) -> Result<RwLockWriteGuard<'_, T>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, T>>>
    {
        match &self.0 {
            RwLockInner::Std(lock) => Ok(RwLockWriteGuard::from(lock.write()?)),
            #[cfg(tokio_sync)]
            RwLockInner::Tokio(lock) => Ok(RwLockWriteGuard::from(lock.write().await)),
        }
    }

    /// Attempts to lock this RwLock with exclusive write access, returning immediately if it cannot be acquired.
    pub async fn try_write(
        &self,
    ) -> Result<RwLockWriteGuard<'_, T>, std::sync::TryLockError<std::sync::RwLockWriteGuard<'_, T>>>
    {
        match &self.0 {
            RwLockInner::Std(lock) => Ok(RwLockWriteGuard::from(lock.try_write()?)),
            #[cfg(tokio_sync)]
            RwLockInner::Tokio(lock) => Ok(RwLockWriteGuard::from(
                lock.try_write()
                    .map_err(|_| std::sync::TryLockError::WouldBlock)?,
            )),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::SyncRuntime;

    #[test]
    fn test_rwlock_new_sync() {
        let rwlock = RwLock::new(42);
        assert!(matches!(rwlock.0, RwLockInner::Std(_)));
    }

    #[cfg(tokio_sync)]
    #[tokio::test]
    async fn test_rwlock_new_tokio() {
        let rwlock = RwLock::new(42);
        assert!(matches!(rwlock.0, RwLockInner::Tokio(_)));
    }

    #[test]
    fn test_rwlock_clear_poison() {
        let rwlock = RwLock::new(42);
        assert!(!rwlock.is_poisoned());
        rwlock.clear_poison();
        assert!(!rwlock.is_poisoned());
    }

    #[test]
    fn test_rwlock_read() {
        let rwlock = RwLock::new(42);
        let read_guard = SyncRuntime::block_on(rwlock.read()).unwrap();
        assert_eq!(*read_guard, 42);
    }

    #[cfg(tokio_sync)]
    #[tokio::test]
    async fn test_rwlock_read_tokio() {
        let rwlock = RwLock::new(42);
        let read_guard = rwlock.read().await.unwrap();
        assert_eq!(*read_guard, 42);
    }

    #[test]
    fn test_rwlock_try_read() {
        let rwlock = RwLock::new(42);
        let read_guard = SyncRuntime::block_on(rwlock.try_read()).unwrap();
        assert_eq!(*read_guard, 42);
    }

    #[cfg(tokio_sync)]
    #[tokio::test]
    async fn test_rwlock_try_read_tokio() {
        let rwlock = RwLock::new(42);
        let read_guard = rwlock.try_read().await.unwrap();
        assert_eq!(*read_guard, 42);
    }

    #[test]
    fn test_rwlock_write() {
        let rwlock = RwLock::new(42);
        let mut write_guard = SyncRuntime::block_on(rwlock.write()).unwrap();
        *write_guard = 43;
        assert_eq!(*write_guard, 43);

        // Test that the lock is still held after the write
        drop(write_guard);
        let read_guard = SyncRuntime::block_on(rwlock.read()).unwrap();
        assert_eq!(*read_guard, 43);
    }

    #[cfg(tokio_sync)]
    #[tokio::test]
    async fn test_rwlock_write_tokio() {
        let rwlock = RwLock::new(42);
        let mut write_guard = rwlock.write().await.unwrap();
        *write_guard = 43;
        assert_eq!(*write_guard, 43);

        // Test that the lock is still held after the write
        drop(write_guard);
        let read_guard = rwlock.read().await.unwrap();
        assert_eq!(*read_guard, 43);
    }

    #[test]
    fn test_rwlock_try_write() {
        let rwlock = RwLock::new(42);
        let mut write_guard = SyncRuntime::block_on(rwlock.try_write()).unwrap();
        *write_guard = 43;
        assert_eq!(*write_guard, 43);

        // Test that the lock is still held after the write
        drop(write_guard);
        let read_guard = SyncRuntime::block_on(rwlock.read()).unwrap();
        assert_eq!(*read_guard, 43);
    }

    #[cfg(tokio_sync)]
    #[tokio::test]
    async fn test_rwlock_try_write_tokio() {
        let rwlock = RwLock::new(42);
        let mut write_guard = rwlock.try_write().await.unwrap();
        *write_guard = 43;
        assert_eq!(*write_guard, 43);

        // Test that the lock is still held after the write
        drop(write_guard);
        let read_guard = rwlock.read().await.unwrap();
        assert_eq!(*read_guard, 43);
    }
}
