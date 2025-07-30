use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{burn, transfer, Burn, Mint, Token, TokenAccount, Transfer},
};

use crate::{
    constants::{CONF_SEED, LP_SEED},
    error::AmmError,
    state::Config,
};

#[derive(Accounts)]
pub struct Withdraw<'a> {
    #[account(mut)]
    pub taker: Signer<'a>,

    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds =[CONF_SEED, &config.seed.to_le_bytes()],
        bump = config.config_bump
    )]
    pub config: Account<'a, Config>,

    // token information
    pub mint_y: Account<'a, Mint>,
    pub mint_x: Account<'a, Mint>,
    #[account(
        mut,
        seeds=[LP_SEED,&config.seed.to_le_bytes()],
        bump = config.lp_bump
    )]
    pub lp_mint: Account<'a, Mint>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = lp_mint,
        associated_token::authority = config
    )]
    pub taker_lp: Account<'a,Mint>,

    // token storage/accounts/vaults
    #[account( 
        init_if_needed,
        payer = taker,
        associated_token::mint= mint_x,
        associated_token::authority=taker, 
    )]
    pub taker_x: Account<'a, TokenAccount>,

    #[account( 
        init_if_needed,
        payer = taker,
        associated_token::mint= mint_y,
        associated_token::authority=taker, 
    )]
    pub taker_y: Account<'a, TokenAccount>,

    #[account( mut, associated_token::authority=config, associated_token::mint= mint_x)]
    pub vault_x: Account<'a, TokenAccount>,

    #[account( mut, associated_token::authority=config, associated_token::mint= mint_y)]
    pub vault_y: Account<'a, TokenAccount>,

    // Programs:
    pub system_program: Program<'a, System>,
    pub token_program: Program<'a, Token>,
    pub associated_token_program: Program<'a, AssociatedToken>,
}

impl <'a> Withdraw <'a> {
    pub fn withdraw(
        &mut self, 
        amount:u64, // LP tokens amount to burn
        min_x:u64, // tokens user will get
        min_y:u64 // tokens user sill get
    ) -> Result<()> {
        require_neq!(self.config.locked,true, AmmError::Locked);
        require_neq!(amount,0,AmmError::InvalidAmount);
        require_neq!(min_x,0,AmmError::InvalidAmount);
        require_neq!(min_y,0,AmmError::InvalidAmount);

        let amounts = constant_product_curve::ConstantProduct::xy_withdraw_amounts_from_l(
            self.vault_x.amount, 
            self.vault_y.amount, 
            self.lp_mint.supply, 
            amount,
            6)
            .map_err(AmmError::from)?;

        require_gte!(amounts.x, min_x,AmmError::SlippageExceeded);
        require_gte!(amounts.y, min_y,AmmError::SlippageExceeded);

        self.withdraw_tokens(true,amounts.x)?;
        self.withdraw_tokens(false,amounts.y)?;
        self.burn_lp_tokens(amount)?;

        todo!()
    }

    fn withdraw_tokens(&self, is_x: bool, amount: u64) -> Result<()> {

        let (from,to) = match is_x {
           true => (self.vault_x.to_account_info(), self.taker_x.to_account_info()) ,
            false => (self.vault_y.to_account_info(), self.taker_y.to_account_info())
        };
        let program = self.token_program.to_account_info();

        let accounts = Transfer{
            from,
            to,
            authority:self.taker.to_account_info()
        };

        let seeds = &[
            &CONF_SEED[..],
            &self.config.seed.to_le_bytes(),
            &[self.config.config_bump]
        ];

        let signer_seeds = &[&seeds[..]];
        let ctx = CpiContext::new_with_signer(program,accounts, signer_seeds);

        transfer(ctx, amount)?;

        todo!()
    }

    fn burn_lp_tokens(&self, amount: u64) -> Result<()> {
        let program = self.token_program.to_account_info();

        let accounts = Burn{
            mint:self.lp_mint.to_account_info(),
            from: self.taker_lp.to_account_info(),
            authority:self.taker.to_account_info()
        };
        let ctx = CpiContext::new(program, accounts);
        burn(ctx, amount)?;
        Ok(())
    } 
}
