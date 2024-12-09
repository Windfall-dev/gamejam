use anchor_lang::prelude::*;
use trident_derive_accounts_snapshots::AccountsSnapshots;

//use crate::constants::VAULT_TYPE_PREFIX;
use crate::states::*;

#[derive(Accounts, AccountsSnapshots)]
pub struct NominateVaultTypeAuthority<'info> {
    #[account(
        mut,
        // seeds = [VAULT_TYPE_PREFIX, vault_type.mint.key().as_ref(), vault_type.identity.key().as_ref()],
        // bump = vault_type.bump,
        has_one = authority,
    )]
    pub vault_type: Account<'info, VaultType>,

    pub authority: Signer<'info>,

    /// CHECK: This is the new authority that will be nominated and checked by
    /// the `accept_vault_type_authority` instruction.
    pub new_authority: UncheckedAccount<'info>,
}

pub fn nominate_vault_type_authority(ctx: Context<NominateVaultTypeAuthority>) -> Result<()> {
    ctx.accounts.vault_type.pending_authority = Some(ctx.accounts.new_authority.key());

    Ok(())
}
