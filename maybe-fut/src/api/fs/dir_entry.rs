use std::fs::FileType;

use crate::maybe_fut_method;

/// Entries returned by the [`super::ReadDir`] stream.
///
/// An instance of DirEntry represents an entry inside of a directory on the filesystem. Each entry can be inspected via methods to learn about the full path or possibly other metadata through per-platform extension traits.
#[derive(Debug)]
pub struct DirEntry(DirEntryInner);

#[derive(Debug)]
enum DirEntryInner {
    /// Std variant of file <https://docs.rs/rustc-std-workspace-std/latest/std/fs/struct.DirEntry.html>
    Std(std::fs::DirEntry),
    #[cfg(tokio_fs)]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio-fs")))]
    /// Tokio variant of file <https://docs.rs/tokio/latest/tokio/fs/struct.DirEntry.html>
    Tokio(tokio::fs::DirEntry),
}

impl From<std::fs::DirEntry> for DirEntry {
    fn from(inner: std::fs::DirEntry) -> Self {
        Self(DirEntryInner::Std(inner))
    }
}

#[cfg(tokio_fs)]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-fs")))]
impl From<tokio::fs::DirEntry> for DirEntry {
    fn from(inner: tokio::fs::DirEntry) -> Self {
        Self(DirEntryInner::Tokio(inner))
    }
}

impl DirEntry {
    /// Returns the file name of this entry.
    ///
    /// This is the last component of the path, which may be a file name or a directory name.
    pub fn file_name(&self) -> std::ffi::OsString {
        match &self.0 {
            DirEntryInner::Std(inner) => inner.file_name(),
            #[cfg(tokio_fs)]
            DirEntryInner::Tokio(inner) => inner.file_name(),
        }
    }

    maybe_fut_method!(
        /// Returns the file type for the file that this entry points at.
        ///
        /// This function will not traverse symlinks if this entry points at a symlink.
        ///
        /// # Platform-specific behavior
        ///
        /// On Windows and most Unix platforms this function is free (no extra system calls needed),
        /// but some Unix platforms may require the equivalent call to symlink_metadata to learn about the target file type.
        file_type() -> std::io::Result<FileType>,
        DirEntryInner::Std,
        DirEntryInner::Tokio,
        tokio_fs
    );

    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    /// Returns the underlying d_ino field in the contained dirent structure.
    pub fn ino(&self) -> u64 {
        use std::os::unix::fs::DirEntryExt as _;

        match &self.0 {
            DirEntryInner::Std(inner) => inner.ino(),
            #[cfg(tokio_fs)]
            DirEntryInner::Tokio(inner) => inner.ino(),
        }
    }

    /// Returns the full path of this entry.
    pub fn path(&self) -> std::path::PathBuf {
        match &self.0 {
            DirEntryInner::Std(inner) => inner.path(),
            #[cfg(tokio_fs)]
            DirEntryInner::Tokio(inner) => inner.path(),
        }
    }

    maybe_fut_method!(
        /// Returns the metadata for the file that this entry points at.
        ///
        /// This function will not traverse symlinks if this entry points at a symlink.
        ///
        /// # Platform-specific behavior
        ///
        /// On Windows and most Unix platforms this function is free (no extra system calls needed),
        /// but some Unix platforms may require the equivalent call to symlink_metadata to learn about the target file type.
        metadata() -> std::io::Result<std::fs::Metadata>,
        DirEntryInner::Std,
        DirEntryInner::Tokio,
        tokio_fs
    );
}
