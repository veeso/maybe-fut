//! File system utilities
//!
//! This module contains utilty methods for working with the file system.
//! This includes reading/writingt to files, and working with directories.

mod dir_builder;
mod dir_entry;
mod file;
mod open_options;
mod read_dir;

pub use self::dir_builder::DirBuilder;
pub use self::dir_entry::DirEntry;
pub use self::file::File;
pub use self::open_options::OpenOptions;
pub use self::read_dir::ReadDir;
use crate::maybe_fut_function;

maybe_fut_function!(
    /// Returns the canonical, absolute form of a path with all intermediate components normalized and symbolic links resolved.
    canonicalize(path: impl AsRef<std::path::Path>) -> std::io::Result<std::path::PathBuf>,
    std::fs::canonicalize,
    tokio::fs::canonicalize,
    tokio_fs
);

maybe_fut_function!(
    /// Copies the contents of one file to another.
    /// This function will also copy the permission bits of the original file to the destination file.
    /// This function will overwrite the contents of to.
    copy(from: impl AsRef<std::path::Path>, to: impl AsRef<std::path::Path>) -> std::io::Result<u64>,
    std::fs::copy,
    tokio::fs::copy,
    tokio_fs
);

maybe_fut_function!(
    /// Creates a new directory at the specified path.
    create_dir(path: impl AsRef<std::path::Path>) -> std::io::Result<()>,
    std::fs::create_dir,
    tokio::fs::create_dir,
    tokio_fs
);

maybe_fut_function!(
    /// Creates a new directory at the specified path, including all parent directories.
    create_dir_all(path: impl AsRef<std::path::Path>) -> std::io::Result<()>,
    std::fs::create_dir_all,
    tokio::fs::create_dir_all,
    tokio_fs
);

maybe_fut_function!(
    /// Creates a new hard link on the filesystem.
    ///
    /// The `link` path will be a link pointing to the `original` path.
    /// Note that systems often require these two paths to both be located on the same filesystem.
    hard_link(original: impl AsRef<std::path::Path>, link: impl AsRef<std::path::Path>) -> std::io::Result<()>,
    std::fs::hard_link,
    tokio::fs::hard_link,
    tokio_fs
);

maybe_fut_function!(
    /// Given a path, queries the file system to get information about a file, directory, etc.
    ///
    /// This function will traverse symbolic links to query information about the destination file.
    metadata(path: impl AsRef<std::path::Path>) -> std::io::Result<std::fs::Metadata>,
    std::fs::metadata,
    tokio::fs::metadata,
    tokio_fs
);

maybe_fut_function!(
    /// Reads the entire contents of a file into a bytes vector.
    ///
    /// This is a convenience function for using [`File::open`] and `read_to_end` with fewer imports and without an
    /// intermediate variable.
    /// It pre-allocates a buffer based on the file size when available, so it is generally faster than reading into a vector
    /// created with [`Vec::new`].
    read(path: impl AsRef<std::path::Path>) -> std::io::Result<Vec<u8>>,
    std::fs::read,
    tokio::fs::read,
    tokio_fs
);

/// Returns a stream over the entries within a directory
pub async fn read_dir(path: impl AsRef<std::path::Path>) -> std::io::Result<ReadDir> {
    #[cfg(tokio_fs)]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio-fs")))]
    {
        if crate::context::is_async_context() {
            tokio::fs::read_dir(path).await.map(ReadDir::from)
        } else {
            std::fs::read_dir(path).map(ReadDir::from)
        }
    }
    #[cfg(not(tokio_fs))]
    {
        std::fs::read_dir(path).map(ReadDir::from)
    }
}

maybe_fut_function!(
    /// Reads a symbolic link, returning the file that the link points to.
    read_link(path: impl AsRef<std::path::Path>) -> std::io::Result<std::path::PathBuf>,
    std::fs::read_link,
    tokio::fs::read_link,
    tokio_fs
);

maybe_fut_function!(
    /// Reads the entire contents of a file into a string.
    read_to_string(path: impl AsRef<std::path::Path>) -> std::io::Result<String>,
    std::fs::read_to_string,
    tokio::fs::read_to_string,
    tokio_fs
);

maybe_fut_function!(
    /// Removes an empty directory.
    ///
    /// If you want to remove a directory and all of its contents, use [`remove_dir_all`].
    remove_dir(path: impl AsRef<std::path::Path>) -> std::io::Result<()>,
    std::fs::remove_dir,
    tokio::fs::remove_dir,
    tokio_fs
);

