use std::net::SocketAddr;

use crate::{maybe_fut_constructor_result, maybe_fut_method, maybe_fut_method_sync};

/// A TCP stream between a local and a remote socket.
///
/// A TCP Stream can either be created by connecting to an endpoint, via the [`TcpStream::connect`] method,
/// or by [`super::TcpListener::accept`]ing a connection from a [`super::TcpListener`].
///
/// Reading and writing to a [`TcpStream`] is usually done by using the [`crate::io::Read`] and [`crate::io::Write`] traits.
#[derive(Debug, Unwrap, Read, Write)]
#[io(feature("tokio-net"))]
#[unwrap_types(
    std(std::net::TcpStream),
    tokio(tokio::net::TcpStream),
    tokio_gated("tokio-net")
)]
pub struct TcpStream(TcpStreamInner);

#[derive(Debug)]
enum TcpStreamInner {
    Std(std::net::TcpStream),
    #[cfg(tokio_net)]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio-net")))]
    Tokio(tokio::net::TcpStream),
}

impl From<std::net::TcpStream> for TcpStream {
    fn from(stream: std::net::TcpStream) -> Self {
        Self(TcpStreamInner::Std(stream))
    }
}

#[cfg(tokio_net)]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-net")))]
impl From<tokio::net::TcpStream> for TcpStream {
    fn from(stream: tokio::net::TcpStream) -> Self {
        Self(TcpStreamInner::Tokio(stream))
    }
}

#[cfg(unix)]
impl std::os::fd::AsFd for TcpStream {
    fn as_fd(&self) -> std::os::fd::BorrowedFd<'_> {
        match &self.0 {
            TcpStreamInner::Std(file) => file.as_fd(),
            #[cfg(tokio_fs)]
            TcpStreamInner::Tokio(file) => file.as_fd(),
        }
    }
}

