use anchor_lang::prelude::*;

#[error_code]
pub enum MarketError {
    #[msg("Custom error message")]
    CustomError,

    #[msg("Name too long")]
    LongName,
    #[msg("Empty name")]
    EmptyName,
    #[msg("Zero price")]
    ZeroPrice,
}
