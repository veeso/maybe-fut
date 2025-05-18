//! The main type for interacting with the file system is the [`File`] type.
//! This type provides methods for reading and writing to files.

use std::path::Path;

use super::OpenOptions;
use crate::{maybe_fut_constructor_result, maybe_fut_method};

#[derive(Debug, Read, Seek, Write, Unwrap)]
#[io(feature("tokio-fs"))]
#[unwrap_types(std(std::fs::File), tokio(tokio::fs::File), tokio_gated("tokio-fs"))]
/// A reference to an open file on the filesystem.
pub struct File(FileInner);

/// Inner pointer to sync or async file.
#[derive(Debug)]
enum FileInner {
    /// Std variant of file <https://docs.rs/rustc-std-workspace-std/latest/std/fs/struct.File.html>
    Std(std::fs::File),
    #[cfg(tokio_fs)]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio-fs")))]
    /// Tokio variant of file <https://docs.rs/tokio/latest/tokio/fs/struct.File.html>
    Tokio(tokio::fs::File),
}

impl From<std::fs::File> for File {
    fn from(file: std::fs::File) -> Self {
        Self(FileInner::Std(file))
    }
}

#[cfg(tokio_fs)]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-fs")))]
impl From<tokio::fs::File> for File {
    fn from(file: tokio::fs::File) -> Self {
        Self(FileInner::Tokio(file))
    }
}

impl File {
    maybe_fut_constructor_result!(
        /// Attempts to open a file in read-only mode.
        /// See [`std::fs::OpenOptions`] for more details.
        ///
        /// # Errors
        ///
        /// This function will return an error if called from outside of the Tokio runtime (if async) or if path does not already exist.
        /// Other errors may also be returned according to OpenOptions::open.
        ///
        /// See <https://docs.rs/rustc-std-workspace-std/latest/std/fs/struct.File.html#method.open>
        open(path: impl AsRef<Path>) -> std::io::Result<Self>,
        std::fs::File::open,
        tokio::fs::File::open,
        tokio_fs
    );

    maybe_fut_constructor_result!(
        /// Attempts to open a file in read-only mode with buffering.
        ///
        /// # Errors
        ///
        /// This function will return an error if `path` does not already exist,
        /// or if memory allocation fails for the new buffer.
        /// Other errors may also be returned according to [`std::fs::OpenOptions::open`].
        ///
        /// See <https://docs.rs/rustc-std-workspace-std/latest/std/fs/struct.File.html#method.create>
        create(path: impl AsRef<Path>) -> std::io::Result<Self>,
        std::fs::File::create,
        tokio::fs::File::create,
        tokio_fs
    );

    maybe_fut_constructor_result!(
        /// Opens a file in read-write mode.
        ///
        /// This function will create a file if it does not exist, or return an error
        /// if it does. This way, if the call succeeds, the file returned is guaranteed
        /// to be new.
        ///
        /// This option is useful because it is atomic. Otherwise between checking
        /// whether a file exists and creating a new one, the file may have been
        /// created by another process (a TOCTOU race condition / attack).
        ///
        /// This can also be written using `File::options().read(true).write(true).create_new(true).open(...)`.
        ///
        /// See [`std::fs::OpenOptions`] for more details.
        /// See <https://docs.rs/rustc-std-workspace-std/latest/std/fs/struct.File.html#method.create_new>
        create_new(path: impl AsRef<Path>) -> std::io::Result<Self>,
        std::fs::File::create_new,
        tokio::fs::File::create_new,
        tokio_fs
    );

    maybe_fut_method!(
        /// Queries metadata about the underlying file.
        metadata() -> std::io::Result<std::fs::Metadata>,
        FileInner::Std,
        FileInner::Tokio,
        tokio_fs
    );

    /// Returns a new [`OpenOptions`] object.
    ///
    /// This function returns a new OpenOptions object that you can use to open or create a file with specific options if open() or create() are not appropriate.
    ///
    /// It is equivalent to [`OpenOptions::new`], but allows you to write more readable code. Instead of `OpenOptions::new().append(true).open("example.log")`, you can write `File::options().append(true).open("example.log").await`.
    /// This also avoids the need to import [`OpenOptions`].
    ///
    /// See the [`OpenOptions::new`] function for more details.
    #[inline]
    pub fn open_options() -> OpenOptions {
        OpenOptions::new()
    }

