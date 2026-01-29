use anchor_lang::prelude::*;

declare_id!("Ddo3XibafryhyEh8zbAaW4yy1AwszT6QHee4YMRj1fLQ");

#[program]
pub mod pxsol_ss {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        let account_user = &ctx.accounts.user;
        let account_user_pda = &mut ctx.accounts.user_pda;
        account_user_pda.auth = account_user.key();
        account_user_pda.bump = ctx.bumps.user_pda;
        account_user_pda.data = Vec::new();

        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: Vec<u8>) -> Result<()> {
        let account_user = &ctx.accounts.user;
        let account_user_pda = &mut ctx.accounts.user_pda;

        // Update the data field with the new data.
        account_user_pda.data = data;

        // If the account was shrunk, Anchor won't automatically refund excess lamports. Refund any surplus (over the
        // new rent-exempt minimum) back to the user.
        let account_user_pda_info = account_user_pda.to_account_info();
        let rent_exemption = Rent::get()?.minimum_balance(account_user_pda_info.data_len());
        let hold = **account_user_pda_info.lamports.borrow();
        if hold > rent_exemption {
            let refund = hold.saturating_sub(rent_exemption);
            **account_user_pda_info.lamports.borrow_mut() = rent_exemption;
            **account_user.lamports.borrow_mut() =
                account_user.lamports().checked_add(refund).unwrap();
        }
        Ok(())
    }
}

/// Fixed seed prefix for PDA derivation, ensuring a stable namespace and avoiding collisions.
const SEED: &[u8] = b"data";

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// Instruction caller; must be mutable to pay costs and sign.
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,                                  // Creates the account on first call.
        payer = user,                          // Account rent/creation cost paid by user.
        seeds = [SEED, user.key().as_ref()],   // Must match initialization to locate the same PDA.
        bump,                                  // Must match initialization to locate the same PDA.
        space = Data::space_for(0)             // Initializes space with empty data.
    )]
    pub user_pda: Account<'info, Data>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(new_data: Vec<u8>)]
pub struct Update<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED, user.key().as_ref()],
        // 使用之前存储在账户中的 bump 值, 确保 pda 地址的唯一性与合法性. 这个 bump 是在 init 时记录的.
        bump = user_pda.bump, 
        realloc = Data::space_for(new_data.len()),
        // 指定当账户需要扩容时, 由 user 支付额外的租金. 如果 user 余额不足, 交易会失败.
        realloc::payer = user, 
        realloc::zero = false,
        // 自定义约束检查, 验证调用者 user 的公钥必须与 pda 账户中存储的 auth 字段一致. 
        // 如果不一致, 会抛出 PxsolError::Unauthorized 错误. 这是一个关键的权限检查, 确保只有账户的拥有者才能更新数据.
        constraint = user_pda.auth == user.key() @ PxsolError::Unauthorized,
    )]
    pub user_pda: Account<'info, Data>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Data {
    pub auth: Pubkey,  // Authority public key; only this key can update.
    pub bump: u8,      // PDA bump value ensuring unique address.
    pub data: Vec<u8>, // Business data with variable length.
}

impl Data {
    pub fn space_for(data_len: usize) -> usize {
        // 8 (discriminator) + 32 (authority) + 1 (bump) + 4 (vec len) + data_len.
        8 + 32 + 1 + 4 + data_len
    }
}

#[error_code]
pub enum PxsolError {
    #[msg("Unauthorized")]
    Unauthorized,
}
