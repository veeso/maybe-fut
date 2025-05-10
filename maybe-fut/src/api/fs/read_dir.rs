use super::DirEntry;

#[derive(Debug)]
/// Reads the entries in a directory.
///
/// This struct is returned from the [`super::read_dir`] function of this module and will yield instances of [`DirEntry`].
/// Through a [`DirEntry`] information like the entry’s path and possibly other metadata can be learned.
pub struct ReadDir(ReadDirInner);

/// Inner pointer to sync or async read dir.
#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
enum ReadDirInner {
    /// Std variant of file <https://docs.rs/rustc-std-workspace-std/latest/std/fs/struct.ReadDir.html>
    Std(std::fs::ReadDir),
    #[cfg(tokio_fs)]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio-fs")))]
    /// Tokio variant of file <https://docs.rs/tokio/latest/tokio/fs/struct.ReadDir.html>
    Tokio(tokio::fs::ReadDir),
}

impl From<std::fs::ReadDir> for ReadDir {
    fn from(inner: std::fs::ReadDir) -> Self {
        Self(ReadDirInner::Std(inner))
    }
}

#[cfg(tokio_fs)]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-fs")))]
impl From<tokio::fs::ReadDir> for ReadDir {
    fn from(inner: tokio::fs::ReadDir) -> Self {
        Self(ReadDirInner::Tokio(inner))
    }
}

impl ReadDir {
    /// Returns the next entry in the directory stream.
    pub async fn next_entry(&mut self) -> std::io::Result<Option<DirEntry>> {
        match &mut self.0 {
            ReadDirInner::Std(inner) => inner
                .next()
                .map(|entry| entry.map(DirEntry::from))
                .transpose(),
            #[cfg(tokio_fs)]
            #[cfg_attr(docsrs, doc(cfg(feature = "tokio-fs")))]
            ReadDirInner::Tokio(inner) => {
                inner.next_entry().await.map(|res| res.map(DirEntry::from))
            }
        }
    }
}
