use anchor_lang::prelude::*;
use trident_derive_accounts_snapshots::AccountsSnapshots;

//use crate::constants::VAULT_TYPE_PREFIX;
use crate::states::*;

#[derive(Accounts, AccountsSnapshots)]
pub struct AcceptVaultTypeAuthority<'info> {
    #[account(
        mut,
        // seeds = [VAULT_TYPE_PREFIX, vault_type.mint.key().as_ref(), vault_type.identity.key().as_ref()],
        // bump = vault_type.bump,
        constraint = vault_type.pending_authority == Some(new_authority.key()),
    )]
    pub vault_type: Account<'info, VaultType>,

    pub new_authority: Signer<'info>,
}

pub fn accept_vault_type_authority(ctx: Context<AcceptVaultTypeAuthority>) -> Result<()> {
    ctx.accounts.vault_type.authority = ctx.accounts.new_authority.key();
    ctx.accounts.vault_type.pending_authority = None;

    Ok(())
}
