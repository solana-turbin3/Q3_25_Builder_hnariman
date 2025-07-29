use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        self, mint_to, spl_token::error::TokenError, transfer, Mint, MintTo, Token, TokenAccount,
        Transfer,
    },
    token_2022::spl_token_2022::extension::{
        memo_transfer::instruction::RequiredMemoTransfersInstruction,
        transfer_fee::instruction::TransferFeeInstruction,
    },
};
use constant_product_curve::ConstantProduct;

use crate::{
    constants::{CONFIG_SEED, LP_SEED},
    error::AmmError,
    state::Config,
};

#[derive(Accounts)]
pub struct Deposit<'a> {
    // collect pubkeys to identify what we have to work with
    #[account(mut)]
    pub user: Signer<'a>, // user
    pub mint_x: Account<'a, Mint>, // token x
    pub mint_y: Account<'a, Mint>, // token y

    // get config PDA we created in init, so we can use configs in program
    #[account(
        has_one= mint_x,
        has_one=mint_y,
        seeds = [CONFIG_SEED, config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'a, Config>,

    #[account( mut, seeds = [LP_SEED, config.key().as_ref()], bump = config.lp_bump)]
    pub mint_lp: Account<'a, Mint>, // LP for program

    // ATA to store tokens X & Y for PROGRAM
    #[account( mut, associated_token::mint = mint_x, associated_token::authority = config)]
    pub vault_x: Account<'a, TokenAccount>,

    #[account( mut, associated_token::mint = mint_y, associated_token::authority = config)]
    pub vault_y: Account<'a, TokenAccount>,

    // ATA to store tokens X & Y for USER
    #[account( mut, associated_token::mint = mint_y, associated_token::authority = user)]
    pub user_y: Account<'a, TokenAccount>,

    #[account( mut, associated_token::mint = mint_x, associated_token::authority = user)]
    pub user_x: Account<'a, TokenAccount>,

    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint=mint_lp,
        associated_token::authority=user,
    )]
    pub user_lp: Account<'a, TokenAccount>, // LP for user ?

    // programs:
    pub token_program: Program<'a, Token>,
    pub associated_token_program: Program<'a, AssociatedToken>,
    pub system_program: Program<'a, System>,
}

impl<'a> Deposit<'a> {
    pub fn deposit(&mut self, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        require_neq!(self.config.locked, false, AmmError::PoolLocked);
        require_neq!(amount, 0, AmmError::InvalidAmount);

        let lp_empty = self.mint_lp.supply == 0;
        let no_slippage_limit = self.vault_x.amount == 0 && self.vault_y.amount == 0;

        let (x, y) = match lp_empty && no_slippage_limit {
            true => (max_x, max_y),
            false => {
                let amount = ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount,
                    self.vault_y.amount,
                    self.mint_lp.supply,
                    amount,
                    6,
                )
                .unwrap();
                (amount.x, amount.y)
            }
        };

        // possibly redundant assertion?
        require_gte!(max_x, x, AmmError::SlippageExceeded);
        require_gte!(max_y, y, AmmError::SlippageExceeded);

        self.deposit_tokens(true, x);
        self.deposit_tokens(true, y);
        self.mint_lp_tokens(amount);
        Ok(())
    }

    pub fn deposit_tokens(&self, is_x: bool, amount: u64) -> Result<()> {
        // TODO: maybe can make something like:
        // deposit(token,vault,amount)?;
        // so no need to check for x - les CU?
        // -- also can use some builder pattern here, maybe?

        // define the token we deposit to vault (as we can't store X token in Y vault)
        let (from, to) = match is_x {
            true => (
                self.user_x.to_account_info(),
                self.vault_x.to_account_info(),
            ),
            false => (
                self.user_y.to_account_info(),
                self.vault_y.to_account_info(),
            ),
        };

        // setup context, program address & accounts
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.user.to_account_info(),
        };
        let ctx = CpiContext::new(cpi_program, cpi_accounts);

        // transfer tokens to account
        transfer(ctx, amount)?;
        Ok(())
    }

    pub fn mint_lp_tokens(&self, amount: u64) -> Result<()> {
        // setup context, accounts & seeds
        let program = self.token_program.to_account_info();
        let accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.user_lp.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let seeds = &[
            CONFIG_SEED,
            &self.config.seed.to_le_bytes(),
            &[self.config.config_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        // using with_signer, as we're now signing on the name of the program, not user
        let ctx = CpiContext::new_with_signer(program, accounts, signer_seeds);

        // Anchor method to -> set ix & invoke_signed
        mint_to(ctx, amount)?;
        Ok(())
    }
}
