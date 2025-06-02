use crate::ffi_ring::FfiRingBuffer;
use crate::io_uring_adapter::IoUringAdapter;
use once_cell::sync::Lazy;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::sync::Arc;
use std::sync::Mutex;

static ADAPTER: Lazy<Mutex<Option<IoUringAdapter>>> = Lazy::new(|| Mutex::new(None));
static RING: Lazy<FfiRingBuffer> = Lazy::new(|| FfiRingBuffer::new(65536));

#[pyfunction]
pub fn init_adapter(fd: i32, queue_depth: u32) -> PyResult<()> {
    let adapter = IoUringAdapter::new(fd, queue_depth)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    *ADAPTER.lock().unwrap() = Some(adapter);
    Ok(())
}

#[pyfunction]
pub fn read_tick_py(py: Python) -> PyResult<PyObject> {
    let mut buf = vec![0u8; 128];
    let mut guard = ADAPTER.lock().unwrap();
    if let Some(adapter) = guard.as_mut() {
        let n = adapter
            .poll(&mut buf)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(PyBytes::new(py, &buf[..n]).into())
    } else {
        Err(pyo3::exceptions::PyRuntimeError::new_err("Adapter not initialized"))
    }
}

#[pyfunction]
fn gil_release_example(py: Python) -> PyResult<()> {
    py.allow_threads(|| {
        // Place CPU-intensive or blocking Rust code here
        // ...
    });
    Ok(())
}

#[pyfunction]
pub fn ffi_ring_push(py: Python, msg: &[u8]) -> PyResult<bool> {
    py.allow_threads(|| {
        RING.push(msg.to_vec()).is_ok()
    });
    Ok(true)
}

#[pyfunction]
pub fn ffi_ring_pop(py: Python) -> PyResult<Option<PyObject>> {
    let msg = py.allow_threads(|| RING.pop());
    match msg {
        Some(buf) => Ok(Some(PyBytes::new(py, &buf).into())),
        None => Ok(None),
    }
}

#[pyfunction]
pub fn ffi_ring_len() -> usize {
    RING.arc().len()
}

#[pymodule]
fn exchange_adapter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_adapter, m)?)?;
    m.add_function(wrap_pyfunction!(read_tick_py, m)?)?;
    m.add_function(wrap_pyfunction!(gil_release_example, m)?)?;
    m.add_function(wrap_pyfunction!(ffi_ring_push, m)?)?;
    m.add_function(wrap_pyfunction!(ffi_ring_pop, m)?)?;
    m.add_function(wrap_pyfunction!(ffi_ring_len, m)?)?;
    Ok(())
}
