use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::state::Oracle;

pub fn init(
    accounts: &[AccountInfo],
    owner: Pubkey,
    price: u64,
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();
    let oracle_account = next_account_info(account_info_iter)?;

    let mut data = oracle_account.data.borrow_mut();
    let mut oracle = Oracle::try_from_slice(&data)?;

    // Write your code here

    oracle.serialize(&mut &mut data[..])?;

    Ok(())
}
