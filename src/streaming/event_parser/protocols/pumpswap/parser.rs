use solana_sdk::pubkey::Pubkey;

use crate::{
    impl_event_parser_delegate,
    streaming::event_parser::{
        common::{read_u64_le, EventMetadata, EventType, ProtocolType},
        core::traits::{GenericEventParseConfig, GenericEventParser, UnifiedEvent},
        protocols::pumpswap::{
            discriminators, pump_swap_buy_event_log_decode, pump_swap_create_pool_event_log_decode,
            pump_swap_deposit_event_log_decode, pump_swap_sell_event_log_decode,
            pump_swap_withdraw_event_log_decode, PumpSwapBuyEvent, PumpSwapCreatePoolEvent,
            PumpSwapDepositEvent, PumpSwapSellEvent, PumpSwapWithdrawEvent,
        },
    },
};

/// PumpSwap程序ID
pub const PUMPSWAP_PROGRAM_ID: Pubkey =
    solana_sdk::pubkey!("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA");

/// PumpSwap事件解析器
pub struct PumpSwapEventParser {
    inner: GenericEventParser,
}

impl Default for PumpSwapEventParser {
    fn default() -> Self {
        Self::new()
    }
}

impl PumpSwapEventParser {
    pub fn new() -> Self {
        // 配置所有事件类型
        let configs = vec![
            GenericEventParseConfig {
                program_id: PUMPSWAP_PROGRAM_ID,
                protocol_type: ProtocolType::PumpSwap,
                inner_instruction_discriminator: discriminators::BUY_EVENT,
                instruction_discriminator: discriminators::BUY_IX,
                event_type: EventType::PumpSwapBuy,
                inner_instruction_parser: Some(Self::parse_buy_inner_instruction),
                instruction_parser: Some(Self::parse_buy_instruction),
            },
            GenericEventParseConfig {
                program_id: PUMPSWAP_PROGRAM_ID,
                protocol_type: ProtocolType::PumpSwap,
                inner_instruction_discriminator: discriminators::SELL_EVENT,
                instruction_discriminator: discriminators::SELL_IX,
                event_type: EventType::PumpSwapSell,
                inner_instruction_parser: Some(Self::parse_sell_inner_instruction),
                instruction_parser: Some(Self::parse_sell_instruction),
            },
            GenericEventParseConfig {
                program_id: PUMPSWAP_PROGRAM_ID,
                protocol_type: ProtocolType::PumpSwap,
                inner_instruction_discriminator: discriminators::CREATE_POOL_EVENT,
                instruction_discriminator: discriminators::CREATE_POOL_IX,
                event_type: EventType::PumpSwapCreatePool,
                inner_instruction_parser: Some(Self::parse_create_pool_inner_instruction),
                instruction_parser: Some(Self::parse_create_pool_instruction),
            },
            GenericEventParseConfig {
                program_id: PUMPSWAP_PROGRAM_ID,
                protocol_type: ProtocolType::PumpSwap,
                inner_instruction_discriminator: discriminators::DEPOSIT_EVENT,
                instruction_discriminator: discriminators::DEPOSIT_IX,
                event_type: EventType::PumpSwapDeposit,
                inner_instruction_parser: Some(Self::parse_deposit_inner_instruction),
                instruction_parser: Some(Self::parse_deposit_instruction),
            },
            GenericEventParseConfig {
                program_id: PUMPSWAP_PROGRAM_ID,
                protocol_type: ProtocolType::PumpSwap,
                inner_instruction_discriminator: discriminators::WITHDRAW_EVENT,
                instruction_discriminator: discriminators::WITHDRAW_IX,
                event_type: EventType::PumpSwapWithdraw,
                inner_instruction_parser: Some(Self::parse_withdraw_inner_instruction),
                instruction_parser: Some(Self::parse_withdraw_instruction),
            },
        ];

        let inner = GenericEventParser::new(vec![PUMPSWAP_PROGRAM_ID], configs);

        Self { inner }
    }

