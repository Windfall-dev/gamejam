use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface, TokenAccount, transfer_checked, TransferChecked};
use trident_derive_accounts_snapshots::AccountsSnapshots;

use crate::errors::ErrorCode;
use crate::constants::VAULT_TYPE_PREFIX;
use crate::states::*;

#[derive(Accounts, AccountsSnapshots)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        // seeds = [VAULT_PREFIX, vault_type.key().as_ref(), user_authority.key().as_ref()],
        // bump = vault.bump,
        has_one = user_authority,
        has_one = vault_type,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        // seeds = [VAULT_TYPE_PREFIX, vault_type.mint.key().as_ref(), vault_type.identity.key().as_ref()],
        // bump = vault_type.bump,
        has_one = mint,
        has_one = pool,
        has_one = reserve,
        has_one = token_program,
    )]
    pub vault_type: Account<'info, VaultType>,

    pub user_authority: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = vault_type,
        token::token_program = token_program,
    )]
    pub pool: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = vault_type,
        token::token_program = token_program,
    )]
    pub reserve: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = to.key() != pool.key() @ ErrorCode::InvalidAccount,
        constraint = to.key() != reserve.key() @ ErrorCode::InvalidAccount,
        token::mint = mint,
        token::authority = user_authority,
        token::token_program = token_program,
    )]
    pub to: InterfaceAccount<'info, TokenAccount>,
    
    // #[account(address = vault_type.token_program)]
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let v = &mut ctx.accounts.vault;
    let vt = &mut ctx.accounts.vault_type;

    if v.status == VaultStatus::Deactivating {
        let now = Clock::get()?.unix_timestamp;
        msg!("Vault is now inactive (inactive_at: {}, now: {})", v.inactive_at, now);
        if v.inactive_at <= now {
            v.status = VaultStatus::Inactive;
        }

        // TODO: crank season as well?
    }

    require!(amount > 0, ErrorCode::InvalidAmount);
    require!(v.amount >= amount, ErrorCode::InsufficientDeposit);
    require!(v.status == VaultStatus::Inactive, ErrorCode::InvalidStatus);

    v.amount = v.amount.checked_sub(amount).ok_or(ErrorCode::ArithmeticError)?;
    vt.total_deposit = vt.total_deposit.checked_sub(amount).ok_or(ErrorCode::ArithmeticError)?;

    let seeds: &[&[&[u8]]] = &[&[
        VAULT_TYPE_PREFIX,
        &vt.mint.to_bytes(),
        &vt.identity.to_bytes(),
        &[vt.bump],
    ]];

    // if reserve is insufficient, supply from pool
    if ctx.accounts.reserve.amount < amount {
        let shortfall = amount.checked_sub(ctx.accounts.reserve.amount).ok_or(ErrorCode::ArithmeticError)?;
        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.pool.to_account_info(),
                    to: ctx.accounts.reserve.to_account_info(),
                    authority: ctx.accounts.vault_type.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
            )
            .with_signer(seeds),
            shortfall,
            ctx.accounts.mint.decimals,
        )?;
    }

    // all user withdrawals are from reserve account
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.reserve.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
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
