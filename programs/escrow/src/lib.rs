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

    pub fn initialize(ctx: Context<Initialize>, seeds: u64, receive: u64) -> Result<()> {
        let bump = ctx.bumps.escrow;
        ctx.accounts.initialize(seeds, receive, bump)?;
        ctx.accounts.deposit(receive)
    }
}
