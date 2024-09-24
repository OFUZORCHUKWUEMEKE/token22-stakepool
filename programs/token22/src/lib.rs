mod errors;
mod utils;
mod instructions;
mod state;
use instructions::*;
use anchor_lang::prelude::*;
declare_id!("2M8W4edWQnie9q3HKachzwXkd1FrrCA4aC3JUdNn6WYY");

#[program]
pub mod token22 {
    use super::*;
    
    pub fn init_pool(ctx: Context<InitializePool>) -> Result<()> {
        init_pool::handler(ctx)
    }

    pub fn init_stake_entry(ctx: Context<InitializeStakeEntry>) -> Result<()> {
        init_stake_entry::handler(ctx)
    }
    
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        stake::handler(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        unstake::handler(ctx)
    }
}
