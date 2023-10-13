use crate::account::{NftpdaPda, NftPda};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use serde_json::json;

pub fn delete_nftpda_pda(ctx: Context<DeleteNftpdaPda>) -> Result<()> {
    let log = json!({
        "Func":"deleteNftpdaPda",
        "nftPda":&ctx.accounts.nftpda_pda.key().to_string()
    });
    msg!("{}", serde_json::to_string(&log).unwrap());
    Ok(())
}

#[derive(Accounts)]
pub struct DeleteNftpdaPda<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(
        mut,
        close = payer,
        seeds = [
            b"pdanft_pda",
            native_nft_mint.key().as_ref(),
            &nft_pda.pda_num.to_le_bytes()
        ],
        bump
    )]
    pub nftpda_pda: Account<'info, NftpdaPda>,

    #[account(
    mut,
    seeds = [
    b"nft_pda",
    native_nft_mint.key().as_ref(),
    ],
    bump,
    )]
    nft_pda: Account<'info, NftPda>,

    #[account(constraint = nft_pda.nft_mint == native_nft_mint.key())]
    native_nft_mint: Account<'info, Mint>,

    #[account(mut, constraint = native_nft_token_account.amount == 1 
        && native_nft_token_account.owner == payer.key()
        && native_nft_token_account.mint == native_nft_mint.key())]
    native_nft_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
}
