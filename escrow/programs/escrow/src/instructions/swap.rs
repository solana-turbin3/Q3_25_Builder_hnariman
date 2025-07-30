use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
    token_2022::{
        spl_token_2022::extension::memo_transfer::instruction::RequiredMemoTransfersInstruction,
        AmountToUiAmountBumps,
    },
    token_interface::immutable_owner,
};
use constant_product_curve::LiquidityPair;

use crate::{
    constants::{CONF_SEED, LP_SEED},
    error::AmmError,
    state::Config,
};

#[derive(Accounts)]
pub struct Swap<'a> {
    #[account(mut)] // user who came to swap, from lp, pays the fee
    pub taker: Signer<'a>,
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [CONF_SEED, &config.seed.to_le_bytes()],
        bump = config.config_bump
    )]
    pub config: Account<'a, Config>,

    // token specifications
    pub mint_x: Account<'a, Mint>,
    pub mint_y: Account<'a, Mint>,

    // token vaults/storage -> ATA accounts
    // ATA for user
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::authority = taker,
        associated_token::mint = mint_x
    )]
    pub taker_x: Account<'a, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::authority = taker,
        associated_token::mint = mint_x
    )]
    pub taker_y: Account<'a, TokenAccount>,
    #[account( mut, associated_token::authority = config, associated_token::mint = mint_x)]
    // ATA for amm
    pub vault_x: Account<'a, TokenAccount>,
    #[account( mut, associated_token::authority = config, associated_token::mint = mint_y)]
    pub vault_y: Account<'a, TokenAccount>,

    // programs:
    pub system_program: Program<'a, System>,
    pub token_program: Program<'a, Token>,
    pub associated_token_program: Program<'a, AssociatedToken>,
}

impl<'a> Swap<'a> {
    pub fn swap(&mut self, is_x: bool, amount: u64, min: u64) -> Result<()> {
        require_eq!(self.config.locked, false, AmmError::Locked);
        require_gt!(amount, 0, AmmError::InvalidAmount);

        let mut curve = constant_product_curve::ConstantProduct::init(
            self.vault_x.amount,
            self.vault_y.amount,
            self.vault_x.amount,
            self.config.fee,
            None,
        )
        .map_err(AmmError::from)?;

        let p = match is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y,
        };

        let res = curve.swap(p, amount, min).map_err(AmmError::from)?;

        require_neq!(res.deposit, 0, AmmError::InvalidAmount);
        require_neq!(res.withdraw, 0, AmmError::InvalidAmount);
        self.deposit_token(is_x, res.deposit)?;
        self.withdraw_token(is_x, res.withdraw)?;
        Ok(())
    }

    fn deposit_token(&mut self, is_x: bool, amount: u64) -> Result<()> {
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

        let program = self.token_program.to_account_info();

        let accounts = Transfer {
            from,
            to,
            authority: self.taker.to_account_info(),
        };

        let ctx = CpiContext::new(program, accounts);
        transfer(ctx, amount)?;
        Ok(())
    }

    fn withdraw_token(&mut self, is_x: bool, amount: u64) -> Result<()> {
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

        let program = self.token_program.to_account_info();

        let accounts = Transfer {
            from,
            to,
            authority: self.taker.to_account_info(),
        };

        let seeds = &[
            &CONF_SEED[..],
            &self.config.seed.to_be_bytes(),
            &[self.config.config_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(program, accounts, signer_seeds);
        transfer(ctx, amount)?;
        Ok(())
    }
}
