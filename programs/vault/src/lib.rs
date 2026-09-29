use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("VauLt1111111111111111111111111111111111111");

#[program]
pub mod vault {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, max_withdrawl: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner = ctx.accounts.owner.key();
        vault.balance = 0;
        vault.max_withdrawal = max_withdrawl;
        vault.paused = false;
        vault.bump = ctx.bumps.vault;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::ZeroAmt);

        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.depositor.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        );

        transfer(cpi_ctx, amount)?;

        let vault = &mut ctx.accounts.vault;
        vault.balance = vault
            .balance
            .checked_add(amount)
            .ok_or(VaultError::Overflow)?;

        let new_balance = vault.balance;
    
        emit!(DepositEvent{
            vault: ctx.accounts.vault.key(),
            depositor: ctx.accounts.depositor.key(),
            amount,
            new_balance,
        });
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::ZeroAmt);

        let vault = &mut ctx.accounts.vault;
        require!(amount <= vault.balance, VaultError::InsufficientFunds);

        vault.balance = vault
            .balance
            .checked_sub(amount)
            .ok_or(VaultError::InsufficientFunds)?;

        let new_balance = vault.balance;

        let vault_lamports = ctx.accounts.vault.to_account_info().lamports();
        let owner_lamports = ctx.accounts.owner.to_account_info().lamports();

        **ctx
            .accounts
            .vault
            .to_account_info()
            .try_borrow_mut_lamports()? = vault_lamports
            .checked_sub(amount)
            .ok_or(VaultError::InsufficientFunds)?;

        **ctx
            .accounts
            .owner
            .to_account_info()
            .try_borrow_mut_lamports()? = owner_lamports
            .checked_add(amount)
            .ok_or(VaultError::Overflow)?;

        emit!(WithdrawEvent{
            vault: ctx.accounts.vault.key(),
            owner: ctx.accounts.owner.key(),
            amount,
            new_balance
        });

        Ok(())
    }

    pub fn close_vault(ctx: Context<CloseVault>) -> Result<()>{
        emit!(VaultClosed{
            vault: ctx.accounts.vault.key(),
            owner: ctx.accounts.owner.key(),
            swept_lamports: ctx.accounts.vault.to_account_info().lamports(),
        });

        Ok(())

    }

    pub fn set_paused(ctx: Context<SetPaused>, paused: bool) -> Result<()>{
        ctx.accounts.vault.paused = paused;

        emit!(PauseEvent{
            vault: ctx.accounts.vault.key(),
            paused
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault", owner.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"vault", vault.owner.as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub depositor: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        has_one = owner,
        seeds = [b"vault", owner.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub owner: Signer<'info>,
}


#[derive(Accounts)]
pub struct SetPaused<'info> {
    #[account(
        mut,
        has_one = owner,
        seeds = [b"vault", owner.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseVault<'info> {
    #[account(
        mut,
        close = owner, 
        has_one = owner,
        seeds = [b"vault", owner.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[event]
pub struct VaultClosed {
    pub vault: Pubkey,
    pub owner: Pubkey,
    pub swept_lamports: u64,
}

#[event]
pub struct DepositEvent {
    pub vault: Pubkey,
    pub depositor: Pubkey,
    pub amount: u64,
    pub new_balance: u64,
}

#[event]
pub struct PauseEvent{
    pub vault: Pubkey,
    pub paused: bool
}

#[event]
pub struct VaultPaused {
    pub vault: Pubkey,
    pub owner: Pubkey,
    pub swept_lamports: u64,
}

#[event]
pub struct WithdrawEvent{
    pub vault: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub new_balance: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub owner: Pubkey,
    pub balance: u64,
    pub max_withdrawal: u64,
    pub paused: bool,
    pub bump: u8,
}

#[error_code]
pub enum VaultError {
    #[msg("Amount exceeds vault balance.")]
    InsufficientFunds,
    #[msg("Amount is zero.")]
    ZeroAmt,
    #[msg("overflow")]
    Overflow,
     #[msg("Amount exceeds the per-transaction withdrawal limit.")]
    WithdrawalLimitExceeded,
    #[msg("Withdrawal limit must be greater than zero.")]
    InvalidLimit,
    #[msg("Vault is paused.")]
    VaultPaused,
}
