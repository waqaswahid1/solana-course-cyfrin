use borsh::BorshDeserialize;
use litesvm::LiteSVM;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use auction::{Cmd, state::Auction};

#[test]
fn test() {
    let mut svm = LiteSVM::new();

    let seller = Keypair::new();
    let buyer = Keypair::new();
    let auction = Keypair::new();
    let program_keypair = Keypair::new();
    let program_id = program_keypair.pubkey();
    svm.add_program_from_file(program_id, "target/deploy/auction.so")
        .unwrap();

    svm.airdrop(&seller.pubkey(), 1_000_000_000).unwrap();
    svm.airdrop(&buyer.pubkey(), 1_000_000_000).unwrap();

    // A = sell, B = buy
    // let mint_a = create_mint(&client, &payer, &payer.pubkey(), 6);
    // let mint_b = create_mint(&client, &payer, &payer.pubkey(), 6);

    /*
    let oracle_account = Account {
        lamports: 1_000_000,
        owner: program_id,
        data: vec![0u8; std::mem::size_of::<auction>()],
        ..Account::default()
    };
    svm.set_account(auction.pubkey(), oracle_account).unwrap();

    // Init
    let init_ix = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(auction.pubkey(), false)],
        data: borsh::to_vec(&Cmd::Init(owner.pubkey(), 123)).unwrap(),
    };

    svm.send_transaction(Transaction::new_signed_with_payer(
        &[init_ix],
        Some(&owner.pubkey()),
        &[&owner],
        svm.latest_blockhash(),
    ))
    .unwrap();

    let data = svm.get_account(&auction.pubkey()).unwrap().data;
    let oracle_state = Auction::try_from_slice(&data).unwrap();
    assert_eq!(oracle_state.owner, owner.pubkey());
    assert_eq!(oracle_state.price, 123);

    // Re-init
    let init_ix = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(auction.pubkey(), false)],
        data: borsh::to_vec(&Cmd::Init(owner.pubkey(), 999)).unwrap(),
    };

    let res = svm.send_transaction(Transaction::new_signed_with_payer(
        &[init_ix],
        Some(&owner.pubkey()),
        &[&owner],
        svm.latest_blockhash(),
    ));

    assert!(
        res.is_err(),
        "Reinitialization should fail because auction.owner is already set"
    );

    // Update
    let update_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(auction.pubkey(), false),
            AccountMeta::new(owner.pubkey(), true),
        ],
        data: borsh::to_vec(&Cmd::Update(1234)).unwrap(),
    };

    svm.send_transaction(Transaction::new_signed_with_payer(
        &[update_ix],
        Some(&owner.pubkey()),
        &[&owner],
        svm.latest_blockhash(),
    ))
    .unwrap();

    let data = svm.get_account(&auction.pubkey()).unwrap().data;
    let oracle_state = Auction::try_from_slice(&data).unwrap();
    assert_eq!(oracle_state.price, 1234);

    // Invalid update (by attacker)
    let update_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(auction.pubkey(), false),
            AccountMeta::new(attacker.pubkey(), true),
        ],
        data: borsh::to_vec(&Cmd::Update(9999)).unwrap(),
    };

    let res = svm.send_transaction(Transaction::new_signed_with_payer(
        &[update_ix],
        Some(&attacker.pubkey()),
        &[&attacker],
        svm.latest_blockhash(),
    ));

    assert!(
        res.is_err(),
        "Unauthorized signer should not be able to update auction"
    );

    let data = svm.get_account(&auction.pubkey()).unwrap().data;
    let oracle_state = Auction::try_from_slice(&data).unwrap();
    assert_eq!(oracle_state.price, 1234);
    */
}