    maybe_fut_method!(
        /// Truncates or extends the underlying file, updating the size of this file to become size.
        ///
        /// If the size is less than the current file’s size, then the file will be shrunk.
        /// If it is greater than the current file’s size, then the file will be extended to size and have all of the intermediate data filled in with 0s.
        ///
        /// # Errors
        ///
        /// This function will return an error if the file is not opened for writing.
        set_len(size: u64) -> std::io::Result<()>,
        FileInner::Std,
        FileInner::Tokio,
        tokio_fs
    );

    maybe_fut_method!(
        /// Changes the permissions on the underlying file.
        ///
        /// Platform-specific behavior
        /// This function currently corresponds to the fchmod function on Unix and the SetFileInformationByHandle function on Windows. Note that, this may change in the future.
        ///
        /// # Errors
        ///
        /// This function will return an error if the user lacks permission change attributes on the underlying file. It may also return an error in other os-specific unspecified cases.
        set_permissions(perm: std::fs::Permissions) -> std::io::Result<()>,
        FileInner::Std,
        FileInner::Tokio,
        tokio_fs
    );

    maybe_fut_method!(
        /// Attempts to sync all OS-internal metadata to disk.
        ///
        /// This function will attempt to ensure that all in-core data reaches the filesystem before returning.
        sync_all() -> std::io::Result<()>,
        FileInner::Std,
        FileInner::Tokio,
        tokio_fs
    );

    maybe_fut_method!(
        /// This function is similar to [`Self::sync_all`], except that it may not synchronize file metadata to the filesystem.
        ///
        /// This is intended for use cases that must synchronize content, but don’t need the metadata on disk.
        /// The goal of this method is to reduce disk operations.
        ///
        /// Note that some platforms may simply implement this in terms of sync_all.
        sync_data() -> std::io::Result<()>,
        FileInner::Std,
        FileInner::Tokio,
        tokio_fs
    );

    /// Creates a new [`File`] instance that shares the same underlying file handle as the existing [`File`] instance.
    /// Reads, writes, and seeks will affect both [`File`] instances simultaneously.
    pub async fn try_clone(&self) -> std::io::Result<Self> {
        match &self.0 {
            FileInner::Std(file) => file.try_clone().map(Self::from),
            #[cfg(tokio_fs)]
            FileInner::Tokio(file) => file.try_clone().await.map(Self::from),
        }
    }
    /// Converts the [`File`] inner instance to a [`std::fs::File`] instance if it is currently a [`tokio::fs::File`].
    ///
    /// This can be useful when you need for instance to pass an `impl std::io::Write` to a function.
    pub async fn to_std(self) -> std::fs::File {
        match self.0 {
            FileInner::Std(file) => file,
            #[cfg(tokio_fs)]
            FileInner::Tokio(file) => file.into_std().await,
        }
    }

    /// Converts the [`File`] inner instance to a [`tokio::fs::File`] instance if it is currently a [`std::fs::File`].
    ///
    /// This can be useful when you need for instance to pass an `impl tokio::io::AsyncWrite` to a function.
    #[cfg(tokio_fs)]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio-fs")))]
    pub async fn to_tokio(self) -> tokio::fs::File {
        match self.0 {
            FileInner::Std(file) => tokio::fs::File::from_std(file),
            FileInner::Tokio(file) => file,
        }
    }
}

#[cfg(unix)]
impl std::os::fd::AsFd for File {
    fn as_fd(&self) -> std::os::fd::BorrowedFd<'_> {
        match &self.0 {
            FileInner::Std(file) => file.as_fd(),
            #[cfg(tokio_fs)]
            FileInner::Tokio(file) => file.as_fd(),
        }
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsHandle for File {
    fn as_handle(&self) -> std::os::windows::io::BorrowedHandle<'_> {
        match &self.0 {
            FileInner::Std(file) => file.as_handle(),
            #[cfg(tokio_fs)]
            FileInner::Tokio(file) => file.as_handle(),
        }
    }
}

