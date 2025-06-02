use io_uring::{IoUring, opcode, types};
use std::os::unix::io::RawFd;
use std::io::{self, Read};
use std::net::TcpStream;
use anyhow::Result;
use libc::{c_void, MSG_DONTWAIT};

pub struct IoUringAdapter {
    ring: IoUring,
    fd: RawFd,
}

impl IoUringAdapter {
    pub fn new(fd: RawFd, queue_depth: u32) -> Result<Self> {
        let ring = IoUring::new(queue_depth)?;
        Ok(Self { ring, fd })
    }

    /// Submits a read to io_uring and waits for completion. Returns bytes read.
    pub fn poll(&mut self, buf: &mut [u8]) -> Result<usize> {
        use std::ptr::null_mut;
        let read_e = opcode::Read::new(
            types::Fd(self.fd),
            buf.as_mut_ptr(),
            buf.len() as u32,
        )
        .build();
        unsafe {
            self.ring.submission().push(&read_e).expect("submission queue full");
        }
        self.ring.submit_and_wait(1)?;
        let cqe = self.ring.completion().next().expect("no completion event");
        let res = cqe.result();
        if res < 0 {
            return Err(anyhow::anyhow!(io::Error::from_raw_os_error(-res)));
        }
        Ok(res as usize)
    }

    /// Utility: set TCP_NODELAY and SO_TIMESTAMPING on a socket
    pub fn configure_socket(stream: &TcpStream) -> io::Result<()> {
        use std::os::unix::io::AsRawFd;
        let fd = stream.as_raw_fd();
        // TCP_NODELAY
        let flag: libc::c_int = 1;
        unsafe {
            libc::setsockopt(
                fd,
                libc::IPPROTO_TCP,
                libc::TCP_NODELAY,
                &flag as *const _ as *const c_void,
                std::mem::size_of_val(&flag) as libc::socklen_t,
            );
            // SO_TIMESTAMPING (best effort)
            let ts_flag: libc::c_int = 0x1F; // SOF_TIMESTAMPING_TX_SOFTWARE | ...
            libc::setsockopt(
                fd,
                libc::SOL_SOCKET,
                37, // SO_TIMESTAMPING
                &ts_flag as *const _ as *const c_void,
                std::mem::size_of_val(&ts_flag) as libc::socklen_t,
            );
        }
        Ok(())
    }
}
