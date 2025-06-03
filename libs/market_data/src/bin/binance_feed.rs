use hft_bot::run_binance_feed;

#[tokio::main]
async fn main() {
    if let Err(e) = run_binance_feed().await {
        eprintln!("Feed error: {:?}", e);
    }
}
