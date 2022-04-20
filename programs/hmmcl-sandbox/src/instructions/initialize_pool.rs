use crate::cl_pool::cl_math::PoolMath;
use crate::constants::*;
// use crate::decimal::Decimal;
use crate::state::pool_state::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub struct Pool;
impl PoolMath for Pool {}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        space = 8 + std::mem::size_of::<PoolState>(),
        payer = payer,
        seeds = [ POOL_STATE_SEED, lp_token_mint.key().as_ref() ],
        bump,
    )]
    pub pool_state: Box<Account<'info, PoolState>>,

    /// token_a_mint. Eg BTC
    pub token_x_mint: Box<Account<'info, Mint>>,

    // token_b_mint: Eg USDC
    pub token_y_mint: Box<Account<'info, Mint>>,

    #[account(
        constraint = lp_token_mint.mint_authority.unwrap() == pool_state.key()
    )]
    /// lp_token_mint: Eg xlp-hyd-usdc
    pub lp_token_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = payer,
        token::mint = token_x_mint,
        token::authority = pool_state,
        seeds = [ TOKEN_VAULT_SEED, token_x_mint.key().as_ref(), lp_token_mint.key().as_ref() ],
        bump,
    )]
    pub token_x_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = payer,
        token::mint = token_y_mint,
        token::authority = pool_state,
        seeds = [ TOKEN_VAULT_SEED, token_y_mint.key().as_ref(), lp_token_mint.key().as_ref() ],
        bump,
    )]
    pub token_y_vault: Box<Account<'info, TokenAccount>>,

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

pub fn handle(ctx: Context<InitializePool>, _bootstrap_rp: u64, tick: u64) -> Result<()> {
    // save authority
    ctx.accounts.pool_state.authority = *ctx.accounts.authority.to_account_info().key;

    // save token_a_mint, token_b_mint and lp_token_mint
    ctx.accounts.pool_state.token_x_mint = *ctx.accounts.token_x_mint.to_account_info().key;
    ctx.accounts.pool_state.token_y_mint = *ctx.accounts.token_y_mint.to_account_info().key;
    ctx.accounts.pool_state.lp_token_mint = *ctx.accounts.lp_token_mint.to_account_info().key;

    // save token_a_vault and token_b_vault Pubkeys
    ctx.accounts.pool_state.token_x_vault = ctx.accounts.token_x_vault.to_account_info().key();
    ctx.accounts.pool_state.token_y_vault = ctx.accounts.token_y_vault.to_account_info().key();

    // save bumps from context
    ctx.accounts.pool_state.pool_state_bump = *ctx.bumps.get("pool_state").unwrap();
    ctx.accounts.pool_state.token_x_vault_bump = *ctx.bumps.get("token_x_vault").unwrap();
    ctx.accounts.pool_state.token_y_vault_bump = *ctx.bumps.get("token_y_vault").unwrap();
    ctx.accounts.pool_state.lp_token_vault_bump = *ctx.bumps.get("lp_token_vault").unwrap();

    // setup GlobalState
    let global_state = &mut ctx.accounts.pool_state.pool_global_state;

    global_state.global_fee.fee_scale = FEE_SCALE;
    global_state.liq_scale = LIQUIDITY_SCALE;
    global_state.rp_scale = ROOT_PRICE_SCALE;

    global_state.root_price = Pool::tick_to_rp(tick as u128)
        .to_scale(global_state.rp_scale)
        .value;
    global_state.tick = tick;

    Ok(())
}
