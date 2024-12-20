use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

use instructions::*;

declare_id!("vauLT785GULpEsFx9L7AVdCrGvy6tDX2WvWgN5BrftL");

#[program]
pub mod vault {
    use super::*;

    pub fn new_vault_type(
        ctx: Context<NewVaultType>, season_start: i64, season_duration: i64, deactivation_lock_window: i64, cooldown_window: i64, max_deposit_per_user: u64, instant_deactivation: bool,
    ) -> Result<()> {
        instructions::new_vault_type(ctx, season_start, season_duration, deactivation_lock_window, cooldown_window, max_deposit_per_user, instant_deactivation)
    }

    pub fn lock_vault_type(ctx: Context<LockVaultType>, is_locked: bool) -> Result<()> {
        instructions::lock_vault_type(ctx, is_locked)
    }

    pub fn nominate_vault_type_authority(ctx: Context<NominateVaultTypeAuthority>) -> Result<()> {
        instructions::nominate_vault_type_authority(ctx)
    }

    pub fn accept_vault_type_authority(ctx: Context<AcceptVaultTypeAuthority>) -> Result<()> {
        instructions::accept_vault_type_authority(ctx)
    }

    pub fn roll_over_vault_type(ctx: Context<RollOverVaultType>) -> Result<()> {
        instructions::roll_over_vault_type(ctx)
    }

    pub fn transfer_vault_type_token(ctx: Context<TransferVaultTypeToken>, amount: u64) -> Result<()> {
        instructions::transfer_vault_type_token(ctx, amount)
    }

    pub fn close_vault_type(ctx: Context<CloseVaultType>) -> Result<()> {
        instructions::close_vault_type(ctx)
    }

    pub fn new_vault(ctx: Context<NewVault>) -> Result<()> {
        instructions::new_vault(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)
    }

    pub fn activate(ctx: Context<Activate>) -> Result<()> {
        instructions::activate(ctx)
    }

    pub fn deactivate(ctx: Context<Deactivate>) -> Result<()> {
        instructions::deactivate(ctx)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }

    pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
        instructions::close_vault(ctx)
    }
}
