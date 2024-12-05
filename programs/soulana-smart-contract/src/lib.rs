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
        amount: u64, // Bağış miktarı (SOL)
    ) -> Result<()> {
        // Bağış miktarı başına ödül hesaplama
        let reward_amount = amount * 1000;

        msg!("Donated amount: {} lamports", amount);
        msg!("Reward tokens to mint: {}", reward_amount);

        // Token mint etme işlemi
        let mint_to_ctx = ctx.accounts.mint_to_context();
        token::mint_to(mint_to_ctx, reward_amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct DonateAndReward<'info> {
    #[account(mut)]
    pub donor: Signer<'info>,

    /// CHECK: Bu hesap Associated Token Program tarafından başlatılacak
    #[account(mut)]
    pub reward_account: UncheckedAccount<'info>,

    /// CHECK: Bu hesap token mint hesabıdır ve program tarafından kontrol edilecektir
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> DonateAndReward<'info> {
    pub fn mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.mint.clone(),
            to: self.reward_account.to_account_info(),
            authority: self.mint.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}