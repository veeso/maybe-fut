use std::net::SocketAddr;

use crate::{maybe_fut_constructor_result, maybe_fut_method_sync};

/// A TCP socket server, listening for connections.
///
/// You can accept a new connection by using the [`TcpListener::accept`] method.
///
/// A [`TcpListener`] is created by calling [`TcpListener::bind`].
#[derive(Unwrap, Debug)]
#[unwrap_types(
    std(std::net::TcpListener),
    tokio(tokio::net::TcpListener),
    tokio_gated("tokio-net")
)]
pub struct TcpListener(TcpListenerInner);

#[derive(Debug)]
enum TcpListenerInner {
    Std(std::net::TcpListener),
    #[cfg(tokio_net)]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio-net")))]
    Tokio(tokio::net::TcpListener),
}

impl From<std::net::TcpListener> for TcpListener {
    fn from(listener: std::net::TcpListener) -> Self {
        Self(TcpListenerInner::Std(listener))
    }
}

#[cfg(tokio_net)]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-net")))]
impl From<tokio::net::TcpListener> for TcpListener {
    fn from(listener: tokio::net::TcpListener) -> Self {
        Self(TcpListenerInner::Tokio(listener))
    }
}

impl TcpListener {
    maybe_fut_constructor_result!(
        /// Creates a new [`TcpListener`] bound to the specified address.
        ///
        /// The returned listener is ready for accepting connections.
        bind(addr: SocketAddr) -> std::io::Result<Self>,
        std::net::TcpListener::bind,
        tokio::net::TcpListener::bind,
        tokio_net
    );

    /// Accepts a new incoming connection.
    ///
    ///  This method will block until a new connection is established.
    pub async fn accept(&self) -> std::io::Result<(crate::net::TcpStream, SocketAddr)> {
        match &self.0 {
            TcpListenerInner::Std(listener) => {
                let (stream, addr) = listener.accept()?;
                Ok((crate::net::TcpStream::from(stream), addr))
            }
            #[cfg(tokio_net)]
            TcpListenerInner::Tokio(listener) => {
                let (stream, addr) = listener.accept().await?;
                Ok((crate::net::TcpStream::from(stream), addr))
            }
        }
    }

    maybe_fut_method_sync!(
        /// Returns the local address of this listener.
        local_addr() -> std::io::Result<SocketAddr>,
        TcpListenerInner::Std,
        TcpListenerInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Sets the value for the `IP_TTL` option on this socket.
        set_ttl(ttl: u32) -> std::io::Result<()>,
        TcpListenerInner::Std,
        TcpListenerInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Gets the value for the `IP_TTL` option on this socket.
        ttl() -> std::io::Result<u32>,
        TcpListenerInner::Std,
        TcpListenerInner::Tokio,
        tokio_net
    );
}

#[cfg(unix)]
impl std::os::fd::AsFd for TcpListener {
    fn as_fd(&self) -> std::os::fd::BorrowedFd<'_> {
        match &self.0 {
            TcpListenerInner::Std(file) => file.as_fd(),
            #[cfg(tokio_fs)]
            TcpListenerInner::Tokio(file) => file.as_fd(),
        }
    }
}

