use solana_address::Address;
use solana_client::rpc_client::RpcClient;
use solana_program_pack::Pack;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    system_instruction,
    transaction::Transaction,
};
use std::path::PathBuf;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use amm::Cmd;
use amm::state::Pool;

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

    let users = vec![Keypair::new(), Keypair::new()];

    airdrop(&client, &payer.pubkey(), 1e9 as u64);
    for user in users.iter() {
        airdrop(&client, &user.pubkey(), 1e9 as u64);
    }

    // Mints
    let mint_a = create_mint(&client, &payer, &payer.pubkey(), 6);
    let mint_b = create_mint(&client, &payer, &payer.pubkey(), 6);

    // Pool PDA
    let fee: u16 = 500;
    let (pool_pda, pool_bump) = Pubkey::find_program_address(
        &[
            amm::constants::POOL_AUTH,
            mint_a.as_ref(),
            mint_b.as_ref(),
            fee.to_le_bytes().as_ref(),
        ],
        &program_id,
    );

    let (mint_pool_pda, mint_pool_bump) = Pubkey::find_program_address(
        &[
            amm::constants::POOL_MINT,
            mint_a.as_ref(),
            mint_b.as_ref(),
            fee.to_le_bytes().as_ref(),
        ],
        &program_id,
    );

    // ATA
    let mut atas_a = Vec::new();
    let mut atas_b = Vec::new();
    for user in users.iter() {
        let ata_a = create_ata(&client, &payer, &user.pubkey(), &mint_a);
        atas_a.push(ata_a);

        let ata_b = create_ata(&client, &payer, &user.pubkey(), &mint_b);
        atas_b.push(ata_b);

        mint_to(&client, &payer, &mint_a, &ata_a, 1e9 as u64);
        mint_to(&client, &payer, &mint_b, &ata_b, 1e9 as u64);
    }

    let pool_a = get_ata(&mint_a, &pool_pda);
    let pool_b = get_ata(&mint_b, &pool_pda);

    let mut atas_liq = Vec::new();
    for user in users.iter() {
        atas_liq.push(get_ata(&mint_pool_pda, &user.pubkey()));
    }

    // Init pool
    println!("--- Init pool ---");

    let fee: u16 = 500;

    let ix = create_init_pool_ix(
        program_id,
        payer.pubkey(),
        fee,
        mint_a,
        mint_b,
        pool_pda,
        pool_bump,
        mint_pool_pda,
        mint_pool_bump,
        pool_a,
        pool_b,
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    let blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[&payer], blockhash);

    let res = client.send_and_confirm_transaction(&tx);
    res.unwrap();

    // Add liquidity
    println!("--- Add liquidity ---");
    let amt_a = (10.0 * 1e6) as u64;
    let amt_b = (10.0 * 1e6) as u64;

    let ix = create_add_liquidity_ix(
        program_id,
        users[0].pubkey(),
        fee,
        mint_a,
        mint_b,
        pool_pda,
        pool_bump,
        mint_pool_pda,
        mint_pool_bump,
        pool_a,
        pool_b,
        amt_a,
        amt_b,
        atas_a[0],
        atas_b[0],
        atas_liq[0],
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&users[0].pubkey()));
    let blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[&users[0]], blockhash);

    let res = client.send_and_confirm_transaction(&tx);
    res.unwrap();

    println!("Pool A: {:?}", get_token_balance(&client, &pool_a));
    println!("Pool B: {:?}", get_token_balance(&client, &pool_b));

    // SwaV
    println!("--- Swap ---");

    let a_for_b = true;
    let amt_in = 1e6 as u64;
    let min_amt_out = (0.95 * 1e6) as u64;

    let ix = create_swap_ix(
        program_id,
        users[1].pubkey(),
        fee,
        mint_a,
        mint_b,
        pool_pda,
        pool_bump,
        pool_a,
        pool_b,
        atas_a[1],
        atas_b[1],
        a_for_b,
        amt_in,
        min_amt_out,
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&users[1].pubkey()));
    let blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[&users[1]], blockhash);

    let res = client.send_and_confirm_transaction(&tx);
    res.unwrap();

    println!("User 1 A: {:?}", get_token_balance(&client, &atas_a[1]));
    println!("User 1 B: {:?}", get_token_balance(&client, &atas_b[1]));
    println!("Pool A: {:?}", get_token_balance(&client, &pool_a));
    println!("Pool B: {:?}", get_token_balance(&client, &pool_b));

    // Remove liquidity
    println!("--- Remove liquidity ---");

    let shares = amt_a + amt_b;
    let min_amt_a = 1;
    let min_amt_b = 1;

    let ix = create_remove_liquidity_ix(
        program_id,
        users[0].pubkey(),
        fee,
        mint_a,
        mint_b,
        pool_pda,
        pool_bump,
        mint_pool_pda,
        mint_pool_bump,
        pool_a,
        pool_b,
        shares,
        min_amt_a,
        min_amt_b,
        atas_a[0],
        atas_b[0],
        atas_liq[0],
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&users[0].pubkey()));
    let blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[&users[0]], blockhash);

    let res = client.send_and_confirm_transaction(&tx);
    res.unwrap();

    println!("User 0 A: {:?}", get_token_balance(&client, &atas_a[0]));
    println!("User 0 B: {:?}", get_token_balance(&client, &atas_b[0]));
    println!("Pool A: {:?}", get_token_balance(&client, &pool_a));
    println!("Pool B: {:?}", get_token_balance(&client, &pool_b));
}

