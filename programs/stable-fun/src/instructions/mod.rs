pub mod initialize;
pub mod mint;
pub mod redeem;
pub mod update;

pub use initialize::*;
pub use mint::*;
pub use redeem::*;
pub use update::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};
use switchboard_v2::AggregatorAccountData;

use crate::utils::switchboard::get_feed_result;
use crate::state::{StablecoinMint, StablecoinVault, StablecoinSettings};
use crate::error::*;

/// Seeds for PDA derivation
pub const STABLECOIN_SEED: &[u8] = b"stablecoin";
pub const VAULT_SEED: &[u8] = b"vault";
pub const MINT_AUTHORITY_SEED: &[u8] = b"mint-authority";

/// Constants for validation
pub const MIN_NAME_LENGTH: usize = 3;
pub const MIN_SYMBOL_LENGTH: usize = 2;
pub const BASIS_POINTS_DIVISOR: u16 = 10000;
pub const DEFAULT_COLLATERAL_RATIO: u16 = 15000; // 150%
pub const MIN_COLLATERAL_RATIO: u16 = 10000; // 100%

/// Helper function to verify oracle price data
pub fn verify_oracle_price(
    oracle_account: &AccountLoader<AggregatorAccountData>,
) -> Result<u64> {
    let oracle_data = oracle_account.load()?;
    let price = get_feed_result(&oracle_data)?;
    
    require!(price > 0, ProgramError::InvalidOraclePrice);
    require!(
        oracle_data.latest_confirmed_round.round_open_timestamp > 0,
        ProgramError::StaleOraclePrice
    );

    Ok(price as u64)
}

/// Helper function to calculate token amounts based on price
pub fn calculate_token_amount(
    amount: u64,
    price: u64,
    decimals: u8,
) -> Result<u64> {
    let scale = 10u64.pow(decimals as u32);
    amount
        .checked_mul(scale)
        .and_then(|a| a.checked_div(price))
        .ok_or(ProgramError::MathOverflow.into())
}

/// Helper function to validate collateral ratio
pub fn validate_collateral_ratio(
    collateral_amount: u64,
    collateral_value: u64,
    min_ratio: u16,
) -> Result<()> {
    let ratio = collateral_value
        .checked_mul(BASIS_POINTS_DIVISOR as u64)
        .and_then(|v| v.checked_div(collateral_amount))
        .ok_or(ProgramError::MathOverflow)?;

    require!(
        ratio >= min_ratio as u64,
        ProgramError::InsufficientCollateral
    );

    Ok(())
}

/// Helper function to transfer tokens
pub fn transfer_tokens<'info>(
    from: &Account<'info, TokenAccount>,
    to: &Account<'info, TokenAccount>,
    authority: &Signer<'info>,
    token_program: &Program<'info, Token>,
    amount: u64,
) -> Result<()> {
    token::transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: from.to_account_info(),
                to: to.to_account_info(),
                authority: authority.to_account_info(),
            },
        ),
        amount,
    )
}

#[error_code]
pub enum ProgramError {
    #[msg("Invalid oracle price")]
    InvalidOraclePrice,
    #[msg("Stale oracle price")]
    StaleOraclePrice,
    #[msg("Math overflow in calculation")]
    MathOverflow,
    #[msg("Insufficient collateral ratio")]
    InsufficientCollateral,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_token_amount() {
        // Test with 6 decimals
        let amount = 1000000; // 1 USDC
        let price = 500000; // $0.50 per token
        let decimals = 6;

        let result = calculate_token_amount(amount, price, decimals).unwrap();
        assert_eq!(result, 2000000); // Should get 2 tokens

        // Test with different decimals
        let amount = 1000000000; // 1 TOKEN
        let price = 2000000; // $2 per token
        let decimals = 9;

        let result = calculate_token_amount(amount, price, decimals).unwrap();
        assert_eq!(result, 500000000); // Should get 0.5 tokens
    }

    #[test]
    fn test_validate_collateral_ratio() {
        // Test valid ratio (150%)
        let result = validate_collateral_ratio(
            1000000, // 1 TOKEN
            1500000, // $1.50 value
            15000,   // 150% minimum
        );
        assert!(result.is_ok());

        // Test invalid ratio (90%)
        let result = validate_collateral_ratio(
            1000000, // 1 TOKEN
            900000,  // $0.90 value
            15000,   // 150% minimum
        );
        assert!(result.is_err());
    }
}