use crate::constants::POOL_STATE_SEED;
use crate::decimal::{Add, Decimal};
use crate::state::pool_state::PoolState;
use crate::state::position_state::PositionState;
use crate::state::tick_state::TickState;

use crate::errors::ErrorCode;
use crate::events::InsufficientPositionLiquidity;

// use crate::instructions::manage_tick::UpdateTick;

use anchor_lang::prelude::*;

use super::manage_tick::update_tick_direct;

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
        // space = 8 + 2 + 4 + 200 + 1,
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
        mut,
        seeds = [b"tick", pool_state.key().as_ref(), lower_tick.to_le_bytes().as_ref()], 
        bump = lower_tick_state.bump,
    )]
    pub lower_tick_state: Account<'info, TickState>,

    #[account(
        mut,
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

#[derive(Accounts)]
#[instruction(liquidity: u64, negative: bool, lower_tick: u64, upper_tick: u64)]
pub struct UpdatePosition<'info> {
    #[account(
        mut,
        seeds = [ POOL_STATE_SEED, pool_state.lp_token_mint.as_ref() ],
        bump = pool_state.pool_state_bump,
    )]
    pub pool_state: Account<'info, PoolState>,

    #[account(
        mut,
        seeds = [
            b"position", 
            pool_state.key().as_ref(),
            user.key().as_ref(),
            lower_tick.to_le_bytes().as_ref(),
            upper_tick.to_le_bytes().as_ref(),
        ],
        bump = position_state.bump,
        constraint = lower_tick < upper_tick,
    )]
    pub position_state: Account<'info, PositionState>,

    #[account(
        mut,
        seeds = [b"tick", pool_state.key().as_ref(), lower_tick.to_le_bytes().as_ref()],
        bump = lower_tick_state.bump,
        constraint = lower_tick_state.tick == position_state.lower_tick,
    )]
    pub lower_tick_state: Account<'info, TickState>,

    #[account(
        mut,
        seeds = [b"tick", pool_state.key().as_ref(), upper_tick.to_le_bytes().as_ref()],
        bump = upper_tick_state.bump,
        constraint = upper_tick_state.tick == position_state.upper_tick,
    )]
    pub upper_tick_state: Account<'info, TickState>,

    //+ pub user_account:  Account<'info, UserAccount>, // for PositionList and TempFees
    pub user: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    // /// CHECK: only used as a signing PDA
    // pub authority: UncheckedAccount<'info>,
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
    position_state.authority = *ctx.accounts.pool_state.to_account_info().key;
    position_state.liquidity = 0;
    Ok(())
}

pub fn update_position_direct<'info>(
    position_state: &mut Account<'info, PositionState>,
    lower_tick_state: &mut Account<'info, TickState>,
    upper_tick_state: &mut Account<'info, TickState>,
    liquidity_delta: Decimal,
    lower_tick: u64,
    upper_tick: u64,
) -> Result<()> {
    // Update position liquidity

    let ps_liquidity = Decimal::from_account(position_state.liquidity, 0);

    let new_liquidity = ps_liquidity.add(liquidity_delta).unwrap();
    if new_liquidity.negative {
        emit!(InsufficientPositionLiquidity {
            original_liquidity: ps_liquidity.to_int(),
            attempted_removal: liquidity_delta.to_int(),
        });
        return Err(ErrorCode::InsufficientPositionLiquidity.into());
    }

    position_state.liquidity = new_liquidity.to_account_value();

    // Update liquidity on respective tick_states
    update_tick_direct(lower_tick_state, lower_tick, liquidity_delta, false).unwrap();
    update_tick_direct(upper_tick_state, upper_tick, liquidity_delta, true).unwrap();

    //* global liquidity is updated in deposit handler

    Ok(())
}
