use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{error::ErrorCode, state::{staking_data::StakingData, user_stake::UserStake}};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub staking_data: Account<'info, StakingData>,   // Account containing staking data
    #[account(mut)]
    pub user_stake: Account<'info, UserStake>,       // User's staking record
    #[account(mut)]
    pub staker: Signer<'info>,                       // The signer initiating the staking
    #[account(mut)]
    pub staker_token_account: Account<'info, TokenAccount>, // User's token account
    #[account(mut)]
    pub staking_pool_account: Account<'info, TokenAccount>, // Staking pool account
    pub authority: Signer<'info>,                    // The program's authority (usually the program itself)
    pub token_program: Program<'info, Token>,        // Token program for token operations
}

pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let staking_data = &ctx.accounts.staking_data;
    let user_stake = &mut ctx.accounts.user_stake;

    let current_time = Clock::get()?.unix_timestamp;

    // Ensure staking is allowed in the given timeframe
    require!(
        current_time >= staking_data.staking_start && current_time <= staking_data.staking_end,
        ErrorCode::StakingNotAllowed
    );

    // Check if the user has already staked
    require!(user_stake.staked_amount == 0, ErrorCode::AlreadyStaked);

    // Prepare the accounts for the token transfer
    let cpi_accounts = Transfer {
        from: ctx.accounts.staker_token_account.to_account_info(),
        to: ctx.accounts.staking_pool_account.to_account_info(),
        authority: ctx.accounts.staker.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    // Perform the token transfer
    token::transfer(cpi_ctx, amount)?;

    // Update the user's staking record
    user_stake.owner = ctx.accounts.staker.key();
    user_stake.staked_amount = amount;
    user_stake.start_time = current_time;
    user_stake.unstake_time = current_time + staking_data.lock_duration;
    user_stake.apy_at_stake = staking_data.apy;
    user_stake.reward_debt = 0;
    user_stake.claimed_rewards = 0;

    Ok(())
}
