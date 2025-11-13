use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

declare_id!("AZbizcSksZLiaKoFMB5hTJGtKUf6BxhyKVd2Ffdd41Ym");

#[program]
pub mod piggy {
    pub use super::instructions::*;
    use super::*;

    pub fn lock(ctx: Context<Lock>, amt: u64, exp: u64) -> Result<()> {
        instructions::lock(ctx, amt, exp)?;
        Ok(())
    }

    pub fn unlock(ctx: Context<Unlock>) -> Result<()> {
        instructions::unlock(ctx)?;
        Ok(())
    }
}
