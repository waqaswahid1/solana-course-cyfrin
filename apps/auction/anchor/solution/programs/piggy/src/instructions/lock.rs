use anchor_lang::prelude::*;

use crate::error;
use crate::state;

#[derive(Accounts)]
pub struct Lock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // Validate dst exists and dst approves SOL to be sent later
    #[account(mut)]
    pub dst: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + state::Lock::INIT_SPACE,
        seeds = [state::Lock::SEED_PREFIX, payer.key().as_ref(), dst.key().as_ref()],
        // Calculated off-chain, verified on-chain by this program
        bump,
    )]
    pub lock: Account<'info, state::Lock>,

    pub system_program: Program<'info, System>,
}

pub fn lock(ctx: Context<Lock>, amt: u64, exp: u64) -> Result<()> {
    let clock = Clock::get()?;

    // Require amt > 0
    require!(amt > 0, error::Error::InvalidAmount);

    // Ensure expiration is in the future
    require!(
        exp > u64::try_from(clock.unix_timestamp).unwrap(),
        error::Error::InvalidExpiration
    );

    // Store lock state
    let lock = &mut ctx.accounts.lock;
    lock.dst = ctx.accounts.dst.key();
    lock.exp = exp;

    // Transfer SOL from payer to PDA
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.payer.key(),
        &ctx.accounts.lock.key(),
        amt,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.lock.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    Ok(())
}
