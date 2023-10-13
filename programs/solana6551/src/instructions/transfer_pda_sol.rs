use crate::account::NftPda;
use crate::utils::transfer_sol_with_pda;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use serde_json::json;

pub fn transfer_pda_sol(ctx: Context<TransferPdaSol>, amount: u64) -> Result<()> {

    transfer_sol_with_pda(
        ctx.accounts.nft_pda.to_account_info(),
        ctx.accounts.to.to_account_info(),
        amount,
    )?;

    let log = json!({
        "Func":"transferPdaSol",
        "nftpda":&ctx.accounts.nft_pda.key().to_string(),
        "from":&ctx.accounts.payer.key().to_string(),
        "to":&ctx.accounts.to.key().to_string(),
        "amount":amount
    });
    msg!("{}", serde_json::to_string(&log).unwrap());

    Ok(())
}

#[derive(Accounts)]
pub struct TransferPdaSol<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    to: AccountInfo<'info>,

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

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}
