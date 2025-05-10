use crate::maybe_fut_method;

/// A builder for creating directories in various manners.
#[derive(Debug)]
pub struct DirBuilder(DirBuilderInner);

#[derive(Debug)]
enum DirBuilderInner {
    /// Std variant of file <https://docs.rs/rustc-std-workspace-std/latest/std/fs/struct.DirBuilder.html>
    Std(std::fs::DirBuilder),
    #[cfg(tokio_fs)]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio-fs")))]
    /// Tokio variant of file <https://docs.rs/tokio/latest/tokio/fs/struct.DirBuilder.html>
    Tokio(tokio::fs::DirBuilder),
}

impl Default for DirBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<std::fs::DirBuilder> for DirBuilder {
    fn from(inner: std::fs::DirBuilder) -> Self {
        Self(DirBuilderInner::Std(inner))
    }
}

#[cfg(tokio_fs)]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-fs")))]
impl From<tokio::fs::DirBuilder> for DirBuilder {
    fn from(inner: tokio::fs::DirBuilder) -> Self {
        Self(DirBuilderInner::Tokio(inner))
    }
}

impl DirBuilder {
    /// Creates a new set of options with default mode/security settings for all platforms and also non-recursive.
    pub fn new() -> Self {
        #[cfg(tokio_fs)]
        {
            if crate::context::is_async_context() {
                tokio::fs::DirBuilder::new().into()
            } else {
                std::fs::DirBuilder::new().into()
            }
        }
        #[cfg(not(tokio_fs))]
        {
            std::fs::DirBuilder::new().into()
        }
    }

    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    /// Sets the mode to create new directories with.
    ///
    /// This option defaults to `0o777`.
    pub fn mode(&mut self, mode: u32) -> &mut Self {
        use std::os::unix::fs::DirBuilderExt as _;

        match &mut self.0 {
            DirBuilderInner::Std(inner) => {
                inner.mode(mode);
            }
            #[cfg(tokio_fs)]
            #[cfg_attr(docsrs, doc(cfg(feature = "tokio-fs")))]
            DirBuilderInner::Tokio(inner) => {
                inner.mode(mode);
            }
        }

        self
    }

    /// Indicates whether to create directories recursively (including all parent directories).
    /// Parents that do not exist are created with the same security and permissions settings.
    ///
    /// This option defaults to `false`.
    pub fn recursive(&mut self, recursive: bool) -> &mut Self {
        match &mut self.0 {
            DirBuilderInner::Std(inner) => {
                inner.recursive(recursive);
            }
            #[cfg(tokio_fs)]
            #[cfg_attr(docsrs, doc(cfg(feature = "tokio-fs")))]
            DirBuilderInner::Tokio(inner) => {
                inner.recursive(recursive);
            }
        }

        self
    }

    maybe_fut_method!(
        /// Creates the specified directory with the configured options.
        ///
        /// It is considered an error if the directory already exists unless recursive mode is enabled.
        ///
        /// This is an async version of std::fs::DirBuilder::create.
        ///
        /// # Errors
        ///
        /// An error will be returned under the following circumstances:
        ///
        /// - Path already points to an existing file.
        /// - Path already points to an existing directory and the mode is non-recursive.
        /// - The calling process doesn’t have permissions to create the directory or its missing parents.
        /// - Other I/O error occurred.
        create(path: impl AsRef<std::path::Path>) -> std::io::Result<()>,
        DirBuilderInner::Std,
        DirBuilderInner::Tokio,
        tokio_fs
    );
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::SyncRuntime;

    #[test]
    fn test_dir_builder_sync() {
        let tempdir = tempfile::tempdir().unwrap();
        let path = tempdir.path().join("test_dir");
        let mut builder = DirBuilder::new();
        builder.recursive(true);
        SyncRuntime::block_on(builder.create(&path)).expect("Failed to create directory");
        assert!(path.exists());
    }

    #[tokio::test]
    async fn test_dir_builder_async() {
        let tempdir = tempfile::tempdir().unwrap();
        let path = tempdir.path().join("test_dir");
        let mut builder = DirBuilder::new();
        builder.recursive(true);
        builder
            .create(&path)
            .await
            .expect("Failed to create directory");
        assert!(path.exists());
    }
}
