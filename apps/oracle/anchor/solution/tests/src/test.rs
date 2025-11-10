use std::str::FromStr;

use anchor_client::solana_sdk::signature::Signer;
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        system_program,
    },
    Client, Cluster,
};

#[test]
fn test() {
    let program_id = oracle::ID;
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(
        Cluster::Localnet,
        &payer,
        CommitmentConfig::confirmed(),
    );
    let program = client.program(program_id).unwrap();

    // Initialize
    let oracle_account = Keypair::new();

    let price: u64 = 123;

    program
        .request()
        .accounts(oracle::accounts::Init {
            payer: payer.pubkey(),
            owner: payer.pubkey(),
            oracle: oracle_account.pubkey(),
            system_program: system_program::ID,
        })
        .signer(&payer)
        .signer(&oracle_account)
        .args(oracle::instruction::Init { price })
        .send()
        .unwrap();

    let oracle_state: oracle::state::Oracle =
        program.account(oracle_account.pubkey()).unwrap();

    assert_eq!(oracle_state.owner, payer.pubkey(), "oracle.owner");
    assert_eq!(oracle_state.price, price, "oracle.price");

    // Cannot re-initialize
    let res = program
        .request()
        .accounts(oracle::accounts::Init {
            payer: payer.pubkey(),
            owner: payer.pubkey(),
            oracle: oracle_account.pubkey(),
            system_program: system_program::ID,
        })
        .signer(&payer)
        .signer(&oracle_account)
        .args(oracle::instruction::Init { price })
        .send();

    assert!(res.is_err(), "re-initialize");

    // Update
    let price: u64 = 1234;

    let res = program
        .request()
        .accounts(oracle::accounts::Update {
            owner: payer.pubkey(),
            oracle: oracle_account.pubkey(),
        })
        .signer(&payer)
        .args(oracle::instruction::Update { price })
        .send();

    assert!(res.is_ok(), "update");

    let oracle_state: oracle::state::Oracle =
        program.account(oracle_account.pubkey()).unwrap();

    assert_eq!(oracle_state.owner, payer.pubkey());
    assert_eq!(oracle_state.price, price);

    // Update - not authorized
    let res = program
        .request()
        .accounts(oracle::accounts::Update {
            owner: oracle_account.pubkey(),
            oracle: oracle_account.pubkey(),
        })
        .signer(&oracle_account)
        .args(oracle::instruction::Update { price })
        .send();

    assert!(res.is_err(), "update - not authorized");
}
