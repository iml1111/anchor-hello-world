use anchor_lang::prelude::*;

declare_id!("GUDKwDnbiJqEbBATk4bALewDCGNdqqiN49ddFkk7HPHU");

#[program]
pub mod incrementer {
    use super::*;

    pub fn create(_ctx: Context<Create>) -> Result<()> {
        let base_account = &mut _ctx.accounts.base_account;
        base_account.count = 0;
        Ok(())
    }

    pub fn increment(_ctx: Context<Increment>) -> Result<()> {
        let base_account = &mut _ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }
}

// Transaction Instructions
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 16 + 16)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// An account that goes inside a transaction instruction
#[account]
pub struct BaseAccount {
    pub count: u64,
}
