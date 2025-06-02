//! Lock-free ring buffer for Rust→Python FFI, using crossbeam for SPSC/MPMC
//! Designed for zero-copy FlatBuffer message passing

use crossbeam_queue::ArrayQueue;
use std::sync::Arc;

pub struct FfiRingBuffer {
    queue: Arc<ArrayQueue<Vec<u8>>>,
}

impl FfiRingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self { queue: Arc::new(ArrayQueue::new(capacity)) }
    }
    pub fn push(&self, msg: Vec<u8>) -> Result<(), Vec<u8>> {
        self.queue.push(msg)
    }
    pub fn pop(&self) -> Option<Vec<u8>> {
        self.queue.pop().ok()
    }
    pub fn arc(&self) -> Arc<ArrayQueue<Vec<u8>>> {
        Arc::clone(&self.queue)
    }
}
