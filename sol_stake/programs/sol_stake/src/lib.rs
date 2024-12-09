use anchor_lang::prelude::*;

declare_id!("AZ8gzSPBt1EhogUsV4JTUQpM2hQqaYfxP8eRHDZvCbQ6");

#[program]
pub mod sol_stake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
