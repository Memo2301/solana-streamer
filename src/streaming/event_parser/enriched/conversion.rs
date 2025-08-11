use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json;

use crate::streaming::event_parser::{
    protocols::{
        pumpfun::PumpFunTradeEvent,
        bonk::BonkTradeEvent,
    },
};

use super::{
    EnrichedTradeEvent, CoreTradeData, TradeDirection, TradeAmounts, TradePrice,
    PlatformData, PumpFunData, BonkData,
    FeeData, FeeComponent, OriginalEventWrapper,
    EnrichedMetadata, TimingData, NetworkData, ProcessingData, ProcessingPerformance,
    ValidationData, ValidationError, ValidationWarning,
};

/// Context needed for event conversion
#[derive(Clone)]
pub struct ConversionContext {
    /// Cache of token decimals to avoid repeated lookups
    pub token_decimals_cache: Arc<RwLock<HashMap<String, u8>>>,
    
    /// Current SOL price in USD (if available)
    pub sol_price_usd: Option<f64>,
    
    /// Default token decimals to use when lookup fails
    pub default_token_decimals: u8,
}

/// Errors that can occur during conversion
#[derive(Debug, Clone)]
pub enum ConversionError {
    MissingTokenDecimals(String),
    InvalidAmounts { sol: u64, token: u64 },
    PriceCalculationFailed(String),
    PlatformDataMissing(String),
    MetadataIncomplete,
    SerializationError(String),
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionError::MissingTokenDecimals(mint) => 
                write!(f, "Token decimals not found for mint: {}", mint),
            ConversionError::InvalidAmounts { sol, token } => 
                write!(f, "Invalid amounts: SOL={}, Token={}", sol, token),
            ConversionError::PriceCalculationFailed(reason) => 
                write!(f, "Price calculation failed: {}", reason),
            ConversionError::PlatformDataMissing(field) => 
                write!(f, "Platform data missing: {}", field),
            ConversionError::MetadataIncomplete => 
                write!(f, "Metadata incomplete"),
            ConversionError::SerializationError(msg) => 
                write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for ConversionError {}

/// Trait for converting events to enriched format
pub trait ToEnrichedEvent {
    fn to_enriched(&self, context: &ConversionContext) -> impl std::future::Future<Output = Result<EnrichedTradeEvent, ConversionError>> + Send;
}

impl ConversionContext {
    pub fn new() -> Self {
        Self {
            token_decimals_cache: Arc::new(RwLock::new(HashMap::new())),
            sol_price_usd: None,
            default_token_decimals: 6, // Most tokens use 6 decimals
        }
    }
    
    pub fn with_sol_price(mut self, price: f64) -> Self {
        self.sol_price_usd = Some(price);
        self
    }
    
    pub fn with_default_decimals(mut self, decimals: u8) -> Self {
        self.default_token_decimals = decimals;
        self
    }
    
    /// Get token decimals, using cache or default
    pub async fn get_token_decimals(&self, mint: &str) -> u8 {
        let cache = self.token_decimals_cache.read().await;
        cache.get(mint).copied().unwrap_or(self.default_token_decimals)
    }
    
