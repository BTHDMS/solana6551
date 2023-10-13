use crate::account::NftPda;
use crate::utils::transfer_token_with_signer;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use serde_json::json;

pub fn transfer_pda_token(ctx: Context<TransferPdaToken>, amount: u64) -> Result<()> {
    let seeds = &[
        "nft_pda".as_bytes(),
        ctx.accounts.native_nft_token_account.mint.as_ref(),
        &[*ctx.bumps.get("nft_pda").unwrap()],
    ];
    let signer = &[&seeds[..]];

    transfer_token_with_signer(
        ctx.accounts.from_token_account.to_account_info(),
        ctx.accounts.to_token_account.to_account_info(),
        ctx.accounts.nft_pda.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        amount,
        signer,
    )?;

    let log = json!({
        "Func":"transferPdaToken",
        "tokenAddress":&ctx.accounts.from_token_account.mint.to_string(),
        "nftpda":&ctx.accounts.nft_pda.key().to_string(),
        "from":&ctx.accounts.payer.key().to_string(),
        "to":&ctx.accounts.to.key().to_string(),
        "amount":amount
    });
    msg!("{}", serde_json::to_string(&log).unwrap());

    Ok(())
}

#[derive(Accounts)]
pub struct TransferPdaToken<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    to: AccountInfo<'info>,

    #[account(mut)]
    from_token_account: Account<'info, TokenAccount>,

    #[account(constraint = token_mint.key() == from_token_account.mint)]
    token_mint: Account<'info, Mint>,

    #[account(
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

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = token_mint,
        associated_token::authority = to,
    )]
    to_token_account: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}
