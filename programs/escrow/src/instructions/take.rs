use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked,transfer_checked, CloseAccount, close_account};
use anchor_spl::associated_token::AssociatedToken;
use crate::Escrow;
#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    pub user: SystemAccount<'info>,
      #[account(
        mint::token_program = token_program
    )]
    pub minta: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mint::token_program = token_program
    )]
    pub mintb: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = minta,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, associated_token::mint = mintb,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mintb,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account[
        mut,
        close = user,
        seeds = [b"escrow".as_ref(),
        escrow.user.key().as_ref(),
        &escrow.seeds.to_le_bytes()],
        bump = escrow.bump,
        has_one = user,
        has_one = minta,
        has_one = mintb]]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut, 
        associated_token::mint = minta,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}
impl <'info> Take<'info> {
    pub fn deposit(&mut self) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            to: self.user_ata_b.to_account_info(),
            mint: self.mintb.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        
        let cpi_ctx = CpiContext::new(self.token_program.key(), cpi_accounts);
        transfer_checked(cpi_ctx, self.escrow.recieve, self.mintb.decimals)
}

    pub fn withdraw_and_close(&mut self) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            mint: self.minta.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow".as_ref(),
            self.escrow.user.as_ref(),
            &self.escrow.seeds.to_le_bytes(),
            &[self.escrow.bump],
        ]];
        let cpi_ctx = CpiContext::new_with_signer(self.token_program.key(), cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, self.vault.amount, self.minta.decimals)?;

        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.user.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer( self.token_program.key(), cpi_accounts, signer_seeds);
        close_account(cpi_ctx)
    }
}