#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::{prelude::*, system_program::Transfer};

use crate::{VaultState, STATE_SEED, VAULT_SEED};

// deposit SOL to vault
#[derive(Accounts)]
pub struct Deposit<'info> {
    // who's paying
    #[account(mut)]
    pub user: Signer<'info>,

    // generate vault account hash by seeds & bump
    // mutable bc. we will update account balance (deposit/withdraw)
    #[account( mut, seeds= [VAULT_SEED, vault_state.key().as_ref()], bump )]
    pub vault: SystemAccount<'info>,

    // generate state account hash by seeds & bump
    #[account( seeds=[STATE_SEED, user.key().as_ref()], bump )]
    pub vault_state: Account<'info, VaultState>,

    // add system program into struct
    // typeof program, subtype of system-program
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amout: u64) -> Result<()> {
        // FIXME: assert if lamports are enough (maybe 1 lamport?)
        // FIXME: any other validation/assertion?

        // get system program cpi address
        let cpi_program = self.system_program.to_account_info();

        // build transaction config
        let cpi_addresses = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        // build cpi context object
        let cpi_ctx = CpiContext::new(cpi_program, cpi_addresses);

        // send SOL & ctx to cpi
        anchor_lang::system_program::transfer(cpi_ctx, amout)?;
        Ok(())
    }
}
