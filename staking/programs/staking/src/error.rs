#[allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Stake Freeze Period Not Over")]
    FreePeriodNotOver,
}
