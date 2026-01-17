use anchor_lang::prelude::*;

declare_id!("CqgzbgfP7ZwFYK2Wi9DkKJ74Rn12UMN1EggAR9ur3V3D");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
