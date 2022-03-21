use anchor_lang::prelude::*;

// TickState

#[account]
#[derive(Default)]
pub struct TickState {
    ///Tick Indexed State. gross liquidity cannot be negative
    pub liq_net: u128, // LiquidityNet value
    pub liq_net_scale: u8,   // LiquidityNet scale
    pub liq_net_neg: u8,     // LiquidityNet sign
    pub liq_gross: u128,     // LiquidityGross value
    pub liq_gross_scale: u8, // LiquidityGross scale
    pub tick: u64,
    pub bump: u8,
}
