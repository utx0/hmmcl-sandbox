use crate::constants::POOL_STATE_SEED;
use crate::decimal::{Add, Decimal};
use crate::instructions::manage_tick::update_tick;
use crate::state::pool_state::PoolState;
use crate::state::position_state::PositionState;
use crate::state::tick_state::TickState;

use crate::errors::ErrorCode;
use crate::events::InsufficientPositionLiquidity;

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(lower_tick: u64, upper_tick: u64)]
pub struct CreatePosition<'info> {
    #[account(
        // mut,
        seeds = [ POOL_STATE_SEED, pool_state.lp_token_mint.as_ref() ],
        bump = pool_state.pool_state_bump,
    )]
    pub pool_state: Account<'info, PoolState>,

    #[account(
        init,
        payer = payer,
        seeds = [
            b"position",
            pool_state.key().as_ref(),
            user.key().as_ref(),
            lower_tick.to_le_bytes().as_ref(),
            upper_tick.to_le_bytes().as_ref()
        ],
        bump,
        constraint = lower_tick < upper_tick,
    )]
    pub position_state: Account<'info, PositionState>,

    #[account(
        seeds = [b"tick", pool_state.key().as_ref(), lower_tick.to_le_bytes().as_ref()], 
        bump = lower_tick_state.bump,
    )]
    pub lower_tick_state: Account<'info, TickState>,

    #[account(
        seeds = [b"tick", pool_state.key().as_ref(), upper_tick.to_le_bytes().as_ref()], 
        bump = upper_tick_state.bump,
    )]
    pub upper_tick_state: Account<'info, TickState>,

    //+ pub user_account:  Account<'info, UserAccount>, // for PositionList and TempFees
    pub user: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_position(
    ctx: Context<CreatePosition>,
    lower_tick: u64,
    upper_tick: u64,
) -> Result<()> {
    let position_state = &mut ctx.accounts.position_state;

    position_state.bump = *ctx.bumps.get("position_state").unwrap();
    position_state.lower_tick = lower_tick;
    position_state.upper_tick = upper_tick;

    let (zero, scale, _) = Decimal::from_u64(0).to_account();
    position_state.liquidity = zero;
    position_state.liq_scale = scale;
    Ok(())
}

pub fn update_position<'info>(
    position_state: &mut Account<'info, PositionState>,
    lower_tick_state: &mut Account<'info, TickState>,
    upper_tick_state: &mut Account<'info, TickState>,
    liquidity_delta: Decimal,
    lower_tick: u64,
    upper_tick: u64,
) -> Result<()> {
    // Update position liquidity

    let ps_liquidity = Decimal::from_account(position_state.liquidity, position_state.liq_scale, 0);

    let new_liquidity = ps_liquidity.add(liquidity_delta).unwrap();
    if new_liquidity.negative {
        emit!(InsufficientPositionLiquidity {
            original_liquidity: ps_liquidity.to_zero_scale_u64(),
            attempted_removal: liquidity_delta.to_zero_scale_u64(),
        });
        return Err(ErrorCode::InsufficientPositionLiquidity.into());
    }

    let (pos_val, pos_scale, _) = new_liquidity.to_account();
    position_state.liquidity = pos_val;
    position_state.liq_scale = pos_scale;

    // Update liquidity on respective tick_states
    update_tick(lower_tick_state, lower_tick, liquidity_delta, false).unwrap();
    update_tick(upper_tick_state, upper_tick, liquidity_delta, true).unwrap();

    //* global liquidity is updated in deposit handler

    Ok(())
}
