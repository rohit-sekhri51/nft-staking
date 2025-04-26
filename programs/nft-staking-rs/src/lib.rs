pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("3UdWfY9nSzjHzRg84Q88b2jnaq3bM9JAaFpLTghJwfSv");

#[program]
pub mod nft_staking_rs {
    use super::*;

    pub fn init_config(ctx: Context<InitializeConfig>,
        points_per_stake: u8, max_stake: u8, freeze_period: u32) -> Result<()> {

        ctx.accounts.initialize_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)
    }

    pub fn init_user(ctx: Context<InitializeUser>) -> Result<()> {

        ctx.accounts.initialize_user(&ctx.bumps)
    }

    
}
