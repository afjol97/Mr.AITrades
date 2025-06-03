pub struct MarketDataAdapter {
    fd: i32,
}

impl MarketDataAdapter {
    pub fn new(fd: i32) -> Self {
        Self { fd }
    }
    pub fn poll(&mut self) -> Order {
        Order { px: 100.0 }
    }
}

pub struct Order {
    pub px: f64,
}

impl Order {
    pub fn features(&self) -> [f32; 64] {
        [0.0; 64]
    }
    pub fn price(&self) -> f64 {
        self.px
    }
}
