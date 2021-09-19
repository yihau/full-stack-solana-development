use anchor_lang::prelude::*;

declare_id!("136uVTBv6tex1u4qnEq8XLP4fcDUqBWMxQ9kKspwShTZ");

#[program]
pub mod helloworld2 {
    use super::*;
    // init的操作
    pub fn initialize(ctx: Context<Initialize>, data: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy);
        Ok(())
    }
    // 更新資料
    pub fn update(ctx: Context<Update>, data: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 64 + 64)]
    pub base_account: Account<'info, BaseAccount>,
    pub user: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// 儲存訊息的結構
#[account]
pub struct BaseAccount {
    // 當前資料
    pub data: String,
    // 歷史資料
    pub data_list: Vec<String>,
}