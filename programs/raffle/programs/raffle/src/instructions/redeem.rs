use anchor_lang::prelude::*;
use crate::states::*;

#[derive(Accounts)]
pub struct Redeem<'info> {
    pub user_authority: Signer<'info>,
    #[account(mut)]
    pub raffle: Account<'info, Raffle>,

    #[account(
        init,
        seeds = [b"raffle", raffle.key().as_ref(), user_authority.key().as_ref()],
        bump,
        payer = payer,
        space = 8 + 32 + 32 + 8 + 8 + 8 + 8,
    )]
    pub user_record: Account<'info, UserRecord>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn redeem(
    ctx: Context<Redeem>,
    tickets_allocated: u64,
    proof: Vec<[u8; 32]>,
) -> Result<()> {
    let raffle = &ctx.accounts.raffle;

    // Verify raffle status and timestamps
    require!(raffle.is_active, ErrorCode::RaffleInactive);
    raffle.check_timestamp(Clock::get()?.unix_timestamp)?;

    if !raffle.is_public {
        // Verify the number of tickets using a Merkle tree
        let node = hashv(&[
            &[0u8],
            &hashv(&[&ctx.accounts.user_authority.key().to_bytes()]).to_bytes(),
            &tickets_allocated.to_le_bytes(),
        ]);

        require!(
            verify(proof, ctx.accounts.raffle.merkle_root, node.to_bytes()),
            ErrorCode::InvalidProof
        );
    } else {
        // Verify the number of tickets allowed per user
        require!(
            raffle.max_tickets_per_user >= tickets_allocated,
            ErrorCode::InvalidNumTickets
        );
    }

    let ur = &mut ctx.accounts.user_record;
    ur.tickets_allocated = tickets_allocated;
    ur.user_authority = ctx.accounts.user_authority.key();
    ur.raffle = raffle.key();
    ur.bump = ctx.bumps.user_record;

    Ok(())
}
