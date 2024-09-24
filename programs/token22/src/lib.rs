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

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
