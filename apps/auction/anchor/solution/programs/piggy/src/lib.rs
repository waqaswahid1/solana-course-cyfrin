use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

declare_id!("Hp6iqFudQ9vr2Rz9cdXTk2gCHf4eu6Zr8jLyWe6vsPiL");

#[program]
pub mod auction {
    pub use super::instructions::*;
    use super::*;

    // init
    // - create PDA, lock sell token
    // - state (sell token, buy token, start price, min price, start time, end time, seller)
    // buy
    // - close PDA
    // - refund seller (buy token + PDA rent)
    // - send sell token to buyer
    // cancel
    // - close PDA
    // - refund seller (token + PDA rent)

    pub fn lock(ctx: Context<Lock>, amt: u64, exp: u64) -> Result<()> {
        instructions::lock(ctx, amt, exp)?;
        Ok(())
    }

    pub fn unlock(ctx: Context<Unlock>) -> Result<()> {
        instructions::unlock(ctx)?;
        Ok(())
    }
}
