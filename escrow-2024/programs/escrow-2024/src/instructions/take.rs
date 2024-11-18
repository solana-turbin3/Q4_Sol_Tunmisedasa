use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,     // The vault holding the tokens
    #[account(mut)]
    pub taker: Signer<'info>,                     // The account of the taker
    #[account(mut)]
    pub maker: Account<'info, TokenAccount>,       // The maker account (the one receiving the tokens)
    pub token_program: Program<'info, Token>,      // The token program (e.g., SPL Token program)
}

pub fn take(ctx: Context<Take>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let taker = &mut ctx.accounts.taker;
    let maker = &mut ctx.accounts.maker;
    let token_program = &ctx.accounts.token_program;

    // Deposit tokens from taker to maker
    let cpi_ctx = CpiContext::new(
        token_program.to_account_info(),
        token::Transfer {
            from: taker.to_account_info(),
            to: maker.to_account_info(),
            authority: taker.to_account_info(),
        },
    );

    token::transfer(cpi_ctx, amount)?;

    // Transfer tokens from the vault to the taker
    let cpi_ctx = CpiContext::new(
        token_program.to_account_info(),
        token::Transfer {
            from: vault.to_account_info(),
            to: taker.to_account_info(),
            authority: maker.to_account_info(),
        },
    );

    token::transfer(cpi_ctx, amount)?;

    // Close the vault account
    token:: CpiContext<'_, '_, '_, '_, > = close_account(vault.to_account_info())?;

    Ok(())
}
