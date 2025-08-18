mod errors;
mod instructions;
mod state;
mod utils;

use anchor_lang::prelude::*;

declare_id!("4Lh7VjsEgG9XnShnq1CAsH37sWevuiWPfzqjWKo2s4DK");

#[program]
pub mod protector {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
