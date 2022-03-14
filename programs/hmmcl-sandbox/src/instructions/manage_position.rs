use crate::decimal::{Add, Decimal};
use crate::state::pool_state::PoolState;
use crate::state::position_state::PositionState;
use crate::state::tick_state::TickState;

use crate::errors::ErrorCode;
use crate::events::InsufficientPositionLiquidity;

// use crate::instruction::UpdateTick;

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(lower_tick: u64, upper_tick: u64)]
pub struct CreatePosition<'info> {
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

    pub pool_state: Account<'info, PoolState>,

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

    pub pool_state: Account<'info, PoolState>,

    //+ pub user_account:  Account<'info, UserAccount>, // for PositionList and TempFees
    pub user: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
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
    Ok(())
}

pub fn update_position(
    ctx: Context<UpdatePosition>,
    liquidity_abs_value: u64,
    liquidity_negative: bool,
    lower_tick: u64,
    upper_tick: u64,
) -> Result<()> {
    let position_state = &mut ctx.accounts.position_state;

    let mut liquidity_delta = Decimal::from_u64(liquidity_abs_value);
    if liquidity_negative {
        liquidity_delta = Decimal::flip_sign(liquidity_delta);
    }

    let new_liquidity = position_state.liquidity.add(liquidity_delta).unwrap();

    if new_liquidity.negative {
        emit!(InsufficientPositionLiquidity {
            original_liquidity: position_state.liquidity.to_u64(),
            attempted_removal: liquidity_abs_value,
        });
        return Err(ErrorCode::InsufficientPositionLiquidity.into());
    }

    position_state.liquidity = new_liquidity;

    // ctx_lt_acc = UpdateTick {
    //     tick: lower_tick,
    //     liq: liquidity,
    //     upper: false,
    // };
    msg!("{}", lower_tick);
    msg!("{}", upper_tick);
    Ok(())
}

//+ INIT_IF_NEEDED version
// #[derive(Accounts)]
// #[instruction(lower_tick: u64, upper_tick: u64)]
// pub struct SetPosition<'info> {
//     #[account(
//         init_if_needed,
//         seeds = [
//             b"position",
//             pool_state.key().as_ref(),
//             user.key().as_ref(),
//             lower_tick.to_le_bytes().as_ref(),
//             upper_tick.to_le_bytes().as_ref()
//         ],
//         bump,
//         payer=payer,
//         // constraint = position_state.upper_tick == upper_tick,
//         // constraint = position_state.lower_tick == lower_tick,
//     )]
//     pub position_state: Account<'info, PositionState>,
//     pub pool_state: Account<'info, PoolState>,
//     // pub user_account:  Account<'info, UserAccount>, // for PositionList and TempFees
//     pub user: Signer<'info>,

//     #[account(mut)]
//     pub payer: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }
