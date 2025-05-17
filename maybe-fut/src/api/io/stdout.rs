/// A handle to the standard output stream of a process.
#[derive(Debug, Write)]
#[io(feature("tokio"))]
pub struct Stdout(StdoutInner);

#[derive(Debug)]
enum StdoutInner {
    Std(std::io::Stdout),
    #[cfg(tokio)]
    Tokio(tokio::io::Stdout),
}

impl From<std::io::Stdout> for Stdout {
    fn from(stdout: std::io::Stdout) -> Self {
        Self(StdoutInner::Std(stdout))
    }
}

#[cfg(tokio)]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl From<tokio::io::Stdout> for Stdout {
    fn from(stdout: tokio::io::Stdout) -> Self {
        Self(StdoutInner::Tokio(stdout))
    }
}

/// Constructs a new handle to the standard output of the current process.
pub fn stdout() -> Stdout {
    #[cfg(tokio)]
    {
        if crate::is_async_context() {
            tokio::io::stdout().into()
        } else {
            std::io::stdout().into()
        }
    }
    #[cfg(not(tokio))]
    {
        std::io::stdout().into()
    }
}

#[cfg(unix)]
impl std::os::fd::AsFd for Stdout {
    fn as_fd(&self) -> std::os::fd::BorrowedFd<'_> {
        match &self.0 {
            StdoutInner::Std(file) => file.as_fd(),
            #[cfg(tokio)]
            StdoutInner::Tokio(file) => file.as_fd(),
        }
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsHandle for Stdout {
    fn as_handle(&self) -> std::os::windows::io::BorrowedHandle<'_> {
        match &self.0 {
            FileInner::Std(file) => file.as_handle(),
            #[cfg(tokio)]
            FileInner::Tokio(file) => file.as_handle(),
        }
    }
}

#[cfg(unix)]
impl std::os::fd::AsRawFd for Stdout {
    fn as_raw_fd(&self) -> std::os::fd::RawFd {
        match &self.0 {
            StdoutInner::Std(file) => file.as_raw_fd(),
            #[cfg(tokio)]
            StdoutInner::Tokio(file) => file.as_raw_fd(),
        }
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsRawHandle for Stdout {
    fn as_raw_handle(&self) -> std::os::windows::io::RawHandle {
        match &self.0 {
            StdoutInner::Std(file) => file.as_raw_handle(),
            #[cfg(tokio)]
            StdoutInner::Tokio(file) => file.as_raw_handle(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_should_stdout_sync() {
        let stdout = stdout();
        assert!(matches!(stdout.0, StdoutInner::Std(_)));
    }

    #[cfg(tokio)]
    #[tokio::test]
    async fn test_should_stdout_async() {
        let stdout = stdout();
        assert!(matches!(stdout.0, StdoutInner::Tokio(_)));
    }
}
