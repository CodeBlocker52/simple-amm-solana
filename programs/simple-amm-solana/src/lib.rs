use anchor_lang::prelude::*;

declare_id!("Ffh6uoJ2k9Hakw8awn6vHWARTKvo67y3n1weRAZdbCgB");

#[program]
pub mod simple_amm_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
