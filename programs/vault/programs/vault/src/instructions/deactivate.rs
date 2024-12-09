use anchor_lang::prelude::*;
use trident_derive_accounts_snapshots::AccountsSnapshots;

// use crate::constants::VAULT_PREFIX;
use crate::states::*;
use crate::errors::ErrorCode;

#[derive(Accounts, AccountsSnapshots)]
pub struct Deactivate<'info> {
    #[account(
        mut,
        // seeds = [VAULT_PREFIX, vault_type.key().as_ref(), user_authority.key().as_ref()],
        // bump = vault.bump,
        has_one = user_authority,
        has_one = vault_type,
    )]
    pub vault: Account<'info, Vault>,

    pub vault_type: Account<'info, VaultType>,
    
    pub user_authority: Signer<'info>,
}

pub fn deactivate(ctx: Context<Deactivate>) -> Result<()> {
    let v = &mut ctx.accounts.vault;
    let vt = &ctx.accounts.vault_type;

    require!(!vt.is_locked, ErrorCode::DeactivationLocked);

    require!(v.status != VaultStatus::Inactive, ErrorCode::InvalidStatus);

    if vt.instant_deactivation {
        v.status = VaultStatus::Inactive;
        msg!("Vault is now inactive");
    } else {
        let season_end = vt.season_start.checked_add(vt.season_duration).ok_or(ErrorCode::ArithmeticError)?;
        let deactivation_lock_start = season_end.checked_sub(vt.deactivation_lock_window).ok_or(ErrorCode::ArithmeticError)?;
        let now = Clock::get()?.unix_timestamp;
        require!(now < deactivation_lock_start, ErrorCode::DeactivationLocked);

        v.status = VaultStatus::Deactivating;
        v.inactive_at = season_end;
        msg!("Vault can be inactive after {}", v.inactive_at);
    }

    Ok(())
}
