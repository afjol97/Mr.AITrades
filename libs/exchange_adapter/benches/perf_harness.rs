use criterion::{criterion_group, criterion_main, Criterion, black_box};
use exchange_adapter::io_uring_adapter::IoUringAdapter;
use std::os::unix::io::AsRawFd;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Write;

fn io_uring_perf_bench(c: &mut Criterion) {
    // Simulate a tickstream server
    let listener = TcpListener::bind("127.0.0.1:19000").unwrap();
    thread::spawn(|| {
        let mut stream = TcpStream::connect("127.0.0.1:19000").unwrap();
        for _ in 0..1000 {
            let tick = [0u8; 64];
            stream.write_all(&tick).unwrap();
        }
    });
    let (mut socket, _) = listener.accept().unwrap();
    let fd = socket.as_raw_fd();
    let mut adapter = IoUringAdapter::new(fd, 64).unwrap();
    let mut buf = [0u8; 64];
    c.bench_function("io_uring_tick_ingest", |b| {
        b.iter(|| {
            let n = adapter.poll(&mut buf).unwrap();
            black_box(n);
        })
    });
}

criterion_group!(benches, io_uring_perf_bench);
criterion_main!(benches);
