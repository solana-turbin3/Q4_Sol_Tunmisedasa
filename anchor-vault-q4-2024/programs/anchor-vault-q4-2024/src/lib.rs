use anchor_lang::prelude::*;

declare_id!("838KLiLUfC9WcNgr12XsG78PN2BPKHWAfeyGjGiy7ED3");

#[program]
pub mod anchor_vault_q4_2024 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
