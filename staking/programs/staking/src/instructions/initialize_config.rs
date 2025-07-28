use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::{StakeConfig, INITIALIZE_SEED, REWARDS_SEED};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8+ StakeConfig::INIT_SPACE,
        seeds = [INITIALIZE_SEED.as_ref()],
        bump,
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        init, // should be init_if_needed
        payer = admin,
        seeds = [REWARDS_SEED.as_ref(), config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub rewards_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> InitializeConfig<'info> {
    pub fn init(
        &mut self,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u64,
        bumps: &InitializeConfigBumps,
    ) -> Result<()> {
        self.config.set_inner(StakeConfig {
            points_per_stake,
            max_stake,
            freeze_period,
            rewards_bump: bumps.rewards_mint,
            bump: bumps.config,
        });
        Ok(())
    }
}
