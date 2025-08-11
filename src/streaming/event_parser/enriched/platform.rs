use serde::{Deserialize, Serialize};

/// Platform-specific data for different DEX protocols
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PlatformData {
    PumpFun(PumpFunData),
    Bonk(BonkData),
    PumpSwap(PumpSwapData),
    RaydiumCpmm(RaydiumCpmmData),
    RaydiumClmm(RaydiumClmmData),
}

impl PlatformData {
    pub fn platform_name(&self) -> &'static str {
        match self {
            PlatformData::PumpFun(_) => "PumpFun",
            PlatformData::Bonk(_) => "Bonk",
            PlatformData::PumpSwap(_) => "PumpSwap",
            PlatformData::RaydiumCpmm(_) => "RaydiumCpmm",
            PlatformData::RaydiumClmm(_) => "RaydiumClmm",
        }
    }
    
    pub fn fees(&self) -> &FeeData {
        match self {
            PlatformData::PumpFun(data) => &data.fees,
            PlatformData::Bonk(data) => &data.fees,
            PlatformData::PumpSwap(data) => &data.fees,
            PlatformData::RaydiumCpmm(data) => &data.fees,
            PlatformData::RaydiumClmm(data) => &data.fees,
        }
    }
}

/// PumpFun-specific trading data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PumpFunData {
    /// Bonding curve address (needed for buy execution)
    pub bonding_curve: String,
    
    /// Token creator address
    pub creator: String,
    
    /// Virtual SOL reserves for price calculation
    pub virtual_sol_reserves: u64,
    
    /// Virtual token reserves for price calculation
    pub virtual_token_reserves: u64,
    
    /// Real SOL reserves (actual liquidity)
    pub real_sol_reserves: u64,
    
    /// Real token reserves (actual liquidity)
    pub real_token_reserves: u64,
    
    /// Fee information
    pub fees: FeeData,
    
    /// Market cap derived from bonding curve
    pub market_cap: Option<f64>,
    
    /// Progress towards Raydium graduation (0.0 to 1.0)
    pub graduation_progress: Option<f64>,
}

/// Bonk-specific trading data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BonkData {
    /// Pool address if available
    pub pool: Option<String>,
    
    /// Fee information
    pub fees: FeeData,
    
    /// Pool reserves if available
    pub pool_reserves: Option<PoolReserves>,
}

/// PumpSwap-specific trading data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PumpSwapData {
    /// Pool address (always available for PumpSwap)
    pub pool: String,
    
    /// Fee information
    pub fees: FeeData,
    
    /// Pool reserves
    pub pool_reserves: Option<PoolReserves>,
}

/// Raydium CPMM-specific trading data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RaydiumCpmmData {
    /// Pool address
    pub pool: String,
    
    /// Fee information
    pub fees: FeeData,
    
    /// Pool reserves
    pub pool_reserves: Option<PoolReserves>,
    
    /// Pool configuration
    pub pool_config: Option<RaydiumPoolConfig>,
}

/// Raydium CLMM-specific trading data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RaydiumClmmData {
    /// Pool address
    pub pool: String,
    
    /// Fee information
    pub fees: FeeData,
    
    /// Current tick and liquidity info
    pub tick_info: Option<TickInfo>,
    
    /// Pool configuration
    pub pool_config: Option<RaydiumPoolConfig>,
}

/// Unified fee data structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeeData {
    /// Total fee amount paid (in raw units)
    pub total_fee_amount: u64,
    
    /// Fee as percentage (0.0 to 100.0)
    pub fee_percentage: f64,
    
    /// Creator fee if applicable (in raw units)
    pub creator_fee: Option<u64>,
    
    /// Platform fee breakdown
    pub fee_breakdown: Vec<FeeComponent>,
}

/// Individual fee component
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeeComponent {
    pub name: String,
    pub amount: u64,
    pub percentage: f64,
    pub recipient: Option<String>,
}

/// Pool reserves information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PoolReserves {
    pub base_reserves: u64,
    pub quote_reserves: u64,
    pub base_decimals: u8,
    pub quote_decimals: u8,
}

/// Raydium pool configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RaydiumPoolConfig {
    pub mint_a: String,
    pub mint_b: String,
    pub vault_a: String,
    pub vault_b: String,
    pub fee_rate: u64,
}

/// CLMM tick information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TickInfo {
    pub current_tick: i32,
    pub sqrt_price: u128,
    pub liquidity: u128,
    pub tick_spacing: u16,
}

impl FeeData {
    pub fn new(total_fee_amount: u64, fee_percentage: f64) -> Self {
        Self {
            total_fee_amount,
            fee_percentage,
            creator_fee: None,
            fee_breakdown: Vec::new(),
        }
    }
    
    pub fn with_creator_fee(mut self, creator_fee: u64) -> Self {
        self.creator_fee = Some(creator_fee);
        self
    }
    
    pub fn add_fee_component(&mut self, component: FeeComponent) {
        self.fee_breakdown.push(component);
    }
}

impl FeeComponent {
    pub fn new(name: String, amount: u64, percentage: f64) -> Self {
        Self {
            name,
            amount,
            percentage,
            recipient: None,
        }
    }
    
    pub fn with_recipient(mut self, recipient: String) -> Self {
        self.recipient = Some(recipient);
        self
    }
}
