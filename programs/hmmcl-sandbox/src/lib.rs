pub mod decimal;
mod instructions;
pub mod state;

use instructions::initialize_pool::*;
// use instructions::deposit::*;
// use instructions::withdraw::*;
use instructions::manage_position::*;
use instructions::manage_tick::*;

use crate::decimal::Decimal;
use anchor_lang::prelude::*;

declare_id!("2bcRNDgkrYrNvP19bX4qQvBZLMEwxh2E4jRNJrmoVN9r");

pub mod constants {
    pub const LP_TOKEN_VAULT_SEED: &[u8] = b"lp_token_vault_seed";
    pub const TOKEN_VAULT_SEED: &[u8] = b"token_vault_seed";
    pub const POOL_STATE_SEED: &[u8] = b"pool_state_seed";
}

#[program]
pub mod hmmcl_sandbox {
    use super::*;

    /// initialize a new empty pool
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        bootstrap_rp: u64,
        tick: u64,
    ) -> Result<()> {
        instructions::initialize_pool::handle(ctx, bootstrap_rp, tick)
    }

    /// initialize a tick, this is trigerred by SetPosition below, called during deposits when needed
    pub fn initialize_tick(ctx: Context<InitializeTick>, tick: u64) -> Result<()> {
        instructions::manage_tick::initialize_tick(ctx, tick)
    }

    /// update a tick ( this is done by pool - triggered by deposits or swaps)
    pub fn update_tick(
        ctx: Context<UpdateTick>,
        tick: u64,
        liq: Decimal,
        upper: bool,
    ) -> Result<()> {
        instructions::manage_tick::update_tick(ctx, tick, liq, upper)
    }

    /// unset tick: flags a tick inactive when no position is referencing it
    pub fn unset_tick(ctx: Context<UnsetTick>, tick: u64) -> Result<()> {
        instructions::manage_tick::unset_tick(ctx, tick)
    }

    /// crossing a tick during the swap process
    pub fn cross_tick(
        ctx: Context<CrossTick>,
        provided_tick: u64,
        left_to_right: bool,
    ) -> Result<()> {
        instructions::manage_tick::cross_tick(ctx, provided_tick, left_to_right)
    }

    /// user creates a new position ( this will be used by deposits and withdrawals by user)
    pub fn create_position(
        ctx: Context<CreatePosition>,
        lower_tick: u64,
        upper_tick: u64,
    ) -> Result<()> {
        instructions::manage_position::create_position(ctx, lower_tick, upper_tick)
    }

    /// user sets a position ( this will be used by deposits and withdrawals by user)
    pub fn set_position(ctx: Context<SetPosition>, lower_tick: u64, upper_tick: u64) -> Result<()> {
        instructions::manage_position::set_position(ctx, lower_tick, upper_tick)
    }
}
