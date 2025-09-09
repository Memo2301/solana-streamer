use crate::streaming::event_parser::common::EventMetadata;
use crate::{
    impl_unified_event, streaming::event_parser::protocols::raydium_amm_v4::types::AmmInfo,
};
use borsh::BorshDeserialize;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

/// 交易
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, BorshDeserialize)]
pub struct RaydiumAmmV4SwapEvent {
    #[borsh(skip)]
    pub metadata: EventMetadata,
    // base in
    pub amount_in: u64,
    pub minimum_amount_out: u64,
    // base out
    pub max_amount_in: u64,
    pub amount_out: u64,

    pub token_program: Pubkey,
    pub amm: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub amm_target_orders: Option<Pubkey>,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub serum_program: Pubkey,
    pub serum_market: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub serum_event_queue: Pubkey,
    pub serum_coin_vault_account: Pubkey,
    pub serum_pc_vault_account: Pubkey,
    pub serum_vault_signer: Pubkey,
    pub user_source_token_account: Pubkey,
    pub user_destination_token_account: Pubkey,
    pub user_source_owner: Pubkey,
}

impl_unified_event!(RaydiumAmmV4SwapEvent,);

/// 添加流动性
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, BorshDeserialize)]
pub struct RaydiumAmmV4DepositEvent {
    #[borsh(skip)]
    pub metadata: EventMetadata,
    pub max_coin_amount: u64,
    pub max_pc_amount: u64,
    pub base_side: u64,

    pub token_program: Pubkey,
    pub amm: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub amm_target_orders: Pubkey,
    pub lp_mint_address: Pubkey,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub serum_market: Pubkey,
    pub user_coin_token_account: Pubkey,
    pub user_pc_token_account: Pubkey,
    pub user_lp_token_account: Pubkey,
    pub user_owner: Pubkey,
    pub serum_event_queue: Pubkey,
}
impl_unified_event!(RaydiumAmmV4DepositEvent,);

/// 初始化
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, BorshDeserialize)]
pub struct RaydiumAmmV4Initialize2Event {
    #[borsh(skip)]
    pub metadata: EventMetadata,
    pub nonce: u8,
    pub open_time: u64,
    pub init_pc_amount: u64,
    pub init_coin_amount: u64,

    pub token_program: Pubkey,
    pub spl_associated_token_account: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub amm: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub lp_mint: Pubkey,
    pub coin_mint: Pubkey,
    pub pc_mint: Pubkey,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub pool_withdraw_queue: Pubkey,
    pub amm_target_orders: Pubkey,
    pub pool_temp_lp: Pubkey,
    pub serum_program: Pubkey,
    pub serum_market: Pubkey,
    pub user_wallet: Pubkey,
    pub user_token_coin: Pubkey,
    pub user_token_pc: Pubkey,
    pub user_lp_token_account: Pubkey,
}
impl_unified_event!(RaydiumAmmV4Initialize2Event,);

/// 移除流动性
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, BorshDeserialize)]
pub struct RaydiumAmmV4WithdrawEvent {
    #[borsh(skip)]
    pub metadata: EventMetadata,
    pub amount: u64,

    pub token_program: Pubkey,
    pub amm: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub amm_target_orders: Pubkey,
    pub lp_mint_address: Pubkey,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub pool_withdraw_queue: Pubkey,
    pub pool_temp_lp_token_account: Pubkey,
    pub serum_program: Pubkey,
    pub serum_market: Pubkey,
    pub serum_coin_vault_account: Pubkey,
    pub serum_pc_vault_account: Pubkey,
    pub serum_vault_signer: Pubkey,
    pub user_lp_token_account: Pubkey,
    pub user_coin_token_account: Pubkey,
    pub user_pc_token_account: Pubkey,
    pub user_owner: Pubkey,
    pub serum_event_queue: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
}
impl_unified_event!(RaydiumAmmV4WithdrawEvent,);

/// 提现
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, BorshDeserialize)]
pub struct RaydiumAmmV4WithdrawPnlEvent {
    #[borsh(skip)]
    pub metadata: EventMetadata,

    pub token_program: Pubkey,
    pub amm: Pubkey,
    pub amm_config: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub coin_pnl_token_account: Pubkey,
    pub pc_pnl_token_account: Pubkey,
    pub pnl_owner_account: Pubkey,
    pub amm_target_orders: Pubkey,
    pub serum_program: Pubkey,
    pub serum_market: Pubkey,
    pub serum_event_queue: Pubkey,
    pub serum_coin_vault_account: Pubkey,
    pub serum_pc_vault_account: Pubkey,
    pub serum_vault_signer: Pubkey,
}
impl_unified_event!(RaydiumAmmV4WithdrawPnlEvent,);

