use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

declare_id!("C8ft4mixLcvxcum1JiMMi8SLR8muASoEKvsQG8XQf7JJ");

#[program]
pub mod amm {
    pub use super::instructions::*;
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>, fee: u16) -> Result<()> {
        // Write your code here
        Ok(())
    }

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        fee: u16,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<()> {
        // Write your code here
        Ok(())
    }

    pub fn remove_liquidity(
        ctx: Context<RemoveLiquidity>,
        fee: u16,
        shares: u64,
        min_amount_a: u64,
        min_amount_b: u64,
    ) -> Result<()> {
        // Write your code here
        Ok(())
    }

    pub fn swap(
        ctx: Context<Swap>,
        fee: u16,
        a_for_b: bool,
        amount_in: u64,
        min_amount_out: u64,
    ) -> Result<()> {
        // Write your code here
        Ok(())
    }
}
