use anchor_lang::prelude::*;

#[account]
pub struct StakingData {
    pub authority: Pubkey,
    pub apy: u64,
    pub staking_start: i64,
    pub staking_end: i64,
    pub lock_duration: i64,
    pub reward_pool: u64,
}
