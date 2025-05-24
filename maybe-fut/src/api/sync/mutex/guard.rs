use std::fmt::Display;
use std::ops::{Deref, DerefMut};

/// An RAII implementation of a “scoped lck” of a mutex. When this structure is dropped (falls out of scope), the lock will be unlocked.
///
/// The data protected by the mutex can be accessed through this guard via its [`Deref`] and [`DerefMut`] implementations.
///
/// This structure is created by the [`super::Mutex::lock`] and [`super::Mutex::try_lock`] methods on [`super::Mutex`].
#[derive(Debug)]
pub struct MutexGuard<'a, T: ?Sized + 'a>(MutexGuardInner<'a, T>);

#[derive(Debug)]
enum MutexGuardInner<'a, T: ?Sized + 'a> {
    /// Std mutex guard
    Std(std::sync::MutexGuard<'a, T>),
    /// Tokio mutex guard
    #[cfg(tokio_sync)]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio-sync")))]
    Tokio(tokio::sync::MutexGuard<'a, T>),
}

impl<'a, T> From<std::sync::MutexGuard<'a, T>> for MutexGuard<'a, T> {
    fn from(guard: std::sync::MutexGuard<'a, T>) -> Self {
        MutexGuard(MutexGuardInner::Std(guard))
    }
}

#[cfg(tokio_sync)]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-sync")))]
impl<'a, T> From<tokio::sync::MutexGuard<'a, T>> for MutexGuard<'a, T> {
    fn from(guard: tokio::sync::MutexGuard<'a, T>) -> Self {
        MutexGuard(MutexGuardInner::Tokio(guard))
    }
}

impl<'a, T> Deref for MutexGuard<'a, T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match &self.0 {
            MutexGuardInner::Std(guard) => guard.deref(),
            #[cfg(tokio_sync)]
            MutexGuardInner::Tokio(guard) => guard.deref(),
        }
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T>
where
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        match &mut self.0 {
            MutexGuardInner::Std(guard) => guard.deref_mut(),
            #[cfg(tokio_sync)]
            MutexGuardInner::Tokio(guard) => guard.deref_mut(),
        }
    }
}

impl Display for MutexGuard<'_, str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            MutexGuardInner::Std(guard) => guard.fmt(f),
            #[cfg(tokio_sync)]
            MutexGuardInner::Tokio(guard) => guard.fmt(f),
        }
    }
}
