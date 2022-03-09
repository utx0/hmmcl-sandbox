use crate::state::pool_state::*;

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(tick: u64)]
pub struct InitializeTick<'info> {
    #[account(
        init,
        payer = user,
        // space = 8 + 2 + 4 + 200 + 1,
        seeds = [b"tick", pool_state.key().as_ref()], // and tick (u64)
        bump
    )]
    pub tick_state: Account<'info, TickState>,
    pub pool_state: Account<'info, PoolState>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(tick: u64)]
pub struct UpdateTick<'info> {
    #[account(
        mut,
        seeds = [b"tick", pool_state.key().as_ref()], // and tick (u64)
        bump = tick_state.bump,
        constraint = tick_state.tick == tick,
    )]
    pub tick_state: Account<'info, TickState>,
    pub pool_state: Account<'info, PoolState>,
    // pub authority: Signer<'info>,
}

pub fn initialize_tick(ctx: Context<InitializeTick>, tick: u64) -> Result<()> {
    Ok(())
}
pub fn update_tick(ctx: Context<UpdateTick>, tick: u64) -> Result<()> {
    Ok(())
}
