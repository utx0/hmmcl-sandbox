use crate::state::pool_state::*;

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(lower_tick: u64, upper_tick: u64)]
pub struct CreatePosition<'info> {
    #[account(
        init,
        payer = user,
        // space = 8 + 2 + 4 + 200 + 1,
        seeds = [b"position", user.key().as_ref()], // + also need upper and lower ticks (u64)
        bump
    )]
    pub position_state: Account<'info, PositionState>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(lower_tick: u64, upper_tick: u64)]
pub struct SetPosition<'info> {
    #[account(
        mut,
        seeds = [b"position", user.key().as_ref()],//+  also need upper and lower ticks
        bump = position_state.bump,
        constraint = position_state.upper_tick == upper_tick,
        constraint = position_state.lower_tick == lower_tick,
    )]
    pub position_state: Account<'info, PositionState>,
    pub user: Signer<'info>,
}

pub fn create_position(
    ctx: Context<CreatePosition>,
    lower_tick: u64,
    upper_tick: u64,
) -> Result<()> {
    Ok(())
}
pub fn set_position(ctx: Context<SetPosition>, lower_tick: u64, upper_tick: u64) -> Result<()> {
    Ok(())
}
