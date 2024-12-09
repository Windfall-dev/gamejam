use anchor_lang::prelude::*;
use trident_derive_accounts_snapshots::AccountsSnapshots;

use crate::constants::VAULT_PREFIX;
use crate::states::*;

#[derive(Accounts, AccountsSnapshots)]
pub struct NewVault<'info> {
    #[account(
        init,
        seeds = [VAULT_PREFIX, vault_type.key().as_ref(), user_authority.key().as_ref()],
        bump,
        payer = payer,
        space = 8 + Vault::SIZE,
    )]
    pub vault: Account<'info, Vault>,

    pub vault_type: Account<'info, VaultType>,

    pub user_authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn new_vault(ctx: Context<NewVault>) -> Result<()> {
    let v = &mut ctx.accounts.vault;

    v.user_authority = ctx.accounts.user_authority.key();
    v.vault_type = ctx.accounts.vault_type.key();
    v.amount = 0;
    v.inactive_at = 0;
    v.status = VaultStatus::Inactive;
    v.bump = ctx.bumps.vault;

    Ok(())
}
