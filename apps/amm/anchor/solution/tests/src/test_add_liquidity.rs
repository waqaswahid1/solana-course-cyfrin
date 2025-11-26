use anchor_client::solana_sdk::signature::Signer;
use anchor_client::solana_sdk::{signature::read_keypair_file, system_program};
use anchor_spl::associated_token::spl_associated_token_account;
use anchor_spl::token::{self};

use super::test_helper;
use super::token_helper;

#[test]
fn test_add_liquidity() {
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

    // Add liquidity
    let amount_a = (10.0 * 1e6) as u64;
    let amount_b = (10.0 * 1e6) as u64;

    program
        .request()
        .accounts(amm::accounts::AddLiquidity {
            payer: users[0].pubkey(),
            pool: pool_pda,
            mint_a,
            mint_b,
            pool_a,
            pool_b,
            mint_pool: mint_pool_pda,
            payer_a: atas_a[0],
            payer_b: atas_b[0],
            payer_liquidity: atas_pool[0],
            token_program: token::ID,
            associated_token_program: spl_associated_token_account::ID,
            system_program: system_program::ID,
        })
        .signer(&users[0])
        .args(amm::instruction::AddLiquidity {
            fee,
            amount_a,
            amount_b,
        })
        .send()
        .unwrap();

    assert_eq!(
        token_helper::get_balance(&token_program, &pool_a).unwrap(),
        amount_a,
        "pool amount a"
    );
    assert_eq!(
        token_helper::get_balance(&token_program, &pool_b).unwrap(),
        amount_b,
        "pool amount b"
    );
    assert_eq!(
        token_helper::get_balance(&token_program, &atas_pool[0]).unwrap(),
        amount_a + amount_b,
        "user liquidity"
    );
}