maybe_fut_function!(
    /// Removes a directory at this path, after removing all its contents. Use carefully!
    ///
    /// This function does **not** follow symbolic links and it will simply remove the symbolic link itself.
    remove_dir_all(path: impl AsRef<std::path::Path>) -> std::io::Result<()>,
    std::fs::remove_dir_all,
    tokio::fs::remove_dir_all,
    tokio_fs
);

maybe_fut_function!(
    /// Removes a file at this path.
    ///
    /// Note that there is no guarantee that the file is immediately deleted
    /// (e.g., depending on platform, other open file descriptors may prevent immediate removal).
    remove_file(path: impl AsRef<std::path::Path>) -> std::io::Result<()>,
    std::fs::remove_file,
    tokio::fs::remove_file,
    tokio_fs
);

maybe_fut_function!(
    /// Renames a file or directory to a new name, replacing the original file if to already exists.
    ///
    /// This will not work if the new name is on a different mount point.
    rename(
        from: impl AsRef<std::path::Path>,
        to: impl AsRef<std::path::Path>,
    ) -> std::io::Result<()>,
    std::fs::rename,
    tokio::fs::rename,
    tokio_fs
);

maybe_fut_function!(
    /// Changes the permissions found on a file or a directory.
    set_permissions(path: impl AsRef<std::path::Path>, perm: std::fs::Permissions) -> std::io::Result<()>,
    std::fs::set_permissions,
    tokio::fs::set_permissions,
    tokio_fs
);

maybe_fut_function!(
    /// Queries the metadata about a file without following symlinks.
    symlink_metadata(path: impl AsRef<std::path::Path>) -> std::io::Result<std::fs::Metadata>,
    std::fs::symlink_metadata,
    tokio::fs::symlink_metadata,
    tokio_fs
);

maybe_fut_function!(
    /// Writes a slice as the entire contents of a file.
    ///
    /// This function will create a file if it does not exist, and will entirely replace its contents if it does.
    ///
    /// Depending on the platform, this function may fail if the full directory path does not exist.
    ///
    /// This is a convenience function for using File::create and write_all with fewer imports.
    write(path: impl AsRef<std::path::Path>, contents: impl AsRef<[u8]>) -> std::io::Result<()>,
    std::fs::write,
    tokio::fs::write,
    tokio_fs
);

#[cfg(test)]
mod test {

    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt as _;

    use super::*;
    use crate::SyncRuntime;

    #[test]
    fn test_should_canonicalize_sync() {
        let tempdir = tempfile::tempdir().unwrap();

        SyncRuntime::block_on(canonicalize(tempdir.path())).expect("canonicalize failed");
    }

    #[tokio::test]
    async fn test_should_canonicalize_async() {
        let tempdir = tempfile::tempdir().unwrap();

        canonicalize(tempdir.path())
            .await
            .expect("canonicalize failed");
    }

    #[test]
    fn test_should_copy_sync() {
        let tempdir = tempfile::tempdir().unwrap();
        let src = tempdir.path().join("src.txt");
        let dst = tempdir.path().join("dst.txt");

        std::fs::write(&src, "Hello, world!").unwrap();

        SyncRuntime::block_on(copy(&src, &dst)).expect("copy failed");
    }

    #[tokio::test]
    async fn test_should_copy_async() {
        let tempdir = tempfile::tempdir().unwrap();
        let src = tempdir.path().join("src.txt");
        let dst = tempdir.path().join("dst.txt");

        std::fs::write(&src, "Hello, world!").unwrap();

        copy(&src, &dst).await.expect("copy failed");
    }

    #[test]
    fn test_should_create_dir_sync() {
        let tempdir = tempfile::tempdir().unwrap();
        let dir = tempdir.path().join("new_dir");

        SyncRuntime::block_on(create_dir(&dir)).expect("create_dir failed");
    }

    #[tokio::test]
    async fn test_should_create_dir_async() {
        let tempdir = tempfile::tempdir().unwrap();
        let dir = tempdir.path().join("new_dir");

        create_dir(&dir).await.expect("create_dir failed");
    }

