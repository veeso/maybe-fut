/// A handle to the standard error stream of a process.
#[derive(Debug, Write, Unwrap)]
#[io(feature("tokio"))]
#[unwrap_types(std(std::io::Stderr), tokio(tokio::io::Stderr), tokio_gated("tokio"))]
pub struct Stderr(StderrInner);

#[derive(Debug)]
enum StderrInner {
    Std(std::io::Stderr),
    #[cfg(tokio)]
    Tokio(tokio::io::Stderr),
}

impl From<std::io::Stderr> for Stderr {
    fn from(stderr: std::io::Stderr) -> Self {
        Self(StderrInner::Std(stderr))
    }
}

#[cfg(tokio)]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl From<tokio::io::Stderr> for Stderr {
    fn from(stderr: tokio::io::Stderr) -> Self {
        Self(StderrInner::Tokio(stderr))
    }
}

/// Constructs a new handle to the error output of the current process.
pub fn stderr() -> Stderr {
    #[cfg(tokio)]
    {
        if crate::is_async_context() {
            tokio::io::stderr().into()
        } else {
            std::io::stderr().into()
        }
    }
    #[cfg(not(tokio))]
    {
        std::io::stderr().into()
    }
}

#[cfg(unix)]
impl std::os::fd::AsFd for Stderr {
    fn as_fd(&self) -> std::os::fd::BorrowedFd<'_> {
        match &self.0 {
            StderrInner::Std(file) => file.as_fd(),
            #[cfg(tokio)]
            StderrInner::Tokio(file) => file.as_fd(),
        }
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsHandle for Stderr {
    fn as_handle(&self) -> std::os::windows::io::BorrowedHandle<'_> {
        match &self.0 {
            StderrInner::Std(file) => file.as_handle(),
            #[cfg(tokio)]
            StderrInner::Tokio(file) => file.as_handle(),
        }
    }
}

#[cfg(unix)]
impl std::os::fd::AsRawFd for Stderr {
    fn as_raw_fd(&self) -> std::os::fd::RawFd {
        match &self.0 {
            StderrInner::Std(file) => file.as_raw_fd(),
            #[cfg(tokio)]
            StderrInner::Tokio(file) => file.as_raw_fd(),
        }
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsRawHandle for Stderr {
    fn as_raw_handle(&self) -> std::os::windows::io::RawHandle {
        match &self.0 {
            StderrInner::Std(file) => file.as_raw_handle(),
            #[cfg(tokio)]
            StderrInner::Tokio(file) => file.as_raw_handle(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_should_stderr_sync() {
        let stderr = stderr();
        assert!(matches!(stderr.0, StderrInner::Std(_)));
    }

    #[cfg(tokio)]
    #[tokio::test]
    async fn test_should_stderr_async() {
        let stderr = stderr();
        assert!(matches!(stderr.0, StderrInner::Tokio(_)));
    }
}
