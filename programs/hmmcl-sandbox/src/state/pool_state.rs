use crate::constants::{LIQUIDITY_SCALE, ROOT_PRICE_SCALE};
use crate::state::fees::Fee;
// use crate::state::tick_bitmap::PoolTickBitmap;

// use crate::state::tick_bitmap::{PoolTickBitmap, TICK_BITMAP_SIZE};
// use fixedbitset::FixedBitSet;
// use std::io::Write;

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
    // pub tick_bitmap: BitMap,
    // pub tick_bitmap: PoolTickBitmap,
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

// #[derive(Debug, Clone)]
// pub struct BitMap {
//     pub fixed_bitset: FixedBitSet,
// }

// impl Default for BitMap {
//     fn default() -> Self {
//         Self {
//             fixed_bitset: FixedBitSet::with_capacity(TICK_BITMAP_SIZE),
//         }
//     }
// }

// impl AnchorSerialize for BitMap {
//     fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
//         writer.write_all(&self.fixed_bitset.as_slice().try_to_vec()?[..])
//     }
// }

// impl AnchorDeserialize for BitMap {
//     fn deserialize(_buf: &mut &[u8]) -> std::io::Result<Self> {
//         let mut fixed_bitset =
//             FixedBitSet::with_capacity_and_blocks(TICK_BITMAP_SIZE, _buf.try_to_vec());

//         Ok(Self { fixed_bitset })
//     }
// }
