use crate::decimal::*;
use anchor_lang::prelude::*;

// TickState

#[account]
#[derive(Default)]
pub struct TickState {
    ///Tick Indexed State
    pub liq_net: Decimal, // LiquidityNet
    pub liq_gross: Decimal, // LiquidityGross
    pub tick: u64,
    pub bump: u8,
}
