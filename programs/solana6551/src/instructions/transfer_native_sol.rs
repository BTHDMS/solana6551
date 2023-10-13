use crate::utils::transfer_sol;
use anchor_lang::prelude::*;
use serde_json::json;

pub fn transfer_native_sol(ctx: Context<TransferNativeSol>, amount: u64) -> Result<()> {
    transfer_sol(
        ctx.accounts.from.to_account_info(),
        ctx.accounts.to.to_account_info(),
        ctx.accounts.from.to_account_info(),
        amount,
    )?;

    let log = json!({
        "Func":"transferNativeSol",
        "from":&ctx.accounts.from.key().to_string(),
        "to":&ctx.accounts.to.key().to_string(),
        "amount":amount
    });
    msg!("{}", serde_json::to_string(&log).unwrap());

    Ok(())
}

#[derive(Accounts)]
pub struct TransferNativeSol<'info> {
    #[account(mut)]
    from: Signer<'info>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    to: AccountInfo<'info>,

    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}
