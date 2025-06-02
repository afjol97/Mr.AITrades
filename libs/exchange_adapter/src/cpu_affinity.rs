// CPU pinning and IRQ affinity utilities for Linux
// Uses libc and nix for thread affinity

use nix::sched::{sched_setaffinity, CpuSet};
use nix::unistd::Pid;
use std::io;

/// Pin the current thread to a specific CPU core
pub fn pin_thread_to_cpu(cpu: usize) -> io::Result<()> {
    let mut cpuset = CpuSet::new();
    cpuset.set(cpu).map_err(|_| io::Error::new(io::ErrorKind::Other, "Invalid CPU index"))?;
    sched_setaffinity(Pid::this(), &cpuset)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Affinity error: {}", e)))
}
