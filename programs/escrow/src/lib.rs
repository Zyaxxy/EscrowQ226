pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("J5BXxu5jbxKrQz4roxntix383yqEMP7gpmsoK36Euoq8");

#[program]
pub mod escrow {
    use super::*;
    #[instruction(discriminator = 0)]
    pub fn initialize(ctx: Context<Initialize>, seeds: u64, receive: u64, deposit: u64) -> Result<()> {
        ctx.accounts.initialize(seeds, receive, ctx.bumps.escrow, deposit)?;
        ctx.accounts.deposit(deposit)
    }

    #[instruction(discriminator = 1)]
    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close()
    }

    #[instruction(discriminator = 2)]
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund()}
}