    #[test]
    fn test_should_create_dir_all_sync() {
        let tempdir = tempfile::tempdir().unwrap();
        let dir = tempdir.path().join("new_dir").join("sub_dir");

        SyncRuntime::block_on(create_dir_all(&dir)).expect("create_dir_all failed");
    }

    #[tokio::test]
    async fn test_should_create_dir_all_async() {
        let tempdir = tempfile::tempdir().unwrap();
        let dir = tempdir.path().join("new_dir").join("sub_dir");

        create_dir_all(&dir).await.expect("create_dir_all failed");
    }

    #[test]
    fn test_should_hard_link_sync() {
        let tempdir = tempfile::tempdir().unwrap();
        let src = tempdir.path().join("src.txt");
        let link = tempdir.path().join("link.txt");

        std::fs::write(&src, "Hello, world!").unwrap();

        SyncRuntime::block_on(hard_link(&src, &link)).expect("hard_link failed");
    }

    #[tokio::test]
    async fn test_should_hard_link_async() {
        let tempdir = tempfile::tempdir().unwrap();
        let src = tempdir.path().join("src.txt");
        let link = tempdir.path().join("link.txt");

        std::fs::write(&src, "Hello, world!").unwrap();

        hard_link(&src, &link).await.expect("hard_link failed");
    }

    #[test]
    fn test_should_metadata_sync() {
        let tempdir = tempfile::tempdir().unwrap();
        let file = tempdir.path().join("file.txt");

        std::fs::write(&file, "Hello, world!").unwrap();

        SyncRuntime::block_on(metadata(&file)).expect("metadata failed");
    }

    #[tokio::test]
    async fn test_should_metadata_async() {
        let tempdir = tempfile::tempdir().unwrap();
        let file = tempdir.path().join("file.txt");

        std::fs::write(&file, "Hello, world!").unwrap();

        metadata(&file).await.expect("metadata failed");
    }

    #[test]
    fn test_should_read_sync() {
        let tempdir = tempfile::tempdir().unwrap();
        let file = tempdir.path().join("file.txt");

        std::fs::write(&file, "Hello, world!").unwrap();

        SyncRuntime::block_on(read(&file)).expect("read failed");
    }

    #[tokio::test]
    async fn test_should_read_async() {
        let tempdir = tempfile::tempdir().unwrap();
        let file = tempdir.path().join("file.txt");

        std::fs::write(&file, "Hello, world!").unwrap();

        read(&file).await.expect("read failed");
    }

