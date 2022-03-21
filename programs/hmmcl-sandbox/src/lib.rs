pub mod cl_pool;
pub mod decimal;
pub mod errors;
pub mod events;
mod instructions;
pub mod state;

use instructions::deposit::*;
use instructions::initialize_pool::*;
use instructions::manage_position::*;
use instructions::manage_tick::*;
// use instructions::withdraw::*;

use anchor_lang::prelude::*;

declare_id!("2bcRNDgkrYrNvP19bX4qQvBZLMEwxh2E4jRNJrmoVN9r");

pub mod constants {
    pub const LP_TOKEN_VAULT_SEED: &[u8] = b"lp_token_vault_seed";
    pub const TOKEN_VAULT_SEED: &[u8] = b"token_vault_seed";
    pub const POOL_STATE_SEED: &[u8] = b"pool_state_seed";
    pub const FEE_SCALE: u8 = 12;
    pub const LIQUIDITY_SCALE: u8 = 12;
    pub const ROOT_PRICE_SCALE: u8 = 12;
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

    /// user creates a new position ( this will be used by deposits and withdrawals by user)
    pub fn create_position(
        ctx: Context<CreatePosition>,
        lower_tick: u64,
        upper_tick: u64,
    ) -> Result<()> {
        instructions::manage_position::create_position(ctx, lower_tick, upper_tick)
    }

    /// user deposits x and y to the pool between lower_tick and upper_tick
    pub fn deposit(
        ctx: Context<Deposit>,
        lower_tick: u64,
        upper_tick: u64,
        current_tick: u64,
        token_x_amount: u64,
        token_y_amount: u64,
    ) -> Result<()> {
        instructions::deposit::handle(
            ctx,
            lower_tick,
            upper_tick,
            current_tick,
            token_x_amount,
            token_y_amount,
        )
    }
}
