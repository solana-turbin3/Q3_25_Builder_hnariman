use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    // locking/security mechanism:
    // we lock & set Authority to None
    // but what if someone sneaky will set their Pubkey instead?
    pub authority: Option<Pubkey>,
    pub locked: bool,

    pub seed: u64,

    // what tokens we have? -> (two types)
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,

    pub fee: u16,

    // pool & config are PDA's
    pub lp_bump: u8,
    pub config_bump: u8,
}
