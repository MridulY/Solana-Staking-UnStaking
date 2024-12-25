use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{error::ErrorCode, state::{staking_data::StakingData, user_stake::UserStake}};

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
   #[account(mut)] // The staking pool's data account.
   pub staking_data: Account<'info, StakingData>,
   #[account(mut)] // User's stake account holding staking info.
   pub user_stake: Account<'info, UserStake>,
   #[account(mut)] // The staker claiming rewards.
   pub staker: Signer<'info>,
   #[account(mut)] // Staker's token account to receive rewards.
   pub staker_token_account: Account<'info, TokenAccount>,
   #[account(mut)] // Staking pool's token account to transfer rewards from.
   pub staking_pool_account: Account<'info, TokenAccount>,
   pub authority: Signer<'info>, // Authorized account for transferring tokens.
   pub token_program: Program<'info, Token>, // Solana token program for transfers.
}

impl<'info> ClaimRewards<'info> {
   /// Creates a context for transferring rewards to the staker.
   pub fn into_transfer_rewards_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
       CpiContext::new(
           self.token_program.to_account_info(), // Token program info.
           Transfer {
               from: self.staking_pool_account.to_account_info(), // Source: staking pool.
               to: self.staker_token_account.to_account_info(),   // Destination: staker's token account.
               authority: self.authority.to_account_info(),       // Transfer authority.
           },
       )
   }
}

/// Handles claiming rewards for a user.
pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
   // Calculate the time the tokens have been staked.
   let time_staked = Clock::get()?.unix_timestamp - ctx.accounts.user_stake.start_time;
   require!(time_staked > 0, ErrorCode::NoRewardsAvailable); // Ensure staking duration is valid.

   // Compute the rewards based on staked amount, APY, and time staked.
   let rewards = calculate_rewards(
       ctx.accounts.user_stake.staked_amount,
       ctx.accounts.user_stake.apy_at_stake,
       time_staked
   );

   // Transfer the calculated rewards to the staker's token account.
   token::transfer(
       ctx.accounts.into_transfer_rewards_context(),
       rewards
   )?;

   // Update the user's claimed rewards.
   ctx.accounts.user_stake.claimed_rewards += rewards;
   Ok(())
}

/// Calculates rewards based on staked amount, annual percentage yield (APY), and staking duration.
fn calculate_rewards(amount: u64, apy: u64, time_staked: i64) -> u64 {
   let seconds_in_year: i64 = 365 * 24 * 60 * 60; // Total seconds in a year.
   let rate_per_second = apy as f64 / 100.0 / seconds_in_year as f64; // APY converted to a per-second rate.
   (amount as f64 * rate_per_second * time_staked as f64) as u64 // Compute rewards.
}
