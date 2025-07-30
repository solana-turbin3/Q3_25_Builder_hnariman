use anchor_lang::prelude::*;
use constant_product_curve::CurveError;

#[error_code]
pub enum AmmError {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Locked")]
    Locked,
    #[msg("Invalid Precision")]
    InvalidPrecision,
    #[msg("Overflow")]
    Overflow,
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("Slippage Exceeded")]
    SlippageExceeded,
}

impl From<CurveError> for AmmError {
    fn from(value: CurveError) -> Self {
        match value {
            CurveError::InvalidPrecision => AmmError::InvalidPrecision,
            CurveError::Overflow => AmmError::Overflow,
            _ => AmmError::CustomError,
        }
    }
}
