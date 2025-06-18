use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};

use crate::{maybe_fut_constructor_result, maybe_fut_method, maybe_fut_method_sync};

/// A UDP Socket.
///
/// UDP is "connectionless", unlike TCP.
///
/// Meaning, regardless of what address you’ve bound to, a [`UdpSocket`] is free to communicate with many different remotes.
#[derive(Debug, Unwrap)]
#[unwrap_types(
    std(std::net::UdpSocket),
    tokio(tokio::net::UdpSocket),
    tokio_gated("tokio-net")
)]
pub struct UdpSocket(UdpSocketInner);

#[derive(Debug)]
enum UdpSocketInner {
    Std(std::net::UdpSocket),
    #[cfg(feature = "tokio-net")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio-net")))]
    Tokio(tokio::net::UdpSocket),
}

impl From<std::net::UdpSocket> for UdpSocket {
    fn from(socket: std::net::UdpSocket) -> Self {
        UdpSocket(UdpSocketInner::Std(socket))
    }
}

#[cfg(feature = "tokio-net")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-net")))]
impl From<tokio::net::UdpSocket> for UdpSocket {
    fn from(socket: tokio::net::UdpSocket) -> Self {
        UdpSocket(UdpSocketInner::Tokio(socket))
    }
}

#[cfg(unix)]
impl std::os::fd::AsFd for UdpSocket {
    fn as_fd(&self) -> std::os::fd::BorrowedFd<'_> {
        match &self.0 {
            UdpSocketInner::Std(file) => file.as_fd(),
            #[cfg(tokio_net)]
            UdpSocketInner::Tokio(file) => file.as_fd(),
        }
    }
}