fn airdrop(client: &RpcClient, pubkey: &Pubkey, lamports: u64) {
    let bal = client.get_balance(pubkey).unwrap();
    if bal >= lamports {
        return;
    }

    let sig = client.request_airdrop(pubkey, lamports).unwrap();

    // Wait for airdrop confirmation
    while !client.confirm_transaction(&sig).unwrap_or(false) {
        thread::sleep(Duration::from_millis(500));
    }
}

fn create_mint(
    client: &RpcClient,
    payer: &Keypair,
    auth: &Pubkey,
    decimals: u8,
) -> Pubkey {
    let mint = Keypair::new();
    let rent = client
        .get_minimum_balance_for_rent_exemption(
            spl_token_interface::state::Mint::LEN,
        )
        .unwrap();

    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        rent,
        spl_token_interface::state::Mint::LEN as u64,
        &Pubkey::from(spl_token_interface::ID.to_bytes()),
    );

    let init_ix = spl_token_interface::instruction::initialize_mint(
        &Address::from(spl_token_interface::ID.to_bytes()),
        &Address::from(mint.pubkey().to_bytes()),
        &Address::from(auth.to_bytes()),
        None,
        decimals,
    )
    .unwrap();

    let init_ix = Instruction {
        program_id: Pubkey::from(init_ix.program_id.to_bytes()),
        accounts: init_ix
            .accounts
            .iter()
            .map(|acc| AccountMeta {
                pubkey: Pubkey::from(acc.pubkey.to_bytes()),
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect(),
        data: init_ix.data,
    };

    let mut tx = Transaction::new_with_payer(
        &[create_account_ix, init_ix],
        Some(&payer.pubkey()),
    );
    let blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[payer, &mint], blockhash);

    client.send_and_confirm_transaction(&tx).unwrap();

    mint.pubkey()
}

fn get_ata(mint: &Pubkey, owner: &Pubkey) -> Pubkey {
    let ata_addr = spl_associated_token_account_interface::address::get_associated_token_address(
        &Address::from(owner.to_bytes()),
        &Address::from(mint.to_bytes()),
    );
    Pubkey::from(ata_addr.to_bytes())
}

