use anchor_lang::prelude::*;

#[derive(Debug, Clone, Copy, Default, AnchorSerialize, AnchorDeserialize)]
pub struct Fee {
    /// generic struct for fee handling. fees cannot be negative
    pub fee_scale: u8,
    pub f_x: u128, // pure transaction fee for token x
    pub f_y: u128, // pure transaction fee for token y
    pub h_x: u128, // hmm adjustment fee for token x
    pub h_y: u128, // hmm adjustment fee for token y
}
