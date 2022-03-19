use crate::constants::POOL_STATE_SEED;
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
        // mut,
        seeds = [ POOL_STATE_SEED, pool_state.lp_token_mint.as_ref() ],
        bump = pool_state.pool_state_bump,
    )]
    pub pool_state: Account<'info, PoolState>,

    #[account(
        init,
        payer = payer,
        // space = 8 + 2 + 4 + 200 + 1,
        seeds = [b"tick", pool_state.key().as_ref(), tick.to_ne_bytes().as_ref()], 
        // seeds = [b"tick", pool_state.key().as_ref()], 
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
    tick_state.authority = *ctx.accounts.pool_state.to_account_info().key;
    tick_state.liq_net = 0;
    tick_state.liq_net_neg = 0;
    tick_state.liq_gross = 0;

    Ok(())
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

    let mut ts_liq_net = Decimal::from_account(tick_state.liq_net, tick_state.liq_net_neg);

    let applied_net_liquidity = match upper {
        false => liquidity_delta,
        true => Decimal::flip_sign(liquidity_delta),
    };
    ts_liq_net = ts_liq_net.add(applied_net_liquidity).unwrap();

    tick_state.liq_net = ts_liq_net.to_account_value();
    tick_state.liq_net_neg = ts_liq_net.to_account_sign();

    let ts_liq_gross = Decimal::from_account(tick_state.liq_gross, 0);

    let new_gross_liquidity = ts_liq_gross.add(liquidity_delta).unwrap();
    if new_gross_liquidity.negative {
        emit!(NegativeTickGrossLiquidity {
            original_liquidity: ts_liq_gross.to_zero_scale_u64(),
            attempted_removal: liquidity_delta.to_zero_scale_u64(),
        });
        return Err(ErrorCode::NegativeTickGrossLiquidity.into());
    }
    tick_state.liq_gross = new_gross_liquidity.to_account_value();

    //TODO : unset tick if liq_gross becomes zero

    Ok(())
}
