use anchor_lang::prelude::*;

#[error_code]
pub enum StableFunError {
    #[msg("Name must be at least 3 characters")]
    NameTooShort,
    
    #[msg("Symbol must be at least 2 characters")]
    SymbolTooShort,
    
    #[msg("Invalid name provided")]
    InvalidName,
    
    #[msg("Invalid symbol provided")]
    InvalidSymbol,
    
    #[msg("Invalid currency specified")]
    InvalidCurrency,
    
    #[msg("Invalid amount")]
    InvalidAmount,
    
    #[msg("Insufficient collateral")]
    InsufficientCollateral,
    
    #[msg("Invalid oracle price")]
    InvalidOraclePrice,
    
    #[msg("Stale oracle price")]
    StaleOraclePrice,
    
    #[msg("Math overflow in calculation")]
    MathOverflow,
    
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    
    #[msg("Invalid vault account")]
    InvalidVault,
    
    #[msg("Maximum supply exceeded")]
    MaxSupplyExceeded,
    
    #[msg("Insufficient balance")]
    InsufficientBalance,
    
    #[msg("Account owner mismatch")]
    AccountOwnerMismatch,
    
    #[msg("Invalid oracle account")]
    InvalidOracle,
    
    #[msg("Invalid mint account")]
    InvalidMint,
    
    #[msg("Invalid stablebond account")]
    InvalidStablebond,
    
    #[msg("Invalid vault token account")]
    InvalidVaultAccount,
    
    #[msg("Minting is paused")]
    MintingPaused,
    
    #[msg("Redeeming is paused")]
    RedeemingPaused,
    
    #[msg("Collateral ratio too low")]
    CollateralRatioTooLow,
    
    #[msg("Collateral ratio too high")]
    CollateralRatioTooHigh,
    
    #[msg("Fee too high")]
    FeeTooHigh,
    
    #[msg("Amount too small")]
    AmountTooSmall,
    
    #[msg("Amount too large")]
    AmountTooLarge,
    
    #[msg("Invalid token owner")]
    InvalidTokenOwner,
    
    #[msg("Empty vault")]
    EmptyVault,
}