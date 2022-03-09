use crate::constants::*;
use crate::state::pool_state::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct InitializePool<'info> {
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [ POOL_STATE_SEED, lp_token_mint.key().as_ref() ],
        bump,
    )]
    pub pool_state: Box<Account<'info, PoolState>>,

    /// token_a_mint. Eg BTC
    pub base_token_mint: Box<Account<'info, Mint>>,

    // token_b_mint: Eg USDC
    pub quote_token_mint: Box<Account<'info, Mint>>,

    #[account(
        constraint = lp_token_mint.mint_authority.unwrap() == pool_state.key()
    )]
    /// lp_token_mint: Eg xlp-hyd-usdc
    pub lp_token_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = payer,
        token::mint = base_token_mint,
        token::authority = pool_state,
        seeds = [ TOKEN_VAULT_SEED, base_token_mint.key().as_ref(), lp_token_mint.key().as_ref() ],
        bump,
    )]
    pub base_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = payer,
        token::mint = quote_token_mint,
        token::authority = pool_state,
        seeds = [ TOKEN_VAULT_SEED, quote_token_mint.key().as_ref(), lp_token_mint.key().as_ref() ],
        bump,
    )]
    pub quote_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = payer,
        token::mint = lp_token_mint,
        token::authority = pool_state,
        seeds = [ LP_TOKEN_VAULT_SEED, pool_state.key().as_ref(), lp_token_mint.key().as_ref() ],
        bump,
    )]
    pub lp_token_vault: Box<Account<'info, TokenAccount>>,

    // system accounts
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle(ctx: Context<InitializePool>) -> Result<()> {
    // save authority
    ctx.accounts.pool_state.authority = *ctx.accounts.authority.to_account_info().key;

    // save token_a_mint, token_b_mint and lp_token_mint
    ctx.accounts.pool_state.base_token_mint = *ctx.accounts.base_token_mint.to_account_info().key;
    ctx.accounts.pool_state.quote_token_mint = *ctx.accounts.quote_token_mint.to_account_info().key;
    ctx.accounts.pool_state.lp_token_mint = *ctx.accounts.lp_token_mint.to_account_info().key;

    // save token_a_vault and token_b_vault Pubkeys
    ctx.accounts.pool_state.base_token_vault =
        ctx.accounts.base_token_vault.to_account_info().key();
    ctx.accounts.pool_state.quote_token_vault =
        ctx.accounts.quote_token_vault.to_account_info().key();

    // save bumps from context
    ctx.accounts.pool_state.pool_state_bump = *ctx.bumps.get("pool_state").unwrap();
    ctx.accounts.pool_state.base_token_vault_bump = *ctx.bumps.get("base_token_vault").unwrap();
    ctx.accounts.pool_state.quote_token_vault_bump = *ctx.bumps.get("quote_token_vault").unwrap();
    ctx.accounts.pool_state.lp_token_vault_bump = *ctx.bumps.get("lp_token_vault").unwrap();

    Ok(())
}
