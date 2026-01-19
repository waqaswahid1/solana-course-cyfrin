use anchor_lang::prelude::*;

// Specifies the on-chain address of the program
// If your deployment has a different program ID than the one in your Rust code,
// Anchor’s runtime will reject deployment
// Same public key as
// solana address -k target/deploy/hello-keypair.json
declare_id!("1ZQgkLKuc8wDctkAhbbD7jjQCiFvPHkLBHc16W7hPDx");

#[program]
pub mod hello {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?} {:?}", ctx.program_id, crate::ID);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
