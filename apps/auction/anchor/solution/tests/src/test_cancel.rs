use anchor_client::solana_sdk::signature::Signer;
use anchor_client::solana_sdk::{signature::read_keypair_file, system_program};
use anchor_spl::associated_token::{
    get_associated_token_address, spl_associated_token_account,
};
use anchor_spl::token::{self};
use std::time::{SystemTime, UNIX_EPOCH};

use super::test_helper;
use super::token_helper;

#[test]
fn test_cancel() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let test_helper::Test {
        program,
        token_program,
        seller,
        buyer,
        auction_pda,
        auction_bump,
        mint_sell,
        mint_buy,
        seller_sell_ata,
        buyer_sell_ata,
        seller_buy_ata,
        buyer_buy_ata,
    } = test_helper::set_up(&payer);

    // Init
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let start_price = (2.0 * 1e6) as u64;
    let end_price = (1.1 * 1e6) as u64;
    let start_time = now + 1;
    let end_time = start_time + 10;
    let sell_amt = 100 * (1e6 as u64);
    let auction_sell_ata =
        get_associated_token_address(&auction_pda, &mint_sell.pubkey());

    program
        .request()
        .accounts(auction::accounts::Init {
            payer: seller.pubkey(),
            mint_sell: mint_sell.pubkey(),
            mint_buy: mint_buy.pubkey(),
            auction: auction_pda,
            auction_sell_ata,
            seller_sell_ata,
            seller_buy_ata,
            token_program: token::ID,
            associated_token_program: spl_associated_token_account::ID,
            system_program: system_program::ID,
        })
        .signer(&seller)
        .args(auction::instruction::Init {
            start_price,
            end_price,
            start_time,
            end_time,
            sell_amt,
        })
        .send()
        .unwrap();

    // Cancel
    program
        .request()
        .accounts(auction::accounts::Cancel {
            payer: seller.pubkey(),
            mint_sell: mint_sell.pubkey(),
            mint_buy: mint_buy.pubkey(),
            auction: auction_pda,
            auction_sell_ata,
            seller_sell_ata,
            token_program: token::ID,
            associated_token_program: spl_associated_token_account::ID,
            system_program: system_program::ID,
        })
        .signer(&seller)
        .args(auction::instruction::Cancel {})
        .send()
        .unwrap();

    assert!(
        program
            .account::<auction::state::Auction>(auction_pda)
            .is_err(),
        "Auction not closed"
    );

    assert_eq!(
        token_helper::get_balance(&token_program, &seller_sell_ata).unwrap(),
        sell_amt,
        "Seller sell ATA balance"
    );
    assert_eq!(
        token_helper::get_balance(&token_program, &auction_sell_ata)
            .unwrap_or(0),
        0,
        "Auction sell ATA balance"
    );

    assert_eq!(
        program.rpc().get_balance(&auction_sell_ata).unwrap_or(0),
        0,
        "Auction sell ATA not closed"
    );
}
