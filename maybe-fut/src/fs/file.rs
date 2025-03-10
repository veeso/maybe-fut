//! The main type for interacting with the file system is the [`File`] type.
//! This type provides methods for reading and writing to files.

#[derive(Debug)]
/// A reference to an open file on the filesystem.
pub struct File(FileInner);

/// Inner pointer to sync or async file.
#[derive(Debug)]
enum FileInner {
    /// Std variant of file
    Std(std::fs::File),
    #[cfg(tokio_fs)]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio-fs")))]
    /// Tokio variant of file
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

#[cfg(test)]
mod test {

    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_should_instantiate_file() {
        let temp = NamedTempFile::new().expect("Failed to create temp file");
        let f = std::fs::File::create(temp.path()).expect("Failed to create file");

        let maybe_fut_file = File::from(f);
    }
}
