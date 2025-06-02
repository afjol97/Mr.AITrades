use crate::io_uring_adapter::IoUringAdapter;
use once_cell::sync::Lazy;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::sync::Mutex;

static ADAPTER: Lazy<Mutex<Option<IoUringAdapter>>> = Lazy::new(|| Mutex::new(None));

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

#[pymodule]
fn exchange_adapter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_adapter, m)?)?;
    m.add_function(wrap_pyfunction!(read_tick_py, m)?)?;
    Ok(())
}
