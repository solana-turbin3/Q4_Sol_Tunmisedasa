use anchor_lang::prelude::*;

#[error_code]

pub enum ErrorCode {
    #[msg("maximum stake reached")]
    MaxStakeReached,

    #[msg("freeze period not passed")]
    FreezePeriodNotPassed,
}