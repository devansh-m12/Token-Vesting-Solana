#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

#[program]
pub mod tokenvesting {
    use super::*;

  pub fn close(_ctx: Context<CloseTokenvesting>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.tokenvesting.count = ctx.accounts.tokenvesting.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.tokenvesting.count = ctx.accounts.tokenvesting.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeTokenvesting>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.tokenvesting.count = value.clone();
    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeTokenvesting<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + Tokenvesting::INIT_SPACE,
  payer = payer
  )]
  pub tokenvesting: Account<'info, Tokenvesting>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseTokenvesting<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub tokenvesting: Account<'info, Tokenvesting>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub tokenvesting: Account<'info, Tokenvesting>,
}

#[account]
#[derive(InitSpace)]
pub struct Tokenvesting {
  count: u8,
}
