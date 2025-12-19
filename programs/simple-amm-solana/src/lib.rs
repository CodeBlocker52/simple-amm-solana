pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;
pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Ffh6uoJ2k9Hakw8awn6vHWARTKvo67y3n1weRAZdbCgB");

#[program]
pub mod simple_amm_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize> , fee :u16) -> Result<()> {
        ctx.accounts.handler(fee , &ctx.bumps)

        
    }

    pub fn create_pool(ctx: Context<CreatePool>, bumps: CreatePoolBumps) -> Result<()> {
        ctx.accounts.handler(&ctx.bumps)
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_x: u64, amount_y: u64) -> Result<()> {
        ctx.accounts.handler(amount_x, amount_y)
    }

      pub fn remove_liquidity(ctx: Context<RemoveLiquidity>, shares: u64) -> Result<()> {
        ctx.accounts.handler(shares)
    }

     pub fn swap(
        ctx: Context<Swap>,
        x_to_y: bool,
        amount_in: u64,
        minimum_amount_out: u64,
    ) -> Result<()> {
        ctx.accounts
            .handler(x_to_y, amount_in, minimum_amount_out, &ctx.bumps)
    }

}

