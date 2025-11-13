use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{Sysvar, clock::Clock, rent::Rent},
};

use super::lib::get_pda;
use crate::state::Lock;

pub fn lock(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amt: u64,
    exp: u64,
    bump: u8,
) -> Result<(), ProgramError> {
    let account_iter = &mut accounts.iter();
    let payer = next_account_info(account_iter)?;
    let dst = next_account_info(account_iter)?;
    let pda = next_account_info(account_iter)?;
    let sys_program = next_account_info(account_iter)?;

    // Check dst signed, verifies dst exists and approved to later receive SOL
    if !dst.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check that the given account key matches expected PDA
    if *pda.key != get_pda(program_id, payer.key, &dst.key, bump)? {
        return Err(ProgramError::InvalidSeeds);
    }

    // Check amt > 0
    if amt == 0 {
        return Err(ProgramError::InvalidArgument);
    }

    // Verify expiration is in the future
    let clock = Clock::get()?;
    if exp <= clock.unix_timestamp.try_into().unwrap() {
        return Err(ProgramError::InvalidArgument);
    }

    // Don't use std::mem::size_of (adds padding)
    // let space = std::mem::size_of::<Lock>();
    // 32 + 8
    let space = 40;
    let rent = Rent::get()?.minimum_balance(space);

    // Create PDA account
    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            pda.key,
            rent,
            space as u64,
            program_id,
        ),
        &[payer.clone(), pda.clone(), sys_program.clone()],
        &[&[b"lock", payer.key.as_ref(), dst.key.as_ref(), &[bump]]],
    )?;

    // Transfer SOL from payer to PDA
    invoke(
        &system_instruction::transfer(payer.key, pda.key, amt),
        &[payer.clone(), pda.clone(), sys_program.clone()],
    )?;

    // Create and save lock state into PDA data
    let mut data = pda.data.borrow_mut();
    let lock = Lock { dst: *dst.key, exp };
    lock.serialize(&mut &mut data[..])?;

    msg!("Lock created: amt={}, exp={}", amt, exp);

    Ok(())
}
