use anchor_client::solana_sdk::signature::Signer;
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey,
        signature::Keypair, system_program,
    },
    Client, Cluster, Program,
};
use anchor_spl::associated_token::spl_associated_token_account;
use anchor_spl::token::{self};

use super::token_helper;

pub struct Test<'a> {
    pub program: Program<&'a Keypair>,
    pub token_program: Program<&'a Keypair>,
    pub seller: Keypair,
    pub buyer: Keypair,
    pub auction_pda: Pubkey,
    pub auction_bump: u8,
    pub mint_sell: Keypair,
    pub mint_buy: Keypair,
    pub seller_sell_ata: Pubkey,
    pub buyer_sell_ata: Pubkey,
    pub seller_buy_ata: Pubkey,
    pub buyer_buy_ata: Pubkey,
}

pub fn set_up<'a>(payer: &'a Keypair) -> Test<'a> {
    let program_id = auction::ID;

    // Seller and buyer
    let seller = Keypair::new();
    let buyer = Keypair::new();

    let client = Client::new_with_options(
        Cluster::Localnet,
        payer,
        CommitmentConfig::confirmed(),
    );
    let program = client.program(program_id).unwrap();

    let rpc = program.rpc();

    // Airdrop
    rpc.request_airdrop(&seller.pubkey(), 100 * (1e9 as u64))
        .unwrap();
    rpc.request_airdrop(&buyer.pubkey(), 100 * (1e9 as u64))
        .unwrap();

    // Mint sell and buy tokens
    let token_program = client.program(token::ID).unwrap();
    let mint_sell = Keypair::new();
    let mint_buy = Keypair::new();

    token_helper::create_mint(&token_program, payer, &mint_sell, 6);
    token_helper::create_mint(&token_program, payer, &mint_buy, 6);

    // Create associated token accounts
    let seller_sell_ata = token_helper::create_ata(
        &token_program,
        payer,
        &mint_sell.pubkey(),
        &seller.pubkey(),
    )
    .unwrap();

    let buyer_sell_ata = token_helper::create_ata(
        &token_program,
        payer,
        &mint_sell.pubkey(),
        &buyer.pubkey(),
    )
    .unwrap();

    let seller_buy_ata = token_helper::create_ata(
        &token_program,
        payer,
        &mint_buy.pubkey(),
        &seller.pubkey(),
    )
    .unwrap();

    let buyer_buy_ata = token_helper::create_ata(
        &token_program,
        payer,
        &mint_buy.pubkey(),
        &buyer.pubkey(),
    )
    .unwrap();

    // Mint tokens
    token_helper::mint_to(
        &token_program,
        payer,
        &mint_sell.pubkey(),
        &seller_sell_ata,
        100 * (1e6 as u64),
    )
    .unwrap();
    token_helper::mint_to(
        &token_program,
        payer,
        &mint_buy.pubkey(),
        &buyer_buy_ata,
        200 * (1e6 as u64),
    )
    .unwrap();

    // Calculate Auction PDA
    let (pda, bump) = Pubkey::find_program_address(
        &[
            auction::state::Auction::SEED_PREFIX,
            &seller.pubkey().as_ref(),
            mint_sell.pubkey().as_ref(),
            mint_buy.pubkey().as_ref(),
        ],
        &program_id,
    );

    Test {
        program,
        token_program,
        seller,
        buyer,
        auction_pda: pda,
        auction_bump: bump,
        mint_sell,
        mint_buy,
        seller_sell_ata,
        buyer_sell_ata,
        seller_buy_ata,
        buyer_buy_ata,
    }
}
