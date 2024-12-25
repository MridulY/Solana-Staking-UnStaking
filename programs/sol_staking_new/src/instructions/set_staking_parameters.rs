use anchor_lang::prelude::*;

use crate::state::staking_data::StakingData;

/// Updates staking parameters in the staking data account.
pub fn set_staking_parameters(
    ctx: Context<SetStakingParameters>,
    apy: u64,             // Annual Percentage Yield (APY) for staking rewards.
    staking_start: i64,   // Timestamp for when staking starts.
    staking_end: i64,     // Timestamp for when staking ends.
    lock_duration: i64,   // Duration (in seconds) for which funds are locked.
) -> Result<()> {
    let staking_data = &mut ctx.accounts.staking_data;

    // Update the staking parameters in the staking data account.
    staking_data.apy = apy;
    staking_data.staking_start = staking_start;
    staking_data.staking_end = staking_end;
    staking_data.lock_duration = lock_duration;

    Ok(())
}

#[derive(Accounts)]
pub struct SetStakingParameters<'info> {
    #[account(mut, has_one = authority)] // Ensure staking_data belongs to the provided authority.
    pub staking_data: Account<'info, StakingData>, // Account to store staking parameters.
    pub authority: Signer<'info>, // Authorized account to update staking parameters.
}
