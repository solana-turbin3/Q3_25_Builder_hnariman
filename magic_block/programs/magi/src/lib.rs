#![allow(deprecated, unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

use ephemeral_rollups_sdk::anchor::{commit, delegate, ephemeral};

declare_id!("CLiC3inxLTpBhQxVecfXShgzU36xT3YFJxXWPa9wXubN");

pub const TEST_PDA_SEED: &[u8] = b"test-pda";
#[ephemeral]
#[program]
pub mod magi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;

        if counter.count > 10000 {
            counter.count = 0;
        }
        Ok(())
    }

    pub fn delegate(ctx: Context) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer=user,
        space=8+8,
        seeds = [TEST_PDA_SEED],
        bump
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    pub count: u64,
}

#[derive(Accounts)]
pub struct Increment {}

#[delegate]
#[derive(Accounts)]
pub struct DelegateCounter {}