    /// 解析买入日志事件
    fn parse_buy_inner_instruction(
        data: &[u8],
        metadata: EventMetadata,
    ) -> Option<Box<dyn UnifiedEvent>> {
        if let Some(mut event) = pump_swap_buy_event_log_decode(data) {
            // 🚨 CRITICAL FIX: Populate #[borsh(skip)] fields from metadata
            // Instead of leaving them as defaults, extract from transaction data
            if let Some(ref swap_data) = metadata.swap_data {
                // Determine base/quote mints from swap direction
                if swap_data.from_mint.to_string() == "So11111111111111111111111111111111111111112" {
                    // WSOL → Token (Buy): from=quote(WSOL), to=base(token)
                    event.quote_mint = swap_data.from_mint;
                    event.base_mint = swap_data.to_mint;
                } else {
                    // Token → WSOL: from=base(token), to=quote(WSOL)
                    event.base_mint = swap_data.from_mint;
                    event.quote_mint = swap_data.to_mint;
                }
            }
            
            // Populate fee fields with proper PumpSwap addresses
            event.fee_config = solana_sdk::pubkey!("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ"); // PumpSwap Fee Config
            event.fee_program = solana_sdk::pubkey!("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ"); // PumpSwap Fee Program
            
            event.metadata = metadata;
            Some(Box::new(event))
        } else {
            None
        }
    }

    /// 解析卖出日志事件
    fn parse_sell_inner_instruction(
        data: &[u8],
        metadata: EventMetadata,
    ) -> Option<Box<dyn UnifiedEvent>> {
        if let Some(mut event) = pump_swap_sell_event_log_decode(data) {
            // 🚨 CRITICAL FIX: Populate #[borsh(skip)] fields from metadata
            // Instead of leaving them as defaults, extract from transaction data
            if let Some(ref swap_data) = metadata.swap_data {
                // Determine base/quote mints from swap direction
                if swap_data.from_mint.to_string() == "So11111111111111111111111111111111111111112" {
                    // WSOL → Token: from=quote(WSOL), to=base(token)
                    event.quote_mint = swap_data.from_mint;
                    event.base_mint = swap_data.to_mint;
                } else {
                    // Token → WSOL (Sell): from=base(token), to=quote(WSOL)
                    event.base_mint = swap_data.from_mint;
                    event.quote_mint = swap_data.to_mint;
                }
            }
            
            // Populate fee fields with proper PumpSwap addresses
            event.fee_config = solana_sdk::pubkey!("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ"); // PumpSwap Fee Config
            event.fee_program = solana_sdk::pubkey!("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ"); // PumpSwap Fee Program
            
            event.metadata = metadata;
            Some(Box::new(event))
        } else {
            None
        }
    }

    /// 解析创建池子日志事件
    fn parse_create_pool_inner_instruction(
        data: &[u8],
        metadata: EventMetadata,
    ) -> Option<Box<dyn UnifiedEvent>> {
        if let Some(event) = pump_swap_create_pool_event_log_decode(data) {
            Some(Box::new(PumpSwapCreatePoolEvent { metadata, ..event }))
        } else {
            None
        }
    }

    /// 解析存款日志事件
    fn parse_deposit_inner_instruction(
        data: &[u8],
        metadata: EventMetadata,
    ) -> Option<Box<dyn UnifiedEvent>> {
        if let Some(event) = pump_swap_deposit_event_log_decode(data) {
            Some(Box::new(PumpSwapDepositEvent { metadata, ..event }))
        } else {
            None
        }
    }

    /// 解析提款日志事件
    fn parse_withdraw_inner_instruction(
        data: &[u8],
        metadata: EventMetadata,
    ) -> Option<Box<dyn UnifiedEvent>> {
        if let Some(event) = pump_swap_withdraw_event_log_decode(data) {
            Some(Box::new(PumpSwapWithdrawEvent { metadata, ..event }))
        } else {
            None
        }
    }

    /// 解析买入指令事件
    fn parse_buy_instruction(
        data: &[u8],
        accounts: &[Pubkey],
        metadata: EventMetadata,
    ) -> Option<Box<dyn UnifiedEvent>> {
        if data.len() < 16 || accounts.len() < 11 {
            return None;
        }

        let base_amount_out = read_u64_le(data, 0)?;
        let max_quote_amount_in = read_u64_le(data, 8)?;

        Some(Box::new(PumpSwapBuyEvent {
            metadata,
            base_amount_out,
            max_quote_amount_in,
            pool: accounts[0],
            user: accounts[1],
            base_mint: accounts[3],
            quote_mint: accounts[4],
            user_base_token_account: accounts[5],
            user_quote_token_account: accounts[6],
            protocol_fee_recipient: accounts[9],
            protocol_fee_recipient_token_account: accounts[10],
            coin_creator_vault_ata: accounts.get(17).copied().unwrap_or_default(),
            coin_creator_vault_authority: accounts.get(18).copied().unwrap_or_default(),
            fee_config: accounts.get(21).copied().unwrap_or_default(),
            fee_program: accounts.get(22).copied().unwrap_or_default(),
            ..Default::default()
        }))
    }

