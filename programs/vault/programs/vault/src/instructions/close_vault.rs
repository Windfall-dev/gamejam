use anchor_lang::prelude::*;
use trident_derive_accounts_snapshots::AccountsSnapshots;

// use crate::constants::VAULT_PREFIX;
use crate::states::*;
use crate::errors::ErrorCode;

#[derive(Accounts, AccountsSnapshots)]
pub struct CloseVault<'info> {
    #[account(
        mut,
        close = payer,
        // seeds = [VAULT_PREFIX, vault_type.key().as_ref(), user_authority.key().as_ref()],
        // bump = vault.bump,
        has_one = user_authority,
        has_one = vault_type,
    )]
    pub vault: Account<'info, Vault>,

    pub vault_type: Account<'info, VaultType>,

    pub user_authority: Signer<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
}

pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
    require!(ctx.accounts.vault.amount == 0, ErrorCode::DepositRemaining);

    Ok(())
}
