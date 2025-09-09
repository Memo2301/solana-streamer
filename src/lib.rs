pub mod common;
pub mod protos;
pub mod streaming;

// Re-export trade detection types for easy access
pub use streaming::event_parser::protocols::raydium_cpmm::types::{TradeDirection, TradeInfo, CopyTradeableEvent};
pub use streaming::event_parser::common::types::SignatureExt;
