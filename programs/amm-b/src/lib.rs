use anchor_lang::prelude::*;

declare_id!("GtVgafrcRjczpMSpFtwNBF24NFzamXbQCVMsYm18iLZ6");

pub mod contexts;
pub use contexts::*;

pub mod state;
pub use state::*;

pub mod errors;
pub use errors::*;

#[program]
pub mod amm_b {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed:u64, fee: u16) -> Result<()> {
        //we are going to have a fee
        ctx.accounts.init(seed, fee, &ctx.bumps)
    }

    //deposit tokens for LP
    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        unimplemented!()
    }

    //withdraw tokens for LP
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        unimplemented!()
    }

    //to swap tokens
    pub fn swap(ctx: Context<Swap>) -> Result<()> {
        unimplemented!()
    }
}
