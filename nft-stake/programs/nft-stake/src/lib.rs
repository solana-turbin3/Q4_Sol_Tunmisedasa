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
}

#[derive(Accounts)]
pub struct Initialize {}
