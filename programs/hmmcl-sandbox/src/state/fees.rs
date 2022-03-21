use anchor_lang::prelude::*;

pub const FEE_SCALE: u8 = 12;

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub struct Fee {
    /// generic struct for fee handling. fees cannot be negative
    pub fee_scale: u8,
    pub f_x: u128, // pure transaction fee for token x
    pub f_y: u128, // pure transaction fee for token y
    pub h_x: u128, // hmm adjustment fee for token x
    pub h_y: u128, // hmm adjustment fee for token y
}

impl Default for Fee {
    fn default() -> Self {
        Self {
            fee_scale: FEE_SCALE,
            f_x: 0,
            f_y: 0,
            h_x: 0,
            h_y: 0,
        }
    }
}
