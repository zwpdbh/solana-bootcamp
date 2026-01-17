use anchor_lang::prelude::*;

declare_id!("CqgzbgfP7ZwFYK2Wi9DkKJ74Rn12UMN1EggAR9ur3V3D");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        use anchor_lang::system_program::{transfer, Transfer};

        // 1. verify the vault is empty to prevent double deposits
        require_eq!(
            ctx.accounts.vault.lamports(),
            0,
            VaultError::VaultAlreadyExists
        );

        // 2. Ensurethe deposite amout exceeds the rent-exempt minimum for a SystemAccount
        require_eq!(
            amount,
            Rent::get()?.minimum_balance(0),
            VaultError::InvalidAmount
        );

        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.signer.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }
    pub fn withdraw(ctx: Context<VaultAction>) -> Result<()> {
        // withdraw logic
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct VaultAction<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump,
    )]
    // a PDA that holds the lamports for the signer.
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum VaultError {
    #[msg("Vault already exists")]
    VaultAlreadyExists,
    #[msg("Invalid amount")]
    InvalidAmount,
}
