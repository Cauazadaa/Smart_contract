use anchor_lang::prelude::*;

declare_id!("2HBRBmNiqJLJsQKzzXGngAQyRMMwQqFMt5fjqS3eTsWg");

#[program]
pub mod smart_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn increment(ctx: Context<UpdateData>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.data += 1;
        Ok(())
    }

    pub fn get_data(ctx: Context<GetData>) -> Result<u64> {
        let base_account = &ctx.accounts.base_account;
        Ok(base_account.data)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[derive(Accounts)]
pub struct GetData<'info> {
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub data: u64,
}   