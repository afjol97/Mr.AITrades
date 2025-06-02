use exchange_adapter::flatbuf_decoder::{parse_agg_trade, parse_depth_update};
use serde_json::json;

#[test]
fn test_parse_agg_trade() {
    let msg = json!({
        "e": "aggTrade", "s": "BTCUSDT", "p": "43000.1", "q": "0.002", "T": 1620000000000i64, "m": true
    });
    let parsed = parse_agg_trade(&msg).unwrap();
    assert_eq!(parsed.0, "BTCUSDT");
    assert_eq!(parsed.1, 43000.1);
    assert_eq!(parsed.2, 0.002);
    assert_eq!(parsed.3, 1620000000000);
    assert_eq!(parsed.4, "sell");
}

#[test]
fn test_parse_depth_update() {
    let msg = json!({
        "e": "depthUpdate", "s": "BTCUSDT", "u": 12345,
        "b": [["43000.1", "0.5"], ["42999.9", "1.0"]],
        "a": [["43001.0", "0.3"]]
    });
    let parsed = parse_depth_update(&msg).unwrap();
    assert_eq!(parsed.0, "BTCUSDT");
    assert_eq!(parsed.1, 12345);
    assert_eq!(parsed.2, vec![(43000.1, 0.5), (42999.9, 1.0)]);
    assert_eq!(parsed.3, vec![(43001.0, 0.3)]);
}
