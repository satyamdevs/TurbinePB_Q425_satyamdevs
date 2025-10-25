use anchor_lang::prelude::*;

declare_id!("GPNewSiqQgyKcWcwRxmBqTPoSWb7h9nqLdiSpysN3QwD"); // Replace with deployed program ID later

#[program]
pub mod counter_anchor {
    use super::*;

    // Initialize the counter account
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.count = 0;
        msg!("Counter initialized to 0!");
        Ok(())
    }

    // Increment the counter
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.count += 1;
        msg!("Counter incremented: {}", counter_account.count);
        Ok(())
    }

    // Decrement the counter
    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.count -= 1;
        msg!("Counter decremented: {}", counter_account.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub counter_account: Account<'info, CounterAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, CounterAccount>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, CounterAccount>,
}

#[account]
pub struct CounterAccount {
    pub count: i32,
}
