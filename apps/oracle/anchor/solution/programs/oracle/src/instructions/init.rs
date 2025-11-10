use anchor_lang::prelude::*;

use crate::state::Oracle;

#[derive(Accounts)]
pub struct Init<'info> {
    // Order of fields in the struct is the order of accounts the client must pass them in
    // mut - Program can update account's data or lamports
    #[account(mut)]
    // Payer signed this transaction
    pub payer: Signer<'info>,
    // Owner signed this transaction
    pub owner: Signer<'info>,
    #[account(
        // Initialize new account
        // Transaction fails if this account is already initialized
        init,
        // 8 = Anchor discriminator
        space = 8 + Oracle::INIT_SPACE,
        payer = payer
    )]
    pub oracle: Account<'info, Oracle>,
    // Required to create oracle account
    pub system_program: Program<'info, System>,
}

pub fn init(ctx: Context<Init>, price: u64) -> Result<()> {
    let oracle = &mut ctx.accounts.oracle;
    oracle.owner = ctx.accounts.owner.key();
    oracle.price = price;
    Ok(())
}
