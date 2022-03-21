use crate::constants::LIQUIDITY_SCALE;
use crate::state::fees::Fee;

use anchor_lang::prelude::*;

// TickState

#[account]
pub struct TickState {
    ///Tick Indexed State. gross liquidity cannot be negative
    pub liq_net: u128, // LiquidityNet value
    pub liq_net_scale: u8,   // LiquidityNet scale
    pub liq_net_neg: u8,     // LiquidityNet sign
    pub liq_gross: u128,     // LiquidityGross value
    pub liq_gross_scale: u8, // LiquidityGross scale
    pub tick: u64,
    pub tick_fee: Fee, // tick-level fee (per liquidity unit) --> f0
    pub bump: u8,
}

impl Default for TickState {
    fn default() -> TickState {
        TickState {
            liq_net: 0,
            liq_net_scale: LIQUIDITY_SCALE,
            liq_net_neg: 0,
            liq_gross: 0,
            liq_gross_scale: LIQUIDITY_SCALE,
            tick: 0,
            tick_fee: Fee::default(),
            bump: 0,
        }
    }
}
