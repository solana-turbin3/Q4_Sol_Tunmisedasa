use anchor_lang::prelude::*;



#[account]
#[derive(InitSapce)]

pub struct StakeAccount {
    pub owner: PubKey,
    pub mint: PubKey,
    pub stake_at: u8,
    pub bump: u8,
}