    /// 解析卖出指令事件
    fn parse_sell_instruction(
        data: &[u8],
        accounts: &[Pubkey],
        metadata: EventMetadata,
    ) -> Option<Box<dyn UnifiedEvent>> {
        if data.len() < 16 || accounts.len() < 11 {
            return None;
        }

        let base_amount_in = read_u64_le(data, 0)?;
        let min_quote_amount_out = read_u64_le(data, 8)?;

        Some(Box::new(PumpSwapSellEvent {
            metadata,
            base_amount_in,
            min_quote_amount_out,
            pool: accounts[0],
            user: accounts[1],
            base_mint: accounts[3],
            quote_mint: accounts[4],
            user_base_token_account: accounts[5],
            user_quote_token_account: accounts[6],
            protocol_fee_recipient: accounts[9],
            protocol_fee_recipient_token_account: accounts[10],
            coin_creator_vault_ata: accounts.get(17).copied().unwrap_or_default(),
            coin_creator_vault_authority: accounts.get(18).copied().unwrap_or_default(),
            fee_config: accounts.get(21).copied().unwrap_or_default(),
            fee_program: accounts.get(22).copied().unwrap_or_default(),
            ..Default::default()
        }))
    }

    /// 解析创建池子指令事件
    fn parse_create_pool_instruction(
        data: &[u8],
        accounts: &[Pubkey],
        metadata: EventMetadata,
    ) -> Option<Box<dyn UnifiedEvent>> {
        if data.len() < 18 || accounts.len() < 11 {
            return None;
        }

        let index = u16::from_le_bytes(data[0..2].try_into().ok()?);
        let base_amount_in = u64::from_le_bytes(data[2..10].try_into().ok()?);
        let quote_amount_in = u64::from_le_bytes(data[10..18].try_into().ok()?);
        let coin_creator = if data.len() >= 50 {
            Pubkey::new_from_array(data[18..50].try_into().ok()?)
        } else {
            Pubkey::default()
        };

        Some(Box::new(PumpSwapCreatePoolEvent {
            metadata,
            index,
            base_amount_in,
            quote_amount_in,
            pool: accounts[0],
            creator: accounts[2],
            base_mint: accounts[3],
            quote_mint: accounts[4],
            lp_mint: accounts[5],
            user_base_token_account: accounts[6],
            user_quote_token_account: accounts[7],
            user_pool_token_account: accounts[8],
            coin_creator,
            ..Default::default()
        }))
    }

    /// 解析存款指令事件
    fn parse_deposit_instruction(
        data: &[u8],
        accounts: &[Pubkey],
        metadata: EventMetadata,
    ) -> Option<Box<dyn UnifiedEvent>> {
        if data.len() < 24 || accounts.len() < 11 {
            return None;
        }

        let lp_token_amount_out = u64::from_le_bytes(data[0..8].try_into().ok()?);
        let max_base_amount_in = u64::from_le_bytes(data[8..16].try_into().ok()?);
        let max_quote_amount_in = u64::from_le_bytes(data[16..24].try_into().ok()?);

        Some(Box::new(PumpSwapDepositEvent {
            metadata,
            lp_token_amount_out,
            max_base_amount_in,
            max_quote_amount_in,
            pool: accounts[0],
            user: accounts[2],
            base_mint: accounts[3],
            quote_mint: accounts[4],
            user_base_token_account: accounts[6],
            user_quote_token_account: accounts[7],
            user_pool_token_account: accounts[8],
            ..Default::default()
        }))
    }

    /// 解析提款指令事件
    fn parse_withdraw_instruction(
        data: &[u8],
        accounts: &[Pubkey],
        metadata: EventMetadata,
    ) -> Option<Box<dyn UnifiedEvent>> {
        if data.len() < 24 || accounts.len() < 11 {
            return None;
        }

        let lp_token_amount_in = u64::from_le_bytes(data[0..8].try_into().ok()?);
        let min_base_amount_out = u64::from_le_bytes(data[8..16].try_into().ok()?);
        let min_quote_amount_out = u64::from_le_bytes(data[16..24].try_into().ok()?);

        Some(Box::new(PumpSwapWithdrawEvent {
            metadata,
            lp_token_amount_in,
            min_base_amount_out,
            min_quote_amount_out,
            pool: accounts[0],
            user: accounts[2],
            base_mint: accounts[3],
            quote_mint: accounts[4],
            user_base_token_account: accounts[6],
            user_quote_token_account: accounts[7],
            user_pool_token_account: accounts[8],
            ..Default::default()
        }))
    }
}

impl_event_parser_delegate!(PumpSwapEventParser);
