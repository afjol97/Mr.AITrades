pub mod market_data; // generated from FBS
pub use market_data::market_data::{Quote, QuoteArgs};

use flatbuffers::root;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::types::PyDict;
use shared_mem::SharedTensor;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::os::unix::prelude::AsRawFd;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tungstenite::connect;
use url::Url;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::connect_async;
use futures_util::StreamExt;
use serde_json::Value;
use anyhow::Result;

#[pyfunction]
fn greet(name: &str) -> PyResult<String> {
    Ok(format!("Welcome to HFT Bot, {name}!"))
}

#[pyfunction]
fn parse_quote(buf: &[u8]) -> PyResult<String> {
    let quote = root::<Quote>(buf).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    Ok(format!(
        "Symbol: {}, Bid: {}, Ask: {}, Timestamp: {}",
        quote.symbol().unwrap_or("N/A"),
        quote.bid(),
        quote.ask(),
        quote.timestamp(),
    ))
}

fn parse_quote_dict<'p>(py: Python<'p>, buf: &[u8]) -> PyResult<&'p pyo3::types::PyDict> {
    let quote = flatbuffers::root::<market_data::market_data::Quote>(buf)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    let dict = pyo3::types::PyDict::new(py);
    dict.set_item("symbol", quote.symbol().unwrap_or("N/A"))?;
    dict.set_item("bid", quote.bid())?;
    dict.set_item("ask", quote.ask())?;
    dict.set_item("timestamp", quote.timestamp())?;
    Ok(dict)
}

#[pyfunction]
fn read_shared_quote(py: Python, path: &str, size: usize) -> PyResult<PyObject> {
    let tensor = SharedTensor::create(path, size);
    let buf: Vec<u8> = unsafe {
        let ptr = tensor.mmap.as_ptr();
        std::slice::from_raw_parts(ptr, size).to_vec()
    };
    Ok(parse_quote_dict(py, &buf)?.into())
}

#[pyfunction]
fn start_feed_loop(path: &str, size: usize, py_callback: PyObject) -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    let path = path.to_string();
    let cb = Arc::new(py_callback);

    thread::spawn(move || {
        let mut buf = vec![0u8; size];
        loop {
            match File::open(&path) {
                Ok(mut f) => {
                    if let Err(e) = f.read_exact(&mut buf) {
                        eprintln!("Read error: {:?}", e);
                        continue;
                    }

                    Python::with_gil(|py| {
                        if let Ok(dict) = parse_quote_dict(py, &buf) {
                            let _ = cb.call1(py, (dict,));
                        }
                    });
                }
                Err(e) => eprintln!("Failed to open shared memory: {:?}", e),
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    Ok(())
}

pub async fn run_binance_feed() -> Result<()> {
    let ws_url = "wss://stream.binance.com:9443/stream?streams=btcusdt@ticker/ethusdt@ticker";
    let shm_path = "/dev/shm/feed";
    let (ws_stream, _) = connect_async(Url::parse(ws_url)?).await?;
    let (_, mut reader) = ws_stream.split();

    while let Some(msg) = reader.next().await {
        let msg = msg?;
        if msg.is_text() {
            let txt = msg.to_text()?;
            if let Ok(json) = serde_json::from_str::<Value>(txt) {
                let symbol = json["s"].as_str().unwrap_or("");
                let bid = json["b"].as_str().unwrap_or("0").parse::<f32>().unwrap_or(0.0);
                let ask = json["a"].as_str().unwrap_or("0").parse::<f32>().unwrap_or(0.0);
                let timestamp = json["E"].as_u64().unwrap_or(0);

                let mut builder = flatbuffers::FlatBufferBuilder::new();
                let symbol_fb = builder.create_string(symbol);
                let quote = Quote::create(
                    &mut builder,
                    &QuoteArgs { symbol: Some(symbol_fb), bid, ask, timestamp },
                );
                builder.finish(quote, None);
                let data = builder.finished_data();

                let mut file = OpenOptions::new().write(true).create(true).open(shm_path).await?;
                file.set_len(128).await?;
                file.write_all(data).await?;
            }
        }
    }
    Ok(())
}

#[pymodule]
fn hft_bot(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    m.add_function(wrap_pyfunction!(parse_quote, m)?)?;
    m.add_function(wrap_pyfunction!(read_shared_quote, m)?)?;
    m.add_function(wrap_pyfunction!(start_feed_loop, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
