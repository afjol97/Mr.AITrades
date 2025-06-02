//! Standalone Binance ingest runner for HFT adapter
//! Connects to Binance, parses messages, updates order book, pushes to Python FFI.

use exchange_adapter::binance_ws::BinanceWsIngest;
use tokio::runtime::Builder;

fn main() {
    let rt = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let ingest = BinanceWsIngest {
            url: "wss://stream.binance.com:9443/ws/btcusdt@aggTrade btcusdt@depth@100ms".to_string(),
        };
        if let Err(e) = ingest.run().await {
            eprintln!("Binance ingest error: {}", e);
        }
    });
}
