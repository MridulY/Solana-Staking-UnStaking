use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer};
use crate::{error::ErrorCode, state::{staking_data::StakingData, user_stake::UserStake}};

pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
    // Get amounts before transfer
    let amount = ctx.accounts.user_stake.staked_amount;
    let current_time = Clock::get()?.unix_timestamp;

    require!(
        current_time >= ctx.accounts.user_stake.unstake_time,
        ErrorCode::LockPeriodNotElapsed
    );

    token::transfer(ctx.accounts.into_transfer_to_user_context(), amount)?;

    // Update state after transfer
    ctx.accounts.user_stake.staked_amount = 0;
    ctx.accounts.user_stake.reward_debt = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub staking_data: Account<'info, StakingData>,
    #[account(mut, close = staker)]
    pub user_stake: Account<'info, UserStake>,
    #[account(mut)]
    pub staker: Signer<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Unstake<'info> {
    pub fn into_transfer_to_user_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.staking_data.to_account_info(),
            to: self.staker.to_account_info(),
            authority: self.staking_data.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