fn create_ata(
    client: &RpcClient,
    payer: &Keypair,
    owner: &Pubkey,
    mint: &Pubkey,
) -> Pubkey {
    let ata = get_ata(mint, owner);

    if client.get_account(&ata).is_ok() {
        return ata;
    }

    let create_ata_ix =
        spl_associated_token_account_interface::instruction::create_associated_token_account(
            &Address::from(payer.pubkey().to_bytes()),
            &Address::from(owner.to_bytes()),
            &Address::from(mint.to_bytes()),
            &Address::from(spl_token_interface::ID.to_bytes()),
        );

    let create_ata_ix = Instruction {
        program_id: Pubkey::from(create_ata_ix.program_id.to_bytes()),
        accounts: create_ata_ix
            .accounts
            .iter()
            .map(|acc| AccountMeta {
                pubkey: Pubkey::from(acc.pubkey.to_bytes()),
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect(),
        data: create_ata_ix.data,
    };

    let mut tx =
        Transaction::new_with_payer(&[create_ata_ix], Some(&payer.pubkey()));
    let blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[payer], blockhash);

    client.send_and_confirm_transaction(&tx).unwrap();

    ata
}

fn mint_to(
    client: &RpcClient,
    payer: &Keypair,
    mint: &Pubkey,
    dst: &Pubkey,
    amt: u64,
) {
    let mint_ix = spl_token_interface::instruction::mint_to(
        &Address::from(spl_token_interface::ID.to_bytes()),
        &Address::from(mint.to_bytes()),
        &Address::from(dst.to_bytes()),
        &Address::from(payer.pubkey().to_bytes()),
        &[],
        amt,
    )
    .unwrap();

    let mint_ix = Instruction {
        program_id: Pubkey::from(mint_ix.program_id.to_bytes()),
        accounts: mint_ix
            .accounts
            .iter()
            .map(|acc| AccountMeta {
                pubkey: Pubkey::from(acc.pubkey.to_bytes()),
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect(),
        data: mint_ix.data,
    };

    let mut tx = Transaction::new_with_payer(&[mint_ix], Some(&payer.pubkey()));
    let blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[payer], blockhash);

    client.send_and_confirm_transaction(&tx).unwrap();
}

fn get_token_balance(client: &RpcClient, token_account: &Pubkey) -> u64 {
    let data = client.get_account_data(token_account).unwrap();
    let account = spl_token_interface::state::Account::unpack(&data).unwrap();
    account.amount
}

fn create_init_pool_ix(
    program_id: Pubkey,
    payer: Pubkey,
    fee: u16,
    mint_a: Pubkey,
    mint_b: Pubkey,
    pool: Pubkey,
    pool_bump: u8,
    mint_pool: Pubkey,
    mint_pool_bump: u8,
    pool_a: Pubkey,
    pool_b: Pubkey,
) -> Instruction {
    let cmd = Cmd::InitPool {
        fee,
        pool_bump,
        mint_pool_bump,
    };

    Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from(spl_token_interface::ID.to_bytes()),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from(
                    spl_associated_token_account_interface::program::ID
                        .to_bytes(),
                ),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: solana_sdk::system_program::id(),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: solana_sdk::sysvar::rent::id(),
                is_signer: false,
                is_writable: true,
            },
        ],
    )
}

fn create_add_liquidity_ix(
    program_id: Pubkey,
    payer: Pubkey,
    fee: u16,
    mint_a: Pubkey,
    mint_b: Pubkey,
    pool: Pubkey,
    pool_bump: u8,
    mint_pool: Pubkey,
    mint_pool_bump: u8,
    pool_a: Pubkey,
    pool_b: Pubkey,
    amount_a: u64,
    amount_b: u64,
    payer_a: Pubkey,
    payer_b: Pubkey,
    payer_liq: Pubkey,
) -> Instruction {
    let cmd = Cmd::AddLiquidity {
        fee,
        pool_bump,
        mint_pool_bump,
        amount_a,
        amount_b,
    };

    Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer_liq,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from(spl_token_interface::ID.to_bytes()),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from(
                    spl_associated_token_account_interface::program::ID
                        .to_bytes(),
                ),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: solana_sdk::system_program::id(),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: solana_sdk::sysvar::rent::id(),
                is_signer: false,
                is_writable: true,
            },
        ],
    )
}

fn create_remove_liquidity_ix(
    program_id: Pubkey,
    payer: Pubkey,
    fee: u16,
    mint_a: Pubkey,
    mint_b: Pubkey,
    pool: Pubkey,
    pool_bump: u8,
    mint_pool: Pubkey,
    mint_pool_bump: u8,
    pool_a: Pubkey,
    pool_b: Pubkey,
    shares: u64,
    min_amount_a: u64,
    min_amount_b: u64,
    payer_a: Pubkey,
    payer_b: Pubkey,
    payer_liq: Pubkey,
) -> Instruction {
    let cmd = Cmd::RemoveLiquidity {
        fee,
        pool_bump,
        mint_pool_bump,
        shares,
        min_amount_a,
        min_amount_b,
    };

    Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer_liq,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from(spl_token_interface::ID.to_bytes()),
                is_signer: false,
                is_writable: true,
            },
        ],
    )
}

fn create_swap_ix(
    program_id: Pubkey,
    payer: Pubkey,
    fee: u16,
    mint_a: Pubkey,
    mint_b: Pubkey,
    pool: Pubkey,
    pool_bump: u8,
    pool_a: Pubkey,
    pool_b: Pubkey,
    payer_a: Pubkey,
    payer_b: Pubkey,
    a_for_b: bool,
    amount_in: u64,
    min_amount_out: u64,
) -> Instruction {
    let cmd = Cmd::Swap {
        fee,
        pool_bump,
        a_for_b,
        amount_in,
        min_amount_out,
    };

    Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from(spl_token_interface::ID.to_bytes()),
                is_signer: false,
                is_writable: true,
            },
        ],
    )
}
