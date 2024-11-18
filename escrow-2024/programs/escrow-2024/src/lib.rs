

// use anchor_lang::prelude::*;
// use instructions::*;




// #[derive(Accounts)]
// pub struct Initialize {}


// pub mod instructions;
// pub mod state;

// declare_id!("BFHe17q9oRXvHq4pkGnhPnhVJhxfnk2fHzF3ejoG1NBo");

// #[program]
// pub mod anchor_project {
//     use super::*;

//     pub fn make(ctx: Context<make::Make>, amount: u64, beneficiary: Pubkey) -> Result<()> {
//         instructions::make::make(ctx, amount, beneficiary)
//     }

//     pub fn take(ctx: Context<take::Take>) -> Result<()> {
//         instructions::take::take(ctx)
//     }

//     pub fn refund(ctx: Context<refund::Refund>) -> Result<()> {
//         instructions::refund::refund(ctx)
//     }
// }

// #[error_code]
// pub enum CustomError {
//     #[msg("The escrow has already been taken.")]
//     AlreadyTaken,
// }



use anchor_lang::prelude::*;

mod state;
mod instructions;

use instructions::*;

declare_id!("BFHe17q9oRXvHq4pkGnhPnhVJhxfnk2fHzF3ejoG1NBo");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}