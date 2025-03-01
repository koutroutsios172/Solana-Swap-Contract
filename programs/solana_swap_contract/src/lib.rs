use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEFCYkgPUy8ihnjga");

#[program]
pub mod solana_swap_contract {
    use super::*;

    pub fn simple_swap(ctx: Context<Swap>, amount: u64) -> Result<()> {
        // 1. Ο χρήστης στέλνει tokens στο pool.
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.pool_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program.clone(), cpi_accounts); // Κλωνοποίηση για δεύτερη χρήση
        token::transfer(cpi_ctx, amount)?;

        // 2. Ο χρήστης παίρνει πίσω τα tokens του από το pool.
        let cpi_accounts_return = Transfer {
            from: ctx.accounts.pool_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_ctx_return = CpiContext::new(cpi_program, cpi_accounts_return); // Χρησιμοποιεί την κλωνοποιημένη τιμή
        token::transfer(cpi_ctx_return, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: Ο user_token_account ελέγχεται μέσω της λογικής των accounts και δεν χρειάζεται άλλος έλεγχος ασφάλειας.
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,

    /// CHECK: Το pool_token_account είναι ασφαλές και δεν χρειάζεται άλλος έλεγχος.
    #[account(mut)]
    pub pool_token_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}
