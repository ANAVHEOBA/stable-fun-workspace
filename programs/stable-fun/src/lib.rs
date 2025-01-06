use anchor_lang::prelude::*;

// Import instruction types directly from their modules
use crate::instructions::initialize::Initialize;
use crate::instructions::mint::MintStablecoin;
use crate::instructions::redeem::RedeemStablecoin;
use crate::instructions::update::{UpdateSettings, UpdateSettingsParams};
use crate::error::StableFunError;

declare_id!("GjMGzVov7eb6igXCfFVSLeBnnzbS5QwyhLwTzhP2dABU");

// Module declarations
pub mod state;
pub mod instructions;
pub mod error;
pub mod utils;
pub mod constants;

#[program]
pub mod stable_fun {
    use super::*;
    use crate::constants::{MIN_NAME_LENGTH, MIN_SYMBOL_LENGTH, MIN_COLLATERAL_RATIO};

    #[inline(always)]
    pub fn initialize(
        ctx: Context<Initialize>,
        name: String,
        symbol: String,
        target_currency: String,
        initial_supply: u64,
    ) -> Result<()> {
        require!(
            name.len() >= MIN_NAME_LENGTH,
            StableFunError::NameTooShort
        );
        require!(
            symbol.len() >= MIN_SYMBOL_LENGTH,
            StableFunError::SymbolTooShort
        );
        instructions::initialize::handler(ctx, name, symbol, target_currency, initial_supply)
    }

    #[inline(always)]
    pub fn mint(
        ctx: Context<MintStablecoin>,
        amount: u64
    ) -> Result<()> {
        require!(amount > 0, StableFunError::InvalidAmount);
        instructions::mint::handler(ctx, amount)
    }

    #[inline(always)]
    pub fn redeem(
        ctx: Context<RedeemStablecoin>,
        amount: u64
    ) -> Result<()> {
        require!(amount > 0, StableFunError::InvalidAmount);
        instructions::redeem::handler(ctx, amount)
    }

    #[inline(always)]
    pub fn update_settings(
        ctx: Context<UpdateSettings>,
        params: UpdateSettingsParams,
    ) -> Result<()> {
        require!(
            params.min_collateral_ratio.unwrap_or(MIN_COLLATERAL_RATIO) >= MIN_COLLATERAL_RATIO,
            StableFunError::CollateralRatioTooLow
        );
        instructions::update::handler(ctx, params)
    }
}

// Re-exports (moved to a separate module to avoid conflicts)
pub mod prelude {
    pub use crate::error::StableFunError;
    pub use crate::state::{StablecoinMint, StablecoinVault};
    pub use crate::instructions::{Initialize, MintStablecoin, RedeemStablecoin, UpdateSettings};
}