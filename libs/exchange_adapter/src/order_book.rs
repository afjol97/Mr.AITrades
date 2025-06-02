//! Order book management for Binance L2/L3 updates
//! Maintains in-memory order book, applies deltas, validates sequence numbers.

use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct OrderBookLevel {
    pub price: f64,
    pub qty: f64,
}

#[derive(Debug, Clone)]
pub struct OrderBook {
    pub symbol: String,
    pub bids: BTreeMap<OrderedFloat<f64>, f64>, // price -> qty
    pub asks: BTreeMap<OrderedFloat<f64>, f64>,
    pub last_update_id: u64,
}

impl OrderBook {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            last_update_id: 0,
        }
    }

    pub fn apply_snapshot(&mut self, bids: &[(f64, f64)], asks: &[(f64, f64)], update_id: u64) {
        self.bids.clear();
        self.asks.clear();
        for &(p, q) in bids {
            if q > 0.0 {
                self.bids.insert(OrderedFloat(p), q);
            }
        }
        for &(p, q) in asks {
            if q > 0.0 {
                self.asks.insert(OrderedFloat(p), q);
            }
        }
        self.last_update_id = update_id;
    }

    pub fn apply_delta(&mut self, bids: &[(f64, f64)], asks: &[(f64, f64)], update_id: u64) -> Result<(), &'static str> {
        if update_id <= self.last_update_id {
            return Err("Out-of-order or duplicate update");
        }
        for &(p, q) in bids {
            let key = OrderedFloat(p);
            if q == 0.0 {
                self.bids.remove(&key);
            } else {
                self.bids.insert(key, q);
            }
        }
        for &(p, q) in asks {
            let key = OrderedFloat(p);
            if q == 0.0 {
                self.asks.remove(&key);
            } else {
                self.asks.insert(key, q);
            }
        }
        self.last_update_id = update_id;
        Ok(())
    }
}
