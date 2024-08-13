use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("3Ym3aCtqvj78Y1M7k9yPPxeL4V49uY16LKk24qiuwd3z");

#[program]
pub mod solana_withdraw_10_deposit {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        // Chuyển SOL từ tài khoản của signer vào user_vault_account
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.user_vault_account.to_account_info(),
            },
        );
        system_program::transfer(cpi_context, amount)?;

        // Cập nhật số lần deposit và số tiền đã deposit
        let user_interactions = &mut ctx.accounts.user_interactions_counter;
        let user_balance = &mut ctx.accounts.user_balance;
        user_interactions.total_deposits = user_interactions
            .total_deposits
            .checked_add(1)
            .ok_or_else(|| error!(ErrorCode::Overflow))?;
        user_balance.total_amount_deposited = user_balance
            .total_amount_deposited
            .checked_add(amount)
            .ok_or_else(|| error!(ErrorCode::Overflow))?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        // Xác định số tiền rút ra (10% của tổng số tiền đã deposit)
        let amount = ctx
            .accounts
            .user_balance
            .total_amount_deposited
            .checked_div(10)
            .ok_or_else(|| error!(ErrorCode::DivideByZero))?;

        let bump = ctx.bumps.user_vault_account;
        let vault_seed = b"vault";
        let signer_key = ctx.accounts.signer.key();
        let signer_seeds = &[
            vault_seed.as_ref(),
            signer_key.as_ref(),
            &[bump],
        ];

        // Tạo lệnh chuyển SOL từ user_vault_account về tài khoản của signer
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user_vault_account.key(),
            &ctx.accounts.signer.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[
                ctx.accounts.user_vault_account.to_account_info(),
                ctx.accounts.signer.to_account_info(),
            ],
            &[&signer_seeds[..]],
        )?;

        // Cập nhật số lần rút tiền
        let user_interactions = &mut ctx.accounts.user_interactions_counter;
        user_interactions.total_withdrawals = user_interactions
            .total_withdrawals
            .checked_add(1)
            .ok_or_else(|| error!(ErrorCode::Overflow))?;

        Ok(())
    }
}

#[account]
pub struct UserInteractions {
    total_deposits: u64,
    total_withdrawals: u64,
}

#[account]
pub struct UserBalance {
    total_amount_deposited: u64,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// CHECK: Tài khoản này được kiểm soát bởi PDA và chỉ được sử dụng để lưu trữ SOL
    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump,
        constraint = *user_vault_account.owner == system_program.key() @ ErrorCode::InvalidAccountOwner
    )]
    pub user_vault_account: AccountInfo<'info>,

    #[account(
        init_if_needed,
        space = 8 + 8 + 8,
        seeds = [b"counter", signer.key().as_ref()],
        bump,
        payer = signer
    )]
    pub user_interactions_counter: Account<'info, UserInteractions>,

    #[account(
        init_if_needed,
        space = 8 + 8,
        seeds = [b"balance", signer.key().as_ref()],
        bump,
        payer = signer
    )]
    pub user_balance: Account<'info, UserBalance>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: This is safe because we're using it as a pure account and not parsing it
    #[account(mut, seeds = [b"vault", signer.key().as_ref()], bump)]
    pub user_vault_account: AccountInfo<'info>,

    #[account(mut, seeds = [b"counter", signer.key().as_ref()], bump)]
    pub user_interactions_counter: Account<'info, UserInteractions>,

    #[account(mut, seeds = [b"balance", signer.key().as_ref()], bump)]
    pub user_balance: Account<'info, UserBalance>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Operation overflowed.")]
    Overflow,
    #[msg("Division by zero error.")]
    DivideByZero,
    #[msg("Invalid account owner.")]
    InvalidAccountOwner,
}