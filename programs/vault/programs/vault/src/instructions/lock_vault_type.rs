use anchor_lang::prelude::*;
use trident_derive_accounts_snapshots::AccountsSnapshots;

// use crate::constants::VAULT_TYPE_PREFIX;
use crate::states::*;

#[derive(Accounts, AccountsSnapshots)]
pub struct LockVaultType<'info> {
    #[account(
        mut,
        // seeds = [VAULT_TYPE_PREFIX, vault_type.mint.key().as_ref(), vault_type.identity.key().as_ref()],
        // bump = vault_type.bump,
        has_one = authority,
    )]
    pub vault_type: Account<'info, VaultType>,
    
    pub authority: Signer<'info>,
}

pub fn lock_vault_type(ctx: Context<LockVaultType>, is_locked: bool) -> Result<()> {
    ctx.accounts.vault_type.is_locked = is_locked;

    Ok(())
}
