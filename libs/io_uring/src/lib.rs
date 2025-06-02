use flatbuffers::root;
use io_uring::{opcode, types, IoUring};
use order_generated::market::Order;
use std::os::unix::io::RawFd;

pub mod order_generated;

pub struct DataFeed {
    ring: IoUring,
    fd: RawFd,
}

impl DataFeed {
    pub fn new(fd: RawFd) -> Self {
        let ring = IoUring::new(4096).unwrap();
        Self { ring, fd }
    }

    pub fn poll(&mut self, buf: &mut [u8]) -> usize {
        let sqe = opcode::Recv::new(types::Fd(self.fd), buf.as_mut_ptr(), buf.len() as _).build();
        unsafe { self.ring.submission().push(&sqe).unwrap(); }
        self.ring.submit_and_wait(1).unwrap();
        let cqe = self.ring.completion().next().unwrap();
        cqe.result() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::net::{TcpListener, TcpStream};
    use std::os::unix::io::AsRawFd;
    use std::thread;

    fn parse_flatbuffer(buf: &[u8]) -> Option<crate::order_generated::market::Order> {
        flatbuffers::root::<crate::order_generated::market::Order>(buf).ok()
    }

    #[test]
    fn test_datafeed_poll() {
        let listener = TcpListener::bind("127.0.0.1:18080").unwrap();
        thread::spawn(|| {
            let mut stream = TcpStream::connect("127.0.0.1:18080").unwrap();
            stream.write_all(b"mock market data").unwrap();
        });
        let (mut socket, _) = listener.accept().unwrap();
        let fd = socket.as_raw_fd();
        let mut feed = DataFeed::new(fd);
        let mut buf = [0u8; 64];
        let n = feed.poll(&mut buf);
        assert!(n > 0);
        assert_eq!(&buf[..n], b"mock market data");
    }

    #[test]
    fn test_flatbuffer_parse() {
        use flatbuffers::FlatBufferBuilder;
        use crate::order_generated::market::OrderArgs;
        let mut fbb = FlatBufferBuilder::new();
        let id = fbb.create_string("ORD1");
        let symbol = fbb.create_string("BTCUSD");
        let side = fbb.create_string("buy");
        let features = fbb.create_vector(&[1.0f32, 2.0, 3.0]);
        let order = crate::order_generated::market::Order::create(&mut fbb, &OrderArgs {
            id: Some(id),
            symbol: Some(symbol),
            price: 50000.0,
            qty: 0.5,
            side: Some(side),
            features: Some(features),
        });
        fbb.finish(order, None);
        let buf = fbb.finished_data();
        let parsed = parse_flatbuffer(buf).unwrap();
        assert_eq!(parsed.symbol().unwrap(), "BTCUSD");
        assert_eq!(parsed.price(), 50000.0);
        assert_eq!(parsed.qty(), 0.5);
        assert_eq!(parsed.side().unwrap(), "buy");
        let feats = parsed.features().unwrap();
        assert_eq!(feats.len(), 3);
        assert_eq!(feats.get(0), 1.0);
    }
}
