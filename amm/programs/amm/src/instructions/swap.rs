use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};
use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::{error::AmmError, Config, CONFIG_SEED, LP_SEED};

#[derive(Accounts)]
pub struct Swap<'a> {
    #[account(mut)]
    pub taker: Signer<'a>,
    pub mint_x: Account<'a, Mint>,
    pub mint_y: Account<'a, Mint>,

    #[account(
        has_one=mint_x,
        has_one=mint_y,
        seeds = [CONFIG_SEED, config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'a, Config>,

    // ATA to store tokens X & Y for PROGRAM
    #[account( mut, associated_token::mint=mint_x, associated_token::authority=config)]
    pub vault_x: Account<'a, TokenAccount>,
    #[account( mut, associated_token::mint=mint_y, associated_token::authority=config)]
    pub vault_y: Account<'a, TokenAccount>,

    // ATA to store tokens X & Y for USER
    #[account( mut, associated_token::mint=mint_x, associated_token::authority=taker)]
    pub taker_x: Account<'a, TokenAccount>,
    #[account( mut, associated_token::mint=mint_y, associated_token::authority=taker)]
    pub taker_y: Account<'a, TokenAccount>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=mint_lp,
        associated_token::authority=taker,
    )]
    pub user_lp: Account<'a, TokenAccount>, // LP for user ?

    #[account( mut, seeds = [LP_SEED, config.key().as_ref()], bump = config.lp_bump)]
    pub mint_lp: Account<'a, Mint>, // LP for program

    // PROGRAMS:
    pub system_program: Program<'a, System>,
    pub token_program: Program<'a, Token>,
    pub associated_token_program: Program<'a, AssociatedToken>,
}

impl<'a> Swap<'a> {
    pub fn swap(&self, is_x: bool, amount: u64, min: u64) -> Result<()> {
        require_eq!(self.config.locked, true, AmmError::PoolLocked);
        require_neq!(amount, 0, AmmError::InvalidAmount);

        let mut curve = ConstantProduct::init(
            self.vault_x.amount,
            self.vault_y.amount,
            self.mint_lp.supply,
            self.config.fee,
            None,
        )
        .map_err(AmmError::from)?;

        let pool = if is_x {
            LiquidityPair::X
        } else {
            LiquidityPair::Y
        };

        let result = curve.swap(pool, amount, min).map_err(AmmError::from)?;
        self.deposit_tokens(is_x, result.deposit);
        self.withdraw_tokens(is_x, result.withdraw);

        Ok(())
    }

    pub fn deposit_tokens(&self, is_x: bool, amount: u64) -> Result<()> {
        // define the token we deposit to vault (as we can't store X token in Y vault)
        let (from, to) = match is_x {
            true => (
                self.taker_x.to_account_info(),
                self.vault_x.to_account_info(),
            ),
            false => (
                self.taker_y.to_account_info(),
                self.vault_y.to_account_info(),
            ),
        };

        // setup context, program address & accounts
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.taker.to_account_info(),
        };
        let ctx = CpiContext::new(cpi_program, cpi_accounts);

        // transfer tokens to account
        transfer(ctx, amount)?;
        Ok(())
    }

    pub fn withdraw_tokens(&self, is_x: bool, amount: u64) -> Result<()> {
        // define the token we deposit to vault (as we can't store X token in Y vault)
        let (from, to) = match is_x {
            true => (
                self.vault_x.to_account_info(),
                self.taker_x.to_account_info(),
            ),
            false => (
                self.vault_y.to_account_info(),
                self.taker_y.to_account_info(),
            ),
        };

        // setup context, program address & accounts
        let program = self.token_program.to_account_info();
        let accounts = Transfer {
            from,
            to,
            authority: self.taker.to_account_info(),
        };

        let seeds = &[
            CONFIG_SEED,
            &self.config.seed.to_le_bytes(),
            &[self.config.config_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        // using with_signer, as we're now signing on the name of the program, not user
        let ctx = CpiContext::new_with_signer(program, accounts, signer_seeds);

        // transfer tokens to account
        transfer(ctx, amount)?;
        Ok(())
    }
}