#[cfg(unix)]
impl std::os::fd::AsRawFd for File {
    fn as_raw_fd(&self) -> std::os::fd::RawFd {
        match &self.0 {
            FileInner::Std(file) => file.as_raw_fd(),
            #[cfg(tokio_fs)]
            FileInner::Tokio(file) => file.as_raw_fd(),
        }
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsRawHandle for File {
    fn as_raw_handle(&self) -> std::os::windows::io::RawHandle {
        match &self.0 {
            FileInner::Std(file) => file.as_raw_handle(),
            #[cfg(tokio_fs)]
            FileInner::Tokio(file) => file.as_raw_handle(),
        }
    }
}

#[cfg(unix)]
impl std::os::fd::FromRawFd for File {
    unsafe fn from_raw_fd(fd: std::os::fd::RawFd) -> Self {
        #[cfg(tokio_fs)]
        {
            if crate::context::is_async_context() {
                Self(FileInner::Tokio(unsafe {
                    tokio::fs::File::from_raw_fd(fd)
                }))
            } else {
                Self(FileInner::Std(unsafe { std::fs::File::from_raw_fd(fd) }))
            }
        }
        #[cfg(not(tokio_fs))]
        {
            Self(FileInner::Std(unsafe { std::fs::File::from_raw_fd(fd) }))
        }
    }
}

#[cfg(windows)]
impl std::os::windows::io::FromRawHandle for File {
    unsafe fn from_raw_handle(handle: std::os::windows::io::RawHandle) -> Self {
        #[cfg(tokio_fs)]
        {
            if crate::context::is_async_context() {
                Self(FileInner::Tokio(unsafe {
                    tokio::fs::File::from_raw_handle(handle)
                }))
            } else {
                Self(FileInner::Std(unsafe {
                    std::fs::File::from_raw_handle(handle)
                }))
            }
        }
        #[cfg(not(tokio_fs))]
        {
            Self(FileInner::Std(unsafe {
                std::fs::File::from_raw_handle(handle)
            }))
        }
    }
}

#[cfg(test)]
mod test {

    use tempfile::NamedTempFile;

    use super::*;
    use crate::SyncRuntime;
    use crate::io::{Read, Seek, Write};

    #[test]
    fn test_should_instantiate_file_sync() {
        let temp = NamedTempFile::new().expect("Failed to create temp file");

        // write file
        std::fs::write(temp.path(), b"Hello world").expect("Failed to write file");

        let variant = SyncRuntime::block_on(File::open(temp.path())).expect("Failed to open file");
        assert!(matches!(variant.0, FileInner::Std(_)));
    }

    #[tokio::test]
    async fn test_should_instantiate_file_async() {
        let temp = NamedTempFile::new().expect("Failed to create temp file");

        // write file
        std::fs::write(temp.path(), b"Hello world").expect("Failed to write file");

        let variant = File::open(temp.path()).await.expect("Failed to open file");
        assert!(matches!(variant.0, FileInner::Tokio(_)));
    }

    #[test]
    fn test_should_create_file_sync() {
        let temp = NamedTempFile::new().expect("Failed to create temp file");

        let variant =
            SyncRuntime::block_on(File::create(temp.path())).expect("Failed to open file");
        assert!(matches!(variant.0, FileInner::Std(_)));
    }

    #[tokio::test]
    async fn test_should_create_file_async() {
        let temp = NamedTempFile::new().expect("Failed to create temp file");

        let variant = File::create(temp.path())
            .await
            .expect("Failed to open file");
        assert!(matches!(variant.0, FileInner::Tokio(_)));
    }

    #[test]
    fn test_should_get_metadata_sync() {
        let temp = NamedTempFile::new().expect("Failed to create temp file");

        // write file
        std::fs::write(temp.path(), b"Hello world").expect("Failed to write file");

        let file = SyncRuntime::block_on(File::open(temp.path())).expect("Failed to open file");
        SyncRuntime::block_on(file.metadata()).expect("Failed to get metadata");
    }

    #[tokio::test]
    async fn test_should_get_metadata_async() {
        let temp = NamedTempFile::new().expect("Failed to create temp file");

        // write file
        std::fs::write(temp.path(), b"Hello world").expect("Failed to write file");

        File::open(temp.path())
            .await
            .expect("Failed to open file")
            .metadata()
            .await
            .expect("Failed to get metadata");
    }

    #[test]
    fn test_should_convert_to_std() {
        let temp = NamedTempFile::new().expect("Failed to create temp file");

        // write file
        std::fs::write(temp.path(), b"Hello world").expect("Failed to write file");

        let file = SyncRuntime::block_on(File::open(temp.path())).expect("Failed to open file");
        let _std_file = SyncRuntime::block_on(file.to_std());
    }

    #[tokio::test]
    async fn test_should_convert_to_tokio() {
        let temp = NamedTempFile::new().expect("Failed to create temp file");

        // write file
        std::fs::write(temp.path(), b"Hello world").expect("Failed to write file");

        let file = File::open(temp.path()).await.expect("Failed to open file");
        let _tokio_file = file.to_tokio().await;
    }

    #[test]
    fn test_should_convert_to_std_sync() {
        let temp = NamedTempFile::new().expect("Failed to create temp file");

        // write file
        std::fs::write(temp.path(), b"Hello world").expect("Failed to write file");

        let file = SyncRuntime::block_on(File::open(temp.path())).expect("Failed to open file");
        let _std_file = SyncRuntime::block_on(file.to_tokio());
    }

    #[tokio::test]
    async fn test_should_convert_to_tokio_async() {
        let temp = NamedTempFile::new().expect("Failed to create temp file");

        // write file
        std::fs::write(temp.path(), b"Hello world").expect("Failed to write file");

        let file = File::open(temp.path()).await.expect("Failed to open file");
        let _tokio_file = file.to_tokio().await;
    }

    #[test]
    fn test_should_read_sync() {
        let temp = NamedTempFile::new().expect("Failed to create temp file");

        // write file
        std::fs::write(temp.path(), b"Hello world").expect("Failed to write file");

        let mut file = SyncRuntime::block_on(File::open(temp.path())).expect("Failed to open file");
        let mut buf = vec![0; 11];
        SyncRuntime::block_on(file.read(&mut buf)).expect("Failed to read file");
        assert_eq!(buf, b"Hello world");
    }

    #[tokio::test]
    async fn test_should_read_async() {
        let temp = NamedTempFile::new().expect("Failed to create temp file");

        // write file
        std::fs::write(temp.path(), b"Hello world").expect("Failed to write file");

        let mut file = File::open(temp.path()).await.expect("Failed to open file");
        let mut buf = vec![0; 11];
        file.read(&mut buf).await.expect("Failed to read file");
        assert_eq!(buf, b"Hello world");
    }

    #[test]
    fn test_should_write_sync() {
        let temp = NamedTempFile::new().expect("Failed to create temp file");

        let mut file =
            SyncRuntime::block_on(File::create(temp.path())).expect("Failed to open file");
        SyncRuntime::block_on(file.write(b"Hello world")).expect("Failed to write file");
        SyncRuntime::block_on(file.flush()).expect("Failed to flush file");

        let buf = std::fs::read(temp.path()).expect("Failed to read file");
        assert_eq!(buf, b"Hello world");
    }

    #[tokio::test]
    async fn test_should_write_async() {
        let temp = NamedTempFile::new().expect("Failed to create temp file");

        let mut file = File::create(temp.path())
            .await
            .expect("Failed to open file");
        file.write(b"Hello world")
            .await
            .expect("Failed to write file");
        file.flush().await.expect("Failed to flush file");

        let buf = tokio::fs::read(temp.path())
            .await
            .expect("Failed to read file");
        assert_eq!(buf, b"Hello world");
    }

    #[test]
    fn test_should_seek_sync() {
        let temp = NamedTempFile::new().expect("Failed to create temp file");

        // write file
        std::fs::write(temp.path(), b"Hello world").expect("Failed to write file");

        let mut file = SyncRuntime::block_on(File::open(temp.path())).expect("Failed to open file");
        let mut buf = vec![0; 5];
        SyncRuntime::block_on(file.seek(std::io::SeekFrom::Start(6))).expect("Failed to seek file");
        SyncRuntime::block_on(file.read(&mut buf)).expect("Failed to read file");
        assert_eq!(buf, b"world");
    }

    #[tokio::test]
    async fn test_should_seek_async() {
        let temp = NamedTempFile::new().expect("Failed to create temp file");

        // write file
        std::fs::write(temp.path(), b"Hello world").expect("Failed to write file");

        let mut file = File::open(temp.path()).await.expect("Failed to open file");
        let mut buf = vec![0; 5];
        file.seek(std::io::SeekFrom::Start(6))
            .await
            .expect("Failed to seek file");
        file.read(&mut buf).await.expect("Failed to read file");
        assert_eq!(buf, b"world");
    }
}
