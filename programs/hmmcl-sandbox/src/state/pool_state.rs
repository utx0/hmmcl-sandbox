use crate::constants::{LIQUIDITY_SCALE, ROOT_PRICE_SCALE};
use crate::state::fees::Fee;

use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct PoolState {
    pub authority: Pubkey,
    pub token_x_vault: Pubkey,
    pub token_y_vault: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub lp_token_mint: Pubkey,
    pub pool_state_bump: u8,
    pub token_x_vault_bump: u8,
    pub token_y_vault_bump: u8,
    pub lp_token_vault_bump: u8,
    pub pool_global_state: GlobalState,
}
impl PoolState {}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GlobalState {
    /// contract global state. global liquidity cannot be negative
    pub liquidity: u128, // liquidity
    pub liq_scale: u8,    // decimal scale for liquidity
    pub root_price: u128, // sqrt price
    pub rp_scale: u8,     // decimal scale for root-price
    pub tick: u64,        // current tick
    pub global_fee: Fee,  // global fee (per liquidity unit) --> fg
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            liquidity: 0,
            liq_scale: LIQUIDITY_SCALE,
            root_price: 0,
            rp_scale: ROOT_PRICE_SCALE,
            tick: 0,
            global_fee: Fee::default(),
        }
    }
}
