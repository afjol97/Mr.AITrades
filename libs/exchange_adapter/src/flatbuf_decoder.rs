// This file is generated. Run flatc --rust tick.fbs to regenerate.
pub mod market {
    #![allow(dead_code)]
    include!("../../market_data/src/mod.rs"); // Or use flatc output path
}

use crate::tick_generated::market::Tick;
use flatbuffers::root;

pub fn decode_tick(buf: &[u8]) -> Option<Tick> {
    root::<Tick>(buf).ok()
}
