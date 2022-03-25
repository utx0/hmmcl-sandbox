use crate::constants::{FEE_SCALE, LIQUIDITY_SCALE, POOL_STATE_SEED};
use crate::decimal::{Add, Decimal};
use crate::state::pool_state::PoolState;
use crate::state::tick_state::TickState;

use crate::errors::ErrorCode;
use crate::events::{NegativeTickGrossLiquidity, TickMismatch};

use anchor_lang::prelude::*;

use std::mem;

#[derive(Accounts)]
#[instruction(tick: u64)]
pub struct InitializeTick<'info> {
    #[account(
        seeds = [ POOL_STATE_SEED, pool_state.lp_token_mint.as_ref() ],
        bump = pool_state.pool_state_bump,
    )]
    pub pool_state: Account<'info, PoolState>,

    #[account(
        init,
        space = 8 + mem::size_of::<TickState>(),
        payer = payer,
        seeds = [b"tick", pool_state.key().as_ref(), tick.to_ne_bytes().as_ref()], 
        bump,
    )]
    pub tick_state: Account<'info, TickState>,

    //+ pub tick_list: Account<'info, TickList>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_tick(ctx: Context<InitializeTick>, tick: u64) -> Result<()> {
    let tick_state = &mut ctx.accounts.tick_state;

    tick_state.bump = *ctx.bumps.get("tick_state").unwrap();
    tick_state.tick = tick;

    tick_state.liq_net_scale = LIQUIDITY_SCALE;
    tick_state.liq_gross_scale = LIQUIDITY_SCALE;
    tick_state.tick_fee.fee_scale = FEE_SCALE;

    // let (zero, scale, _) = Decimal::from_u64(0).to_account();
    // tick_state.liq_net = zero;
    // tick_state.liq_net_scale = scale;
    // tick_state.liq_net_neg = 0;
    // tick_state.liq_gross = zero;
    // tick_state.liq_gross_scale = scale;

    Ok(())
}

pub fn update_tick<'info>(
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

    let mut ts_liq_net = Decimal::from_account(
        tick_state.liq_net,
        tick_state.liq_net_scale,
        tick_state.liq_net_neg,
    );

    let applied_net_liquidity = match upper {
        false => liquidity_delta,
        true => Decimal::flip_sign(liquidity_delta),
    };
    ts_liq_net = ts_liq_net.add(applied_net_liquidity).unwrap();

    let (liq_net_val, liq_net_scale, liq_net_neg) = ts_liq_net.to_account();
    tick_state.liq_net = liq_net_val;
    tick_state.liq_net_scale = liq_net_scale;
    tick_state.liq_net_neg = liq_net_neg;

    let ts_liq_gross = Decimal::from_account(tick_state.liq_gross, tick_state.liq_gross_scale, 0);

    let new_gross_liquidity = ts_liq_gross.add(liquidity_delta).unwrap();
    if new_gross_liquidity.negative {
        emit!(NegativeTickGrossLiquidity {
            original_liquidity: ts_liq_gross.to_zero_scale_u64(),
            attempted_removal: liquidity_delta.to_zero_scale_u64(),
        });
        return Err(ErrorCode::NegativeTickGrossLiquidity.into());
    }
    let (liq_gross_val, liq_gross_scale, _) = new_gross_liquidity.to_account();
    tick_state.liq_gross = liq_gross_val;
    tick_state.liq_gross_scale = liq_gross_scale;

    //TODO : unset tick if liq_gross becomes zero

    Ok(())
}
