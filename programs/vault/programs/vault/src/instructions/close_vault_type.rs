use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount, TokenInterface};
use trident_derive_accounts_snapshots::AccountsSnapshots;

use crate::constants::VAULT_TYPE_PREFIX;
// use crate::constants::VAULT_TYPE_PREFIX;
use crate::errors::ErrorCode;
use crate::states::*;

#[derive(Accounts, AccountsSnapshots)]
pub struct CloseVaultType<'info> {
    #[account(
        mut,
        close = payer,
        // seeds = [VAULT_TYPE_PREFIX, vault_type.mint.key().as_ref(), vault_type.identity.key().as_ref()],
        // bump = vault_type.bump,
        has_one = authority,
        has_one = pool,
        has_one = reserve,
        has_one = token_program,
    )]
    pub vault_type: Account<'info, VaultType>,

    pub authority: Signer<'info>,

    // #[account(
    //     token::mint = vault_type.mint,
    //     token::authority = vault_type,
    //     token::token_program = vault_type.token_program,
    // )]
    #[account(mut)]
    pub pool: InterfaceAccount<'info, TokenAccount>,
    
    // #[account(
    //     token::mint = vault_type.mint,
    //     token::authority = vault_type,
    //     token::token_program = vault_type.token_program,
    // )]
    #[account(mut)]
    pub reserve: InterfaceAccount<'info, TokenAccount>,
    
    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn close_vault_type(ctx: Context<CloseVaultType>) -> Result<()> {
    require!(ctx.accounts.vault_type.total_deposit == 0, ErrorCode::DepositRemaining);
    require!(ctx.accounts.pool.amount == 0, ErrorCode::PoolRemaining);
    require!(ctx.accounts.reserve.amount == 0, ErrorCode::ReserveRemaining);
    
    let vt = &ctx.accounts.vault_type;    
    let seeds: &[&[&[u8]]] = &[&[VAULT_TYPE_PREFIX, &vt.mint.to_bytes(), &vt.identity.to_bytes(), &[vt.bump]]];

    anchor_spl::token_interface::close_account(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token_interface::CloseAccount {
                account: ctx.accounts.pool.to_account_info(),
                destination: ctx.accounts.payer.to_account_info(),
                authority: ctx.accounts.vault_type.to_account_info(),
            },
        ).with_signer(seeds),
    )?;

    anchor_spl::token_interface::close_account(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token_interface::CloseAccount {
                account: ctx.accounts.reserve.to_account_info(),
                destination: ctx.accounts.payer.to_account_info(),
                authority: ctx.accounts.vault_type.to_account_info(),
            },
        ).with_signer(seeds),
    )?;

    Ok(())
}
