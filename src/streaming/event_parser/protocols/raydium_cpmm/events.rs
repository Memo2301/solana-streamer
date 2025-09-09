use crate::streaming::event_parser::common::EventMetadata;
use crate::streaming::event_parser::protocols::raydium_cpmm::types::PoolState;
use crate::{
    impl_unified_event, streaming::event_parser::protocols::raydium_cpmm::types::AmmConfig,
};
use borsh::BorshDeserialize;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

/// 交易
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, BorshDeserialize)]
pub struct RaydiumCpmmSwapEvent {
    #[borsh(skip)]
    pub metadata: EventMetadata,
    pub amount_in: u64,
    pub minimum_amount_out: u64,
    pub max_amount_in: u64,
    pub amount_out: u64,
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
    pub input_vault: Pubkey,
    pub output_vault: Pubkey,
    pub input_token_program: Pubkey,
    pub output_token_program: Pubkey,
    pub input_token_mint: Pubkey,
    pub output_token_mint: Pubkey,
    pub observation_state: Pubkey,
}

impl_unified_event!(RaydiumCpmmSwapEvent,);

/// 存款
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, BorshDeserialize)]
pub struct RaydiumCpmmDepositEvent {
    #[borsh(skip)]
    pub metadata: EventMetadata,
    pub lp_token_amount: u64,
    pub maximum_token0_amount: u64,
    pub maximum_token1_amount: u64,

    pub owner: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub owner_lp_token: Pubkey,
    pub token0_account: Pubkey,
    pub token1_account: Pubkey,
    pub token0_vault: Pubkey,
    pub token1_vault: Pubkey,
    pub token_program: Pubkey,
    pub token_program2022: Pubkey,
    pub vault0_mint: Pubkey,
    pub vault1_mint: Pubkey,
    pub lp_mint: Pubkey,
}
impl_unified_event!(RaydiumCpmmDepositEvent,);

/// 初始化
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, BorshDeserialize)]
pub struct RaydiumCpmmInitializeEvent {
    #[borsh(skip)]
    pub metadata: EventMetadata,
    pub init_amount0: u64,
    pub init_amount1: u64,
    pub open_time: u64,

    pub creator: Pubkey,
    pub amm_config: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub token0_mint: Pubkey,
    pub token1_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub creator_token0: Pubkey,
    pub creator_token1: Pubkey,
    pub creator_lp_token: Pubkey,
    pub token0_vault: Pubkey,
    pub token1_vault: Pubkey,
    pub create_pool_fee: Pubkey,
    pub observation_state: Pubkey,
    pub token_program: Pubkey,
    pub token0_program: Pubkey,
    pub token1_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl_unified_event!(RaydiumCpmmInitializeEvent,);

/// 提款
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, BorshDeserialize)]
pub struct RaydiumCpmmWithdrawEvent {
    #[borsh(skip)]
    pub metadata: EventMetadata,
    pub lp_token_amount: u64,
    pub minimum_token0_amount: u64,
    pub minimum_token1_amount: u64,

    pub owner: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub owner_lp_token: Pubkey,
    pub token0_account: Pubkey,
    pub token1_account: Pubkey,
    pub token0_vault: Pubkey,
    pub token1_vault: Pubkey,
    pub token_program: Pubkey,
    pub token_program2022: Pubkey,
    pub vault0_mint: Pubkey,
    pub vault1_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub memo_program: Pubkey,
}
impl_unified_event!(RaydiumCpmmWithdrawEvent,);

/// 池配置
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, BorshDeserialize)]
pub struct RaydiumCpmmAmmConfigAccountEvent {
    #[borsh(skip)]
    pub metadata: EventMetadata,
    pub pubkey: Pubkey,
    pub executable: bool,
    pub lamports: u64,
    pub owner: Pubkey,
    pub rent_epoch: u64,
    pub amm_config: AmmConfig,
}
impl_unified_event!(RaydiumCpmmAmmConfigAccountEvent,);

/// 池状态
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, BorshDeserialize)]
pub struct RaydiumCpmmPoolStateAccountEvent {
    #[borsh(skip)]
    pub metadata: EventMetadata,
    pub pubkey: Pubkey,
    pub executable: bool,
    pub lamports: u64,
    pub owner: Pubkey,
    pub rent_epoch: u64,
    pub pool_state: PoolState,
}
impl_unified_event!(RaydiumCpmmPoolStateAccountEvent,);

/// 事件鉴别器常量
pub mod discriminators {
    // 指令鉴别器
    pub const SWAP_BASE_IN: &[u8] = &[143, 190, 90, 218, 196, 30, 51, 222];
    pub const SWAP_BASE_OUT: &[u8] = &[55, 217, 98, 86, 163, 74, 180, 173];
    pub const DEPOSIT: &[u8] = &[242, 35, 198, 137, 82, 225, 242, 182];
    pub const INITIALIZE: &[u8] = &[175, 175, 109, 31, 13, 152, 155, 237];
    pub const WITHDRAW: &[u8] = &[183, 18, 70, 156, 148, 109, 161, 34];

    // 账号鉴别器
    pub const AMM_CONFIG: &[u8] = &[218, 244, 33, 104, 203, 203, 43, 111];
    pub const POOL_STATE: &[u8] = &[247, 237, 227, 245, 215, 195, 222, 70];
}

use crate::streaming::event_parser::protocols::raydium_cpmm::types::{
    TradeDirection as TradeDirection, TradeInfo, CopyTradeableEvent,
};

// WSOL mint address for trade direction detection
const WSOL_MINT: &str = "So11111111111111111111111111111111111111112";

impl RaydiumCpmmSwapEvent {
    /// Extract trade information with direction detection
    pub fn get_trade_info(&self) -> Option<TradeInfo> {
        // Check if this involves WSOL (copy-tradeable)
        let input_mint = self.input_token_mint.to_string();
        let output_mint = self.output_token_mint.to_string();
        
        // Only process if one of the tokens is WSOL
        if input_mint != WSOL_MINT && output_mint != WSOL_MINT {
            return None;
        }
        
        // Determine trade direction and token
        let (direction, token_mint) = if input_mint == WSOL_MINT {
            // Input is WSOL, output is the token - buying token with SOL
            (TradeDirection::Buy, output_mint.clone())
        } else {
            // Output is WSOL, input is the token - selling token for SOL
            (TradeDirection::Sell, input_mint.clone())
        };
        
        let user_address = self.payer.to_string();
        let sol_amount = if input_mint == WSOL_MINT {
            self.amount_in as f64 / 1_000_000_000.0  // Input is WSOL
        } else {
            self.amount_out as f64 / 1_000_000_000.0  // Output is WSOL
        };
        
        Some(TradeInfo {
            direction,
            user_address,
            token_mint: token_mint.clone(),
            sol_amount,
            platform: "RaydiumCpmm".to_string(),
            input_mint: input_mint.clone(),
            output_mint: output_mint.clone(),
            amount_in: self.amount_in,
            amount_out: self.amount_out,
        })
    }
}

impl CopyTradeableEvent for RaydiumCpmmSwapEvent {
    fn get_trade_info(&self) -> Option<TradeInfo> {
        self.get_trade_info()
    }
}
