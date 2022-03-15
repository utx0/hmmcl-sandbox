use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Slippage Amount Exceeded")]
    SlippageExceeded,

    #[msg("Invalid vault to SwapResult amounts")]
    InvalidVaultToSwapResultAmounts,

    #[msg("Mint address provided doesn't match pools")]
    InvalidMintAddress,

    #[msg("Invalid Fee input")]
    InvalidFee,

    #[msg("Position Liquidity Cannot Be Negative")]
    NegativePositionLiquidity,

    #[msg("Deposit Liquidity Cannot Be Negative")]
    NegativeDepositLiquidity,

    #[msg("Global Liquidity Cannot Be Negative")]
    NegativeGlobalLiquidity,

    #[msg("Insufficient Position Liquidity")]
    InsufficientPositionLiquidity,

    #[msg("Tick Gross Liquidity Cannot Be Negative")]
    NegativeTickGrossLiquidity,

    #[msg("Tick Not Matching Expected")]
    TickMismatch,

    #[msg("Deposit Amount Exceeded")]
    DepositAmountExceeded,
}
