use anchor_lang::prelude::*;

use crate::{AmmConfig , error::AmmErrorCode ,CONFIG_SEED , ANCHOR_DISCRIMINATOR};



#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + AmmConfig::INIT_SPACE,
        seeds=[CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, AmmConfig>,
    pub system_program: Program<'info, System>,

}

impl<'info> Initialize<'info>{
    pub fn handler(&mut self , fee:u16, bumps:&IntializeBumps)->Result<()>{
        //Fee must be less than 100%
        require_gt(10000, fee , AmmErrorCode::InvalidFee);
        self.config.set_inner(AmmConfig{
            authority:self.signer.to_account_info().key(),
            fee,
            bump:bumps.config,
        });
        Ok(())
    }
    
}