#[cfg(unix)]
impl std::os::fd::AsRawFd for TcpStream {
    fn as_raw_fd(&self) -> std::os::fd::RawFd {
        match &self.0 {
            TcpStreamInner::Std(file) => file.as_raw_fd(),
            #[cfg(tokio_fs)]
            TcpStreamInner::Tokio(file) => file.as_raw_fd(),
        }
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsSocket for TcpStream {
    fn as_fd(&self) -> std::os::windows::io::BorrowedSocket<'_> {
        match &self.0 {
            TcpStreamInner::Std(file) => file.as_socket(),
            #[cfg(tokio_fs)]
            TcpStreamInner::Tokio(file) => file.as_socket(),
        }
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsSocket for TcpStream {
    fn as_fd(&self) -> std::os::windows::io::BorrowedSocket<'_> {
        match &self.0 {
            TcpStreamInner::Std(file) => file.as_socket(),
            #[cfg(tokio_fs)]
            TcpStreamInner::Tokio(file) => file.as_socket(),
        }
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsRawSocket for TcpStream {
    fn as_fd(&self) -> std::os::windows::io::RawSocket {
        match &self.0 {
            TcpStreamInner::Std(file) => file.as_raw_socket(),
            #[cfg(tokio_fs)]
            TcpStreamInner::Tokio(file) => file.as_raw_socket(),
        }
    }
}

impl TcpStream {
    maybe_fut_constructor_result!(
        /// Opens a TCP connection to a remote host at the specified address.
        connect(addr: SocketAddr) -> std::io::Result<TcpStream>,
        std::net::TcpStream::connect,
        tokio::net::TcpStream::connect,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Returns the local address that this stream is bound to.
        local_addr() -> std::io::Result<SocketAddr>,
        TcpStreamInner::Std,
        TcpStreamInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Returns the value of the `SO_ERROR` option.
        take_error() -> std::io::Result<Option<std::io::Error>>,
        TcpStreamInner::Std,
        TcpStreamInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Returns the remote address that this stream is connected to.
        peer_addr() -> std::io::Result<SocketAddr>,
        TcpStreamInner::Std,
        TcpStreamInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Gets the value of the `TCP_NODELAY` option on this socket.
        nodelay() -> std::io::Result<bool>,
        TcpStreamInner::Std,
        TcpStreamInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Sets the value of the `TCP_NODELAY` option on this socket.
        set_nodelay(nodelay: bool) -> std::io::Result<()>,
        TcpStreamInner::Std,
        TcpStreamInner::Tokio,
        tokio_net
    );

    maybe_fut_method!(
        /// Receives data on the socket from the remote address to which it is connected, without removing that data from the queue.
        /// On success, returns the number of bytes read.
        peek(buf: &mut [u8]) -> std::io::Result<usize>,
        TcpStreamInner::Std,
        TcpStreamInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Gets the value of the `IP_TTL` option on this socket.
        ttl() -> std::io::Result<u32>,
        TcpStreamInner::Std,
        TcpStreamInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Sets the value of the `IP_TTL` option on this socket.
        set_ttl(ttl: u32) -> std::io::Result<()>,
        TcpStreamInner::Std,
        TcpStreamInner::Tokio,
        tokio_net
    );
}

#[cfg(test)]
mod test {

    use std::io::{Read as _, Write as _};
    use std::net::TcpListener;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::thread::JoinHandle;

    use super::*;
    use crate::block_on;
    use crate::io::{Read as _, Write};

    #[test]
    #[serial_test::serial]
    fn test_should_connect_std() {
        let (_join, peer_addr, exit) = ping_server();
        assert!(block_on(TcpStream::connect(peer_addr)).is_ok());

        exit.store(true, std::sync::atomic::Ordering::Relaxed);
        // join.join().expect("Failed to join server thread");
    }

    #[cfg(tokio_net)]
    #[tokio::test]
    #[serial_test::serial]
    async fn test_should_connect_tokio() {
        let (_join, peer_addr, exit) = ping_server();
        assert!(TcpStream::connect(peer_addr).await.is_ok());

        exit.store(true, std::sync::atomic::Ordering::Relaxed);
        // join.join().expect("Failed to join server thread");
    }

    #[test]
    #[serial_test::serial]
    fn test_should_get_local_and_peer_addr() {
        let (_join, peer_addr, exit) = ping_server();
        let stream = block_on(TcpStream::connect(peer_addr)).unwrap();

        assert!(stream.local_addr().is_ok());
        assert_eq!(stream.peer_addr().unwrap(), peer_addr);

        exit.store(true, std::sync::atomic::Ordering::Relaxed);
        // join.join().expect("Failed to join server thread");
    }

    #[cfg(tokio_net)]
    #[tokio::test]
    #[serial_test::serial]
    async fn test_should_get_local_and_peer_addr_tokio() {
        let (_join, peer_addr, exit) = ping_server();
        let stream = TcpStream::connect(peer_addr).await.unwrap();
        assert!(stream.local_addr().is_ok());
        assert_eq!(stream.peer_addr().unwrap(), peer_addr);

        exit.store(true, std::sync::atomic::Ordering::Relaxed);
        // join.join().expect("Failed to join server thread");
    }

    #[test]
    #[serial_test::serial]
    fn test_should_get_nodelay() {
        let (_join, peer_addr, exit) = ping_server();
        let stream = block_on(TcpStream::connect(peer_addr)).unwrap();
        assert!(stream.nodelay().is_ok());
        assert!(stream.set_nodelay(true).is_ok());
        assert!(stream.nodelay().unwrap());
        assert!(stream.set_nodelay(false).is_ok());
        assert!(!stream.nodelay().unwrap());

        exit.store(true, std::sync::atomic::Ordering::Relaxed);
        // join.join().expect("Failed to join server thread");
    }

    #[cfg(tokio_net)]
    #[tokio::test]
    #[serial_test::serial]
    async fn test_should_get_nodelay_tokio() {
        let (_join, peer_addr, exit) = ping_server();
        let stream = TcpStream::connect(peer_addr).await.unwrap();
        assert!(stream.nodelay().is_ok());
        assert!(stream.set_nodelay(true).is_ok());
        assert!(stream.nodelay().unwrap());
        assert!(stream.set_nodelay(false).is_ok());
        assert!(!stream.nodelay().unwrap());

        exit.store(true, std::sync::atomic::Ordering::Relaxed);
        // join.join().expect("Failed to join server thread");
    }

    #[test]
    #[serial_test::serial]
    fn test_should_get_ttl() {
        let (_join, peer_addr, exit) = ping_server();
        let stream = block_on(TcpStream::connect(peer_addr)).unwrap();
        assert!(stream.ttl().is_ok());
        assert!(stream.set_ttl(64).is_ok());
        assert_eq!(stream.ttl().unwrap(), 64);

        exit.store(true, std::sync::atomic::Ordering::Relaxed);
        // join.join().expect("Failed to join server thread");
    }

    #[cfg(tokio_net)]
    #[tokio::test]
    async fn test_should_get_ttl_tokio() {
        let (_join, peer_addr, exit) = ping_server();
        let stream = TcpStream::connect(peer_addr).await.unwrap();
        assert!(stream.ttl().is_ok());
        assert!(stream.set_ttl(64).is_ok());
        assert_eq!(stream.ttl().unwrap(), 64);

        exit.store(true, std::sync::atomic::Ordering::Relaxed);
        // join.join().expect("Failed to join server thread");
    }

    #[test]
    #[serial_test::serial]
    fn test_should_read_and_write_from_tcp_stream_std() {
        let (_join, peer_addr, exit) = ping_server();

        let mut stream = block_on(TcpStream::connect(peer_addr)).unwrap();
        block_on(stream.write_all(b"Ping")).expect("Failed to write to stream");
        let mut buf = [0; 1024];
        let size = block_on(stream.read(&mut buf)).expect("Failed to read from stream");
        assert_eq!(size, 4);
        assert_eq!(&buf[..size], b"Pong");
        exit.store(true, std::sync::atomic::Ordering::Relaxed);

        // join.join().expect("Failed to join server thread");
    }

    #[cfg(tokio_net)]
    #[tokio::test]
    #[serial_test::serial]
    async fn test_should_read_and_write_from_tcp_stream_tokio() {
        let (_join, peer_addr, exit) = ping_server();

        let mut stream = TcpStream::connect(peer_addr).await.unwrap();
        stream
            .write_all(b"Ping")
            .await
            .expect("Failed to write to stream");
        let mut buf = [0; 1024];
        let size = stream
            .read(&mut buf)
            .await
            .expect("Failed to read from stream");
        assert_eq!(size, 4);
        assert_eq!(&buf[..size], b"Pong");
        exit.store(true, std::sync::atomic::Ordering::Relaxed);

        // join.join().expect("Failed to join server thread");
    }

    fn ping_server() -> (JoinHandle<()>, SocketAddr, Arc<AtomicBool>) {
        // sleep for a random amount of time
        std::thread::sleep(std::time::Duration::from_millis(
            rand::random::<u64>() % 1000,
        ));

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener
            .set_nonblocking(true)
            .expect("Failed to set listener to non-blocking");
        let addr = listener.local_addr().unwrap();

        let exit = Arc::new(AtomicBool::new(false));
        let exit_clone = exit.clone();

        let join = std::thread::spawn(move || {
            while !exit_clone.load(std::sync::atomic::Ordering::Relaxed) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        println!("Accepted connection from {}", stream.peer_addr().unwrap());

                        // read
                        let mut buf = [0; 1024];
                        if let Ok(size) = stream.read(&mut buf) {
                            if size > 0 {
                                println!("Received: {}", String::from_utf8_lossy(&buf[..size]));
                            }
                        }
                        // write
                        if let Err(e) = stream.write_all(b"Pong") {
                            eprintln!("Failed to write to stream: {}", e);
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // wait for next connection
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                    Err(e) => {
                        eprintln!("Failed to accept connection: {}", e);
                        break;
                    }
                }
            }
        });

        (join, addr, exit)
    }
}
