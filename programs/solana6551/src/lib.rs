use anchor_lang::prelude::*;
use instructions::*;

pub mod account;
pub mod instructions;
pub mod errors;
pub mod utils;

declare_id!("CbkGxYsZct6iaLY6n13NUX7D4gvS2qqRcDBKFgr2JCeQ");

#[program]
pub mod solana6551 {
    use super::*;

    pub fn create_nft_pda(ctx: Context<CreateNftPda>) -> Result<()> {
        instructions::create_nft_pda(ctx)
    }

    pub fn create_nftpda_pda(ctx: Context<CreateNftpdaPda>, content: String) -> Result<()> {
        instructions::create_nftpda_pda(ctx, content)
    }

    pub fn delete_nftpda_pda(ctx: Context<DeleteNftpdaPda>) -> Result<()> {
        instructions::delete_nftpda_pda(ctx)
    }

    pub fn transfer_native_nft(ctx: Context<TransferNativeNft>) -> Result<()> {
        instructions::transfer_native_nft(ctx)
    }

    pub fn transfer_native_sol(ctx: Context<TransferNativeSol>, amount: u64) -> Result<()> {
        instructions::transfer_native_sol(ctx, amount)
    }

    pub fn transfer_native_token(ctx: Context<TransferNativeToken>, amount: u64) -> Result<()> {
        instructions::transfer_native_token(ctx, amount)
    }

    pub fn transfer_pda_nft(ctx: Context<TransferPdaNft>) -> Result<()> {
        instructions::transfer_pda_nft(ctx)
    }

    pub fn transfer_pda_sol(ctx: Context<TransferPdaSol>, amount: u64) -> Result<()> {
        instructions::transfer_pda_sol(ctx, amount)
    }

    pub fn transfer_pda_token(ctx: Context<TransferPdaToken>, amount: u64) -> Result<()> {
        instructions::transfer_pda_token(ctx, amount)
    }
}
