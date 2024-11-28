use anchor_lang::prelude::*;
use anchor_spl::{metadata::{mpl_token_metadata::instructions::{FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts}, MasterEditionAccount, Metadata, MetadataAccount}, token::{approve, Approve, Mint, Token, TokenAccount}};

use crate::{state::{StakeAccount, StakeConfig, UserAccount}, error::ErrorCode};




#[derive(Accounts)]


pub struct UnStake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub mint_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>,
    pub config: Account<'info, StakeConfig>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        seeds = [b"stake", mint.key().as_ref(), config.key().as_ref()],
        bump = stake.bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}




impl<'info> UnStake<'info> {
    pub fn unstake(&mut self) -> Result< () > {
        let time_elapsed = (Clock::get()?.Unix_timestamp - self.stake_account.staked_at)/86400 as u32;

        require!(time_elapsed>= self.config.freeze_period, ErrorCode::FreezePeriodNotPassed);
        Ok(())
    }
}
