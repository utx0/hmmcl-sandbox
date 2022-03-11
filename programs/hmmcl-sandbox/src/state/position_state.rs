use crate::decimal::*;
use anchor_lang::prelude::*;

// PositionState

#[account]
#[derive(Default)]
pub struct PositionState {
    ///Position Indexed State
    pub liq: Decimal, // liquidity
    // pub lower_tick: u64,
    // pub upper_tick: u64,
    pub bump: u8,
}
