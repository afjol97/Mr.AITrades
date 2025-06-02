use std::net::TcpStream;
use std::os::unix::io::AsRawFd;

pub fn connect_socket(addr: &str) -> std::io::Result<TcpStream> {
    let stream = TcpStream::connect(addr)?;
    // Set TCP_NODELAY, SO_TIMESTAMPING, etc.
    Ok(stream)
}
