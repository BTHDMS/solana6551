use crate::errors::ErrorCode;
use anchor_lang::prelude::{AccountInfo, CpiContext, Result};
use anchor_lang:: system_program;
use anchor_spl::token::{self, Transfer};

pub fn transfer_nft<'info>(
    payer: AccountInfo<'info>,
    dest: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let cpi_accounts = Transfer {
        from: payer,
        to: dest,
        authority,
    };
    let cpi_ctx = CpiContext::new(token_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)
}

pub fn transfer_token<'info>(
    payer: AccountInfo<'info>,
    dest: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let cpi_accounts = Transfer {
        from: payer,
        to: dest,
        authority,
    };
    let cpi_ctx = CpiContext::new(token_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)
}

pub fn transfer_token_with_signer<'info>(
    payer: AccountInfo<'info>,
    dest: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    amount: u64,
    signer: &[&[&[u8]]],
) -> Result<()> {
    let cpi_accounts = Transfer {
        from: payer,
        to: dest,
        authority,
    };
    let cpi_ctx = CpiContext::new_with_signer(token_program, cpi_accounts, signer);
    token::transfer(cpi_ctx, amount)
}

pub fn transfer_sol<'info>(
    payer: AccountInfo<'info>,
    dest: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let cpi_accounts = system_program::Transfer {
        from: payer,
        to: dest,
    };
    let cpi_context = CpiContext::new(system_program, cpi_accounts);
    system_program::transfer(cpi_context, amount)
}

pub fn transfer_sol_with_pda<'info>(
    payer: AccountInfo<'info>,
    dest: AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    if **payer.try_borrow_lamports()? < amount {
        return Err(ErrorCode::InsufficientFundsForTransaction.into());
    }
    // Debit from_account and credit to_account
    **payer.try_borrow_mut_lamports()? -= amount;
    **dest.try_borrow_mut_lamports()? += amount;
    Ok(())
}



