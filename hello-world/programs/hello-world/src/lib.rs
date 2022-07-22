use anchor_lang::prelude::*;

declare_id!("DBx3vVhSEtFpqhUxe6HcHLiEbCCk32nsKzH8ErGTjxN9");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
