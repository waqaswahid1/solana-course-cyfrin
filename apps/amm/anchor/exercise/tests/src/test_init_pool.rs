use anchor_client::solana_sdk::signature::Signer;
use anchor_client::solana_sdk::{signature::read_keypair_file, system_program};
use anchor_spl::associated_token::spl_associated_token_account;
use anchor_spl::token::{self};

use super::test_helper;
use super::token_helper;

#[test]
fn test_init() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let test_helper::Test {
        program,
        token_program,
        users,
        mint_a,
        mint_b,
        pool_pda,
        pool_bump,
        mint_pool_pda,
        mint_pool_bump,
        pool_a,
        pool_b,
        fee,
        atas_a,
        atas_b,
        atas_pool,
    } = test_helper::set_up(&payer);

    // Init
    program
        .request()
        .accounts(amm::accounts::InitPool {
            payer: users[0].pubkey(),
            pool: pool_pda,
            mint_a,
            mint_b,
            pool_a,
            pool_b,
            mint_pool: mint_pool_pda,
            token_program: token::ID,
            associated_token_program: spl_associated_token_account::ID,
            system_program: system_program::ID,
        })
        .signer(&users[0])
        .args(amm::instruction::InitPool { fee })
        .send()
        .unwrap();

    let pool: amm::state::Pool = program.account(pool_pda).unwrap();
    assert_eq!(pool.mint_a, mint_a, "pool.mint_a");
    assert_eq!(pool.mint_b, mint_b, "pool.mint_b");
}
