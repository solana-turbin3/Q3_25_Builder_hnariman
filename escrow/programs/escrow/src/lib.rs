#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2bjLWGibXhFThGJbPef6xFHhp9DAX8Rivs47edhAKcRN");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("TBA");
        // ctx.accounts.init(seed, fee, authority, bumps)
        Ok(())
    }
}
