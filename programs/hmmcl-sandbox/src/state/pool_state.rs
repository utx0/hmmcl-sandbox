use crate::decimal::*;
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct PoolState {
    pub authority: Pubkey,
    pub base_token_vault: Pubkey,
    pub quote_token_vault: Pubkey,
    pub base_token_mint: Pubkey,
    pub quote_token_mint: Pubkey,
    pub lp_token_mint: Pubkey,
    pub pool_state_bump: u8,
    pub base_token_vault_bump: u8,
    pub quote_token_vault_bump: u8,
    pub lp_token_vault_bump: u8,
    pub pool_global_state: GlobalState,
}
impl PoolState {}

#[derive(Debug, Clone, Default, AnchorSerialize, AnchorDeserialize)]
pub struct GlobalState {
    /// contract global state
    pub liq: Decimal, // liquidity
    pub rp: Decimal, // sqrt price
    pub tick: u64,   // current tick
}

// #[derive(Debug, Clone, Default, AnchorSerialize, AnchorDeserialize)]
#[account]
#[derive(Default)]
pub struct TickState {
    ///Tick Indexed State
    pub liq_net: Decimal, // LiquidityNet
    pub liq_gross: Decimal, // LiquidityGross
    pub tick: u64,
    pub bump: u8,
}
#[account]
#[derive(Default)]
pub struct PositionState {
    ///Position Indexed State
    pub liq: Decimal, // liquidity
    pub lower_tick: u64,
    pub upper_tick: u64,
    pub bump: u8,
}
