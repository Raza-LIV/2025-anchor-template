use anchor_lang::prelude::*;

declare_id!("HJNDhvH673EdiJSMGhkktkH3mFi2VwvmqSBkZCWrq73q");

#[program]
pub mod min {
    use super::*;

    pub fn hello(_ctx: Context<Hello>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello<'info> {
    #[account(signer)]
    pub signer: Signer<'info>,
}
