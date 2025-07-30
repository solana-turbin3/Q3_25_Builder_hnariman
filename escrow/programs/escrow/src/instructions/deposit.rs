use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer},
};

use crate::{error::AmmError, state::Config, CONF_SEED, LP_SEED};

#[derive(Accounts)]
pub struct Deposit<'a> {
    #[account(mut)] // signs/authorizes & pays 4 service
    pub maker: Signer<'a>, // declare user account
    pub mint_x: Account<'a, Mint>, // declare first token 
    pub mint_y: Account<'a, Mint>, // declare second token

    // find config account using const seed & current protocol address
    // also reuse bump generated while init,
    // make sure it has the correct tokens setup
    #[account(
        seeds = [CONF_SEED, config.key().as_ref()], 
        bump = config.config_bump,
        has_one = mint_y,
        has_one = mint_x,
    )]
    pub config: Account<'a, Config>,

    // as we have config information, can find LP mint account
    // and borrow mutably, because we will update supply field
    #[account(mut, seeds = [LP_SEED, config.key().as_ref()], bump = config.lp_bump)]
    pub lp_mint: Account<'a, Mint>,

    // now we also need mutable access to vault ATA accounts (where we store tokens)
    #[account( mut, associated_token::authority=config, associated_token::mint = mint_x)]
    pub vault_x:Account<'a, TokenAccount>,
    #[account( mut, associated_token::authority=config, associated_token::mint = mint_y)]
    pub vault_y:Account<'a, TokenAccount>,

    // and same ATA accounts for user
    // this step is important, so we know which user deposited what
    // also user only is authority, so remaining trustless
    #[account( mut, associated_token::authority=maker, associated_token::mint = mint_x)]
    pub maker_x:Account<'a, TokenAccount>,
    #[account( mut, associated_token::authority=maker, associated_token::mint = mint_y)]
    pub maker_y:Account<'a, TokenAccount>,

    #[account(
        init_if_needed, // user may be trading first time or not (!warning: re-initialize attacks)
        payer = maker,
        associated_token::authority = maker,
        associated_token::mint = lp_mint
    )]
    pub maker_lp:Account<'a, TokenAccount>,

    // CPI program addresses needed
    pub system_program: Program<'a, System>, 
    pub token_program: Program<'a, Token>,
    pub associated_token_program: Program<'a, AssociatedToken>,
}

impl <'a> Deposit<'a> {

    pub fn deposit(&mut self, amount:u64, max_x:u64, max_y:u64)->Result<()> {
        // check lock
        require_neq!(self.config.locked,true,AmmError::Locked);
        // no zero transfer
        require_neq!(amount,0,AmmError::InvalidAmount);

        let no_supply = 
        self.lp_mint.supply == 0 && 
        self.vault_y.amount == 0 && 
        self.vault_x.amount == 0;

        let (x,y) = match no_supply{
            true => (max_x,max_y), // max slippage/spread
            false => {
                let res = constant_product_curve::ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount,
                    self.vault_y.amount,
                    self.lp_mint.supply,
                    amount,
                    6
                ).unwrap(); // TODO: map err
                (res.x,res.y)
            }
        };

        // 100% slippage
        require_gte!(max_x,x,AmmError::SlippageExceeded); 
        require_gte!(max_y,y,AmmError::SlippageExceeded); 

        self.deposit_token(true, amount)?;
        self.deposit_token(false, amount)?;
        self.mint_lp_token(amount)?;

        Ok(())
    }


    pub fn deposit_token(&mut self, is_x:bool, amount:u64)-> Result<()> {

        let (from,to)= match is_x {
            true => (self.maker_x.to_account_info(),self.vault_x.to_account_info()),
            false=>(self.maker_y.to_account_info(), self.vault_y.to_account_info())
        };

        // now that we know direction, we can instruct system/token program to do transfer

        let accounts = Transfer{ from, to,authority:self.maker.to_account_info() };
        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);
        transfer(ctx, amount)//.map_err(|_|AmmError::CustomError)?
    }

    pub fn mint_lp_token(&self, amount:u64)->Result<()> {
        let accounts = MintTo {
            mint:self.lp_mint.to_account_info(),
            to: self.maker_lp.to_account_info(),
            authority:self.config.to_account_info()
        };

        let seeds = &[
            &CONF_SEED[..],
            &self.config.seed.to_le_bytes(),
            &[self.config.config_bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            accounts, 
            signer_seeds
        );

        mint_to(ctx, amount)

    }

}
