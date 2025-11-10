use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    signer::keypair::keypair_from_seed,
    system_instruction,
    transaction::Transaction,
};
use std::path::PathBuf;
use std::str::FromStr;

use borsh::BorshDeserialize;
use oracle::Cmd;
use oracle::state::Oracle;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let keypair_path: PathBuf = [&args[1]].iter().collect();
    let payer =
        read_keypair_file(keypair_path).expect("Cannot read keypair file");

    // Connect to local cluster
    let rpc_url = String::from(&args[2]);
    let client =
        RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let program_id = Pubkey::from_str(&args[3]).expect("Invalid program ID");

    println!("Wallet: {}", payer.pubkey());

    // Wallet balance
    let lamports = client.get_balance(&payer.pubkey()).unwrap();
    let sol =
        lamports as f64 / solana_sdk::native_token::LAMPORTS_PER_SOL as f64;
    println!("Balance: {} SOL ({} lamports)", sol, lamports);

    // Request airdrop of 1 SOL for transaction fees
    if sol < 1.0 {
        println!("Requesting airdrop...");
        let airdrop_signature = client
            .request_airdrop(&payer.pubkey(), 1_000_000_000)
            .expect("Failed to request airdrop");

        // Wait for airdrop confirmation
        while !client
            .confirm_transaction(&airdrop_signature)
            .unwrap_or(false)
        {
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        println!("Airdrop confirmed");
    }

    // Create Oracle account, owned by the Oracle program if it doesn't exist
    // let seed = [1u8; 32];
    // let oracle_account = keypair_from_seed(&seed).unwrap();
    let oracle_account = Keypair::new();

    // 32 + 8
    let space = 40;
    let lamports = client
        .get_minimum_balance_for_rent_exemption(space)
        .unwrap();

    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &oracle_account.pubkey(),
        lamports,
        space as u64,
        &program_id,
    );

    let mut tx = Transaction::new_with_payer(
        &[create_account_ix],
        Some(&payer.pubkey()),
    );

    let blockhash = client.get_latest_blockhash().expect("blockhash");
    tx.sign(&[&payer, &oracle_account], blockhash);

    let res = client.send_and_confirm_transaction(&tx);
    match res {
        Ok(_) => println!("Created Oracle account {}", oracle_account.pubkey()),
        Err(err) => println!("Err creating coutner account: {:#?}", err),
    }

    // Initialize
    let cmd = Cmd::Init(payer.pubkey(), 1);

    let ix = Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![AccountMeta {
            pubkey: oracle_account.pubkey(),
            is_signer: false,
            is_writable: true,
        }],
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], client.get_latest_blockhash().unwrap());

    match client.send_and_confirm_transaction(&tx) {
        Ok(sig) => println!("Transaction signature: {}", sig),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }

    let data = client
        .get_account_data(&oracle_account.pubkey())
        .expect("Failed to fetch account data");

    let oracle_data =
        Oracle::try_from_slice(&data).expect("Failed to deserialize");

    println!("oracle.owner: {:?}", oracle_data.owner);
    println!("oracle.price: {:?}", oracle_data.price);

    // Update
    let cmd = Cmd::Update(2); // set initial price to 0

    let ix = Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: oracle_account.pubkey(),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer.pubkey(),
                is_signer: true,
                is_writable: true,
            },
        ],
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], client.get_latest_blockhash().unwrap());

    match client.send_and_confirm_transaction(&tx) {
        Ok(sig) => println!("Transaction signature: {}", sig),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }

    let data = client
        .get_account_data(&oracle_account.pubkey())
        .expect("Failed to fetch account data");

    let oracle_data =
        Oracle::try_from_slice(&data).expect("Failed to deserialize");

    println!("oracle.owner: {:?}", oracle_data.owner);
    println!("oracle.price: {:?}", oracle_data.price);
}
