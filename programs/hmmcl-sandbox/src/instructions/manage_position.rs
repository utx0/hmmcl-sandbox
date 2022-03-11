use crate::state::pool_state::PoolState;
use crate::state::position_state::PositionState;
use crate::state::tick_state::TickState;

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
            user.key().as_ref(),
            pool_state.key().as_ref(),
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
#[instruction(lower_tick: u64, upper_tick: u64)]
pub struct SetPosition<'info> {
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
}

pub fn create_position(
    ctx: Context<CreatePosition>,
    lower_tick: u64,
    upper_tick: u64,
) -> Result<()> {
    msg!("{}", lower_tick);
    msg!("{}", upper_tick);
    msg!("{}", ctx.program_id);
    Ok(())
}

pub fn set_position(ctx: Context<SetPosition>, lower_tick: u64, upper_tick: u64) -> Result<()> {
    msg!("{}", lower_tick);
    msg!("{}", upper_tick);
    msg!("{}", ctx.program_id);
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
