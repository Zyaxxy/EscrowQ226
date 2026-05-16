use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked,transfer_checked}};
use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seeds: u64)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mint::token_program = token_program
    )]
    pub minta: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub mintb: InterfaceAccount<'info, Mint>,
    #[account(mut, associated_token::mint = minta,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub userata: InterfaceAccount<'info, TokenAccount>,

    
    #[account(
        init,
        payer = user,
        seeds = [b"escrow".as_ref(), 
        user.key().as_ref(),  &seeds.to_le_bytes()],
        space = 8 + 32 + 32 + 32 + 8 + 8 + 1,
        bump,
    )]
    pub escrow: Account<'info, Escrow>,
    pub associate_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info,TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl <'info> Initialize<'info> {
  pub fn initialize(&mut self , seeds:u64, receive: u64, bumps: u8 ) -> Result<()> {
    self.escrow.set_inner(Escrow {
        user: self.user.key(),
        minta: self.minta.key(),
        mintb: self.mintb.key(),
        seeds,
        recieve: receive,
        bump: bumps
    });
    Ok(())
  }

  pub fn deposit(&mut self, amount: u64) -> Result<()> {
    let cpi_accounts =  TransferChecked{
        mint: self.minta.to_account_info(),
        from: self.userata.to_account_info(),
        to: self.escrow.to_account_info(),
        authority: self.user.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new(self.token_program.key(), cpi_accounts);
    transfer_checked(cpi_ctx, amount, self.minta.decimals)
  }
}