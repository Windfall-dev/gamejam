use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked};
use trident_derive_accounts_snapshots::AccountsSnapshots;

use crate::errors::ErrorCode;
use crate::constants::VAULT_TYPE_PREFIX;
use crate::states::*;

#[derive(Accounts, AccountsSnapshots)]
pub struct TransferVaultTypeToken<'info> {
    #[account(
        // seeds = [VAULT_TYPE_PREFIX, vault_type.mint.key().as_ref(), vault_type.identity.key().as_ref()],
        // bump = vault_type.bump,
        has_one = authority,
        has_one = mint,
        has_one = token_program
    )]
    pub vault_type: Account<'info, VaultType>,

    pub authority: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        // token::mint = mint,
        // token::authority = vault_type,
        // token::token_program = token_program,
        constraint = (
            source.key() == vault_type.pool ||
            source.key() == vault_type.reserve
        ) @ ErrorCode::InvalidAccount
    )]
    pub source: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = destination.key() != source.key() @ ErrorCode::InvalidAccount,
        token::mint = mint,
        token::token_program = token_program,
    )]
    pub destination: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn transfer_vault_type_token(ctx: Context<TransferVaultTypeToken>, amount: u64) -> Result<()> {
    let vt = &ctx.accounts.vault_type;

    require!(amount <= vt.total_deposit, ErrorCode::InvalidParameter);
    require!(amount <= ctx.accounts.source.amount, ErrorCode::InvalidAmount);

    let seeds: &[&[&[u8]]] = &[&[VAULT_TYPE_PREFIX, &vt.mint.to_bytes(), &vt.identity.to_bytes(), &[vt.bump]]];
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.source.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.vault_type.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
        )
        .with_signer(seeds),
        amount,
        ctx.accounts.mint.decimals,
    )?;

    Ok(())
}
