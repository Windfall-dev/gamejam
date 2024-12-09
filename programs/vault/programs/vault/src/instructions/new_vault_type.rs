use anchor_lang::prelude::*;
use anchor_spl::token::spl_token;
use anchor_spl::token_interface::{spl_token_2022, Mint, TokenAccount, TokenInterface};
use trident_derive_accounts_snapshots::AccountsSnapshots;

use crate::constants::VAULT_TYPE_PREFIX;
use crate::errors::ErrorCode;
use crate::states::*;

// By deriving the vault type PDA from the `mint` and `authority`, different projects besides Windfall
// can use the same mint. Windfall plans to provide infrastructure to other projects in the future.
//
// Since `authority` is changeable, its initial value is also copied to the `identity` field,
// which will not be changed afterwards. 
//
// If you need to verify PDA derivation, refer to the `mint` and `identity` fields.

#[derive(Accounts, AccountsSnapshots)]
pub struct NewVaultType<'info> {
    #[account(
        init,
        seeds = [VAULT_TYPE_PREFIX, mint.key().as_ref(), authority.key().as_ref()],
        bump,
        payer = payer,
        space = 8 + VaultType::SIZE,
    )]
    pub vault_type: Account<'info, VaultType>,

    pub authority: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        token::mint = mint,
        token::authority = vault_type,
        token::token_program = token_program,
    )]
    pub pool: InterfaceAccount<'info, TokenAccount>,

    #[account(
        token::mint = mint,
        token::authority = vault_type,
        token::token_program = token_program,
        constraint = reserve.key() != pool.key() @ ErrorCode::InvalidAccount,
    )]
    pub reserve: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(
        constraint = (
            token_program.key() == spl_token::ID ||
            token_program.key() == spl_token_2022::ID
        ) @ ErrorCode::InvalidTokenProgram
    )]
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn new_vault_type(
    ctx: Context<NewVaultType>,
    season_start: i64,
    season_duration: i64,
    deactivation_lock_window: i64,
    cooldown_window: i64,
    max_deposit_per_user: u64,
    instant_deactivation: bool,
) -> Result<()> {
    require!(
        season_start > 0
            && season_duration > 0
            && deactivation_lock_window >= 0
            && cooldown_window >= 0,
        ErrorCode::InvalidParameter
    );
    require!(season_duration > deactivation_lock_window, ErrorCode::InvalidParameter);
    require!(season_duration > cooldown_window, ErrorCode::InvalidParameter);

    let vt = &mut ctx.accounts.vault_type;

    vt.identity = ctx.accounts.authority.key();
    vt.authority = ctx.accounts.authority.key();
    vt.mint = ctx.accounts.mint.key();
    vt.pool = ctx.accounts.pool.key();
    vt.reserve = ctx.accounts.reserve.key();
    vt.token_program = ctx.accounts.token_program.key();
    vt.season_start = season_start;
    vt.season_duration = season_duration;
    vt.deactivation_lock_window = deactivation_lock_window;
    vt.cooldown_window = cooldown_window;
    vt.max_deposit_per_user = max_deposit_per_user;
    vt.instant_deactivation = instant_deactivation;
    vt.is_locked = false;
    vt.bump = ctx.bumps.vault_type;

    Ok(())
}
