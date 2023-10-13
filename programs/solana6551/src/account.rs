use anchor_lang::prelude::*;

#[account]
pub struct NftPda {
    pub nft_mint: Pubkey,
    pub pda_num: u64,
}

#[account]
pub struct NftpdaPda {
    pub nft_mint: Pubkey,
    pub content: String,
}