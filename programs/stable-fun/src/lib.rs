use anchor_lang::prelude::*;

declare_id!("5Zwh3KUbGG7244b6mczjgnMeT567UyR86PEyEuM1sMft");

pub mod state;
pub mod instructions;
pub mod error;
pub mod utils;
pub mod constants;

use crate::instructions::{
    initialize::Initialize,
    mint::MintStablecoin,
    redeem::RedeemStablecoin,
    update::{UpdateSettings, UpdateSettingsParams},
};
use crate::error::StableFunError;
use crate::constants::{MIN_NAME_LENGTH, MIN_SYMBOL_LENGTH, MIN_COLLATERAL_RATIO};

#[program]
mod stable_fun {
    use super::*;

    #[inline(never)]
    pub fn initialize(
        ctx: Context<Initialize>,
        name: String,
        symbol: String,
        target_currency: String,
        initial_supply: u64,
    ) -> Result<()> {
        msg!("Initializing with name: {}, symbol: {}", name, symbol);
        require!(
            name.len() >= MIN_NAME_LENGTH,
            StableFunError::NameTooShort
        );
        require!(
            symbol.len() >= MIN_SYMBOL_LENGTH,
            StableFunError::SymbolTooShort
        );
        crate::instructions::initialize::handler(ctx, name, symbol, target_currency, initial_supply)
    }

    #[inline(never)]
    pub fn mint(
        ctx: Context<MintStablecoin>,
        amount: u64
    ) -> Result<()> {
        msg!("Minting {} tokens", amount);
        require!(amount > 0, StableFunError::InvalidAmount);
        crate::instructions::mint::handler(ctx, amount)
    }

    #[inline(never)]
    pub fn redeem(
        ctx: Context<RedeemStablecoin>,
        amount: u64
    ) -> Result<()> {
        msg!("Redeeming {} tokens", amount);
        require!(amount > 0, StableFunError::InvalidAmount);
        crate::instructions::redeem::handler(ctx, amount)
    }

    #[inline(never)]
    pub fn update_settings(
        ctx: Context<UpdateSettings>,
        params: UpdateSettingsParams,
    ) -> Result<()> {
        msg!("Updating settings");
        require!(
            params.min_collateral_ratio.unwrap_or(MIN_COLLATERAL_RATIO) >= MIN_COLLATERAL_RATIO,
            StableFunError::CollateralRatioTooLow
        );
        crate::instructions::update::handler(ctx, params)
    }
}