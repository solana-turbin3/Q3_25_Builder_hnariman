#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use crate::{VaultState, STATE_SEED, VAULT_SEED};
use anchor_lang::{prelude::*, system_program::Transfer};

#[derive(Accounts)]
pub struct Close<'info> {
    // owner who can close
    #[account(mut)]
    pub user: Signer<'info>,

    // vault account controlled by system program cpi call
    #[account(mut, seeds=[VAULT_SEED,vault_state.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,

    // account to be closed
    #[account(mut, seeds=[STATE_SEED, user.key().as_ref()], bump)]
    pub vault_state: Account<'info, VaultState>,

    // system program address
    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let vault_balance = self.vault.get_lamports();
        let cpi_program = self.system_program.to_account_info();

        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let ctx = CpiContext::new(cpi_program, accounts);

        anchor_lang::system_program::transfer(ctx, vault_balance)?;

        Ok(())
    }
}
