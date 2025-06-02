#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_real_engine() {
        // This test will fail unless a FIX server is running at 127.0.0.1:9876
        // let mut engine = FixOrderManager::new(
        //     "CLIENT", 
        //     "EXCHANGE",
        //     "127.0.0.1",
        //     9876
        // ).await.unwrap();
        // assert!(engine.submit_order("BTC/USD", 'B', 1.0, 50000.0).await.is_ok());
    }

    #[test]
    fn test_mock_engine() {
        let engine = MockFixEngine::new(true);
        assert!(engine.submit_order("BTC/USD", 'B', 1.0, 50000.0).is_ok());
    }
}
