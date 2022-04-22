use crate::constants::POOL_STATE_SEED;
use crate::state::pool_state::PoolState;
use crate::state::tick_bitmap::PoolTickBitmap;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeBitmap<'info> {
    #[account(
        seeds = [ POOL_STATE_SEED, pool_state.lp_token_mint.as_ref() ],
        bump = pool_state.pool_state_bump,
    )]
    pub pool_state: Account<'info, PoolState>,

    #[account(
        init,
        space = 8 + std::mem::size_of::<PoolTickBitmap>(),
        payer = payer,
        seeds = [ b"bitmap", pool_state.key().as_ref() ],
        bump,
    )]
    pub tick_bitmap: AccountLoader<'info, PoolTickBitmap>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_bitmap(ctx: Context<InitializeBitmap>) -> Result<()> {
    let tick_bitmap = &mut ctx.accounts.tick_bitmap.load_init()?;

    tick_bitmap.bump = *ctx.bumps.get("tick_bitmap").unwrap();

    // tick_bitmap.activate_tick(0);

    Ok(())
}
