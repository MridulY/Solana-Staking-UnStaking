use anchor_lang::prelude::*;
mod error;
mod instructions;
mod state;

use instructions::*;


declare_id!("9rJMCxX5tpXwx8M42Truai3RTmqGA2vsDUP1mTV9QNFY");

#[program]
pub mod sol_staking_new {
    use super::*;

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        instructions::stake::stake(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        instructions::unstake::unstake(ctx)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        instructions::claim_rewards::claim_rewards(ctx)
    }

    pub fn set_staking_parameters(
        ctx: Context<SetStakingParameters>,
        apy: u64,
        staking_start: i64,
        staking_end: i64,
        lock_duration: i64,
    ) -> Result<()> {
        instructions::set_staking_parameters::set_staking_parameters(
            ctx,
            apy,
            staking_start,
            staking_end,
            lock_duration,
        )
    }

    pub fn add_rewards(ctx: Context<AddRewards>, amount: u64) -> Result<()> {
        instructions::add_rewards::add_rewards(ctx, amount)
    }
}

