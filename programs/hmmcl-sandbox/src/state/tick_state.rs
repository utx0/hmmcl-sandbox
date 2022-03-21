use anchor_lang::prelude::*;

// TickState

#[account]
#[derive(Default)]
pub struct TickState {
    ///Tick Indexed State
    pub liq_net: u128, // LiquidityNet
    pub liq_net_neg: u8, // LiquidityNet
    pub liq_gross: u128, // LiquidityGross
    pub tick: u64,
    pub bump: u8,
}
