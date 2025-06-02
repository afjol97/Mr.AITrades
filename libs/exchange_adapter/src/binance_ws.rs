//! Binance WebSocket ingest module for HFT adapter
//! Connects to Binance public market streams and ingests real-time tick data.

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;
use serde_json::Value;
use crate::flatbuf_decoder::{parse_agg_trade, parse_depth_update};
use crate::order_book::OrderBook;
use tracing::{info, warn};

pub struct BinanceWsIngest {
    pub url: String,
}

// Add order book state
static mut ORDER_BOOK: Option<OrderBook> = None;

impl BinanceWsIngest {
    pub async fn run(&self) -> tokio_tungstenite::tungstenite::Result<()> {
        let url = Url::parse(&self.url).expect("Invalid WebSocket URL");
        let (ws_stream, _) = connect_async(url).await?;
        let (mut write, mut read) = ws_stream.split();

        // Subscribe to aggTrade and depth
        let subscribe_msg = r#"{\"method\":\"SUBSCRIBE\",\"params\":[\"btcusdt@aggTrade\",\"btcusdt@depth@100ms\"],\"id\":1}"#;
        write.send(Message::Text(subscribe_msg.to_string())).await?;

        unsafe {
            ORDER_BOOK = Some(OrderBook::new("BTCUSDT"));
        }

        while let Some(msg) = read.next().await {
            match msg? {
                Message::Text(txt) => {
                    if let Ok(json) = serde_json::from_str::<Value>(&txt) {
                        if let Some((symbol, price, qty, ts, side)) = parse_agg_trade(&json) {
                            info!("Trade: {} {}@{} {}", symbol, qty, price, side);
                            // TODO: Convert to FlatBuffers and push to Python FFI
                        } else if let Some((symbol, update_id, bids, asks)) = parse_depth_update(&json) {
                            unsafe {
                                if let Some(ref mut ob) = ORDER_BOOK {
                                    match ob.apply_delta(&bids, &asks, update_id) {
                                        Ok(_) => info!("Order book updated: {} @ {}", symbol, update_id),
                                        Err(e) => warn!("Order book error: {}", e),
                                    }
                                }
                            }
                            // TODO: Convert to FlatBuffers and push to Python FFI
                        }
                    } else {
                        warn!("Malformed JSON: {}", txt);
                    }
                }
                Message::Binary(_) => {}
                Message::Ping(_) | Message::Pong(_) => {}
                Message::Close(_) => break,
                _ => {}
            }
        }
        Ok(())
    }
}
