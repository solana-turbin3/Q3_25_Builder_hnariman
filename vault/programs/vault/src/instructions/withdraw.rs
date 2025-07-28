#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use crate::{VaultState, STATE_SEED, VAULT_SEED};
use anchor_lang::{prelude::*, system_program::Transfer};

// get SOL from vault
#[derive(Accounts)]
pub struct Withdraw<'info> {
    // who's payin
    #[account(mut)]
    pub user: Signer<'info>,

    // obtain address from bump + seeds
    // mutable as we intent to change the balance
    #[account( mut, seeds=[VAULT_SEED, vault_state.key().as_ref()], bump )]
    pub vault: SystemAccount<'info>,

    // obtain address from seeds + bump
    // account typeof struct we defined
    #[account( seeds=[STATE_SEED, user.key().as_ref()], bump )]
    pub vault_state: Account<'info, VaultState>,

    // add sys prog into struct
    // typeof program, sub-type of system-program
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        // FIXME: assertions  & validation

        // get cpi address
        let cpi_program = self.system_program.to_account_info();
        // build transfer object
        let cpi_addresses = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };
        // build ctx object for cpi
        let cpi_ctx = CpiContext::new(cpi_program, cpi_addresses);
        // make transfer
        anchor_lang::system_program::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}
