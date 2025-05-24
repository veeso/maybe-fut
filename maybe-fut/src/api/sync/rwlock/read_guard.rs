use std::fmt::Display;
use std::ops::Deref;

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`super::RwLock::read`] and [`super::RwLock::try_read`] methods on [`super::RwLock`].
#[derive(Debug)]
pub struct RwLockReadGuard<'a, T: ?Sized + 'a>(InnerRwLockReadGuard<'a, T>);

#[derive(Debug)]
enum InnerRwLockReadGuard<'a, T: ?Sized + 'a> {
    Std(std::sync::RwLockReadGuard<'a, T>),
    #[cfg(tokio_sync)]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio-sync")))]
    Tokio(tokio::sync::RwLockReadGuard<'a, T>),
}

impl<'a, T> From<std::sync::RwLockReadGuard<'a, T>> for RwLockReadGuard<'a, T> {
    fn from(guard: std::sync::RwLockReadGuard<'a, T>) -> Self {
        RwLockReadGuard(InnerRwLockReadGuard::Std(guard))
    }
}

#[cfg(tokio_sync)]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-sync")))]
impl<'a, T> From<tokio::sync::RwLockReadGuard<'a, T>> for RwLockReadGuard<'a, T> {
    fn from(guard: tokio::sync::RwLockReadGuard<'a, T>) -> Self {
        RwLockReadGuard(InnerRwLockReadGuard::Tokio(guard))
    }
}

impl<'a, T> Deref for RwLockReadGuard<'a, T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match &self.0 {
            InnerRwLockReadGuard::Std(guard) => guard.deref(),
            #[cfg(tokio_sync)]
            InnerRwLockReadGuard::Tokio(guard) => guard.deref(),
        }
    }
}

impl Display for RwLockReadGuard<'_, str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            InnerRwLockReadGuard::Std(guard) => guard.fmt(f),
            #[cfg(tokio_sync)]
            InnerRwLockReadGuard::Tokio(guard) => guard.fmt(f),
        }
    }
}
