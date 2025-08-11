use serde::{Deserialize, Serialize};

use super::platform::PlatformData;
use super::metadata::EnrichedMetadata;
use super::validation::ValidationData;

/// The main enhanced trade event that contains all necessary information
/// for trading decisions without requiring re-parsing
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnrichedTradeEvent {
    /// Core trading data that every trade needs
    pub core: CoreTradeData,
    
    /// Platform-specific data (bonding curves, pools, etc.)
    pub platform: PlatformData,
    
    /// Enhanced timing and metadata
    pub metadata: EnrichedMetadata,
    
    /// Original event data for edge cases
    pub original: OriginalEventWrapper,
    
    /// Validation results
    pub validation: ValidationData,
}

/// Core trade data that's consistent across all platforms
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoreTradeData {
    /// Trader's wallet address
    pub trader_wallet: String,
    
    /// Token mint address
    pub token_mint: String,
    
    /// Buy or Sell
    pub trade_direction: TradeDirection,
    
    /// Pre-calculated amounts with proper decimal adjustments
    pub amounts: TradeAmounts,
    
    /// Pre-calculated price information
    pub price: TradePrice,
}

/// Simple trade direction enum
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TradeDirection {
    Buy,
    Sell,
}

/// All amount-related data with proper decimal conversions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeAmounts {
    /// SOL amount (normalized to 9 decimals)
    pub sol_amount: f64,
    
    /// Token amount (normalized using actual token decimals)
    pub token_amount: f64,
    
    /// Raw SOL amount (original lamports)
    pub raw_sol_amount: u64,
    
    /// Raw token amount (original units)
    pub raw_token_amount: u64,
    
    /// Token decimals used for conversion
    pub token_decimals: u8,
    
    /// USD value if SOL price is available
    pub usd_value: Option<f64>,
}

/// Comprehensive price information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradePrice {
    /// Primary price: SOL per 1 token
    pub sol_per_token: f64,
    
    /// Inverse price: tokens per 1 SOL
    pub tokens_per_sol: f64,
    
    /// Market cap change (if calculable)
    pub market_cap_change: Option<f64>,
    
    /// Price impact percentage
    pub price_impact: Option<f64>,
    
    /// Whether this price seems reasonable (validation)
    pub is_price_valid: bool,
}

/// Wrapper for original event data to maintain backward compatibility
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OriginalEventWrapper {
    PumpFun(serde_json::Value),  // Using JSON to avoid import complexity
    Bonk(serde_json::Value),
    PumpSwapBuy(serde_json::Value),
    PumpSwapSell(serde_json::Value),
    RaydiumCpmm(serde_json::Value),
    RaydiumClmm(serde_json::Value),
    RaydiumClmmV2(serde_json::Value),
}

impl OriginalEventWrapper {
    pub fn event_type(&self) -> &'static str {
        match self {
            OriginalEventWrapper::PumpFun(_) => "PumpFun",
            OriginalEventWrapper::Bonk(_) => "Bonk",
            OriginalEventWrapper::PumpSwapBuy(_) => "PumpSwapBuy",
            OriginalEventWrapper::PumpSwapSell(_) => "PumpSwapSell",
            OriginalEventWrapper::RaydiumCpmm(_) => "RaydiumCpmm",
            OriginalEventWrapper::RaydiumClmm(_) => "RaydiumClmm",
            OriginalEventWrapper::RaydiumClmmV2(_) => "RaydiumClmmV2",
        }
    }
}

impl EnrichedTradeEvent {
    /// Create a new enriched trade event
    pub fn new(
        core: CoreTradeData,
        platform: PlatformData,
        metadata: EnrichedMetadata,
        original: OriginalEventWrapper,
        validation: ValidationData,
    ) -> Self {
        Self {
            core,
            platform,
            metadata,
            original,
            validation,
        }
    }
    
    /// Get a summary of this trade for logging
    pub fn summary(&self) -> String {
        format!(
            "{} {} {} SOL for {} {} at {:.8} SOL/token on {}",
            self.core.trade_direction.as_str(),
            self.core.amounts.sol_amount,
            if matches!(self.core.trade_direction, TradeDirection::Buy) { "→" } else { "←" },
            self.core.amounts.token_amount,
            self.core.token_mint,
            self.core.price.sol_per_token,
            self.platform.platform_name()
        )
    }
}

impl TradeDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            TradeDirection::Buy => "BUY",
            TradeDirection::Sell => "SELL",
        }
    }
}
