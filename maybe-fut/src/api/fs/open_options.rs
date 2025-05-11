#[derive(Clone, Debug, Unwrap)]
#[unwrap_types(
    std(std::fs::OpenOptions),
    tokio(tokio::fs::OpenOptions),
    tokio_gated("tokio-fs")
)]
/// Options and flags which can be used to configure how a file is opened.
/// This builder exposes the ability to configure how a File is opened and what operations are permitted on the open file. The File::open and File::create methods are aliases for commonly used options using this builder.
///
/// Generally speaking, when using OpenOptions, you’ll first call new, then chain calls to methods to set each option, then call open, passing the path of the file you’re trying to open. This will give you a io::Result with a File inside that you can further operate on.
pub struct OpenOptions(OpenOptionsInner);

impl Default for OpenOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Inner pointer to sync or async file.
#[derive(Debug, Clone)]
enum OpenOptionsInner {
    /// Std variant of file <https://docs.rs/rustc-std-workspace-std/latest/std/fs/struct.OpenOptions.html>
    Std(std::fs::OpenOptions),
    #[cfg(tokio_fs)]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio-fs")))]
    /// Tokio variant of file <https://docs.rs/tokio/latest/tokio/fs/struct.OpenOptions.html>
    Tokio(tokio::fs::OpenOptions),
}

impl From<std::fs::OpenOptions> for OpenOptions {
    fn from(options: std::fs::OpenOptions) -> Self {
        Self(OpenOptionsInner::Std(options))
    }
}

#[cfg(tokio_fs)]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-fs")))]
impl From<tokio::fs::OpenOptions> for OpenOptions {
    fn from(options: tokio::fs::OpenOptions) -> Self {
        Self(OpenOptionsInner::Tokio(options))
    }
}

impl OpenOptions {
    /// Creates a blank new set of options ready for configuration.
    /// All options are initially set to false.
    pub fn new() -> Self {
        #[cfg(tokio_fs)]
        {
            if crate::context::is_async_context() {
                tokio::fs::OpenOptions::new().into()
            } else {
                std::fs::OpenOptions::new().into()
            }
        }
        #[cfg(not(tokio_fs))]
        {
            std::fs::OpenOptions::new().into()
        }
    }

    /// Sets the option for read access.
    ///
    /// This option, when true, will indicate that the file should be
    /// `read`-able if opened.
    pub fn read(&mut self, read: bool) -> &mut OpenOptions {
        match &mut self.0 {
            OpenOptionsInner::Std(inner) => {
                inner.read(read);
            }
            #[cfg(tokio_fs)]
            OpenOptionsInner::Tokio(inner) => {
                inner.read(read);
            }
        }
        self
    }

    /// Sets the option for write access.
    ///
    /// This option, when true, will indicate that the file should be `write`-able if opened.
    pub fn write(&mut self, write: bool) -> &mut OpenOptions {
        match &mut self.0 {
            OpenOptionsInner::Std(inner) => {
                inner.write(write);
            }
            #[cfg(tokio_fs)]
            OpenOptionsInner::Tokio(inner) => {
                inner.write(write);
            }
        }
        self
    }

    /// Sets the option for append mode.
    ///
    /// This option, when true, means that writes will append to a file instead of overwriting previous contents.
    /// Note that setting `.write(true).append(true)` has the same effect as setting only `.append(true)`.
    ///
    /// For most filesystems, the operating system guarantees that all writes are atomic: no writes get mangled because another process writes at the same time.
    ///
    /// One maybe obvious note when using append-mode: make sure that all data that belongs together is written to
    /// the file in one operation. This can be done by concatenating strings before passing them to [`Self::write`], or using a
    /// buffered writer (with a buffer of adequate size), and calling flush() when the message is complete.
    ///
    /// If a file is opened with both read and append access, beware that after opening, and after every write,
    /// the position for reading may be set at the end of the file.
    /// So, before writing, save the current position (using `seek(SeekFrom::Current(0))`), and restore it before the next read.
    ///
    /// ### Note
    ///
    /// This function doesn’t create the file if it doesn’t exist. Use the [`Self::create`] method to do so.
    pub fn append(&mut self, append: bool) -> &mut OpenOptions {
        match &mut self.0 {
            OpenOptionsInner::Std(inner) => {
                inner.append(append);
            }
            #[cfg(tokio_fs)]
            OpenOptionsInner::Tokio(inner) => {
                inner.append(append);
            }
        }
        self
    }

