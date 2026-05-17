use anchor_lang::prelude::*;
use crate::Escrow;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked,transfer_checked , CloseAccount, close_account};
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, associated_token::mint = minta,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, associated_token::mint = minta,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(mut,
        close = user,
        seeds = [b"escrow".as_ref(),
        escrow.user.key().as_ref(),
        &escrow.seeds.to_le_bytes()],
        bump = escrow.bump,
        has_one = user,
        has_one = minta)]
    pub escrow: Account<'info, Escrow>,
    pub minta: InterfaceAccount<'info, Mint>,
    pub mintb: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>          
}

impl <'info> Refund<'info> {
    pub fn refund(&mut self) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.user_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
            mint: self.minta.to_account_info(),
        };
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow".as_ref(),
            self.escrow.user.as_ref(),
            &self.escrow.seeds.to_le_bytes(),
            &[self.escrow.bump],
        ]];
        let cpi_ctx = CpiContext::new_with_signer(self.token_program.key(), cpi_accounts, signer_seeds);
        transfer_checked(cpi_ctx, self.escrow.recieve, self.minta.decimals)?;
        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.user.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer( self.token_program.key(), cpi_accounts, signer_seeds);
        close_account(cpi_ctx)
    }
}