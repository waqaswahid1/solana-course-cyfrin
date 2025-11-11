use solana_client::rpc_client::RpcClient;
use solana_program::system_program;
use solana_program::sysvar::{Sysvar, clock::Clock, rent::Rent};
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
use piggy::Cmd;
use piggy::state::Lock;

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

    let dst = Keypair::new();

    let (pda, bump) = Pubkey::find_program_address(
        &[b"lock", payer.pubkey().as_ref(), dst.pubkey().as_ref()],
        &program_id,
    );

    // 32 + 8
    let space = 40;
    let lamports = client
        .get_minimum_balance_for_rent_exemption(space)
        .unwrap();

    // Initialize
    let now = client.get_block_time(client.get_slot().unwrap()).unwrap() as u64;
    let amt = 1e9 as u64;
    let exp = now + 100;

    let cmd = Cmd::Lock {
        dst: dst.pubkey(),
        amt,
        exp,
        bump,
    };

    let ix = Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: payer.pubkey(),
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pda,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: system_program::ID,
                is_signer: false,
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
    /*

    let data = client
        .get_account_data(&oracle_account.pubkey())
        .expect("Failed to fetch account data");

    let oracle_data =
        Lock::try_from_slice(&data).expect("Failed to deserialize");

    println!("oracle.owner: {:?}", oracle_data.owner);
    println!("oracle.price: {:?}", oracle_data.price);
    */

    /*
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
    */
}