    #[test]
    #[cfg(unix)]
    fn test_should_read_link_sync() {
        let tempdir = tempfile::tempdir().unwrap();
        let link = tempdir.path().join("link.txt");

        std::os::unix::fs::symlink(tempdir.path(), &link).unwrap();

        SyncRuntime::block_on(read_link(&link)).expect("read_link failed");
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn test_should_read_link_async() {
        let tempdir = tempfile::tempdir().unwrap();
        let link = tempdir.path().join("link.txt");

        std::os::unix::fs::symlink(tempdir.path(), &link).unwrap();

        read_link(&link).await.expect("read_link failed");
    }

    #[test]
    fn test_should_read_dir_sync() {
        let tempdir = tempfile::tempdir().unwrap();

        SyncRuntime::block_on(read_dir(tempdir.path())).expect("read_dir failed");
    }

    #[tokio::test]
    async fn test_should_read_dir_async() {
        let tempdir = tempfile::tempdir().unwrap();

        read_dir(tempdir.path()).await.expect("read_dir failed");
    }

    #[test]
    fn test_should_read_to_string_sync() {
        let tempdir = tempfile::tempdir().unwrap();
        let file = tempdir.path().join("file.txt");

        std::fs::write(&file, "Hello, world!").unwrap();

        SyncRuntime::block_on(read_to_string(&file)).expect("read_to_string failed");
    }

    #[tokio::test]
    async fn test_should_read_to_string_async() {
        let tempdir = tempfile::tempdir().unwrap();
        let file = tempdir.path().join("file.txt");

        std::fs::write(&file, "Hello, world!").unwrap();

        read_to_string(&file).await.expect("read_to_string failed");
    }

    #[test]
    fn test_should_remove_dir_sync() {
        let tempdir = tempfile::tempdir().unwrap();
        let dir = tempdir.path().join("new_dir");

        SyncRuntime::block_on(create_dir(&dir)).expect("create_dir failed");
        SyncRuntime::block_on(remove_dir(&dir)).expect("remove_dir failed");
    }

    #[tokio::test]
    async fn test_should_remove_dir_async() {
        let tempdir = tempfile::tempdir().unwrap();
        let dir = tempdir.path().join("new_dir");

        create_dir(&dir).await.expect("create_dir failed");
        remove_dir(&dir).await.expect("remove_dir failed");
    }

    #[test]
    fn test_should_remove_dir_all_sync() {
        let tempdir = tempfile::tempdir().unwrap();
        let dir = tempdir.path().join("new_dir").join("sub_dir");

        SyncRuntime::block_on(create_dir_all(&dir)).expect("create_dir_all failed");
        SyncRuntime::block_on(remove_dir_all(&dir)).expect("remove_dir_all failed");
    }

    #[tokio::test]
    async fn test_should_remove_dir_all_async() {
        let tempdir = tempfile::tempdir().unwrap();
        let dir = tempdir.path().join("new_dir").join("sub_dir");

        create_dir_all(&dir).await.expect("create_dir_all failed");
        remove_dir_all(&dir).await.expect("remove_dir_all failed");
    }

    #[test]
    fn test_should_remove_file_sync() {
        let tempdir = tempfile::tempdir().unwrap();
        let file = tempdir.path().join("file.txt");

        std::fs::write(&file, "Hello, world!").unwrap();

        SyncRuntime::block_on(remove_file(&file)).expect("remove_file failed");
    }

    #[tokio::test]
    async fn test_should_remove_file_async() {
        let tempdir = tempfile::tempdir().unwrap();
        let file = tempdir.path().join("file.txt");

        std::fs::write(&file, "Hello, world!").unwrap();

        remove_file(&file).await.expect("remove_file failed");
    }

    #[test]
    fn test_should_rename_sync() {
        let tempdir = tempfile::tempdir().unwrap();
        let src = tempdir.path().join("src.txt");
        let dst = tempdir.path().join("dst.txt");

        std::fs::write(&src, "Hello, world!").unwrap();

        SyncRuntime::block_on(rename(&src, &dst)).expect("rename failed");
    }

    #[tokio::test]
    async fn test_should_rename_async() {
        let tempdir = tempfile::tempdir().unwrap();
        let src = tempdir.path().join("src.txt");
        let dst = tempdir.path().join("dst.txt");

        std::fs::write(&src, "Hello, world!").unwrap();

        rename(&src, &dst).await.expect("rename failed");
    }

    #[test]
    #[cfg(unix)]
    fn test_should_set_permissions_sync() {
        let tempdir = tempfile::tempdir().unwrap();
        let file = tempdir.path().join("file.txt");

        std::fs::write(&file, "Hello, world!").unwrap();

        SyncRuntime::block_on(set_permissions(
            &file,
            std::fs::Permissions::from_mode(0o644),
        ))
        .expect("set_permissions failed");
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn test_should_set_permissions_async() {
        let tempdir = tempfile::tempdir().unwrap();
        let file = tempdir.path().join("file.txt");

        std::fs::write(&file, "Hello, world!").unwrap();

        set_permissions(&file, std::fs::Permissions::from_mode(0o644))
            .await
            .expect("set_permissions failed");
    }

    #[test]
    #[cfg(unix)]
    fn test_should_symlink_metadata_sync() {
        let tempdir = tempfile::tempdir().unwrap();
        let link = tempdir.path().join("link.txt");

        std::os::unix::fs::symlink(tempdir.path(), &link).unwrap();

        SyncRuntime::block_on(symlink_metadata(&link)).expect("symlink_metadata failed");
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn test_should_symlink_metadata_async() {
        let tempdir = tempfile::tempdir().unwrap();
        let link = tempdir.path().join("link.txt");

        std::os::unix::fs::symlink(tempdir.path(), &link).unwrap();

        symlink_metadata(&link)
            .await
            .expect("symlink_metadata failed");
    }

    #[test]
    fn test_should_write_sync() {
        let tempdir = tempfile::tempdir().unwrap();
        let file = tempdir.path().join("file.txt");

        SyncRuntime::block_on(write(&file, b"Hello, world!")).expect("write failed");
    }

    #[tokio::test]
    async fn test_should_write_async() {
        let tempdir = tempfile::tempdir().unwrap();
        let file = tempdir.path().join("file.txt");

        write(&file, b"Hello, world!").await.expect("write failed");
    }
}
