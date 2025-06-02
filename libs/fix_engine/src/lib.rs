#[derive(Debug, Clone)]
pub struct FixEngine {
    pub verbose: bool,
    pub sequence_number: u32,
}

impl FixEngine {
    /// Creates a new mock FIX engine
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            sequence_number: 1,
        }
    }

    /// Mock order submission (always succeeds)
    pub fn submit_order(
        &mut self,
        symbol: &str,
        side: char,  // 'B' = Buy, 'S' = Sell
        quantity: f64,
        price: f64,
    ) -> Result<u32, &'static str> {  // Returns ClOrdID
        let cl_ord_id = self.sequence_number;
        self.sequence_number += 1;

        if self.verbose {
            println!(
                "[FIX MOCK] NewOrderSingle: ClOrdID={} Symbol={} Side={} Qty={} Price={}",
                cl_ord_id, symbol, side, quantity, price
            );
        }

        Ok(cl_ord_id)
    }

    /// Mock cancellation
    pub fn cancel_order(&mut self, cl_ord_id: u32) -> Result<(), &'static str> {
        if self.verbose {
            println!("[FIX MOCK] OrderCancelRequest: ClOrdID={}", cl_ord_id);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_flow() {
        let mut engine = FixEngine::new(false);
        let order_id = engine.submit_order("BTC/USD", 'B', 1.0, 50000.0).unwrap();
        assert!(engine.cancel_order(order_id).is_ok());
    }
}
