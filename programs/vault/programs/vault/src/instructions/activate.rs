use anchor_lang::prelude::*;
use trident_derive_accounts_snapshots::AccountsSnapshots;

// use crate::constants::VAULT_PREFIX;
use crate::states::*;

#[derive(Accounts, AccountsSnapshots)]
pub struct Activate<'info> {
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

pub fn activate(ctx: Context<Activate>) -> Result<()> {
    ctx.accounts.vault.status = VaultStatus::Active;

    Ok(())
}