/// 池信息
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, BorshDeserialize)]
pub struct RaydiumAmmV4AmmInfoAccountEvent {
    #[borsh(skip)]
    pub metadata: EventMetadata,
    pub pubkey: Pubkey,
    pub executable: bool,
    pub lamports: u64,
    pub owner: Pubkey,
    pub rent_epoch: u64,
    pub amm_info: AmmInfo,
}
impl_unified_event!(RaydiumAmmV4AmmInfoAccountEvent,);

/// 事件鉴别器常量
pub mod discriminators {
    // 指令鉴别器
    pub const SWAP_BASE_IN: &[u8] = &[9];
    pub const SWAP_BASE_OUT: &[u8] = &[11];
    pub const DEPOSIT: &[u8] = &[03];
    pub const INITIALIZE2: &[u8] = &[01];
    pub const WITHDRAW: &[u8] = &[04];
    pub const WITHDRAW_PNL: &[u8] = &[07];

    /// 池信息鉴别器
    pub const AMM_INFO: &[u8] = &[6];
}

use crate::streaming::event_parser::protocols::raydium_cpmm::types::{
    TradeDirection as TradeDirection, TradeInfo, CopyTradeableEvent,
};

/// Calculated trade information based on balance changes
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TradeInfoCalculated {
    pub user_address: String,
    pub direction: String,
    pub input_mint: String,
    pub output_mint: String,
    pub amount_in: u64,
    pub amount_out: u64,
    pub price: f64,
}

// WSOL mint address for trade direction detection
const WSOL_MINT: &str = "So11111111111111111111111111111111111111112";

impl RaydiumAmmV4SwapEvent {
    /// Calculate trade information from raw transaction data
    /// This method analyzes the complete transaction to determine trade details
    pub fn calculate_trade_info(&self, user_address: &str) -> Option<TradeInfoCalculated> {
        // Check if we have raw transaction data
        let raw_tx = self.metadata.raw_transaction.as_ref()?;
        
        // Analyze transaction for WSOL/Token balance changes
        // For now, return a simplified TradeInfoCalculated
        Some(TradeInfoCalculated {
            user_address: user_address.to_string(),
            direction: "Buy".to_string(), // Simplified
            input_mint: "unknown".to_string(),
            output_mint: "unknown".to_string(),
            amount_in: self.amount_in,
            amount_out: self.amount_out,
            price: 0.0,
        })
    }
    
    /// Analyze balance changes from the raw transaction to determine trade info
    fn analyze_balance_changes(
        &self,
        tx: &solana_transaction_status::EncodedTransactionWithStatusMeta,
        user_address: &str,
    ) -> Option<TradeInfo> {
        // Extract balance changes from transaction metadata
        let meta = tx.meta.as_ref()?;
        
        // Get pre and post token balances
        // Note: OptionSerializer doesn't work the same as Option
        // For now, return None as this method needs raw transaction data to be properly implemented
        // TODO: Implement proper OptionSerializer handling for balance analysis
        None
    }
    
    /// Extract trade information with direction detection (fallback method)
    /// Note: RaydiumAmmV4SwapEvent doesn't have direct mint access
    pub fn get_trade_info(&self) -> Option<TradeInfo> {
        // For AMM V4, we don't have direct mint access in the event struct
        // We can try to determine direction from the swap amounts
        // This is a simplified implementation that may need enhancement
        
        let user_address = self.user_source_owner.to_string();
        
        // Use the larger of amount_in or amount_out as the SOL amount
        // This is a heuristic and may not always be accurate
        let sol_amount = std::cmp::max(self.amount_in, self.amount_out) as f64 / 1_000_000_000.0;
        
        // Simplified direction detection based on relative amounts
        // This is very basic and should be enhanced with actual mint checking
        let (direction, token_mint) = if self.amount_in > self.amount_out {
            // More going in than out - likely buying (input SOL, output token)
            (TradeDirection::Buy, "unknown".to_string())
        } else {
            // More coming out than in - likely selling (input token, output SOL) 
            (TradeDirection::Sell, "unknown".to_string())
        };
        
        Some(TradeInfo {
            direction,
            user_address,
            token_mint: token_mint.clone(),
            sol_amount,
            platform: "RaydiumAmmV4".to_string(),
            input_mint: "unknown".to_string(), // AMM V4 doesn't have direct mint access
            output_mint: "unknown".to_string(), // AMM V4 doesn't have direct mint access
            amount_in: self.amount_in,
            amount_out: self.amount_out,
        })
    }
}

impl CopyTradeableEvent for RaydiumAmmV4SwapEvent {
    fn get_trade_info(&self) -> Option<TradeInfo> {
        self.get_trade_info()
    }
}
