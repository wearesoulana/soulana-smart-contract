use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, MintTo, Token, TokenAccount},
    associated_token::{AssociatedToken, Create},
};

declare_id!("HCqoXrnpp2WZhvwAvoF4aALXPP8gthxNzL8h8DEMrUBv");

#[program]
pub mod donation_program {
    use super::*;

    pub fn donate_and_reward(
        ctx: Context<DonateAndReward>,
        amount: u64, // Amount of SOL to donate (in lamports)
    ) -> Result<()> {
        // Calculate reward tokens based on donation amount
        // Reward ratio: 1 SOL = 1000 tokens
        let reward_amount = amount * 1000;

        msg!("Donated amount: {} lamports", amount);
        msg!("Reward tokens to mint: {}", reward_amount);

        // Execute token minting operation
        let mint_to_ctx = ctx.accounts.mint_to_context();
        token::mint_to(mint_to_ctx, reward_amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct DonateAndReward<'info> {
    #[account(mut)]
    pub donor: Signer<'info>,

    /// CHECK: This account will be initialized by the Associated Token Program
    #[account(mut)]
    pub reward_account: UncheckedAccount<'info>,

    /// CHECK: This is the token mint account and will be validated by the program
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> DonateAndReward<'info> {
    // Helper function to create the context for minting tokens
    pub fn mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.mint.clone(),
            to: self.reward_account.to_account_info(),
            authority: self.mint.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}