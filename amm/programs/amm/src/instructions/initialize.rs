use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{
    constants::{CONFIG_SEED, LP_SEED},
    Config,
};

#[derive(Accounts)]
#[instruction(seed: u64)] // additional user input argument
pub struct Initialize<'a> {
    #[account(mut)]
    pub initializer: Signer<'a>,

    pub mint_x: Account<'a, Mint>, // anchor will find size & account address
    pub mint_y: Account<'a, Mint>, // so we get access to token specific features

    #[account(
        init,
        payer = initializer,
        seeds = [LP_SEED, config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub mint_lp: Account<'a, Mint>, // liquidity pool

    #[account(
        init,
        payer = initializer,
        seeds = [CONFIG_SEED, seed.to_le_bytes().as_ref()],
        bump,
        space = 8 + Config::INIT_SPACE
    )]
    pub config: Account<'a, Config>,

    #[account(
        init,
        payer=initializer,
        associated_token::mint = mint_x,
        associated_token::authority = config,
    )]
    pub vault_x: Account<'a, TokenAccount>,

    #[account(
        init,
        payer=initializer,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y: Account<'a, TokenAccount>,
}

impl<'a> Initialize<'a> {
    pub fn init(
        &mut self,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
        bumps: &InitializeBumps,
    ) -> Result<()> {
        self.config.set_inner(Config {
            seed,
            authority,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            fee,
            config_bump: bumps.config,
            lp_bump: bumps.mint_lp,
            locked: false,
        });

        Ok(())
    }
}
