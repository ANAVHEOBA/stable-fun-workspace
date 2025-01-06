use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use switchboard_v2::AggregatorAccountData;

use crate::state::{StablecoinMint, StablecoinVault};
use crate::error::StableFunError;
use crate::utils;  // Import the utils module directly

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct RedeemStablecoin<'info> {
    // ... account struct remains the same ...
}

#[inline(always)]  // Add inline attribute to help with stack size
pub fn handler(ctx: Context<RedeemStablecoin>, amount: u64) -> Result<()> {
    let stablecoin_mint = &mut ctx.accounts.stablecoin_mint;
    let vault = &mut ctx.accounts.vault;

    require!(!stablecoin_mint.settings.paused, StableFunError::RedeemingPaused);
    require!(amount > 0, StableFunError::InvalidAmount);
    require!(
        amount <= ctx.accounts.user_token_account.amount,
        StableFunError::InsufficientBalance
    );

    // Use the utils functions directly
    let oracle_price = utils::get_oracle_price(&ctx.accounts.price_feed)?;

    let collateral_amount = utils::calculate_amount(
        amount,
        oracle_price,
        ctx.accounts.token_mint.decimals,
    )?;

    let fee_amount = calculate_fee(amount, stablecoin_mint.settings.fee_basis_points)?;
    let burn_amount = amount.checked_add(fee_amount)
        .ok_or(error!(StableFunError::MathOverflow))?;

    require!(
        collateral_amount <= vault.total_collateral,
        StableFunError::InsufficientCollateral
    );

    let remaining_collateral = vault.total_collateral
        .checked_sub(collateral_amount)
        .ok_or(error!(StableFunError::MathOverflow))?;

    let remaining_supply = stablecoin_mint.current_supply
        .checked_sub(burn_amount)
        .ok_or(error!(StableFunError::MathOverflow))?;

    if remaining_supply > 0 {
        utils::validate_ratio(
            remaining_collateral,
            remaining_supply,
            stablecoin_mint.settings.min_collateral_ratio,
        )?;
    }

    burn_tokens(ctx.accounts, burn_amount)?;
    transfer_collateral(ctx.accounts, collateral_amount)?;
    update_vault_state(vault, amount, collateral_amount)?;
    update_stablecoin_state(stablecoin_mint, remaining_supply, amount, fee_amount)?;

    emit!(RedeemEvent {
        stablecoin_mint: stablecoin_mint.key(),
        user: ctx.accounts.user.key(),
        amount,
        fee_amount,
        collateral_amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

// Helper functions to break down the handler logic
#[inline(always)]
fn calculate_fee(amount: u64, fee_basis_points: u16) -> Result<u64> {
    amount
        .checked_mul(fee_basis_points as u64)
        .and_then(|v| v.checked_div(10000))
        .ok_or(error!(StableFunError::MathOverflow))
}

#[inline(always)]
fn burn_tokens<'info>(
    accounts: &RedeemStablecoin<'info>,
    burn_amount: u64,
) -> Result<()> {
    token::burn(
        CpiContext::new_with_signer(
            accounts.token_program.to_account_info(),
            token::Burn {
                mint: accounts.token_mint.to_account_info(),
                from: accounts.user_token_account.to_account_info(),
                authority: accounts.burn_authority.to_account_info(),
            },
            &[&[
                b"mint-authority",
                accounts.stablecoin_mint.key().as_ref(),
                &[*ctx.bumps.get("burn_authority").unwrap()],
            ]],
        ),
        burn_amount,
    )
}

#[inline(always)]
fn transfer_collateral<'info>(
    accounts: &RedeemStablecoin<'info>,
    collateral_amount: u64,
) -> Result<()> {
    utils::transfer_tokens(
        &accounts.vault_stablebond_account,
        &accounts.user_stablebond_account,
        &accounts.user,
        &accounts.token_program,
        collateral_amount,
    )
}

#[inline(always)]
fn update_vault_state(
    vault: &mut Account<StablecoinVault>,
    amount: u64,
    collateral_amount: u64,
) -> Result<()> {
    vault.total_collateral = vault.total_collateral
        .checked_sub(collateral_amount)
        .ok_or(error!(StableFunError::MathOverflow))?;
    
    vault.total_value_locked = vault.total_value_locked
        .checked_sub(amount)
        .ok_or(error!(StableFunError::MathOverflow))?;
        
    vault.withdrawal_count = vault.withdrawal_count
        .checked_add(1)
        .ok_or(error!(StableFunError::MathOverflow))?;
        
    vault.last_withdrawal_time = Clock::get()?.unix_timestamp;
    vault.update_collateral_ratio()?;
    
    Ok(())
}

#[inline(always)]
fn update_stablecoin_state(
    stablecoin_mint: &mut Account<StablecoinMint>,
    remaining_supply: u64,
    amount: u64,
    fee_amount: u64,
) -> Result<()> {
    stablecoin_mint.current_supply = remaining_supply;
    stablecoin_mint.stats.total_burned = stablecoin_mint.stats.total_burned
        .checked_add(amount)
        .ok_or(error!(StableFunError::MathOverflow))?;
    stablecoin_mint.stats.total_fees = stablecoin_mint.stats.total_fees
        .checked_add(fee_amount)
        .ok_or(error!(StableFunError::MathOverflow))?;
    
    Ok(())
}

// Rest of the code (RedeemEvent and tests) remains the same 