/// A handle to the standard input stream of a process.
#[derive(Debug, Read, Unwrap)]
#[io(feature("tokio"))]
#[unwrap_types(std(std::io::Stdin), tokio(tokio::io::Stdin), tokio_gated("tokio"))]
pub struct Stdin(StdinInner);

#[derive(Debug)]
enum StdinInner {
    Std(std::io::Stdin),
    #[cfg(tokio)]
    Tokio(tokio::io::Stdin),
}

impl From<std::io::Stdin> for Stdin {
    fn from(stdin: std::io::Stdin) -> Self {
        Self(StdinInner::Std(stdin))
    }
}

#[cfg(tokio)]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl From<tokio::io::Stdin> for Stdin {
    fn from(stdin: tokio::io::Stdin) -> Self {
        Self(StdinInner::Tokio(stdin))
    }
}

/// Constructs a new handle to the standard input of the current process.
pub fn stdin() -> Stdin {
    #[cfg(tokio)]
    {
        if crate::is_async_context() {
            tokio::io::stdin().into()
        } else {
            std::io::stdin().into()
        }
    }
    #[cfg(not(tokio))]
    {
        std::io::stdin().into()
    }
}

#[cfg(unix)]
impl std::os::fd::AsFd for Stdin {
    fn as_fd(&self) -> std::os::fd::BorrowedFd<'_> {
        match &self.0 {
            StdinInner::Std(file) => file.as_fd(),
            #[cfg(tokio)]
            StdinInner::Tokio(file) => file.as_fd(),
        }
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsHandle for Stdin {
    fn as_handle(&self) -> std::os::windows::io::BorrowedHandle<'_> {
        match &self.0 {
            StdinInner::Std(file) => file.as_handle(),
            #[cfg(tokio)]
            StdinInner::Tokio(file) => file.as_handle(),
        }
    }
}

#[cfg(unix)]
impl std::os::fd::AsRawFd for Stdin {
    fn as_raw_fd(&self) -> std::os::fd::RawFd {
        match &self.0 {
            StdinInner::Std(file) => file.as_raw_fd(),
            #[cfg(tokio)]
            StdinInner::Tokio(file) => file.as_raw_fd(),
        }
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsRawHandle for Stdin {
    fn as_raw_handle(&self) -> std::os::windows::io::RawHandle {
        match &self.0 {
            StdinInner::Std(file) => file.as_raw_handle(),
            #[cfg(tokio)]
            StdinInner::Tokio(file) => file.as_raw_handle(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_should_stdin_sync() {
        let stdin = stdin();
        assert!(matches!(stdin.0, StdinInner::Std(_)));
    }

    #[cfg(tokio)]
    #[tokio::test]
    async fn test_should_stdin_async() {
        let stdin = stdin();
        assert!(matches!(stdin.0, StdinInner::Tokio(_)));
    }
}
