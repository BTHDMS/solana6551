use crate::account::NftPda;
use crate::utils::transfer_token_with_signer;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use serde_json::json;

pub fn transfer_pda_nft(ctx: Context<TransferPdaNft>) -> Result<()> {
    let seeds = &[
        "nft_pda".as_bytes(),
        ctx.accounts.native_nft_token_account.mint.as_ref(),
        &[*ctx.bumps.get("nft_pda").unwrap()],
    ];
    let signer = &[&seeds[..]];

    transfer_token_with_signer(
        ctx.accounts.from_nft_token_account.to_account_info(),
        ctx.accounts.to_nft_token_account.to_account_info(),
        ctx.accounts.nft_pda.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        1,
        signer,
    )?;

    let log = json!({
        "Func":"transferPdaNft",
        "tokenAddress":&ctx.accounts.from_nft_token_account.mint.to_string(),
        "nftPda":&ctx.accounts.nft_pda.key().to_string(),
        "from":&ctx.accounts.payer.key().to_string(),
        "to":&ctx.accounts.to.key().to_string()
    });
    msg!("{}", serde_json::to_string(&log).unwrap());

    Ok(())
}

#[derive(Accounts)]
pub struct TransferPdaNft<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    to: AccountInfo<'info>,

    #[account(mut)]
    from_nft_token_account: Account<'info, TokenAccount>,

    #[account(constraint = nft_mint.key() == from_nft_token_account.mint)]
    nft_mint: Account<'info, Mint>,

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
        associated_token::mint = nft_mint,
        associated_token::authority = to,
    )]
    to_nft_token_account: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}
