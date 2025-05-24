mod guard;

use std::sync::{PoisonError, TryLockError};

pub use self::guard::MutexGuard;
use crate::maybe_fut_constructor_sync;

/// A mutual exclusion primitive useful for protecting shared data
///
/// This mutex will block threads waiting for the lock to become available.
/// The mutex can be created via a [`Mutex::new`] constructor.
/// Each mutex has a type parameter `<T>` which represents the data that it is protecting.
///
/// The data can only be accessed through the RAII guards returned from [`Mutex::lock`] and [`Mutex::try_lock`],
/// which guarantees that the data is only ever accessed when the mutex is locked.
pub struct Mutex<T>(MutexInner<T>);

/// Inner wrapper for [`Mutex`].
enum MutexInner<T> {
    /// Std mutex
    Std(std::sync::Mutex<T>),
    /// Tokio mutex
    #[cfg(tokio_sync)]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio-sync")))]
    Tokio(tokio::sync::Mutex<T>),
}

impl<T> From<std::sync::Mutex<T>> for Mutex<T> {
    fn from(mutex: std::sync::Mutex<T>) -> Self {
        Mutex(MutexInner::Std(mutex))
    }
}

#[cfg(tokio_sync)]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-sync")))]
impl<T> From<tokio::sync::Mutex<T>> for Mutex<T> {
    fn from(mutex: tokio::sync::Mutex<T>) -> Self {
        Mutex(MutexInner::Tokio(mutex))
    }
}

impl<T> Mutex<T>
where
    T: Sized,
{
    maybe_fut_constructor_sync!(
        /// Creates a new lock in an unlocked state ready for use.
        new(t: T) -> Self,
        std::sync::Mutex::new,
        tokio::sync::Mutex::new,
        tokio_sync
    );

    /// Clear the poisoned state from a mutex.
    ///
    /// If the mutex is poisoned, it will remain poisoned until this function is called.
    /// This allows recovering from a poisoned state and marking that it has recovered.
    /// For example, if the value is overwritten by a known-good value, then the mutex can be marked as un-poisoned.
    ///
    /// If the inner type is a [`tokio::sync::Mutex`], this function is a no-op.
    pub fn clear_poison(&self) {
        if let MutexInner::Std(mutex) = &self.0 {
            mutex.clear_poison();
        }
    }

    /// Returns `true` if the mutex is poisoned.
    ///
    /// If the inner type is a [`tokio::sync::Mutex`], this function will always return `false`
    pub fn is_poisoned(&self) -> bool {
        match &self.0 {
            MutexInner::Std(mutex) => mutex.is_poisoned(),
            #[cfg(tokio_sync)]
            MutexInner::Tokio(_) => false, // Tokio mutexes are not poisoned
        }
    }

    /// Acquires a mutex, blocking the current thread until it is able to do so.
    ///
    /// This function will block the local thread until it is available to acquire the mutex.
    /// Upon returning, the thread is the only thread with the lock held. An RAII guard is returned to allow scoped
    /// unlock of the lock. When the guard goes out of scope, the mutex will be unlocked.
    pub async fn lock(
        &self,
    ) -> Result<MutexGuard<'_, T>, PoisonError<std::sync::MutexGuard<'_, T>>> {
        match &self.0 {
            MutexInner::Std(mutex) => {
                let guard = mutex.lock()?;
                Ok(MutexGuard::from(guard))
            }
            #[cfg(tokio_sync)]
            MutexInner::Tokio(mutex) => {
                let guard = mutex.lock().await;
                Ok(MutexGuard::from(guard))
            }
        }
    }

    /// Attempts to acquire this lock.
    ///
    /// If the lock could not be acquired at this time, then [`TryLockError`] is returned.
    /// Otherwise, an RAII guard is returned.
    /// The lock will be unlocked when the guard is dropped.
    pub async fn try_lock(
        &self,
    ) -> Result<MutexGuard<'_, T>, TryLockError<std::sync::MutexGuard<'_, T>>> {
        match &self.0 {
            MutexInner::Std(mutex) => {
                let guard = mutex.try_lock()?;
                Ok(MutexGuard::from(guard))
            }
            #[cfg(tokio_sync)]
            MutexInner::Tokio(mutex) => {
                let guard = mutex.try_lock().map_err(|_| TryLockError::WouldBlock)?;
                Ok(MutexGuard::from(guard))
            }
        }
    }
}

impl<T> From<T> for Mutex<T> {
    fn from(t: T) -> Self {
        Mutex::new(t)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::SyncRuntime;

    #[test]
    fn test_mutex_new_sync() {
        let mutex = Mutex::new(42);
        assert!(matches!(mutex.0, MutexInner::Std(_)));
    }

    #[cfg(tokio_sync)]
    #[tokio::test]
    async fn test_mutex_new_tokio_sync() {
        let mutex = Mutex::new(42);
        assert!(matches!(mutex.0, MutexInner::Tokio(_)));
    }

    #[test]
    fn test_should_lock_sync_mutex() {
        let mutex = Mutex::new(42);
        let guard = SyncRuntime::block_on(mutex.lock());
        assert_eq!(*guard.unwrap(), 42);
    }

    #[cfg(tokio_sync)]
    #[tokio::test]
    async fn test_should_lock_tokio_mutex() {
        let mutex = Mutex::new(42);
        let guard = mutex.lock().await;
        assert_eq!(*guard.unwrap(), 42);
    }

    #[test]
    fn test_should_try_lock_sync_mutex() {
        let mutex = Mutex::new(42);
        let guard = SyncRuntime::block_on(mutex.try_lock());
        assert_eq!(*guard.unwrap(), 42);
    }

    #[cfg(tokio_sync)]
    #[tokio::test]
    async fn test_should_try_lock_tokio_mutex() {
        let mutex = Mutex::new(42);
        let guard = mutex.try_lock().await;
        assert_eq!(*guard.unwrap(), 42);
    }

    #[test]
    fn test_mutex_poisoned_sync() {
        let mutex = Mutex::new(42);
        let _guard = SyncRuntime::block_on(mutex.lock()).unwrap();
        mutex.clear_poison();
        assert!(!mutex.is_poisoned());
    }
}
