use crate::{
    constants::{MAX_PRIZES, RAFFLE_PREFIX, USER_RECORD_PREFIX},
    errors::ErrorCode,
    instructions::wyhash::Wyhash,
    states::*,
};
use anchor_lang::solana_program::{
    instruction::Instruction,
    program::{get_return_data, invoke},
};
use anchor_lang::{prelude::*, solana_program::sysvar};
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::sysvar::instructions::get_instruction_relative;

pub const ENTROPY_ACCOUNT_ADDRESS: Pubkey = pubkey!("CTyyJKQHo6JhtVYBaXcota9NozebV3vHF872S8ag2TUS");
pub const FEE_ACCOUNT_ADDRESS: Pubkey = pubkey!("WjtcArL5m5peH8ZmAdTtyFF9qjyNxjQ2qp4Gz1YEQdy");
pub const RNG_PROGRAM_ADDRESS: Pubkey = pubkey!("FEED1qspts3SRuoEyG29NMNpsTKX8yG9NGMinNC4GeYB");

#[derive(Accounts)]
pub struct Draw<'info> {
    #[account(
        mut,
        seeds = [
            USER_RECORD_PREFIX,
            raffle.key().as_ref(),
            user_authority.key().as_ref(),
        ],
        bump = user_record.bump,
        has_one = raffle,
        has_one = user_authority,
    )]
    pub user_record: Account<'info, UserRecord>,

    pub user_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            RAFFLE_PREFIX,
            raffle.authority.as_ref(),
            raffle.name[..raffle.name_length as usize].as_ref(),
        ],
        bump = raffle.bump,
    )]
    pub raffle: Account<'info, Raffle>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = raffle,
        associated_token::token_program = raffle.token_program,
    )]
    pub pool: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = to.key() != pool.key() @ ErrorCode::InvalidAccount,
        token::mint = mint,
        token::authority = user_authority,
        token::token_program = token_program,
    )]
    pub to: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: This is a constant address of the entropy account
    #[account(mut, address = ENTROPY_ACCOUNT_ADDRESS)]
    pub entropy: UncheckedAccount<'info>,

    /// CHECK: This is a constant address of the fee account
    #[account(mut, address = FEE_ACCOUNT_ADDRESS)]
    pub fee: UncheckedAccount<'info>,

    /// CHECK: This is a constant address of the feed protocolRNG program
    #[account(address = RNG_PROGRAM_ADDRESS)]
    pub rng_program: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(address = raffle.token_program)]
    pub token_program: Interface<'info, TokenInterface>,

    /// Instructions sysvar account.
    /// CHECK: account constraints checked by pubkey
    #[account(address = sysvar::instructions::id())]
    sysvar_instructions: UncheckedAccount<'info>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RandomNumber {
    pub random_number: u64,
}

// `get_value` method call will fail when passing accounts cloned from other networks on localnet,
// so we need to implement different initialization method for localnet.

fn initialize_seed(ctx: &Context<Draw>) -> Result<u64> {
    let instruction: Instruction = Instruction {
        program_id: ctx.accounts.rng_program.key(),
        accounts: vec![
            ctx.accounts.payer.to_account_metas(Some(true))[0].clone(),
            ctx.accounts.entropy.to_account_metas(Some(false))[0].clone(),
            ctx.accounts.fee.to_account_metas(Some(false))[0].clone(),
            ctx.accounts.system_program.to_account_metas(Some(false))[0].clone(),
        ],
        data: vec![100],
    };

    let account_infos: &[AccountInfo; 4] = &[
        ctx.accounts.payer.to_account_info().clone(),
        ctx.accounts.entropy.to_account_info().clone(),
        ctx.accounts.fee.to_account_info().clone(),
        ctx.accounts.system_program.to_account_info().clone(),
    ];

    //CPI to RNG_PROGRAM
    invoke(&instruction, account_infos)?;

    let returned_data: (Pubkey, Vec<u8>) = get_return_data().unwrap();

    //Random number is returned from the RNG_PROGRAM
    let random_number: RandomNumber;
    if returned_data.0.eq(&ctx.accounts.rng_program.key()) {
        random_number = RandomNumber::try_from_slice(&returned_data.1)?;
        msg!("{}", random_number.random_number);
        return Ok(random_number.random_number);
    }
    return Err(ErrorCode::RandomNumberFailure.into());
}

