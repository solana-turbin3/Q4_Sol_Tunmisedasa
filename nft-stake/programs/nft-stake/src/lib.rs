pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;


use anchor_lang::prelude::*;


pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("DGZg2jcbBMvPXvmcEiL5aF96JjDy6wmD39KNmR1u6aJZ");

#[program]
pub mod nft_stake {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>, points_per_stake: u8, max_stake: u8, freeze_period: u32) -> Result<()> {
        ctx.accounts.initialize_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)
    }

    pub fn initialize_user(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
}

#[derive(Accounts)]
pub struct Initialize {}
