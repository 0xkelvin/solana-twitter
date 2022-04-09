use anchor_lang::prelude::*;

declare_id!("EGGC7zME9gqRmGLP2UrrBKdjbCterC6cz2RXfwFtQyxg");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
