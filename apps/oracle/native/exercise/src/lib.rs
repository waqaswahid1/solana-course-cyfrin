use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    pubkey::Pubkey,
};

pub mod instructions;
pub mod state;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Cmd {
    Init(Pubkey, u64),
    Update(u64),
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix = Cmd::try_from_slice(instruction_data)?;

    match ix {
        Cmd::Init(owner, price) => {}
        Cmd::Update(price) => {}
    }

    Ok(())
}
