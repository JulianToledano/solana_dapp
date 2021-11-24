use anchor_lang::prelude::*;

declare_id!("JDEXJTiSNgVssYvuvFrCGyu1Loip5H28nZT6DSmzcQpJ");

#[program]
pub mod solana_twitter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