#[cfg(unix)]
impl std::os::fd::AsRawFd for UdpSocket {
    fn as_raw_fd(&self) -> std::os::fd::RawFd {
        match &self.0 {
            UdpSocketInner::Std(file) => file.as_raw_fd(),
            #[cfg(tokio_net)]
            UdpSocketInner::Tokio(file) => file.as_raw_fd(),
        }
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsSocket for UdpSocket {
    fn as_socket(&self) -> std::os::windows::io::BorrowedSocket<'_> {
        match &self.0 {
            UdpSocketInner::Std(file) => file.as_socket(),
            #[cfg(tokio_net)]
            UdpSocketInner::Tokio(file) => file.as_socket(),
        }
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsRawSocket for UdpSocket {
    fn as_raw_socket(&self) -> std::os::windows::io::RawSocket {
        match &self.0 {
            UdpSocketInner::Std(file) => file.as_raw_socket(),
            #[cfg(tokio_net)]
            UdpSocketInner::Tokio(file) => file.as_raw_socket(),
        }
    }
}

impl UdpSocket {
    maybe_fut_constructor_result!(
        /// Creates a new UDP socket from the given address.
        bind(addr: std::net::SocketAddr) -> std::io::Result<UdpSocket>,
        std::net::UdpSocket::bind,
        tokio::net::UdpSocket::bind,
        tokio_net
    );

    maybe_fut_method!(
        /// Receives a single datagram messages on the socket.
        ///
        /// On success, returns the number of bytes read and the source address.
        recv_from(buf: &mut [u8]) -> std::io::Result<(usize, std::net::SocketAddr)>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    maybe_fut_method!(
        /// Receives a single datagram message on the socket, without removing it from the queue.
        ///
        /// On success, returns the number of bytes read and the source address.
        peek_from(buf: &mut [u8]) -> std::io::Result<(usize, std::net::SocketAddr)>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    maybe_fut_method!(
        /// Sends data on the socket to the given address.
        ///
        /// On Success, returns the number of bytes written.
        /// Note that the operating system may refuse buffers larger than `65507` bytes.
        send_to(buf: &[u8], target: std::net::SocketAddr) -> std::io::Result<usize>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Returns the socket address of the remote peer this socket was connected to.
        peer_addr() -> std::io::Result<std::net::SocketAddr>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Returns the socket address of the local endpoint this socket is bound to.
        local_addr() -> std::io::Result<std::net::SocketAddr>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    /// Creates a new independently owned handle to the same socket.
    ///
    /// It doesn't work with Tokio's `UdpSocket` because it doesn't support cloning.
    pub fn try_clone(&self) -> std::io::Result<Self> {
        match &self.0 {
            UdpSocketInner::Std(socket) => socket.try_clone().map(UdpSocket::from),
            #[cfg(feature = "tokio-net")]
            UdpSocketInner::Tokio(_) => Err(std::io::Error::other(
                "Tokio UdpSocket does not support try_clone",
            )),
        }
    }

    /// Sets the read timeout for the socket.
    ///
    /// It doesn't work with Tokio's `UdpSocket` because it doesn't support setting timeouts.
    pub fn set_read_timeout(&self, timeout: Option<std::time::Duration>) -> std::io::Result<()> {
        match &self.0 {
            UdpSocketInner::Std(socket) => socket.set_read_timeout(timeout),
            #[cfg(feature = "tokio-net")]
            UdpSocketInner::Tokio(_) => Err(std::io::Error::other(
                "Tokio UdpSocket does not support set_read_timeout",
            )),
        }
    }

    /// Sets the write timeout for the socket.
    ///
    /// It doesn't work with Tokio's `UdpSocket` because it doesn't support setting timeouts.
    pub fn set_write_timeout(&self, timeout: Option<std::time::Duration>) -> std::io::Result<()> {
        match &self.0 {
            UdpSocketInner::Std(socket) => socket.set_write_timeout(timeout),
            #[cfg(feature = "tokio-net")]
            UdpSocketInner::Tokio(_) => Err(std::io::Error::other(
                "Tokio UdpSocket does not support set_read_timeout",
            )),
        }
    }

    /// Returns the read and write timeouts for the socket.
    ///
    /// It doesn't work with Tokio's `UdpSocket` because it doesn't support timeouts.
    pub fn read_timeout(&self) -> std::io::Result<Option<std::time::Duration>> {
        match &self.0 {
            UdpSocketInner::Std(socket) => socket.read_timeout(),
            #[cfg(feature = "tokio-net")]
            UdpSocketInner::Tokio(_) => Err(std::io::Error::other(
                "Tokio UdpSocket does not support read_timeout",
            )),
        }
    }

    /// Returns the read and write timeouts for the socket.
    ///
    /// It doesn't work with Tokio's `UdpSocket` because it doesn't support timeouts.
    pub fn write_timeout(&self) -> std::io::Result<Option<std::time::Duration>> {
        match &self.0 {
            UdpSocketInner::Std(socket) => socket.write_timeout(),
            #[cfg(feature = "tokio-net")]
            UdpSocketInner::Tokio(_) => Err(std::io::Error::other(
                "Tokio UdpSocket does not support write_timeout",
            )),
        }
    }

    maybe_fut_method_sync!(
        /// Sets the value for the `SO_BROADCAST` option on the socket.
        set_broadcast(broadcast: bool) -> std::io::Result<()>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Gets the value for the `SO_BROADCAST` option on the socket.
        broadcast() -> std::io::Result<bool>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Sets the value of the `IP_MULTICAST_LOOP` option on the socket.
        set_multicast_loop_v4(loop_v4: bool) -> std::io::Result<()>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Gets the value of the `IP_MULTICAST_LOOP` option on the socket.
        multicast_loop_v4() -> std::io::Result<bool>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Sets the value of the `IP_MULTICAST_TTL` option on the socket.
        set_multicast_ttl_v4(ttl: u32) -> std::io::Result<()>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Gets the value of the `IP_MULTICAST_TTL` option on the socket.
        multicast_ttl_v4() -> std::io::Result<u32>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Sets the value of the `IPV6_MULTICAST_LOOP` option on the socket.
        set_multicast_loop_v6(loop_v6: bool) -> std::io::Result<()>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Gets the value of the `IPV6_MULTICAST_LOOP` option on the socket.
        multicast_loop_v6() -> std::io::Result<bool>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Sets the value of the `IP_TTL` option on the socket.
        set_ttl(ttl: u32) -> std::io::Result<()>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    maybe_fut_method_sync!(
        /// Gets the value of the `IP_TTL` option on the socket.
        ttl() -> std::io::Result<u32>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    /// Executes an operation of the `IP_ADD_MEMBERSHIP` type
    pub fn join_multicast_v4(
        &self,
        multiaddr: &Ipv4Addr,
        interface: &Ipv4Addr,
    ) -> std::io::Result<()> {
        match &self.0 {
            UdpSocketInner::Std(socket) => socket.join_multicast_v4(multiaddr, interface),
            #[cfg(feature = "tokio-net")]
            UdpSocketInner::Tokio(socket) => socket.join_multicast_v4(*multiaddr, *interface),
        }
    }

    /// Executes an operation of the `IPV6_ADD_MEMBERSHIP` type
    pub fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> std::io::Result<()> {
        match &self.0 {
            UdpSocketInner::Std(socket) => socket.join_multicast_v6(multiaddr, interface),
            #[cfg(feature = "tokio-net")]
            UdpSocketInner::Tokio(socket) => socket.join_multicast_v6(multiaddr, interface),
        }
    }

    pub fn leave_multicast_v4(
        &self,
        multiaddr: &Ipv4Addr,
        interface: &Ipv4Addr,
    ) -> std::io::Result<()> {
        match &self.0 {
            UdpSocketInner::Std(socket) => socket.leave_multicast_v4(multiaddr, interface),
            #[cfg(feature = "tokio-net")]
            UdpSocketInner::Tokio(socket) => socket.leave_multicast_v4(*multiaddr, *interface),
        }
    }

    pub fn leave_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> std::io::Result<()> {
        match &self.0 {
            UdpSocketInner::Std(socket) => socket.leave_multicast_v6(multiaddr, interface),
            #[cfg(feature = "tokio-net")]
            UdpSocketInner::Tokio(socket) => socket.leave_multicast_v6(multiaddr, interface),
        }
    }

    maybe_fut_method_sync!(
        /// Gets the value of the `SO_ERROR` option on the socket.
        take_error() -> std::io::Result<Option<std::io::Error>>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    /// Connects this UDP socket to a remote address,
    /// allowing the send and recv syscalls to be used to send data and also applies filters to only
    /// receive data from the specified address.
    pub async fn connect(&self, addr: SocketAddr) -> std::io::Result<()> {
        match &self.0 {
            UdpSocketInner::Std(socket) => socket.connect(addr),
            #[cfg(feature = "tokio-net")]
            UdpSocketInner::Tokio(socket) => socket.connect(addr).await,
        }
    }

    maybe_fut_method!(
        /// Sendss data on the socket to the remote address this socket is connected to.
        ///
        /// On Success, returns the number of bytes written.
        send(buf: &[u8]) -> std::io::Result<usize>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    maybe_fut_method!(
        /// Receives a single datagram message on the socket.
        ///
        /// On success, returns the number of bytes read.
        recv(buf: &mut [u8]) -> std::io::Result<usize>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    maybe_fut_method!(
        /// Receives a single datagram message on the socket, without removing it from the queue.
        ///
        /// On success, returns the number of bytes read.
        peek(buf: &mut [u8]) -> std::io::Result<usize>,
        UdpSocketInner::Std,
        UdpSocketInner::Tokio,
        tokio_net
    );

    /// Moves this UDP socket into or out of non-blocking mode.
    ///
    /// It doesn't work with Tokio's `UdpSocket` because it doesn't support non-blocking mode.
    pub fn set_nonblocking(&self, nonblocking: bool) -> std::io::Result<()> {
        match &self.0 {
            UdpSocketInner::Std(socket) => socket.set_nonblocking(nonblocking),
            #[cfg(feature = "tokio-net")]
            UdpSocketInner::Tokio(_) => Err(std::io::Error::other(
                "Tokio UdpSocket does not support set_nonblocking",
            )),
        }
    }
}

#[cfg(test)]
mod test {

    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::thread::JoinHandle;

    use super::*;
    use crate::{Unwrap, block_on};

    #[test]
    #[serial_test::serial]
    fn test_should_bind_udp_std() {
        let socket = block_on(UdpSocket::bind(
            "127.0.0.1:0"
                .parse::<SocketAddr>()
                .expect("failed to parse"),
        ))
        .expect("failed to bind UDP socket");

        assert!(socket.get_std().is_some());
    }

    #[cfg(feature = "tokio-net")]
    #[tokio::test]
    #[serial_test::serial]
    async fn test_should_bind_udp_tokio() {
        let socket = UdpSocket::bind(
            "127.0.0.1:0"
                .parse::<SocketAddr>()
                .expect("failed to parse"),
        )
        .await
        .expect("failed to bind UDP socket");

        assert!(socket.get_tokio().is_some());
    }

    #[test]
    #[serial_test::serial]
    fn test_should_send_and_recv_from_udp_std() {
        let (_server_handle, server_addr, exit) = echo_server();
        let socket = bind_std();

        let msg = b"Hello, UDP!";
        let mut buf = [0; 1024];

        // Send a message to the server
        let sent_bytes = block_on(socket.send_to(msg, server_addr)).expect("failed to send");
        assert_eq!(sent_bytes, msg.len());

        // Receive a response from the server
        let (received_bytes, src) =
            block_on(socket.recv_from(&mut buf)).expect("failed to receive");
        assert_eq!(received_bytes, msg.len());
        assert_eq!(src, server_addr);
        assert_eq!(&buf[..received_bytes], msg);

        exit.store(true, std::sync::atomic::Ordering::Relaxed);
        // server_handle.join().expect("server thread panicked");
    }

    #[cfg(feature = "tokio-net")]
    #[tokio::test]
    #[serial_test::serial]
    async fn test_should_send_and_recv_from_udp_tokio() {
        let (_server_handle, server_addr, exit) = echo_server();
        let socket = bind_tokio().await;

        let msg = b"Hello, UDP!";
        let mut buf = [0; 1024];

        // Send a message to the server
        let sent_bytes = socket
            .send_to(msg, server_addr)
            .await
            .expect("failed to send");
        assert_eq!(sent_bytes, msg.len());

        // Receive a response from the server
        let (received_bytes, src) = socket.recv_from(&mut buf).await.expect("failed to receive");
        assert_eq!(received_bytes, msg.len());
        assert_eq!(src, server_addr);
        assert_eq!(&buf[..received_bytes], msg);

        exit.store(true, std::sync::atomic::Ordering::Relaxed);
        // server_handle.join().expect("server thread panicked");
    }

    #[test]
    fn test_should_get_options_std() {
        let socket = bind_std();

        // Set and get broadcast option
        socket.set_broadcast(true).expect("failed to set broadcast");
        let broadcast = socket.broadcast().expect("failed to get broadcast");
        assert!(broadcast);
        socket
            .set_broadcast(false)
            .expect("failed to set broadcast");
        let broadcast = socket.broadcast().expect("failed to get broadcast");
        assert!(!broadcast);

        // Set and get multicast loop option
        socket
            .set_multicast_loop_v4(true)
            .expect("failed to set multicast loop");
        let loop_v4 = socket
            .multicast_loop_v4()
            .expect("failed to get multicast loop");
        assert!(loop_v4);
        socket
            .set_multicast_loop_v4(false)
            .expect("failed to set multicast loop");
        let loop_v4 = socket
            .multicast_loop_v4()
            .expect("failed to get multicast loop");
        assert!(!loop_v4);

        // Set and get multicast TTL option
        socket
            .set_multicast_ttl_v4(1)
            .expect("failed to set multicast TTL");
        let ttl = socket
            .multicast_ttl_v4()
            .expect("failed to get multicast TTL");
        assert_eq!(ttl, 1);
        socket
            .set_multicast_ttl_v4(64)
            .expect("failed to set multicast TTL");
        let ttl = socket
            .multicast_ttl_v4()
            .expect("failed to get multicast TTL");
        assert_eq!(ttl, 64);

        // Set and get multicast loop v6 option
        // socket
        //     .set_multicast_loop_v6(true)
        //     .expect("failed to set multicast loop v6");
        // let loop_v6 = socket
        //     .multicast_loop_v6()
        //     .expect("failed to get multicast loop v6");
        // assert!(loop_v6);
        // socket
        //     .set_multicast_loop_v6(false)
        //     .expect("failed to set multicast loop v6");
        // let loop_v6 = socket
        //     .multicast_loop_v6()
        //     .expect("failed to get multicast loop v6");
        // assert!(!loop_v6);

        // Set and get TTL option
        socket.set_ttl(64).expect("failed to set TTL");
        let ttl = socket.ttl().expect("failed to get TTL");
        assert_eq!(ttl, 64);
        socket.set_ttl(128).expect("failed to set TTL");
        let ttl = socket.ttl().expect("failed to get TTL");
        assert_eq!(ttl, 128);

        // Join and leave multicast groups
        let multiaddr_v4 = Ipv4Addr::new(224, 0, 0, 1);
        let interface_v4 = Ipv4Addr::new(127, 0, 0, 1);
        socket
            .join_multicast_v4(&multiaddr_v4, &interface_v4)
            .expect("failed to join multicast v4");

        socket
            .leave_multicast_v4(&multiaddr_v4, &interface_v4)
            .expect("failed to leave multicast v4");

        // let multiaddr_v6 = Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 1);
        // let interface_v6 = 0; // Interface index for IPv6, usually 0 for default
        // socket
        //     .join_multicast_v6(&multiaddr_v6, interface_v6)
        //     .expect("failed to join multicast v6");
        // socket
        //     .leave_multicast_v6(&multiaddr_v6, interface_v6)
        //     .expect("failed to leave multicast v6");

        // Set and get SO_ERROR option
        let error = socket.take_error().expect("failed to get SO_ERROR");
        assert!(error.is_none(), "Expected no error, got: {:?}", error);
    }

    #[cfg(feature = "tokio-net")]
    #[tokio::test]
    #[serial_test::serial]
    async fn test_should_get_options_tokio() {
        let socket = bind_tokio().await;

        // Set and get broadcast option
        socket.set_broadcast(true).expect("failed to set broadcast");
        let broadcast = socket.broadcast().expect("failed to get broadcast");
        assert!(broadcast);
        socket
            .set_broadcast(false)
            .expect("failed to set broadcast");
        let broadcast = socket.broadcast().expect("failed to get broadcast");
        assert!(!broadcast);

        // Set and get multicast loop option
        socket
            .set_multicast_loop_v4(true)
            .expect("failed to set multicast loop");
        let loop_v4 = socket
            .multicast_loop_v4()
            .expect("failed to get multicast loop");
        assert!(loop_v4);
        socket
            .set_multicast_loop_v4(false)
            .expect("failed to set multicast loop");
        let loop_v4 = socket
            .multicast_loop_v4()
            .expect("failed to get multicast loop");
        assert!(!loop_v4);

        // Set and get multicast TTL option
        socket
            .set_multicast_ttl_v4(1)
            .expect("failed to set multicast TTL");
        let ttl = socket
            .multicast_ttl_v4()
            .expect("failed to get multicast TTL");
        assert_eq!(ttl, 1);
        socket
            .set_multicast_ttl_v4(64)
            .expect("failed to set multicast TTL");
        let ttl = socket
            .multicast_ttl_v4()
            .expect("failed to get multicast TTL");
        assert_eq!(ttl, 64);

        // Set and get multicast loop v6 option
        // socket
        //     .set_multicast_loop_v6(true)
        //     .expect("failed to set multicast loop v6");
        // let loop_v6 = socket
        //     .multicast_loop_v6()
        //     .expect("failed to get multicast loop v6");
        // assert!(loop_v6);
        // socket
        //     .set_multicast_loop_v6(false)
        //     .expect("failed to set multicast loop v6");
        // let loop_v6 = socket
        //     .multicast_loop_v6()
        //     .expect("failed to get multicast loop v6");
        // assert!(!loop_v6);
        // Set and get TTL option
        socket.set_ttl(64).expect("failed to set TTL");
        let ttl = socket.ttl().expect("failed to get TTL");
        assert_eq!(ttl, 64);
        socket.set_ttl(128).expect("failed to set TTL");
        let ttl = socket.ttl().expect("failed to get TTL");
        assert_eq!(ttl, 128);

        // Join and leave multicast groups
        let multiaddr_v4 = Ipv4Addr::new(224, 0, 0, 1);
        let interface_v4 = Ipv4Addr::new(127, 0, 0, 1);
        socket
            .join_multicast_v4(&multiaddr_v4, &interface_v4)
            .expect("failed to join multicast v4");

        socket
            .leave_multicast_v4(&multiaddr_v4, &interface_v4)
            .expect("failed to leave multicast v4");

        // let multiaddr_v6 = Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 1);
        // let interface_v6 = 0; // Interface index for IPv6, usually 0 for default
        // socket
        //     .join_multicast_v6(&multiaddr_v6, interface_v6)
        //     .expect("failed to join multicast v6");
        // socket
        //     .leave_multicast_v6(&multiaddr_v6, interface_v6)
        //     .expect("failed to leave multicast v6");
        // Set and get SO_ERROR option
        let error = socket.take_error().expect("failed to get SO_ERROR");
        assert!(error.is_none(), "Expected no error, got: {:?}", error);
    }

    fn bind_std() -> UdpSocket {
        block_on(UdpSocket::bind(
            "127.0.0.1:0"
                .parse::<SocketAddr>()
                .expect("failed to parse"),
        ))
        .expect("failed to bind UDP socket")
    }

    #[cfg(feature = "tokio-net")]
    async fn bind_tokio() -> UdpSocket {
        UdpSocket::bind(
            "127.0.0.1:0"
                .parse::<SocketAddr>()
                .expect("failed to parse"),
        )
        .await
        .expect("failed to bind UDP socket")
    }

    fn echo_server() -> (JoinHandle<()>, SocketAddr, Arc<AtomicBool>) {
        std::thread::sleep(std::time::Duration::from_millis(
            rand::random::<u64>() % 1000,
        ));

        let exit = Arc::new(AtomicBool::new(false));
        let exit_clone = exit.clone();

        let server = std::net::UdpSocket::bind("127.0.0.1:0").expect("failed to bind UDP server");
        server
            .set_nonblocking(true)
            .expect("failed to set non-blocking mode");
        let addr = server.local_addr().expect("failed to get local address");
        let handle = std::thread::spawn(move || {
            let mut buf = [0; 1024];
            while !exit_clone.load(std::sync::atomic::Ordering::Relaxed) {
                match server.recv_from(&mut buf) {
                    Ok((size, src)) => {
                        println!("Received {} bytes from {}", size, src);
                        if let Err(e) = server.send_to(&buf[..size], src) {
                            eprintln!("Failed to send response: {}", e);
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // No data available, continue waiting
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        continue;
                    }
                    Err(e) => eprintln!("Failed to receive data: {}", e),
                }
            }
        });
        (handle, addr, exit)
    }
}
