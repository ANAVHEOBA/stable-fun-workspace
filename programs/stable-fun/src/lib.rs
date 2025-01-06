use anchor_lang::prelude::*;

declare_id!("GjMGzVov7eb6igXCfFVSLeBnnzbS5QwyhLwTzhP2dABU");

pub mod state;
pub mod instructions;
pub mod error;
pub mod utils;

#[program]
pub mod stable_fun {
    use super::*;
    
    // Import instruction types and handlers
    use crate::instructions::{
        Initialize,
        MintStablecoin,
        RedeemStablecoin,
        UpdateSettings,
        update::UpdateSettingsParams,
    };

    #[inline(always)]
    pub fn initialize(
        ctx: Context<Initialize>,
        name: String,
        symbol: String,
        target_currency: String,
        initial_supply: u64,
    ) -> Result<()> {
        require!(
            name.len() >= constants::MIN_NAME_LENGTH,
            error::StableFunError::InvalidName
        );
        require!(
            symbol.len() >= constants::MIN_SYMBOL_LENGTH,
            error::StableFunError::InvalidSymbol
        );
        instructions::initialize::handler(ctx, name, symbol, target_currency, initial_supply)
    }

    #[inline(always)]
    pub fn mint(
        ctx: Context<MintStablecoin>,
        amount: u64
    ) -> Result<()> {
        require!(amount > 0, error::StableFunError::InvalidAmount);
        instructions::mint::handler(ctx, amount)
    }

    #[inline(always)]
    pub fn redeem(
        ctx: Context<RedeemStablecoin>,
        amount: u64
    ) -> Result<()> {
        require!(amount > 0, error::StableFunError::InvalidAmount);
        instructions::redeem::handler(ctx, amount)
    }

    #[inline(always)]
    pub fn update_settings(
        ctx: Context<UpdateSettings>,
        params: UpdateSettingsParams,
    ) -> Result<()> {
        require!(
            params.min_collateral_ratio >= constants::MIN_COLLATERAL_RATIO,
            error::StableFunError::InvalidCollateralRatio
        );
        instructions::update::handler(ctx, params)
    }
}

/// Constants used throughout the program
pub mod constants {
    use anchor_lang::prelude::*;

    // Seeds for PDAs
    pub const STABLECOIN_SEED: &[u8] = b"stablecoin";
    pub const VAULT_SEED: &[u8] = b"vault";
    pub const MINT_AUTHORITY_SEED: &[u8] = b"mint-authority";
    
    // Validation constants
    pub const MIN_NAME_LENGTH: usize = 3;
    pub const MIN_SYMBOL_LENGTH: usize = 2;
    pub const MAX_NAME_LENGTH: usize = 32;
    pub const MAX_SYMBOL_LENGTH: usize = 10;
    
    // Financial constants
    pub const BASIS_POINTS_DIVISOR: u16 = 10000;
    pub const DEFAULT_COLLATERAL_RATIO: u16 = 15000; // 150%
    pub const MIN_COLLATERAL_RATIO: u16 = 10000; // 100%
    pub const MAX_COLLATERAL_RATIO: u16 = 30000; // 300%
    pub const MAX_FEE_BPS: u16 = 1000; // 10%
    
    // Oracle constants
    pub const PRICE_DECIMALS: u8 = 6;
    pub const PRICE_SCALE: u64 = 10_u64.pow(PRICE_DECIMALS as u32);
    pub const MAX_PRICE_AGE: i64 = 300; // 5 minutes
    pub const MAX_PRICE_CONFIDENCE: u64 = PRICE_SCALE / 100; // 1%
}

/// Program version and metadata
pub const PROGRAM_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_program_ids() {
        let program_id = crate::ID;
        assert!(!program_id.to_bytes().iter().all(|&x| x == 0));
    }

    #[test]
    fn test_constants() {
        assert!(constants::DEFAULT_COLLATERAL_RATIO >= constants::MIN_COLLATERAL_RATIO);
        assert!(constants::DEFAULT_COLLATERAL_RATIO <= constants::MAX_COLLATERAL_RATIO);
        assert!(constants::MAX_FEE_BPS <= constants::BASIS_POINTS_DIVISOR);
    }

    #[test]
    fn test_price_scale() {
        assert_eq!(constants::PRICE_SCALE, 1_000_000);
    }
}

/// Re-export common types and constants
pub use crate::{
    error::StableFunError,
    state::{StablecoinMint, StablecoinVault},
    instructions::{Initialize, MintStablecoin, RedeemStablecoin, UpdateSettings},
};