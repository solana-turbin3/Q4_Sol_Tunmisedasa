use anchor_lang::prelude::*;

declare_id!("DGZg2jcbBMvPXvmcEiL5aF96JjDy6wmD39KNmR1u6aJZ");

#[program]
pub mod nft_stake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
