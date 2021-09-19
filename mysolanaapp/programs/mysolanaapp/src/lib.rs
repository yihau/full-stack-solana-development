use anchor_lang::prelude::*;

declare_id!("E67Lzkvh2u4PXomXVtvbpUKbMcMHRFDYm5QAem25a2X7");

#[program]
mod mysolanaapp {
    use super::*;

    // 因為Solana account model的關係，我們需要創造一個帳戶來儲存
    // 我們的計數結果，而不是直接把數字存在合約中
    // 這邊我們定義一個create的操作，讓帳戶能在這個合約內被初始化
    pub fn create(ctx: Context<Create>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count = 0;
        Ok(())
    }

    // 這個操作就是+1的地方，這邊會取client傳過來的計數用的帳戶
    // 然後對他+1
    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }
}

// 這個是create操作時所需要的一些參數
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 16 + 16)]
    pub base_account: Account<'info, BaseAccount>,
    pub user: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

// 這個是increment所需要的參數
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// 儲存數量的結構體
#[account]
pub struct BaseAccount {
    pub count: u64,
}