use anchor_lang::prelude::*;
use anchor_spl::{token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked}};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction]
pub struct Make<'info> {
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(mut, associated_token::mint = mint_a, associated_token::authority = maker)]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(init, payer = maker, seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()], bump, space = 8 + Escrow::INIT_SPACE)]
    pub escrow: Account<'info, Escrow>,
    #[account(init, payer = maker, associated_token::mint = mint_a, associated_token::authority = escrow)]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub struct MakeBumps {
    pub escrow: u8,
}

impl<'info> Make<'info> {
    pub fn init_escrow(&mut self, seed: u64, receive: u64, bumps: &MakeBumps) -> Result<()> {
        self.escrow = Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive,
            bump: bumps.escrow,
        };
        Ok(())
    }

    pub fn deposit(&mut self, deposit: u64) -> Result<()> {
        let cpi_program: AccountInfo<'_> = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
            mint: self.mint_a.to_account_info(),
        };

        let cpi_ctx: CpiContext<'_, '_, '_, '_, > = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, deposit, self.mint_a.decimals)?;

        Ok(())
    }
}
