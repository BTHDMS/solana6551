use crate::account::{NftpdaPda, NftPda};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use serde_json::json;

pub fn create_nftpda_pda(ctx: Context<CreateNftpdaPda>, content: String) -> Result<()> {
    let nftpda_pda = &mut ctx.accounts.nftpda_pda;
    nftpda_pda.content = content;
    nftpda_pda.nft_mint = ctx.accounts.native_nft_mint.key();

    let nft_pda = &mut ctx.accounts.nft_pda;
    nft_pda.pda_num += 1;

    let log = json!({
        "Func":"createNftpdaPda",
        "nftpdaPda":&ctx.accounts.nftpda_pda.key().to_string()
    });
    msg!("{}", serde_json::to_string(&log).unwrap());
    Ok(())
}

#[derive(Accounts)]
pub struct CreateNftpdaPda<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 4000,
        seeds = [
            b"nftpda_pda",
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
