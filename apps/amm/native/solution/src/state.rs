use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Pool {
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
}

impl Pool {
    // 32 + 32
    pub const SPACE: u64 = 64;
}
