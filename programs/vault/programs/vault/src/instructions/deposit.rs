use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked};
use trident_derive_accounts_snapshots::AccountsSnapshots;

// use crate::constants::{VAULT_PREFIX, VAULT_TYPE_PREFIX};
use crate::errors::ErrorCode;
use crate::states::*;

#[derive(Accounts, AccountsSnapshots)]
pub struct Deposit<'info> {
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
        constraint = from.key() != pool.key() @ ErrorCode::InvalidAccount,
        token::mint = mint,
        token::authority = user_authority,
        token::token_program = token_program,
    )]
    pub from: InterfaceAccount<'info, TokenAccount>,
    
    //#[account(address = vault_type.token_program)]
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let v = &mut ctx.accounts.vault;
    let vt = &mut ctx.accounts.vault_type;

    require!(amount > 0, ErrorCode::InvalidAmount);

    v.amount = v
        .amount
        .checked_add(amount)
        .ok_or(ErrorCode::ArithmeticError)?;

    require!(
        vt.max_deposit_per_user == 0 || vt.max_deposit_per_user >= v.amount,
        ErrorCode::DepositLimit
    );

    vt.total_deposit = vt
        .total_deposit
        .checked_add(amount)
        .ok_or(ErrorCode::ArithmeticError)?;

    let cpi_accounts = TransferChecked {
        from: ctx.accounts.from.to_account_info(),
        to: ctx.accounts.pool.to_account_info(),
        authority: ctx.accounts.user_authority.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    transfer_checked(cpi_ctx, amount, ctx.accounts.mint.decimals)?;

    v.status = VaultStatus::Active;

    Ok(())
}