    /// Sets the option for truncating a previous file.
    ///
    /// If a file is successfully opened with this option set it will truncate the file to 0 length if it already exists.
    ///
    /// The file must be opened with write access for truncate to work.
    pub fn truncate(&mut self, truncate: bool) -> &mut OpenOptions {
        match &mut self.0 {
            OpenOptionsInner::Std(inner) => {
                inner.truncate(truncate);
            }
            #[cfg(tokio_fs)]
            OpenOptionsInner::Tokio(inner) => {
                inner.truncate(truncate);
            }
        }
        self
    }

    /// Sets the option for creating a new file.
    ///
    /// This option indicates whether a new file will be created if the file does not yet already exist.
    ///
    /// In order for the file to be created, [`Self::write`] or [`Self::append`] access must be used.
    pub fn create(&mut self, create: bool) -> &mut OpenOptions {
        match &mut self.0 {
            OpenOptionsInner::Std(inner) => {
                inner.create(create);
            }
            #[cfg(tokio_fs)]
            OpenOptionsInner::Tokio(inner) => {
                inner.create(create);
            }
        }
        self
    }

    /// Sets the option to always create a new file.
    ///
    /// This option indicates whether a new file will be created. No file is allowed to exist at the target location, also no (dangling) symlink.
    ///
    ///
    /// This option is useful because it is atomic. Otherwise between checking whether a file exists and creating a new one, the file may have been created by another process (a TOCTOU race condition / attack).
    ///
    /// If `.create_new(true)` is set, `.create()` and `.truncate()` are ignored.
    ///
    /// The file must be opened with [`Self::write`] or [`Self::append`] access in order to create a new file.
    pub fn create_new(&mut self, create_new: bool) -> &mut OpenOptions {
        match &mut self.0 {
            OpenOptionsInner::Std(inner) => {
                inner.create_new(create_new);
            }
            #[cfg(tokio_fs)]
            OpenOptionsInner::Tokio(inner) => {
                inner.create_new(create_new);
            }
        }
        self
    }

