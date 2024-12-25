use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{error::ErrorCode, state::staking_data::StakingData};

#[derive(Accounts)]
pub struct AddRewards<'info> {
   #[account(mut)] // The staking pool's data account.
   pub staking_data: Account<'info, StakingData>,
   #[account(mut)] // The user/admin adding rewards.
   pub authority: Signer<'info>,
   #[account(mut)] // Source token account for rewards.
   pub authority_token_account: Account<'info, TokenAccount>,
   #[account(mut)] // Destination staking pool token account.
   pub staking_pool_account: Account<'info, TokenAccount>,
   pub token_program: Program<'info, Token>, // Solana token program for transfers.
}

impl<'info> AddRewards<'info> {
   /// Creates a context for transferring tokens.
   pub fn into_transfer_to_pool_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
       CpiContext::new(
           self.token_program.to_account_info(), // Token program info.
           Transfer {
               from: self.authority_token_account.to_account_info(), // Source account.
               to: self.staking_pool_account.to_account_info(),     // Destination account.
               authority: self.authority.to_account_info(),         // Transfer authority.
           },
       )
   }
}

/// Transfers rewards to the staking pool and updates the reward pool amount.
pub fn add_rewards(ctx: Context<AddRewards>, amount: u64) -> Result<()> {
   // Transfer tokens from the authority to the staking pool.
   token::transfer(ctx.accounts.into_transfer_to_pool_context(), amount)?;
   // Update the reward pool balance.
   ctx.accounts.staking_data.reward_pool += amount;
   Ok(())
}
