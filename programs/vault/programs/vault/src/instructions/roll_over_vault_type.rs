use anchor_lang::prelude::*;
use trident_derive_accounts_snapshots::AccountsSnapshots;

use crate::errors::ErrorCode;
use crate::states::*;

#[derive(Accounts, AccountsSnapshots)]
pub struct RollOverVaultType<'info> {
    #[account(mut)]
    pub vault_type: Account<'info, VaultType>,
}

// This instruction is callable by anyone, not just the `VaultType` owner.
// The design enables anyone, not only Windfall operators, to advance to the next season.
// This prevents users from being blocked from withdrawing even if Windfall operators become inactive.

pub fn roll_over_vault_type(ctx: Context<RollOverVaultType>) -> Result<()> {
    let vt = &mut ctx.accounts.vault_type;
    let season_end = vt
        .season_start
        .checked_add(vt.season_duration)
        .ok_or(ErrorCode::ArithmeticError)?;
    let now = Clock::get()?.unix_timestamp;

    if now > season_end {
        vt.season_start = season_end;
    }

    Ok(())
}
