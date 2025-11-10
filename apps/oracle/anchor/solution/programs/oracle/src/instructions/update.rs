use anchor_lang::prelude::*;

use crate::state::Oracle;

#[derive(Accounts)]
pub struct Update<'info> {
    pub owner: Signer<'info>,
    #[account(mut, constraint = oracle.owner == owner.key())]
    // Check oracle.owner == owner
    // oracle is deserialized to Oracle struct
    pub oracle: Account<'info, Oracle>,
}

pub fn update(ctx: Context<Update>, price: u64) -> Result<()> {
    ctx.accounts.oracle.price = price;
    Ok(())
}
