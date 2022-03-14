use anchor_lang::prelude::*;

#[event]
pub struct SlippageExceeded {
    pub token_x_to_debit: u64,
    pub token_y_to_debit: u64,
    pub token_x_max_amount: u64,
    pub token_y_max_amount: u64,
}

#[event]
pub struct LiquidityAdded {
    pub tokens_x_transferred: u64,
    pub tokens_y_transferred: u64,
    pub lp_tokens_minted: u64,
}

#[event]
pub struct LiquidityRemoved {
    pub tokens_x_credited: u64,
    pub tokens_y_credited: u64,
    pub lp_tokens_burnt: u64,
}

#[event]
pub struct InsufficientPositionLiquidity {
    pub original_liquidity: u64,
    pub attempted_removal: u64,
}
#[event]
pub struct NegativeTickGrossLiquidity {
    pub original_liquidity: u64,
    pub attempted_removal: u64,
}
#[event]
pub struct TickMismatch {
    pub expected_tick: u64,
    pub actual_tick: u64,
}
