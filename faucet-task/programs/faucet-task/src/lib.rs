use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo, Transfer};
use anchor_spl::token_interface::{Mint as MintInterface, TokenAccount as TokenAccountInterface};

declare_id!("J4bunGLcnS3ZSsR8NU7Rf9ujTc4wowwSitD8gpfmqfrg");

#[program]
pub mod faucet {
    use super::*;

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        // პირობა: 0 < amount < 1,000,000
        require!(amount > 0 && amount < 1_000_000, FaucetError::AmountTooLarge);

        let seeds = &[
            b"faucet".as_ref(),
            &[ctx.accounts.faucet_state.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.faucet_state.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn mint_to_recipient(ctx: Context<MintToRecipient>, amount: u64) -> Result<()> {
        require!(amount > 0 && amount < 1_000_000, FaucetError::AmountTooLarge);

        let seeds = &[
            b"faucet".as_ref(),
            &[ctx.accounts.faucet_state.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.faucet_state.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.from_ata.to_account_info(),
            to: ctx.accounts.to_ata.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>, 
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(seeds = [b"faucet"], bump)]
    pub faucet_state: Account<'info, FaucetState>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct MintToRecipient<'info> {
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(seeds = [b"faucet"], bump)]
    pub faucet_state: Account<'info, FaucetState>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub from_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub to_ata: InterfaceAccount<'info, TokenAccount>,
    pub sender: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct FaucetState {
    pub bump: u8,
}

#[error_code]
pub enum FaucetError {
    #[msg("The requested amount is too large. Must be less than 1,000,000.")]
    AmountTooLarge,
}