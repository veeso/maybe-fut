use std::fmt::Display;
use std::ops::{Deref, DerefMut};

/// RAII structure used to release the shared write access of a lock when dropped.
///
/// This structure is created by the [`super::RwLock::write`] and [`super::RwLock::try_write`] methods on [`super::RwLock`].
#[derive(Debug)]
pub struct RwLockWriteGuard<'a, T: ?Sized + 'a>(InnerRwLockWriteGuard<'a, T>);

#[derive(Debug)]
enum InnerRwLockWriteGuard<'a, T: ?Sized + 'a> {
    Std(std::sync::RwLockWriteGuard<'a, T>),
    #[cfg(tokio_sync)]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio-sync")))]
    Tokio(tokio::sync::RwLockWriteGuard<'a, T>),
}

impl<'a, T> From<std::sync::RwLockWriteGuard<'a, T>> for RwLockWriteGuard<'a, T> {
    fn from(guard: std::sync::RwLockWriteGuard<'a, T>) -> Self {
        Self(InnerRwLockWriteGuard::Std(guard))
    }
}

#[cfg(tokio_sync)]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-sync")))]
impl<'a, T> From<tokio::sync::RwLockWriteGuard<'a, T>> for RwLockWriteGuard<'a, T> {
    fn from(guard: tokio::sync::RwLockWriteGuard<'a, T>) -> Self {
        Self(InnerRwLockWriteGuard::Tokio(guard))
    }
}

impl<'a, T> Deref for RwLockWriteGuard<'a, T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match &self.0 {
            InnerRwLockWriteGuard::Std(guard) => guard.deref(),
            #[cfg(tokio_sync)]
            InnerRwLockWriteGuard::Tokio(guard) => guard.deref(),
        }
    }
}

impl<'a, T> DerefMut for RwLockWriteGuard<'a, T>
where
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        match &mut self.0 {
            InnerRwLockWriteGuard::Std(guard) => guard.deref_mut(),
            #[cfg(tokio_sync)]
            InnerRwLockWriteGuard::Tokio(guard) => guard.deref_mut(),
        }
    }
}

impl Display for RwLockWriteGuard<'_, str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            InnerRwLockWriteGuard::Std(guard) => guard.fmt(f),
            #[cfg(tokio_sync)]
            InnerRwLockWriteGuard::Tokio(guard) => guard.fmt(f),
        }
    }
}
