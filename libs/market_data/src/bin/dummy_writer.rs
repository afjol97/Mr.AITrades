use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;

use flatbuffers::{FlatBufferBuilder, WIPOffset};
mod market_data;
use market_data::market_data::{Quote, QuoteArgs};

fn main() {
    let path = "/dev/shm/feed";
    let mut price = 68000.0f32;
    let mut size = 0.1f32;
    let mut symbol_toggle = true;
    loop {
        let mut builder = FlatBufferBuilder::new();
        let symbol = if symbol_toggle { builder.create_string("BTCUSDT") } else { builder.create_string("ETHUSDT") };
        let quote = Quote::create(
            &mut builder,
            &QuoteArgs {
                symbol: Some(symbol),
                bid: price,
                ask: price + 2.0,
                timestamp: chrono::Utc::now().timestamp() as u64,
            },
        );
        builder.finish(quote, None);
        let data = builder.finished_data();
        let mut file = OpenOptions::new().read(true).write(true).create(true).open(path).unwrap();
        file.set_len(128).unwrap();
        file.write_all(data).unwrap();
        println!("Wrote quote: symbol={}, bid={}, ask={}", if symbol_toggle {"BTCUSDT"} else {"ETHUSDT"}, price, price+2.0);
        price += 10.0;
        size += 0.01;
        symbol_toggle = !symbol_toggle;
        thread::sleep(Duration::from_secs(1));
    }
}
