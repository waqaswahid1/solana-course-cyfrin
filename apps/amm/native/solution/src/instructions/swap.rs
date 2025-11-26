use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
};
use spl_token_interface;

use crate::constants;
use crate::state::Pool;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct SwapArgs {
    pub fee: u16,
    pub a_for_b: bool,
    pub amount_in: u64,
    pub min_amount_out: u64,
    pub pool_bump: u8,
}

pub fn swap(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    fee: u16,
    a_for_b: bool,
    amount_in: u64,
    min_amount_out: u64,
    pool_bump: u8,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let pool = next_account_info(accounts_iter)?;
    let mint_a = next_account_info(accounts_iter)?;
    let mint_b = next_account_info(accounts_iter)?;
    let pool_a = next_account_info(accounts_iter)?;
    let pool_b = next_account_info(accounts_iter)?;
    let payer_account_a = next_account_info(accounts_iter)?;
    let payer_account_b = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // Verify payer is signer
    if !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Verify pool PDA
    let expected_pool = Pubkey::create_program_address(
        &[
            constants::POOL_AUTH,
            mint_a.key.as_ref(),
            mint_b.key.as_ref(),
            &fee.to_le_bytes(),
            &[pool_bump],
        ],
        program_id,
    )?;
    if *pool.key != expected_pool {
        return Err(ProgramError::InvalidSeeds);
    }

    // Deserialize and verify pool state
    let pool_data = pool.data.borrow();
    let pool_state = Pool::try_from_slice(&pool_data)?;

    if pool_state.mint_a != *mint_a.key {
        return Err(ProgramError::InvalidAccountData);
    }
    if pool_state.mint_b != *mint_b.key {
        return Err(ProgramError::InvalidAccountData);
    }
    drop(pool_data);

    // Calculate amount out with fee
    let mut amount_out = amount_in;
    let amount_out_fee = amount_out
        .checked_mul(fee as u64)
        .ok_or(ProgramError::ArithmeticOverflow)?
        .checked_div(constants::MAX_POOL_FEE as u64)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    amount_out = amount_out
        .checked_sub(amount_out_fee)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    msg!("Amount in: {}", amount_in);
    msg!("Fee: {}", amount_out_fee);
    msg!("Amount out: {}", amount_out);
    msg!("Min amount out: {}", min_amount_out);

    // Check slippage protection
    if amount_out < min_amount_out {
        msg!(
            "Amount out {} is less than minimum {}",
            amount_out,
            min_amount_out
        );
        return Err(ProgramError::Custom(0)); // MinAmountOut error
    }

    // Determine swap direction
    let (mint_in, mint_out, pool_in, pool_out, payer_in, payer_out) = if a_for_b
    {
        msg!("Swapping A for B");
        (
            mint_a,
            mint_b,
            pool_a,
            pool_b,
            payer_account_a,
            payer_account_b,
        )
    } else {
        msg!("Swapping B for A");
        (
            mint_b,
            mint_a,
            pool_b,
            pool_a,
            payer_account_b,
            payer_account_a,
        )
    };

    // Transfer tokens from payer to pool
    msg!("Transferring {} tokens from payer to pool", amount_in);
    transfer(
        token_program,
        payer_in,
        pool_in,
        mint_in, // ✅ Pass mint_in
        payer,
        amount_in,
    )?;

    // Prepare pool PDA seeds for signed transfer
    let seeds = &[
        constants::POOL_AUTH,
        mint_a.key.as_ref(),
        mint_b.key.as_ref(),
        &fee.to_le_bytes(),
        &[pool_bump],
    ];

    // Transfer tokens from pool to payer
    msg!("Transferring {} tokens from pool to payer", amount_out);
    transfer_from_pool(
        token_program,
        pool_out,
        payer_out,
        mint_out, // ✅ Pass mint_out
        pool,
        amount_out,
        seeds,
    )?;

    Ok(())
}

// Helper function for transfer (user to pool)
fn transfer<'a>(
    token_program: &AccountInfo<'a>,
    from: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    mint: &AccountInfo<'a>, // ✅ Added mint parameter
    authority: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    // ✅ Use transfer_checked for better safety
    let spl_ix = spl_token_interface::instruction::transfer_checked(
        &Address::from(token_program.key.to_bytes()),
        &Address::from(from.key.to_bytes()),
        &Address::from(mint.key.to_bytes()), // ✅ Use actual mint
        &Address::from(to.key.to_bytes()),
        &Address::from(authority.key.to_bytes()),
        &[],
        amount,
        6, // decimals (matches your init)
    )
    .map_err(|_| ProgramError::InvalidInstructionData)?;

    let ix = Instruction {
        program_id: Pubkey::from(spl_ix.program_id.to_bytes()),
        accounts: spl_ix
            .accounts
            .iter()
            .map(|acc| AccountMeta {
                pubkey: Pubkey::from(acc.pubkey.to_bytes()),
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect(),
        data: spl_ix.data,
    };

    invoke(
        &ix,
        &[
            from.clone(),
            mint.clone(), // ✅ Added mint account
            to.clone(),
            authority.clone(),
            token_program.clone(),
        ],
    )
}

// Helper function for transfer from pool (PDA authority)
fn transfer_from_pool<'a>(
    token_program: &AccountInfo<'a>,
    from: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    mint: &AccountInfo<'a>, // ✅ Added mint parameter
    authority: &AccountInfo<'a>,
    amount: u64,
    signer_seeds: &[&[u8]],
) -> ProgramResult {
    // ✅ Use transfer_checked for better safety
    let spl_ix = spl_token_interface::instruction::transfer_checked(
        &Address::from(token_program.key.to_bytes()),
        &Address::from(from.key.to_bytes()),
        &Address::from(mint.key.to_bytes()), // ✅ Use actual mint
        &Address::from(to.key.to_bytes()),
        &Address::from(authority.key.to_bytes()),
        &[],
        amount,
        6, // decimals (matches your init)
    )
    .map_err(|_| ProgramError::InvalidInstructionData)?;

    let ix = Instruction {
        program_id: Pubkey::from(spl_ix.program_id.to_bytes()),
        accounts: spl_ix
            .accounts
            .iter()
            .map(|acc| AccountMeta {
                pubkey: Pubkey::from(acc.pubkey.to_bytes()),
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect(),
        data: spl_ix.data,
    };

    invoke_signed(
        &ix,
        &[
            from.clone(),
            mint.clone(), // ✅ Added mint account
            to.clone(),
            authority.clone(),
            token_program.clone(),
        ],
        &[signer_seeds],
    )
}
