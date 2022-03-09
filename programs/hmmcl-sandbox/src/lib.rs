mod instructions;
pub mod state;

use instructions::initialize_pool::*;

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
        token_a_vault_bump: u8,
        token_b_vault_bump: u8,
        pool_state_bump: u8,
        lp_token_vault_bump: u8,
    ) -> Result<()> {
        instructions::initialize_pool::handle(
            ctx,
            token_a_vault_bump,
            token_b_vault_bump,
            pool_state_bump,
            lp_token_vault_bump,
        )
    }
}
