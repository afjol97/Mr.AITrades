pub struct MarketDataAdapter;

impl MarketDataAdapter {
    pub fn new(_fd: i32) -> Self {
        MarketDataAdapter
    }
    pub fn poll(&mut self) -> Order {
        Order {}
    }
}

pub struct Order;
impl Order {
    pub fn features(&self) -> [f32; 64] {
        [0.0; 64]
    }
    pub fn price(&self) -> f64 {
        0.0
    }
}