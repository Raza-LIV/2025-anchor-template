use anchor_lang::prelude::*;

declare_id!("H7sAAtdJb9xx8FhkmDBctomJYLX3KfsoWdqFuKJS5TZv");

#[program]
mod template {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        let custom_account = &mut ctx.accounts.custom_account;
        custom_account.data = data;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: u64) -> Result<()> {
        let custom_account = &mut ctx.accounts.custom_account;
        custom_account.data = data;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let custom_account = &mut ctx.accounts.custom_account;
        custom_account.data += 1;
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let custom_account = &mut ctx.accounts.custom_account;
        custom_account.data -= 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub custom_account: Account<'info, CustomAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub custom_account: Account<'info, CustomAccount>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub custom_account: Account<'info, CustomAccount>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub custom_account: Account<'info, CustomAccount>,
}

#[account]
pub struct CustomAccount {
    pub data: u64,
}