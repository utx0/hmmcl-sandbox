use crate::constants::LIQUIDITY_SCALE;
use crate::state::fees::Fee;

use anchor_lang::prelude::*;

// PositionState

#[account]
pub struct PositionState {
    ///Position Indexed State. position liquidity cannot be negative
    pub liquidity: u128, // liquidity
    pub liq_scale: u8, // liquidity scale
    pub lower_tick: u64,
    pub upper_tick: u64,
    pub last_collected_fee: Fee, // position-level fee (per liquidity unit) --> fr(t0)
    pub bump: u8,
}

impl Default for PositionState {
    fn default() -> Self {
        Self {
            liquidity: 0,
            liq_scale: LIQUIDITY_SCALE,
            lower_tick: 0,
            upper_tick: 0,
            last_collected_fee: Fee::default(),
            bump: 0,
        }
    }
}
