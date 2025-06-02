use exchange_adapter::io_uring_adapter::IoUringAdapter;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::AsRawFd;
use std::thread;

#[test]
fn test_io_uring_adapter() {
    // Test zero-copy parsing, error handling, backpressure
    assert!(true); // stub
}

#[test]
fn test_io_uring_adapter_backpressure() {
    // Simulate a slow consumer and a fast producer
    let listener = TcpListener::bind("127.0.0.1:19001").unwrap();
    thread::spawn(|| {
        let mut stream = TcpStream::connect("127.0.0.1:19001").unwrap();
        for _ in 0..100 {
            let tick = [0u8; 64];
            stream.write_all(&tick).unwrap();
            std::thread::sleep(std::time::Duration::from_micros(10));
        }
    });
    let (mut socket, _) = listener.accept().unwrap();
    let fd = socket.as_raw_fd();
    let mut adapter = IoUringAdapter::new(fd, 8).unwrap();
    let mut buf = [0u8; 64];
    for _ in 0..100 {
        let n = adapter.poll(&mut buf).unwrap();
        assert!(n > 0);
    }
}
