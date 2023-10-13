use crate::utils::transfer_nft;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use serde_json::json;

pub fn transfer_native_nft(ctx: Context<TransferNativeNft>) -> Result<()> {
    transfer_nft(
        ctx.accounts.from_nft_token_account.to_account_info(),
        ctx.accounts.to_nft_token_account.to_account_info(),
        ctx.accounts.from.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        1,
    )?;

    let log = json!({
        "Func":"transferNativeNft",
        "tokenAddress":&ctx.accounts.from_nft_token_account.mint.to_string(),
        "from":&ctx.accounts.from.key().to_string(),
        "to":&ctx.accounts.to.key().to_string()
    });
    msg!("{}", serde_json::to_string(&log).unwrap());

    Ok(())
}

#[derive(Accounts)]
pub struct TransferNativeNft<'info> {
    #[account(mut)]
    from: Signer<'info>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    to: AccountInfo<'info>,

    #[account(mut)]
    from_nft_token_account: Account<'info, TokenAccount>,

    #[account(constraint = nft_mint.key() == from_nft_token_account.mint)]
    nft_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = from,
        associated_token::mint = nft_mint,
        associated_token::authority = to,
    )]
    to_nft_token_account: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}
