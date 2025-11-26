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
use solana_program_pack::Pack;
use spl_token_interface;

use crate::constants;
use crate::state::Pool;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct RemoveLiquidityArgs {
    pub fee: u16,
    pub shares: u64,
    pub min_amount_a: u64,
    pub min_amount_b: u64,
}

pub fn remove_liquidity(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    fee: u16,
    shares: u64,
    min_amount_a: u64,
    min_amount_b: u64,
    pool_bump: u8,
    mint_pool_bump: u8,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let pool = next_account_info(accounts_iter)?;
    let mint_a = next_account_info(accounts_iter)?;
    let mint_b = next_account_info(accounts_iter)?;
    let pool_a = next_account_info(accounts_iter)?;
    let pool_b = next_account_info(accounts_iter)?;
    let mint_pool = next_account_info(accounts_iter)?;
    let payer_account_a = next_account_info(accounts_iter)?;
    let payer_account_b = next_account_info(accounts_iter)?;
    let payer_account_liquidity = next_account_info(accounts_iter)?;
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

    // Verify mint_pool PDA
    let expected_mint_pool = Pubkey::create_program_address(
        &[
            constants::POOL_MINT,
            mint_a.key.as_ref(),
            mint_b.key.as_ref(),
            &fee.to_le_bytes(),
            &[mint_pool_bump],
        ],
        program_id,
    )?;
    if *mint_pool.key != expected_mint_pool {
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

    // Get pool token amounts
    let pool_a_data = pool_a.data.borrow();
    let pool_a_account =
        spl_token_interface::state::Account::unpack(&pool_a_data).unwrap();
    let pool_a_amount = pool_a_account.amount;
    drop(pool_a_data);

    let pool_b_data = pool_b.data.borrow();
    let pool_b_account =
        spl_token_interface::state::Account::unpack(&pool_b_data).unwrap();
    let pool_b_amount = pool_b_account.amount;
    drop(pool_b_data);

    // Get mint supply
    let mint_pool_data = mint_pool.data.borrow();
    let mint_pool_account =
        spl_token_interface::state::Mint::unpack(&mint_pool_data).unwrap();
    let supply = mint_pool_account.supply;
    drop(mint_pool_data);

    // Calculate amounts to withdraw
    // amount_a = shares * pool_a_amount / supply
    // amount_b = shares * pool_b_amount / supply
    let amount_a = shares
        .checked_mul(pool_a_amount)
        .ok_or(ProgramError::ArithmeticOverflow)?
        .checked_div(supply)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    let amount_b = shares
        .checked_mul(pool_b_amount)
        .ok_or(ProgramError::ArithmeticOverflow)?
        .checked_div(supply)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    msg!("Shares to burn: {}", shares);
    msg!("Amount A to withdraw: {}", amount_a);
    msg!("Amount B to withdraw: {}", amount_b);

    // Check minimum amounts (slippage protection)
    if amount_a < min_amount_a {
        msg!(
            "Amount A {} is less than minimum {}",
            amount_a,
            min_amount_a
        );
        return Err(ProgramError::Custom(0)); // MinAmountOut error
    }

    if amount_b < min_amount_b {
        msg!(
            "Amount B {} is less than minimum {}",
            amount_b,
            min_amount_b
        );
        return Err(ProgramError::Custom(0)); // MinAmountOut error
    }

    // Burn LP tokens from payer
    msg!("Burning {} LP tokens from payer", shares);
    burn(
        token_program,
        mint_pool,
        payer_account_liquidity,
        payer,
        shares,
    )?;

    // Prepare pool PDA seeds for signed transfers
    let seeds = &[
        constants::POOL_AUTH,
        mint_a.key.as_ref(),
        mint_b.key.as_ref(),
        &fee.to_le_bytes(),
        &[pool_bump],
    ];

    // Transfer token A from pool to payer
    if amount_a > 0 {
        msg!("Transferring {} of token A from pool to payer", amount_a);
        transfer_from_pool(
            token_program,
            pool_a,
            payer_account_a,
            mint_a, // ✅ Pass mint_a
            pool,
            amount_a,
            seeds,
        )?;
    }

    // Transfer token B from pool to payer
    if amount_b > 0 {
        msg!("Transferring {} of token B from pool to payer", amount_b);
        transfer_from_pool(
            token_program,
            pool_b,
            payer_account_b,
            mint_b, // ✅ Pass mint_b
            pool,
            amount_b,
            seeds,
        )?;
    }

    Ok(())
}

// Helper function for burn
fn burn<'a>(
    token_program: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    from: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    let spl_ix = spl_token_interface::instruction::burn(
        &Address::from(token_program.key.to_bytes()),
        &Address::from(from.key.to_bytes()),
        &Address::from(mint.key.to_bytes()),
        &Address::from(authority.key.to_bytes()),
        &[],
        amount,
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
            mint.clone(),
            authority.clone(),
            token_program.clone(),
        ],
    )
}

// Helper function for transfer from pool (signed with PDA)
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
