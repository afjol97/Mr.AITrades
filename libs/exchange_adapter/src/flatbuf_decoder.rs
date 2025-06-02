// This file is generated. Run flatc --rust tick.fbs to regenerate.
pub mod market {
    #![allow(dead_code)]
    include!("../../market_data/src/mod.rs"); // Or use flatc output path
}

use crate::tick_generated::market::Tick;
use flatbuffers::root;
use serde_json::Value;
use crate::order_book::{OrderBook, OrderBookLevel};

pub fn decode_tick(buf: &[u8]) -> Option<Tick> {
    root::<Tick>(buf).ok()
}

pub fn parse_agg_trade(json: &Value) -> Option<(String, f64, f64, u64, String)> {
    // symbol, price, qty, ts, side
    let symbol = json.get("s")?.as_str()?.to_string();
    let price = json.get("p")?.as_str()?.parse().ok()?;
    let qty = json.get("q")?.as_str()?.parse().ok()?;
    let ts = json.get("T")?.as_u64()?;
    let side = if json.get("m")?.as_bool()? { "sell" } else { "buy" }.to_string();
    Some((symbol, price, qty, ts, side))
}

pub fn parse_depth_update(json: &Value) -> Option<(String, u64, Vec<(f64, f64)>, Vec<(f64, f64)>)> {
    let symbol = json.get("s")?.as_str()?.to_string();
    let update_id = json.get("u")?.as_u64()?;
    let bids = json.get("b")?.as_array()?.iter().filter_map(|x| {
        let arr = x.as_array()?;
        let p = arr.get(0)?.as_str()?.parse().ok()?;
        let q = arr.get(1)?.as_str()?.parse().ok()?;
        Some((p, q))
    }).collect();
    let asks = json.get("a")?.as_array()?.iter().filter_map(|x| {
        let arr = x.as_array()?;
        let p = arr.get(0)?.as_str()?.parse().ok()?;
        let q = arr.get(1)?.as_str()?.parse().ok()?;
        Some((p, q))
    }).collect();
    Some((symbol, update_id, bids, asks))
}
