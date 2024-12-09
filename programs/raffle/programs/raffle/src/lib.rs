use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::delegate;
use light_sdk::light_program;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

use instructions::*;

declare_id!("UREbNTy4hXkXjdKipxTZtY6xMzw8KKfmN8GPJdSHgRC");

#[delegate]
#[light_program]
#[program]
pub mod raffle {

    use super::*;

    pub fn new_raffle(
        ctx: Context<NewRaffle>,
        start_ts: i64,
        end_ts: i64,
        is_active: bool,
        is_public: bool,
    ) -> Result<()> {
        instructions::new_raffle(ctx, start_ts, end_ts, is_active, is_public)
    }

    pub fn add_prize_to_raffle(
        ctx: Context<AddPrizeToRaffle>,
        payout: u64,
        tickets: u64,
    ) -> Result<()> {
        instructions::add_prize_to_raffle(ctx, payout, tickets)
    }

    pub fn close_raffle(ctx: Context<CloseRaffle>) -> Result<()> {
        instructions::close_raffle(ctx)
    }

    pub fn redeem(
        ctx: Context<Redeem>,
        tickets_allocated: u64,
        proof: Vec<[u8; 32]>,
    ) -> Result<()> {
        instructions::redeem(ctx, tickets_allocated, proof)
    }

    pub fn draw(ctx: Context<Draw>, tickets_to_draw: u64) -> Result<()> {
        instructions::draw(ctx, tickets_to_draw)
    }   

    pub fn delegate(
        ctx: Context<Delegate>,
        valid_until: i64,
        commit_frequency_ms: u32,
    ) -> Result<()> {
        instructions::delegate(ctx, valid_until, commit_frequency_ms)
    }

    pub fn undelegate(ctx: Context<Undelegate>) -> Result<()> {
        instructions::undelegate(ctx)
    }

    // Currently, compress/decompress functions are defined directly here because moving the instruction implementation
    // functions to separate files caused issues (not set at all) with setting content in compressed accounts.
    // This is a temporary solution.

    pub fn compress<'info>(ctx: LightContext<'_, '_, '_, 'info, Compress<'info>>) -> Result<()> {

        // TODO: Verify

        ctx.light_accounts.compressed_user_record.user_authority = ctx.accounts.signer.key();
        ctx.light_accounts.compressed_user_record.raffle = ctx.accounts.raffle.key();
        ctx.light_accounts.compressed_user_record.prizes_won = ctx.accounts.user_record.prizes_won;
        ctx.light_accounts.compressed_user_record.tickets_allocated = ctx.accounts.user_record.tickets_allocated;
        ctx.light_accounts.compressed_user_record.tickets_drawn = ctx.accounts.user_record.tickets_drawn;
        ctx.light_accounts.compressed_user_record.amount_claimed = ctx.accounts.user_record.amount_claimed;

        Ok(())
    }

    pub fn decompress<'info>(
        ctx: LightContext<'_, '_, '_, 'info, Decompress<'info>>,
    ) -> Result<()> {

        // TODO: Verify

        ctx.accounts.user_record.user_authority = ctx.light_accounts.compressed_user_record.user_authority.key();
        ctx.accounts.user_record.raffle = ctx.light_accounts.compressed_user_record.raffle.key();
        ctx.accounts.user_record.prizes_won = ctx.light_accounts.compressed_user_record.prizes_won;
        ctx.accounts.user_record.tickets_allocated = ctx.light_accounts.compressed_user_record.tickets_allocated;
        ctx.accounts.user_record.tickets_drawn = ctx.light_accounts.compressed_user_record.tickets_drawn;
        ctx.accounts.user_record.amount_claimed = ctx.light_accounts.compressed_user_record.amount_claimed;

        Ok(())
    }

    // pub fn compress<'info>(
    //     ctx: LightContext<'_, '_, '_, 'info, Compress<'info>>,
    // ) -> Result<()> {
    //     instructions::compress(&mut ctx)
    // }
    //
    // pub fn decompress<'info>(
    //     ctx: LightContext<'_, '_, '_, 'info, Decompress<'info>>,
    // ) -> Result<()> {
    //     instructions::decompress(&mut ctx)
    // }

    //
    // Instructions to implement
    //
    // Prize pool instructions
    // - new_prize_pool
    // - close_prize_pool
    //
    // Raffle instructions
    // - new_raffle
    // - add_prize_to_raffle
    // - close_raffle
    // - update_raffle
    // - fund_raffle
    //
    // User instructions
    // - redeem
    // - decompress
    // - draw
    // - reroll
    // - claim
    // - compress
    //
    // MagicBlock instructions (maybe internal)
    // - delegate
    // - undelegate
    //
}
