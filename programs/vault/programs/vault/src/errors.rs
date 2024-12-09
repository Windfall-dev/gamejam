use anchor_lang::prelude::*;

// These error codes start at 6000 (0x1770).

/// Custom error codes for the Raffle program
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid parameter")]
    InvalidParameter,
    #[msg("Arithmetic error")]
    ArithmeticError,
    #[msg("Invalid status")]
    InvalidStatus,
    #[msg("Deactivation locked")]
    DeactivationLocked,
    #[msg("Deposit remaining")]
    DepositRemaining,
    #[msg("Insufficient deposit")]
    InsufficientDeposit,
    #[msg("Pool remaining")]
    PoolRemaining,
    #[msg("Reserve remaining")]
    ReserveRemaining,
    #[msg("Deposit limit")]
    DepositLimit,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid token program")]
    InvalidTokenProgram,
    #[msg("Invalid account")]
    InvalidAccount,
}