#[cfg(unix)]
impl std::os::fd::AsRawFd for TcpListener {
    fn as_raw_fd(&self) -> std::os::fd::RawFd {
        match &self.0 {
            TcpListenerInner::Std(file) => file.as_raw_fd(),
            #[cfg(tokio_fs)]
            TcpListenerInner::Tokio(file) => file.as_raw_fd(),
        }
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsSocket for TcpListener {
    fn as_fd(&self) -> std::os::windows::io::BorrowedSocket<'_> {
        match &self.0 {
            TcpListenerInner::Std(file) => file.as_socket(),
            #[cfg(tokio_fs)]
            TcpListenerInner::Tokio(file) => file.as_socket(),
        }
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsSocket for TcpListener {
    fn as_fd(&self) -> std::os::windows::io::BorrowedSocket<'_> {
        match &self.0 {
            TcpListenerInner::Std(file) => file.as_socket(),
            #[cfg(tokio_fs)]
            TcpListenerInner::Tokio(file) => file.as_socket(),
        }
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsRawSocket for TcpListener {
    fn as_fd(&self) -> std::os::windows::io::RawSocket {
        match &self.0 {
            TcpListenerInner::Std(file) => file.as_raw_socket(),
            #[cfg(tokio_fs)]
            TcpListenerInner::Tokio(file) => file.as_raw_socket(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{Unwrap, block_on};

    #[test]
    #[serial_test::serial]
    fn test_should_bind_from_std() {
        let addr = "127.0.0.1:0"
            .parse::<SocketAddr>()
            .expect("Failed to parse address");

        assert!(block_on(TcpListener::bind(addr)).is_ok());
    }

    #[cfg(tokio_net)]
    #[tokio::test]
    #[serial_test::serial]
    async fn test_should_bind_from_tokio() {
        let addr = "127.0.0.1:0"
            .parse::<SocketAddr>()
            .expect("Failed to parse address");

        assert!(TcpListener::bind(addr).await.is_ok());
    }

    #[test]
    #[serial_test::serial]
    fn test_should_accept_from_std() {
        let addr = "127.0.0.1:0"
            .parse::<SocketAddr>()
            .expect("Failed to parse address");

        let listener = block_on(TcpListener::bind(addr)).expect("Failed to bind listener");

        // Create a stream to connect to the listener
        let peer_address = listener.local_addr().expect("Failed to get local address");
        let _stream =
            std::net::TcpStream::connect(peer_address).expect("Failed to connect to listener");
        let (accepted_stream, _accepted_addr) =
            block_on(listener.accept()).expect("Failed to accept connection");

        assert!(accepted_stream.get_std_ref().is_some());
    }

    #[cfg(tokio_net)]
    #[tokio::test]
    #[serial_test::serial]
    async fn test_should_accept_from_tokio() {
        let addr = "127.0.0.1:0"
            .parse::<SocketAddr>()
            .expect("Failed to parse address");

        let listener = TcpListener::bind(addr)
            .await
            .expect("Failed to bind listener");

        let peer_address = listener.local_addr().expect("Failed to get local address");

        // Create a stream to connect to the listener
        let _stream = tokio::net::TcpStream::connect(peer_address)
            .await
            .expect("Failed to connect to listener");
        let (accepted_stream, _accepted_addr) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        assert!(accepted_stream.get_tokio_ref().is_some());
    }

    #[test]
    #[serial_test::serial]
    fn test_should_set_and_get_ttl_from_std() {
        let addr = "127.0.0.1:0"
            .parse::<SocketAddr>()
            .expect("Failed to parse address");

        let listener = block_on(TcpListener::bind(addr)).expect("Failed to bind listener");

        // Set TTL
        let ttl = 64;
        listener.set_ttl(ttl).expect("Failed to set TTL");

        // Get TTL
        let retrieved_ttl = listener.ttl().expect("Failed to get TTL");
        assert_eq!(retrieved_ttl, ttl);
    }

    #[cfg(tokio_net)]
    #[tokio::test]
    #[serial_test::serial]
    async fn test_should_set_and_get_ttl_from_tokio() {
        let addr = "127.0.0.1:0"
            .parse::<SocketAddr>()
            .expect("Failed to parse address");

        let listener = TcpListener::bind(addr)
            .await
            .expect("Failed to bind listener");

        // Set TTL
        let ttl = 64;
        listener.set_ttl(ttl).expect("Failed to set TTL");

        // Get TTL
        let retrieved_ttl = listener.ttl().expect("Failed to get TTL");
        assert_eq!(retrieved_ttl, ttl);
    }

    #[test]
    #[serial_test::serial]
    fn test_should_get_local_addr_from_std() {
        let addr = "127.0.0.1:0"
            .parse::<SocketAddr>()
            .expect("Failed to parse address");

        let listener = block_on(TcpListener::bind(addr)).expect("Failed to bind listener");

        let local_addr = listener.local_addr().expect("Failed to get local address");
        assert_eq!(local_addr.ip(), addr.ip());
        assert!(local_addr.port() > 0);
    }

    #[cfg(tokio_net)]
    #[tokio::test]
    #[serial_test::serial]
    async fn test_should_get_local_addr_from_tokio() {
        let addr = "127.0.0.1:0"
            .parse::<SocketAddr>()
            .expect("Failed to parse address");

        let listener = TcpListener::bind(addr)
            .await
            .expect("Failed to bind listener");

        let local_addr = listener.local_addr().expect("Failed to get local address");
        assert_eq!(local_addr.ip(), addr.ip());
        assert!(local_addr.port() > 0);
    }
}