    /// Opens a file at `path` with the options specified by `self`.
    ///
    /// # Errors
    ///
    /// This function will return an error under a number of different
    /// circumstances. Some of these error conditions are listed here, together
    /// with their [`ErrorKind`]. The mapping to [`ErrorKind`]s is not part of
    /// the compatibility contract of the function, especially the `Other` kind
    /// might change to more specific kinds in the future.
    ///
    /// - [`NotFound`]: The specified file does not exist and neither `create`
    ///   or `create_new` is set.
    /// - [`NotFound`]: One of the directory components of the file path does
    ///   not exist.
    /// - [`PermissionDenied`]: The user lacks permission to get the specified
    ///   access rights for the file.
    /// - [`PermissionDenied`]: The user lacks permission to open one of the
    ///   directory components of the specified path.
    /// - [`AlreadyExists`]: `create_new` was specified and the file already
    ///   exists.
    /// - [`InvalidInput`]: Invalid combinations of open options (truncate
    ///   without write access, no access mode set, etc.).
    /// - [`Other`]: One of the directory components of the specified file path
    ///   was not, in fact, a directory.
    /// - [`Other`]: Filesystem-level errors: full disk, write permission
    ///   requested on a read-only file system, exceeded disk quota, too many
    ///   open files, too long filename, too many symbolic links in the
    ///   specified path (Unix-like systems only), etc.
    pub async fn open(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> std::io::Result<crate::fs::File> {
        match &self.0 {
            OpenOptionsInner::Std(inner) => inner.open(path).map(crate::fs::File::from),
            #[cfg(tokio_fs)]
            OpenOptionsInner::Tokio(inner) => inner.open(path).await.map(crate::fs::File::from),
        }
    }

    /// Sets the mode bits that a new file will be created with.
    ///
    /// If a new file is created as part of an [`Self::open`] call then this specified mode will be used as the permission bits
    /// for the new file. If no mode is set, the default of `0o666` will be used.
    /// The operating system masks out bits with the system’s umask, to produce the final permissions.
    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    pub fn mode(&mut self, mode: u32) -> &mut OpenOptions {
        use std::os::unix::fs::OpenOptionsExt as _;

        match &mut self.0 {
            OpenOptionsInner::Std(inner) => {
                inner.mode(mode);
            }
            #[cfg(tokio_fs)]
            OpenOptionsInner::Tokio(inner) => {
                inner.mode(mode);
            }
        }
        self
    }

    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    /// Passes custom flags to the flags argument of `open`.
    ///
    /// The bits that define the access mode are masked out with `O_ACCMODE`, to ensure they do not interfere with the access mode set by Rusts options.
    ///
    /// Custom flags can only set flags, not remove flags set by Rusts options. This options overwrites any previously set custom flags.
    pub fn custom_flags(&mut self, flags: i32) -> &mut OpenOptions {
        use std::os::unix::fs::OpenOptionsExt as _;

        match &mut self.0 {
            OpenOptionsInner::Std(inner) => {
                inner.custom_flags(flags);
            }
            #[cfg(tokio_fs)]
            OpenOptionsInner::Tokio(inner) => {
                inner.custom_flags(flags);
            }
        }
        self
    }

    #[cfg(windows)]
    #[cfg_attr(docsrs, doc(cfg(windows)))]
    /// Overrides the dwDesiredAccess argument to the call to `CreateFile` with the specified value.
    ///
    /// This will override the read, write, and append flags on the `OpenOptions` structure. This method provides fine-grained control over the permissions to read, write and append data, attributes (like hidden and system), and extended attributes.
    pub fn access_mode(&mut self, access_mode: u32) -> &mut OpenOptions {
        use std::os::windows::fs::OpenOptionsExt as _;

        match &mut self.0 {
            OpenOptionsInner::Std(inner) => {
                inner.access_mode(access_mode);
            }
            #[cfg(tokio_fs)]
            OpenOptionsInner::Tokio(inner) => {
                inner.access_mode(access_mode);
            }
        }
        self
    }

    #[cfg(windows)]
    #[cfg_attr(docsrs, doc(cfg(windows)))]
    ///Overrides the dwShareMode argument to the call to CreateFile with the specified value.
    /// By default share_mode is set to `FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE`.
    /// This allows other processes to read, write, and delete/rename the same file while it is open. Removing any of the flags will prevent other processes from performing the corresponding operation until the file handle is closed.
    pub fn share_mode(&mut self, share_mode: u32) -> &mut OpenOptions {
        use std::os::windows::fs::OpenOptionsExt as _;

        match &mut self.0 {
            OpenOptionsInner::Std(inner) => {
                inner.share_mode(share_mode);
            }
            #[cfg(tokio_fs)]
            OpenOptionsInner::Tokio(inner) => {
                inner.share_mode(share_mode);
            }
        }
        self
    }

    #[cfg(windows)]
    #[cfg_attr(docsrs, doc(cfg(windows)))]
    /// Sets extra flags for the dwFileFlags argument to the call to CreateFile2 to the specified value (or combines it with attributes and security_qos_flags to set the dwFlagsAndAttributes for CreateFile).
    /// Custom flags can only set flags, not remove flags set by Rust’s options. This option overwrites any previously set custom flags.
    pub fn custom_flags(&mut self, flags: u32) -> &mut OpenOptions {
        use std::os::windows::fs::OpenOptionsExt as _;

        match &mut self.0 {
            OpenOptionsInner::Std(inner) => {
                inner.custom_flags(flags);
            }
            #[cfg(tokio_fs)]
            OpenOptionsInner::Tokio(inner) => {
                inner.custom_flags(flags);
            }
        }
        self
    }

    #[cfg(windows)]
    #[cfg_attr(docsrs, doc(cfg(windows)))]
    /// Sets the dwFileAttributes argument to the call to CreateFile2 to the specified value (or combines it with custom_flags and security_qos_flags to set the dwFlagsAndAttributes for CreateFile).
    ///
    /// If a new file is created because it does not yet exist and .create(true) or .create_new(true) are specified, the new file is given the attributes declared with .attributes().
    ///
    /// If an existing file is opened with .create(true).truncate(true), its existing attributes are preserved and combined with the ones declared with .attributes().
    ///
    /// In all other cases the attributes get ignored.
    pub fn attributes(&mut self, attributes: u32) -> &mut OpenOptions {
        use std::os::windows::fs::OpenOptionsExt as _;

        match &mut self.0 {
            OpenOptionsInner::Std(inner) => {
                inner.attributes(attributes);
            }
            #[cfg(tokio_fs)]
            OpenOptionsInner::Tokio(inner) => {
                inner.attributes(attributes);
            }
        }
        self
    }

    #[cfg(windows)]
    #[cfg_attr(docsrs, doc(cfg(windows)))]
    /// Sets the dwSecurityQosFlags argument to the call to CreateFile2 to the specified value (or combines it with custom_flags and attributes to set the dwFlagsAndAttributes for CreateFile).
    ///
    /// By default security_qos_flags is not set. It should be specified when opening a named pipe, to control to which degree a server process can act on behalf of a client process (security impersonation level).
    ///
    /// When security_qos_flags is not set, a malicious program can gain the elevated privileges of a privileged Rust process when it allows opening user-specified paths, by tricking it into opening a named pipe. So arguably security_qos_flags should also be set when opening arbitrary paths. However the bits can then conflict with other flags, specifically FILE_FLAG_OPEN_NO_RECALL.
    ///
    /// For information about possible values, see Impersonation Levels on the Windows Dev Center site. The SECURITY_SQOS_PRESENT flag is set automatically when using this method.
    pub fn security_qos_flags(&mut self, flags: u32) -> &mut OpenOptions {
        use std::os::windows::fs::OpenOptionsExt as _;

        match &mut self.0 {
            OpenOptionsInner::Std(inner) => {
                inner.security_qos_flags(flags);
            }
            #[cfg(tokio_fs)]
            OpenOptionsInner::Tokio(inner) => {
                inner.security_qos_flags(flags);
            }
        }
        self
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{SyncRuntime, Unwrap};

    #[test]
    fn test_open_options() {
        let options = OpenOptions::new();
        assert!(matches!(options.0, OpenOptionsInner::Std(_)));
    }

    #[tokio::test]
    async fn test_open_options_async() {
        let options = OpenOptions::new();
        assert!(matches!(options.0, OpenOptionsInner::Tokio(_)));
    }

    #[test]
    fn test_open_file_sync() {
        let temp = tempfile::NamedTempFile::new().expect("Failed to create temp file");
        std::fs::write(temp.path(), b"Hello world").expect("Failed to write file");

        SyncRuntime::block_on(OpenOptions::new().read(true).write(true).open(temp.path()))
            .expect("Failed to open file");
    }

    #[tokio::test]
    async fn test_open_file_async() {
        let temp = tempfile::NamedTempFile::new().expect("Failed to create temp file");
        std::fs::write(temp.path(), b"Hello world").expect("Failed to write file");

        OpenOptions::new()
            .read(true)
            .write(true)
            .open(temp.path())
            .await
            .expect("Failed to open file");
    }

    #[test]
    fn test_should_get_underlying_type() {
        let options = OpenOptions::new();
        options.unwrap_std();
    }

    #[tokio::test]
    async fn test_should_get_underlying_type_async() {
        let options = OpenOptions::new();
        options.unwrap_tokio();
    }
}
