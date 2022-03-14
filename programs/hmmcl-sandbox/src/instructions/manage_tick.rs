use crate::decimal::{Add, Decimal};
use crate::state::pool_state::PoolState;
use crate::state::tick_state::TickState;

use crate::errors::ErrorCode;
use crate::events::{NegativeTickGrossLiquidity, TickMismatch};

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(tick: u64)]
pub struct InitializeTick<'info> {
    #[account(
        init,
        payer = payer,
        // space = 8 + 2 + 4 + 200 + 1,
        seeds = [b"tick", pool_state.key().as_ref(), tick.to_ne_bytes().as_ref()], 
        // seeds = [b"tick", pool_state.key().as_ref()], 
        bump,
    )]
    pub tick_state: Account<'info, TickState>,

    pub pool_state: Account<'info, PoolState>,
    //+ pub tick_list: Account<'info, TickList>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(tick: u64)]
pub struct UpdateTick<'info> {
    #[account(
        mut,
        seeds = [b"tick", pool_state.key().as_ref(), tick.to_le_bytes().as_ref()], 
        bump = tick_state.bump,
        // has_one = authority,
        constraint = tick_state.authority == pool_state.key(),
        constraint = tick_state.tick == tick,
    )]
    pub tick_state: Account<'info, TickState>,
    pub pool_state: Account<'info, PoolState>,
    //+ pub tick_list: Account<'info, TickList>,
    // pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(tick: u64)]
pub struct UnsetTick<'info> {
    #[account(
        mut,
        seeds = [b"tick", pool_state.key().as_ref(), tick.to_le_bytes().as_ref()], 
        bump = tick_state.bump,
        // has_one = authority,
        constraint = tick_state.authority == pool_state.key(),
        constraint = tick_state.tick == tick,
    )]
    pub tick_state: Account<'info, TickState>,
    pub pool_state: Account<'info, PoolState>,
    //+ pub tick_list: Account<'info, TickList>,
    // pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(provided_tick: u64)]
pub struct CrossTick<'info> {
    #[account(
        mut,
        seeds = [b"tick", pool_state.key().as_ref(), provided_tick.to_le_bytes().as_ref()], 
        bump = tick_state.bump,
        // has_one = authority,
        constraint = tick_state.authority == pool_state.key(),
    )]
    pub tick_state: Account<'info, TickState>,
    pub pool_state: Account<'info, PoolState>,
    // pub authority: Signer<'info>,
}

pub fn initialize_tick(ctx: Context<InitializeTick>, tick: u64) -> Result<()> {
    let tick_state = &mut ctx.accounts.tick_state;

    tick_state.bump = *ctx.bumps.get("tick_state").unwrap();
    tick_state.tick = tick;
    tick_state.authority = *ctx.accounts.pool_state.to_account_info().key;

    Ok(())
}

pub fn _update_tick(
    ctx: Context<UpdateTick>,
    tick: u64,
    liquidity_delta: Decimal,
    upper: bool,
) -> Result<()> {
    let tick_state = &mut ctx.accounts.tick_state;

    // let applied_net_liquidity = match upper {
    //     false => liquidity_delta,
    //     true => Decimal::flip_sign(liquidity_delta),
    // };
    // tick_state.liq_net = tick_state.liq_net.add(applied_net_liquidity).unwrap();

    // let new_gross_liquidity = tick_state.liq_gross.add(liquidity_delta).unwrap();
    // if new_gross_liquidity.negative {
    //     emit!(NegativeTickGrossLiquidity {
    //         original_liquidity: tick_state.liq_gross.to_u64(),
    //         attempted_removal: liquidity_delta.to_u64(),
    //     });
    //     return Err(ErrorCode::NegativeTickGrossLiquidity.into());
    // }

    // Ok(())
    update_tick_direct(tick_state, tick, liquidity_delta, upper)
}

pub fn update_tick_direct<'info>(
    tick_state: &mut Account<'info, TickState>,
    tick: u64,
    liquidity_delta: Decimal,
    upper: bool,
) -> Result<()> {
    if tick != tick_state.tick {
        emit!(TickMismatch {
            expected_tick: tick_state.tick,
            actual_tick: tick,
        });

        return Err(ErrorCode::TickMismatch.into());
    }

    let applied_net_liquidity = match upper {
        false => liquidity_delta,
        true => Decimal::flip_sign(liquidity_delta),
    };
    tick_state.liq_net = tick_state.liq_net.add(applied_net_liquidity).unwrap();

    let new_gross_liquidity = tick_state.liq_gross.add(liquidity_delta).unwrap();
    if new_gross_liquidity.negative {
        emit!(NegativeTickGrossLiquidity {
            original_liquidity: tick_state.liq_gross.to_u64(),
            attempted_removal: liquidity_delta.to_u64(),
        });
        return Err(ErrorCode::NegativeTickGrossLiquidity.into());
    }
    tick_state.liq_gross = new_gross_liquidity;

    //TODO : unset tick if liq_gross becomes zero

    Ok(())
}

pub fn unset_tick(ctx: Context<UnsetTick>, tick: u64) -> Result<()> {
    msg!("{}", tick);
    msg!("{}", ctx.program_id);

    Ok(())
}

pub fn cross_tick(ctx: Context<CrossTick>, provided_tick: u64, left_to_right: bool) -> Result<()> {
    msg!("{}", provided_tick);
    msg!("{}", left_to_right);
    msg!("{}", ctx.program_id);
    Ok(())
}
