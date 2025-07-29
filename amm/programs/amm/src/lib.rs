#![allow(unexpected_cfgs, deprecated, unused)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("23UJS5Szo6zQfvqjjeLeEADdRxeT9zTD4NzQs377bY8z");

#[program]
pub mod amm {
    use super::*;

    // pub fn initialize(ctx: &Context<Initialize>) -> Result<()> {
    //     initialize::handler(ctx)
    // }
}
