use crate::streaming::event_parser::common::EventMetadata;
use crate::{
    impl_unified_event, streaming::event_parser::protocols::raydium_amm_v4::types::AmmInfo,
};
use borsh::BorshDeserialize;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

/// Balance change entry for a specific mint/account
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BalanceChangeEntry {
    pub mint: String,
    pub balance_pre: u64,
    pub balance_post: u64,
    pub change: i64,
}

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

impl RaydiumAmmV4SwapEvent {
    /// Calculate balance changes for the transaction user
    pub fn calculate_balance_changes(&self, user_address: &str) -> Vec<BalanceChangeEntry> {
        let mut balance_changes = Vec::new();
        const WSOL_MINT: &str = "So11111111111111111111111111111111111111112";

        // Extract raw transaction from metadata if available
        let Some(raw_tx_wrapper) = &self.metadata.raw_transaction else { 
            return balance_changes; 
        };
        
        // Serialize the inner transaction to JSON for processing
        let tx_json = match serde_json::to_value(&raw_tx_wrapper.inner) {
            Ok(json) => json,
            Err(_) => return balance_changes,
        };

        let Some(meta) = tx_json.get("meta") else { return balance_changes; };

        // 1) Pull the REAL message.accountKeys so indices match pre/postBalances
        let real_keys: Vec<String> = tx_json.get("transaction")
            .and_then(|t| t.get("message"))
            .and_then(|m| m.get("accountKeys"))
            .and_then(|ks| ks.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default();

        // Find signer index in the REAL key order (or fall back to fee payer index 0)
        let signer_index = real_keys
            .iter()
            .position(|k| k == user_address)
            .unwrap_or(0);

        // 2) Native SOL (lamports) delta for signer
        let (mut native_pre, mut native_post) = (0u64, 0u64);
        if let (Some(pre), Some(post)) = (meta.get("preBalances"), meta.get("postBalances")) {
            if let (Some(pre_arr), Some(post_arr)) = (pre.as_array(), post.as_array()) {
                native_pre = pre_arr.get(signer_index).and_then(|b| b.as_u64()).unwrap_or(0);
                native_post = post_arr.get(signer_index).and_then(|b| b.as_u64()).unwrap_or(0);
            }
        }

        // 3) Token deltas owned by the signer (including wSOL ONLY if owned by signer)
        let mut signer_wsol_delta: i64 = 0;
        let mut token_changes: HashMap<String, (u64, u64)> = HashMap::new();

        let pull_balances = |field: &str, map: &mut HashMap<String, (u64, u64)>, signer: &str, wsol_delta_out: &mut i64| {
            if let Some(arr) = meta.get(field).and_then(|v| v.as_array()) {
                for t in arr {
                    let owner = t.get("owner").and_then(|o| o.as_str()).unwrap_or("");
                    let mint = t.get("mint").and_then(|m| m.as_str()).unwrap_or("");
                    let amt_str = t
                        .get("uiTokenAmount")
                        .and_then(|ui| ui.get("amount"))
                        .and_then(|a| a.as_str())
                        .unwrap_or("0");

                    if owner == signer {
                        if let Ok(amount) = amt_str.parse::<u64>() {
                            let entry = map.entry(mint.to_string()).or_insert((0, 0));
                            if field == "preTokenBalances" {
                                entry.0 = amount;
                            } else {
                                entry.1 = amount;
                            }
                        }
                    }

                    // Track wSOL only when it's actually *owned by the signer*
                    if owner == signer && mint == WSOL_MINT {
                        if let Ok(amount) = amt_str.parse::<u64>() {
                            if field == "preTokenBalances" {
                                *wsol_delta_out -= amount as i64; // will be reconciled with post below
                            } else {
                                *wsol_delta_out += amount as i64;
                            }
                        }
                    }
                }
            }
        };

        pull_balances("preTokenBalances", &mut token_changes, user_address, &mut signer_wsol_delta);
        pull_balances("postTokenBalances", &mut token_changes, user_address, &mut signer_wsol_delta);

        // 4) Emit non-wSOL token changes (owned by signer)
        for (mint, (pre_amt, post_amt)) in token_changes.into_iter() {
            if pre_amt != post_amt && mint != WSOL_MINT {
                balance_changes.push(BalanceChangeEntry {
                    mint,
                    balance_pre: pre_amt,
                    balance_post: post_amt,
                    change: post_amt as i64 - pre_amt as i64,
                });
            }
        }

        // 5) Summaries
        // - Native SOL change already includes unwraps via CloseAccount even if the temp wSOL ATA was NOT owned by signer.
        // - Only add signer_wsol_delta when the signer actually had a wSOL ATA.
        let combined_pre = native_pre;
        let combined_post = (native_post as i64 + signer_wsol_delta).max(0) as u64;
        let combined_change = (combined_post as i64) - (combined_pre as i64);

        // First row: combined view (native + signer-owned wSOL only)
        balance_changes.insert(0, BalanceChangeEntry {
            mint: "SOL (native + signer-owned wSOL)".to_string(),
            balance_pre: combined_pre,
            balance_post: combined_post,
            change: combined_change,
        });

        balance_changes
    }

    /// Calculate accurate trade info based on balance changes
    pub fn calculate_trade_info(&self, user_address: &str) -> Option<TradeInfoCalculated> {
        const SOL_MINT: &str = "So11111111111111111111111111111111111111112";
        
        let balance_changes = self.calculate_balance_changes(user_address);
        
        // Separate SOL changes from token changes
        let mut sol_change: i64 = 0;
        let mut token_changes: Vec<&BalanceChangeEntry> = Vec::new();
        
        for change in &balance_changes {
            if change.mint.contains("SOL") {
                // This is a SOL balance entry - extract the change
                sol_change = change.change;
            } else {
                // This is a token balance entry
                if change.change != 0 {
                    token_changes.push(change);
                }
            }
        }
        
        // Analyze trade direction based on balance changes
        let positive_token_changes: Vec<&BalanceChangeEntry> = token_changes.iter()
            .filter(|change| change.change > 0)
            .copied()
            .collect();
            
        let negative_token_changes: Vec<&BalanceChangeEntry> = token_changes.iter()
            .filter(|change| change.change < 0)
            .copied()
            .collect();
        
        let direction: String;
        let input_mint: String;
        let output_mint: String;
        let amount_in: u64;
        let amount_out: u64;
        
        if sol_change < 0 && positive_token_changes.len() == 1 && negative_token_changes.is_empty() {
            // BUY: Negative SOL change and only one positive token change
            direction = "Buy".to_string();
            input_mint = SOL_MINT.to_string();
            output_mint = positive_token_changes[0].mint.clone();
            amount_in = sol_change.abs() as u64;
            amount_out = positive_token_changes[0].change as u64;
        } else if sol_change > 0 && negative_token_changes.len() == 1 && positive_token_changes.is_empty() {
            // SELL: Positive SOL change and one negative token change
            direction = "Sell".to_string();
            input_mint = negative_token_changes[0].mint.clone();
            output_mint = SOL_MINT.to_string();
            amount_in = negative_token_changes[0].change.abs() as u64;
            amount_out = sol_change as u64;
        } else if positive_token_changes.len() == 1 && negative_token_changes.len() == 1 && sol_change == 0 {
            // TOKEN SWAP: One positive and one negative token change, no SOL change
            direction = "TokenSwap".to_string();
            input_mint = negative_token_changes[0].mint.clone();
            output_mint = positive_token_changes[0].mint.clone();
            amount_in = negative_token_changes[0].change.abs() as u64;
            amount_out = positive_token_changes[0].change as u64;
        } else if token_changes.is_empty() && sol_change != 0 {
            // ARBITRAGE: Only SOL change, no token changes
            direction = "Arbitrage".to_string();
            input_mint = SOL_MINT.to_string();
            output_mint = SOL_MINT.to_string();
            amount_in = 0;
            amount_out = sol_change.abs() as u64;
        } else {
            // UNKNOWN: Complex or unclear trade pattern
            return None;
        }
        
        // Calculate price
        let price = if amount_in > 0 && amount_out > 0 {
            match direction.as_str() {
                "Buy" => {
                    // For buys: price = amount_in / amount_out (SOL per token)
                    // Convert SOL lamports to SOL for price calculation
                    let sol_amount = amount_in as f64 / 1_000_000_000.0;
                    sol_amount / amount_out as f64
                },
                "Sell" => {
                    // For sells: price = amount_out / amount_in (SOL per token)
                    // Convert SOL lamports to SOL for price calculation  
                    let sol_amount = amount_out as f64 / 1_000_000_000.0;
                    sol_amount / amount_in as f64
                },
                "TokenSwap" => {
                    // For token swaps: simple ratio
                    amount_in as f64 / amount_out as f64
                },
                _ => 0.0
            }
        } else {
            0.0
        };
        
        Some(TradeInfoCalculated {
            user_address: user_address.to_string(),
            direction,
            input_mint,
            output_mint,
            amount_in,
            amount_out,
            price,
        })
    }
}

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
    pub pubkey: String,
    pub executable: bool,
    pub lamports: u64,
    pub owner: String,
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