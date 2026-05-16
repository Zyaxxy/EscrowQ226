use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub user: Pubkey,
    pub minta: Pubkey,
    pub mintb: Pubkey,
    pub seeds: u64,
    pub recieve: u64,
    pub bump: u8
}