// #[cfg(not(any(feature = "mainnet", feature = "devnet")))]
// fn initialize_seed(_ctx: &Context<Draw>) -> Result<u64> {
//     return Ok(Clock::get()?.unix_timestamp.unsigned_abs());
// }

pub fn draw(ctx: Context<Draw>, tickets_to_draw: u64) -> Result<()> {
    let raffle = &ctx.accounts.raffle;
    let ix_sysvar = &ctx.accounts.sysvar_instructions;

    // Check if the instruction is the last one in the transaction
    if get_instruction_relative(1, ix_sysvar).is_ok() {
        return Err(ErrorCode::NotLastInstruction.into());
    }

    // Check if the program id of the instruction is the raffle program
    let ix =
        get_instruction_relative(0, ix_sysvar).map_err(|_| ErrorCode::UnexpectedInstruction)?;
    if ix.program_id != crate::ID {
        return Err(ErrorCode::UnexpectedInstruction.into());
    }

    let rn = initialize_seed(&ctx).map_err(|_| ErrorCode::RandomNumberFailure)?;
    msg!("seed: {}", rn);

    let mut wyhash = Wyhash::new(rn);

    // Verify raffle status and timestamps
    require!(raffle.is_active, ErrorCode::RaffleInactive);
    raffle.check_timestamp(Clock::get()?.unix_timestamp)?;

    let raffle = &mut ctx.accounts.raffle;
    let ur = &mut ctx.accounts.user_record;

    // Verify other conditions related to the number of remaining tickets
    let new_tickets_drawn = ur
        .tickets_drawn
        .checked_add(tickets_to_draw)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    require!(
        new_tickets_drawn <= ur.tickets_allocated,
        ErrorCode::InsufficientTickets
    );
    require!(
        !raffle.is_public || new_tickets_drawn <= raffle.max_tickets_per_user,
        ErrorCode::InsufficientTickets
    );

    require!(
        tickets_to_draw <= raffle.remaining_tickets,
        ErrorCode::InsufficientTickets
    );

    let mut payout = 0u64;
    // Draw as many tickets as possible
    for _ in 0..tickets_to_draw {
        let rn = wyhash.next();
        msg!("wyhash random number: {}", rn);
        let mut ticket = rn % raffle.remaining_tickets;
        msg!(
            "ticket: {}, total tickets: {}",
            ticket,
            raffle.remaining_tickets
        );

        for index in 0..MAX_PRIZES {
            if ticket < raffle.prizes[index].tickets {
                raffle.prizes[index].tickets = raffle.prizes[index]
                    .tickets
                    .checked_sub(1)
                    .ok_or(ErrorCode::ArithmeticOverflow)?;
                raffle.remaining_tickets = raffle
                    .remaining_tickets
                    .checked_sub(1)
                    .ok_or(ErrorCode::ArithmeticOverflow)?;
                // if payout == 0, it's a miss so don't count it as a win
                if raffle.prizes[index].payout > 0 {
                    ur.prizes_won = ur
                        .prizes_won
                        .checked_add(1)
                        .ok_or(ErrorCode::ArithmeticOverflow)?;
                    payout = payout
                        .checked_add(raffle.prizes[index].payout)
                        .ok_or(ErrorCode::ArithmeticOverflow)?;
                    msg!(
                        "prize {} (remaining {}) won [{}], total remaining {}",
                        index,
                        raffle.prizes[index].tickets,
                        ur.prizes_won,
                        raffle.remaining_tickets,
                    );
                }
                break;
            } else {
                ticket = ticket
                    .checked_sub(raffle.prizes[index].tickets)
                    .ok_or(ErrorCode::ArithmeticOverflow)?;
            }
        }

        ur.tickets_drawn = ur
            .tickets_drawn
            .checked_add(1)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
    }

    if payout > 0 {
        let seeds: &[&[&[u8]]] = &[&[
            RAFFLE_PREFIX,
            raffle.authority.as_ref(),
            raffle.name[..raffle.name_length as usize].as_ref(),
            &[raffle.bump],
        ]];
        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.pool.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: raffle.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
            )
            .with_signer(seeds),
            payout,
            ctx.accounts.mint.decimals,
        )?;

        ur.amount_claimed = ur
            .amount_claimed
            .checked_add(payout)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
    }

    Ok(())
}
