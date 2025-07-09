#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::{prelude::*, system_program::Transfer};

use crate::{STATE_SEED, VAULT_SEED};

// init account
#[derive(Accounts)]
pub struct Initialize<'info> {
    // who's going to pay for everything
    #[account(mut)]
    pub user: Signer<'info>,

    // create PDA
    // - for users expense
    // - for the size of vault_state struct
    // - seed with user keys
    // - add bump to be deterministic
    #[account(
        init,
        payer = user,
        space = VaultState::INIT_SPACE,
        seeds = [STATE_SEED, user.key().as_ref()],
        bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    // instruct SystemProgram to create PDA
    // - mutable as we want to be able to change balance
    // - seeds to define ownership
    // - bump to have predictable/deterministic address
    #[account(
        mut,
        seeds=[VAULT_SEED,vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    // convert & add system program address into struct
    pub system_program: SystemAccount<'info>,
}

// vault bumps - for deterministic PDA hash
#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, bumps: InitializeBumps) -> Result<()> {
        // calculate lamports for rent excemption based on vault account size (the only 1 we acutally create & own)
        let rent_excempt = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());

        // get system program address, the 32x1 with needed formatting
        let cpi_program = self.system_program.to_account_info();

        // setup trasnfer to deposit excempt to vault account
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        // build state for CPI program execution
        let context = CpiContext::new(cpi_program, cpi_accounts);

        // provide CPI with funds & state
        anchor_lang::system_program::transfer(context, rent_excempt)?;

        // update vault_state bumps with the ones generated during execution?
        self.vault_state.state_bump = bumps.vault_state;
        self.vault_state.vault_bump = bumps.vault;

        Ok(())
    }
}
