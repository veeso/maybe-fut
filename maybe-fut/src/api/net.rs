//! Networking primitives for TCP/UDP communication.
//!
//! This module provides networking functionalities for the Transmission Control and User Datagram Protocols.
//!
//! References:
//!
//! - [Standard Library Networking](https://doc.rust-lang.org/std/net/index.html)
//! - [Tokio Networking](https://docs.rs/tokio/latest/tokio/net/index.html)

mod tcp_listener;
mod tcp_stream;
mod udp_socket;

pub use self::tcp_listener::TcpListener;
pub use self::tcp_stream::TcpStream;
pub use self::udp_socket::UdpSocket;
