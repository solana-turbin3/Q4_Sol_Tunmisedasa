
use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("2Wwp6Ego3JcekXqMhVugLWk4Ti1MUQGNCgNmKfJNhVW7");

#[program]
pub mod anchor_vault_2024 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
       ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()>{
        ctx.accounts.deposit(amount)?;

        Ok(())
    }


}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"state", user.key().as_ref()],
        bump,
        space = VaultState::INIT_SPACE,
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_account: Transfer<'_> = Transfer{
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_account);

        // Implement the actual transfer logic here if necessary

        transfer(cpi_ctx, amount)?;
        

        Ok(())
    }
}

#[derive[Accounts]]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info>{
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program : AccountInfo = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds :&[&[u8];3] = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let cpi_ctx : CpiContext<Transfer> = CpiContext::new_with_signer(
            cpi_program, 
            cpi_accounts,
            seeds
        );

        transfer(cpi_ctx, account)?;

        Ok(())
    }
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bump: &InitializeBumps) -> Result<()> {
        self.vault_state.vault_bump = bump.vault;
        self.vault_state.state_bump = bump.vault_state;
        Ok(())
    }
}

#[account]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl VaultState {
    pub const INIT_SPACE: usize = 8 + 1 + 1;
}
