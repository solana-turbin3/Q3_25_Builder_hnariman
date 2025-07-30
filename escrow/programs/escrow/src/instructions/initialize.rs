use crate::{state::Config, CONF_SEED, LP_SEED};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct Initialize<'a> {
    // init instruction will use some on-chain space
    // so we need someone who pays rent, a founder who inits, 
    // it's mutable -> allow transfer rent fee
    #[account(mut)] 
    pub initializer: Signer<'a>,

    #[account(
        init,
        payer=initializer,
        space= 8 + Config::INIT_SPACE,
        seeds=[CONF_SEED, config.key().as_ref()],
        bump,
    )]
    pub config: Account<'a, Config>, // protocol configs stored in account on-chain

    // BUSINES LOGIC:
    // information/specification about tokens & liquidity pool 
    pub mint_x: Account<'a, Mint>, // trade x for y
    pub mint_y: Account<'a, Mint>, // so we need address with tokens information
    #[account(
        init, 
        payer = initializer,
        seeds = [LP_SEED,config.key().as_ref()],
        bump,
        mint::decimals = 6,        // helps calculate space for account
        mint::authority = config   // required by mint::decimals & provide security
    )]
    pub mint_lp: Account<'a, Mint>, // we create new account to store pool for "token pair" 

    // now that we have token descriptions in Mint
    // we need to store token itself
    // so init basic accunt, as per token mint specs & which is owned by config (ie this protocol)
    #[account( 
        init,
        payer = initializer, // rent is paid
        associated_token::mint = mint_x, // how much rent (size based on token specs)
        associated_token::authority = config, // required by mint - who has access to mint?
    )]
    pub vault_x: Account<'a,TokenAccount>,
    #[account( 
        init,
        payer = initializer, // rent is paid
        associated_token::mint = mint_y, // how much rent (size based on token specs)
        associated_token::authority = config, // required by mint - who has access to mint?
    )]
    pub vault_y: Account<'a,TokenAccount>,

    // programs:
    pub system_program: Program<'a, System>,
    pub token_program: Program<'a, Token>,
    pub associated_token_program: Program<'a, AssociatedToken>,
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
            authority,
            locked: false,
            seed,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            fee,
            lp_bump: bumps.config,
            config_bump: bumps.mint_lp
        });

        Ok(())
    }
}
