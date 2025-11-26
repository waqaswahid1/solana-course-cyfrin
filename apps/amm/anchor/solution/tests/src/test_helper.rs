use anchor_client::solana_sdk::signature::Signer;
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair,
    },
    Client, Cluster, Program,
};
use anchor_spl::associated_token::{
    get_associated_token_address, spl_associated_token_account,
};
use anchor_spl::token::{self};

use super::token_helper;

pub struct Test<'a> {
    pub program: Program<&'a Keypair>,
    pub token_program: Program<&'a Keypair>,
    pub users: Vec<Keypair>,
    pub pool_pda: Pubkey,
    pub pool_bump: u8,
    pub mint_pool_pda: Pubkey,
    pub mint_pool_bump: u8,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub pool_a: Pubkey,
    pub pool_b: Pubkey,
    pub fee: u16,
    pub atas_a: Vec<Pubkey>,
    pub atas_b: Vec<Pubkey>,
    pub atas_pool: Vec<Pubkey>,
}

pub fn set_up<'a>(payer: &'a Keypair) -> Test<'a> {
    let program_id = amm::ID;

    let users = vec![Keypair::new(), Keypair::new()];

    let client = Client::new_with_options(
        Cluster::Localnet,
        payer,
        CommitmentConfig::confirmed(),
    );
    let program = client.program(program_id).unwrap();

    let rpc = program.rpc();

    // Airdrop
    for user in users.iter() {
        rpc.request_airdrop(&user.pubkey(), 100 * (1e9 as u64))
            .unwrap();
    }

    // Mint sell and buy tokens
    let token_program = client.program(token::ID).unwrap();
    let mint_a = Keypair::new();
    let mint_b = Keypair::new();

    token_helper::create_mint(&token_program, payer, &mint_a, 6);
    token_helper::create_mint(&token_program, payer, &mint_b, 6);

    // Create associated token accounts and mint
    let mut atas_a = Vec::new();
    let mut atas_b = Vec::new();
    for user in users.iter() {
        let ata_a = token_helper::create_ata(
            &token_program,
            payer,
            &mint_a.pubkey(),
            &user.pubkey(),
        )
        .unwrap();

        token_helper::mint_to(
            &token_program,
            payer,
            &mint_a.pubkey(),
            &ata_a,
            100 * (1e6 as u64),
        )
        .unwrap();

        atas_a.push(ata_a);

        let ata_b = token_helper::create_ata(
            &token_program,
            payer,
            &mint_b.pubkey(),
            &user.pubkey(),
        )
        .unwrap();

        token_helper::mint_to(
            &token_program,
            payer,
            &mint_b.pubkey(),
            &ata_b,
            100 * (1e6 as u64),
        )
        .unwrap();

        atas_b.push(ata_b);
    }

    // Calculate AMM PDA
    let fee: u16 = 30;
    let (pool_pda, pool_bump) = Pubkey::find_program_address(
        &[
            amm::constants::POOL_AUTH_SEED_PREFIX,
            mint_a.pubkey().as_ref(),
            mint_b.pubkey().as_ref(),
            fee.to_le_bytes().as_ref(),
        ],
        &program_id,
    );
    let (mint_pool_pda, mint_pool_bump) = Pubkey::find_program_address(
        &[
            amm::constants::POOL_MINT_SEED_PREFIX,
            mint_a.pubkey().as_ref(),
            mint_b.pubkey().as_ref(),
            fee.to_le_bytes().as_ref(),
        ],
        &program_id,
    );

    let pool_a = get_associated_token_address(&pool_pda, &mint_a.pubkey());
    let pool_b = get_associated_token_address(&pool_pda, &mint_b.pubkey());

    // Calculate pool liquidity ATAs
    let mut atas_pool = Vec::new();
    for user in users.iter() {
        atas_pool
            .push(get_associated_token_address(&user.pubkey(), &mint_pool_pda));
    }

    Test {
        program,
        token_program,
        users,
        mint_a: mint_a.pubkey(),
        mint_b: mint_b.pubkey(),
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
    }
}
