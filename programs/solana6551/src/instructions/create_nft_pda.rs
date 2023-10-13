use crate::account::NftPda;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use serde_json::json;

pub fn create_nft_pda(ctx: Context<CreateNftPda>) -> Result<()> {
    let nft_pda = &mut ctx.accounts.nft_pda;
    nft_pda.nft_mint = ctx.accounts.nft_mint.key();
    nft_pda.pda_num = 0;

    let log = json!({
        "Func":"createNftPda",
        "nftPda":&ctx.accounts.nft_pda.key().to_string()
    });
    msg!("{}", serde_json::to_string(&log).unwrap());
    Ok(())
}

#[derive(Accounts)]
pub struct CreateNftPda<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(constraint = nft_mint.supply == 1
        && nft_mint.decimals == 0)]
    nft_mint: Account<'info, Mint>,

    // nft token address
    #[account(mut, constraint = nft_token_account.mint == nft_mint.key() 
        && nft_token_account.amount == 1
        && nft_token_account.owner == payer.key())]
    nft_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        space = 85,
        seeds = [
            b"nft_pda",
            nft_mint.key().as_ref(),
        ],
        bump
    )]
    pub nft_pda: Account<'info, NftPda>,
    pub system_program: Program<'info, System>,
}
