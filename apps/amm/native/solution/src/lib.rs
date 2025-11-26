use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub mod constants;
pub mod instructions;
pub mod state;
use state::Pool;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Cmd {
    Init {
        fee: u16,
        pool_bump: u8,
        mint_pool_bump: u8,
    },
    AddLiquidity {
        fee: u16,
        amount_a: u64,
        amount_b: u64,
        pool_bump: u8,
        mint_pool_bump: u8,
    },
    RemoveLiquidity {
        fee: u16,
        shares: u64,
        min_amount_a: u64,
        min_amount_b: u64,
        pool_bump: u8,
        mint_pool_bump: u8,
    },
    Swap {
        fee: u16,
        a_for_b: bool,
        amount_in: u64,
        min_amount_out: u64,
        pool_bump: u8,
    },
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix = Cmd::try_from_slice(instruction_data)?;

    match ix {
        Cmd::Init {
            fee,
            pool_bump,
            mint_pool_bump, // Added
        } => {
            instructions::init(
                program_id,
                accounts,
                fee,
                pool_bump,
                mint_pool_bump,
            )?;
        }
        Cmd::AddLiquidity {
            fee,
            amount_a,
            amount_b,
            pool_bump,
            mint_pool_bump,
        } => {
            instructions::add_liquidity(
                program_id,
                accounts,
                fee,
                amount_a,
                amount_b,
                pool_bump,
                mint_pool_bump,
            )?;
        }
        Cmd::RemoveLiquidity {
            fee,
            shares,
            min_amount_a,
            min_amount_b,
            pool_bump,
            mint_pool_bump,
        } => {
            instructions::remove_liquidity(
                program_id,
                accounts,
                fee,
                shares,
                min_amount_a,
                min_amount_b,
                pool_bump,
                mint_pool_bump,
            )?;
        }
        Cmd::Swap {
            fee,
            a_for_b,
            amount_in,
            min_amount_out,
            pool_bump,
        } => {
            instructions::swap(
                program_id,
                accounts,
                fee,
                a_for_b,
                amount_in,
                min_amount_out,
                pool_bump,
            )?;
        }
    }

    Ok(())
}
