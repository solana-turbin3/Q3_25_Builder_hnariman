use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata,
    },
    token::{revoke, Mint, Revoke, Token, TokenAccount},
};

use crate::{
    constants::{METADATA_SEED, STAKE_SEED, USER_SEED},
    StakeAccount, StakeConfig, UserAccount, CONFIG_SEED, EDITION_SEED,
};

#[derive(Accounts)]
pub struct Unstake<'info> {
    // one who pays
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub collection_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_mint_ata: Account<'info, TokenAccount>,

    #[account(
        seeds=[METADATA_SEED, metadata_program.key().as_ref(), mint.key().as_ref(), EDITION_SEED],
        seeds::program = metadata_program.key(),
        bump
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    #[account(
        mut,
        seeds = [STAKE_SEED, mint.key().as_ref(), config.key().as_ref()],
        bump,
        close = user
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        mut,
        seeds = [USER_SEED, user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        seeds = [CONFIG_SEED.as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self, _bumps: &UnstakeBumps) -> Result<()> {
        // let time_elapsed =
        //     ((Clock::get()?.unix_timestamp - self.stake_account.staked_at) / 86400) as u32;
        //
        // let current_time = Clock::get()?.unix_timestamp;
        // assert!(time_elapsed >= self.config.freeze_period);

        let seeds = &[
            STAKE_SEED,
            self.mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump],
        ];

        let signer_seeds = &[&seeds[..]];
        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.user_mint_ata.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();

        ThawDelegatedAccountCpi::new(
            metadata_program,
            ThawDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program,
            },
        )
        .invoke_signed(signer_seeds);

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Revoke {
            source: self.user_mint_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        revoke(cpi_ctx);

        self.user_account.amount_staked -= 1;

        Ok(())
    }
}

// create method claim
