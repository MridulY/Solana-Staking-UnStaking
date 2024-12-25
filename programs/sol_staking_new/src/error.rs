use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Staking is not allowed at this time.")]
    StakingNotAllowed,
    #[msg("You already have an active stake.")]
    AlreadyStaked,
    #[msg("Lock period has not elapsed.")]
    LockPeriodNotElapsed,
    #[msg("No rewards available to claim.")]
    NoRewardsAvailable,
}
