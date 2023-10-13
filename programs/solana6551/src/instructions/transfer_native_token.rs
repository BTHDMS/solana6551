use crate::utils::transfer_token;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use serde_json::json;

pub fn transfer_native_token(ctx: Context<TransferNativeToken>, amount: u64) -> Result<()> {
    transfer_token(
        ctx.accounts.from_token_account.to_account_info(),
        ctx.accounts.to_token_account.to_account_info(),
        ctx.accounts.from.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        amount,
    )?;

    let log = json!({
        "Func":"transferNativeToken",
        "tokenAddress":&ctx.accounts.from_token_account.mint.to_string(),
        "from":&ctx.accounts.from.key().to_string(),
        "to":&ctx.accounts.to.key().to_string()
    });
    msg!("{}", serde_json::to_string(&log).unwrap());

    Ok(())
}

#[derive(Accounts)]
pub struct TransferNativeToken<'info> {
    #[account(mut)]
    from: Signer<'info>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    to: AccountInfo<'info>,

    #[account(mut)]
    from_token_account: Account<'info, TokenAccount>,

    #[account(constraint = token_mint.key() == from_token_account.mint)]
    token_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = from,
        associated_token::mint = token_mint,
        associated_token::authority = to,
    )]
    to_token_account: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}
