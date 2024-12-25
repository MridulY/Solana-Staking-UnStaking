use anchor_lang::prelude::*;

#[account]
pub struct UserStake {
    pub owner: Pubkey,
    pub staked_amount: u64,
    pub start_time: i64,
    pub reward_debt: u64,
    pub claimed_rewards: u64,
    pub unstake_time: i64,
    pub apy_at_stake: u64,
}