    /// Add token decimals to cache
    pub async fn cache_token_decimals(&self, mint: String, decimals: u8) {
        let mut cache = self.token_decimals_cache.write().await;
        cache.insert(mint, decimals);
    }
}

impl ToEnrichedEvent for PumpFunTradeEvent {
    async fn to_enriched(&self, context: &ConversionContext) -> Result<EnrichedTradeEvent, ConversionError> {
        let mut performance = ProcessingPerformance::default();
        
        // Parse timing
        let parse_start = std::time::Instant::now();
        let timing = TimingData::new(
            self.metadata.slot,
            self.metadata.block_time_ms.max(0) as u64,
            self.metadata.program_received_time_ms.max(0) as u64,
        );
        performance.record_parse_time(parse_start);
        
        // Enrich event
        let enrich_start = std::time::Instant::now();
        
        // Get token decimals
        let mint_str = self.mint.to_string();
        let token_decimals = context.get_token_decimals(&mint_str).await;
        
        // Calculate amounts
        let amounts = calculate_amounts(
            self.sol_amount,
            self.token_amount,
            token_decimals,
            self.is_buy,
            context.sol_price_usd,
        )?;
        
        // Calculate price
        let price = calculate_price(&amounts, self.is_buy)?;
        
        // Build core trade data
        let core = CoreTradeData {
            trader_wallet: self.user.to_string(),
            token_mint: mint_str.clone(),
            trade_direction: if self.is_buy { TradeDirection::Buy } else { TradeDirection::Sell },
            amounts,
            price,
        };
        
        // Build platform data
        let mut fees = FeeData::new(self.fee, calculate_fee_percentage(self.fee, self.sol_amount));
        if self.creator_fee > 0 {
            fees = fees.with_creator_fee(self.creator_fee);
            fees.add_fee_component(FeeComponent::new(
                "Creator Fee".to_string(),
                self.creator_fee,
                calculate_fee_percentage(self.creator_fee, self.sol_amount),
            ).with_recipient(self.creator.to_string()));
        }
        
        let market_cap = calculate_pumpfun_market_cap(
            self.virtual_sol_reserves,
            self.virtual_token_reserves,
        );
        
        let platform = PlatformData::PumpFun(PumpFunData {
            bonding_curve: self.bonding_curve.to_string(),
            creator: self.creator.to_string(),
            virtual_sol_reserves: self.virtual_sol_reserves,
            virtual_token_reserves: self.virtual_token_reserves,
            real_sol_reserves: self.real_sol_reserves,
            real_token_reserves: self.real_token_reserves,
            fees,
            market_cap,
            graduation_progress: calculate_graduation_progress(self.real_sol_reserves),
        });
        
        performance.record_enrichment_time(enrich_start);
        
        // Validate
        let validate_start = std::time::Instant::now();
        let validation = validate_trade_event(&core, &platform, &timing)?;
        performance.record_validation_time(validate_start);
        
        // Build metadata
        let network = NetworkData::new(
            self.metadata.signature.to_string(),
            "PumpFun".to_string(),
        );
        
        let mut processing = ProcessingData::new();
        performance.calculate_total();
        processing.set_performance(performance);
        
        let metadata = EnrichedMetadata {
            timing,
            network,
            processing,
        };
        
        // Serialize original event
        let original = OriginalEventWrapper::PumpFun(
            serde_json::to_value(self).map_err(|e| ConversionError::SerializationError(e.to_string()))?
        );
        
        Ok(EnrichedTradeEvent::new(core, platform, metadata, original, validation))
    }
}

impl ToEnrichedEvent for BonkTradeEvent {
    async fn to_enriched(&self, context: &ConversionContext) -> Result<EnrichedTradeEvent, ConversionError> {
        let timing = TimingData::new(
            self.metadata.slot,
            self.metadata.block_time_ms.max(0) as u64,
            self.metadata.program_received_time_ms.max(0) as u64,
        );
        
        let mint_str = self.base_token_mint.to_string();
        let token_decimals = context.get_token_decimals(&mint_str).await;
        
        let amounts = calculate_amounts(
            self.amount_in,
            self.amount_out,
            token_decimals,
            matches!(self.trade_direction, crate::streaming::event_parser::protocols::bonk::TradeDirection::Buy),
            context.sol_price_usd,
        )?;
        
        let price = calculate_price(&amounts, matches!(self.trade_direction, crate::streaming::event_parser::protocols::bonk::TradeDirection::Buy))?;
        
        let core = CoreTradeData {
            trader_wallet: self.payer.to_string(),
            token_mint: mint_str.clone(),
            trade_direction: match self.trade_direction {
                crate::streaming::event_parser::protocols::bonk::TradeDirection::Buy => TradeDirection::Buy,
                crate::streaming::event_parser::protocols::bonk::TradeDirection::Sell => TradeDirection::Sell,
            },
            amounts,
            price,
        };
        
        let fees = FeeData::new(0, 0.0); // Bonk might not have explicit fees in the event
        let platform = PlatformData::Bonk(BonkData {
            pool: None, // Pool info might not be directly available
            fees,
            pool_reserves: None,
        });
        
        let validation = validate_trade_event(&core, &platform, &timing)?;
        
        let network = NetworkData::new(
            self.metadata.signature.to_string(),
            "Bonk".to_string(),
        );
        
        let processing = ProcessingData::new();
        let metadata = EnrichedMetadata {
            timing,
            network,
            processing,
        };
        
        let original = OriginalEventWrapper::Bonk(
            serde_json::to_value(self).map_err(|e| ConversionError::SerializationError(e.to_string()))?
        );
        
        Ok(EnrichedTradeEvent::new(core, platform, metadata, original, validation))
    }
}

// Helper functions
fn calculate_amounts(
    sol_amount: u64,
    token_amount: u64,
    token_decimals: u8,
    _is_buy: bool,
    sol_price_usd: Option<f64>,
) -> Result<TradeAmounts, ConversionError> {
    if sol_amount == 0 && token_amount == 0 {
        return Err(ConversionError::InvalidAmounts { sol: sol_amount, token: token_amount });
    }
    
    let sol_amount_f64 = sol_amount as f64 / 1_000_000_000.0; // 9 decimals
    let token_amount_f64 = token_amount as f64 / 10_f64.powi(token_decimals as i32);
    
    let usd_value = sol_price_usd.map(|price| sol_amount_f64 * price);
    
    Ok(TradeAmounts {
        sol_amount: sol_amount_f64,
        token_amount: token_amount_f64,
        raw_sol_amount: sol_amount,
        raw_token_amount: token_amount,
        token_decimals,
        usd_value,
    })
}

fn calculate_price(amounts: &TradeAmounts, is_buy: bool) -> Result<TradePrice, ConversionError> {
    if amounts.sol_amount <= 0.0 || amounts.token_amount <= 0.0 {
        return Err(ConversionError::PriceCalculationFailed("Zero amounts".to_string()));
    }
    
    let sol_per_token = if is_buy {
        // BUY: SOL spent / tokens received
        amounts.sol_amount / amounts.token_amount
    } else {
        // SELL: SOL received / tokens sold  
        amounts.sol_amount / amounts.token_amount
    };
    
    let tokens_per_sol = if sol_per_token > 0.0 { 1.0 / sol_per_token } else { 0.0 };
    
    let is_price_valid = sol_per_token > 0.0 && sol_per_token.is_finite() && !sol_per_token.is_nan();
    
    Ok(TradePrice {
        sol_per_token,
        tokens_per_sol,
        market_cap_change: None,
        price_impact: None,
        is_price_valid,
    })
}

fn calculate_fee_percentage(fee_amount: u64, total_amount: u64) -> f64 {
    if total_amount == 0 {
        return 0.0;
    }
    (fee_amount as f64 / total_amount as f64) * 100.0
}

fn calculate_pumpfun_market_cap(virtual_sol_reserves: u64, virtual_token_reserves: u64) -> Option<f64> {
    if virtual_token_reserves == 0 {
        return None;
    }
    
    let sol_reserves = virtual_sol_reserves as f64 / 1_000_000_000.0;
    let token_reserves = virtual_token_reserves as f64 / 1_000_000.0; // Assuming 6 decimals
    
    // Simple bonding curve market cap calculation
    Some(sol_reserves * 1_000_000.0 / token_reserves) // Very simplified
}

fn calculate_graduation_progress(real_sol_reserves: u64) -> Option<f64> {
    // PumpFun tokens graduate to Raydium at ~85 SOL
    const GRADUATION_THRESHOLD: u64 = 85_000_000_000; // 85 SOL in lamports
    let progress = (real_sol_reserves as f64 / GRADUATION_THRESHOLD as f64).min(1.0);
    Some(progress)
}

fn validate_trade_event(
    core: &CoreTradeData,
    _platform: &PlatformData,
    timing: &TimingData,
) -> Result<ValidationData, ConversionError> {
    let mut validation = ValidationData::new();
    
    // Basic price validation
    if !core.price.is_price_valid {
        validation.add_error(ValidationError::InvalidPrice {
            price: core.price.sol_per_token,
            reason: "Invalid price calculation".to_string(),
        });
    }
    
    // Amount validation
    if core.amounts.sol_amount <= 0.0 || core.amounts.token_amount <= 0.0 {
        validation.add_error(ValidationError::InvalidAmounts {
            sol_amount: core.amounts.sol_amount,
            token_amount: core.amounts.token_amount,
            reason: "Zero or negative amounts".to_string(),
        });
    }
    
    // High processing delay warning
    if timing.processing_delay_ms > 1000 {
        validation.add_warning(ValidationWarning::HighProcessingDelay {
            delay_ms: timing.processing_delay_ms,
        });
    }
    
    Ok(validation)
}
