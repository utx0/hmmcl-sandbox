use crate::cl_pool::cl_math::PoolMath;
use crate::constants::*;
use crate::decimal::*;
use crate::state::pool_state::PoolState;
use crate::state::position_state::PositionState;
use crate::state::tick_state::TickState;

use crate::errors::ErrorCode;
use crate::events::*;

use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Mint, MintTo, Token, TokenAccount, Transfer};

use super::manage_position::update_position_direct;

pub struct Pool;
impl PoolMath for Pool {}

#[derive(Accounts)]
#[instruction(lower_tick: u64, upper_tick: u64, current_tick: u64)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [ POOL_STATE_SEED, pool_state.lp_token_mint.as_ref() ],
        bump = pool_state.pool_state_bump,
    )]
    pub pool_state: Account<'info, PoolState>,

    #[account(
        mut,
        seeds = [
            b"position", 
            pool_state.key().as_ref(),
            user.key().as_ref(),
            lower_tick.to_le_bytes().as_ref(),
            upper_tick.to_le_bytes().as_ref(),
        ],
        bump = position_state.bump,
        constraint = lower_tick < upper_tick,
    )]
    pub position_state: Account<'info, PositionState>,

    #[account(
        mut,
        seeds = [b"tick", pool_state.key().as_ref(), lower_tick.to_le_bytes().as_ref()],
        bump = lower_tick_state.bump,
        constraint = lower_tick_state.tick == position_state.lower_tick,
    )]
    pub lower_tick_state: Account<'info, TickState>,

    #[account(
        mut,
        seeds = [b"tick", pool_state.key().as_ref(), upper_tick.to_le_bytes().as_ref()],
        bump = upper_tick_state.bump,
        constraint = upper_tick_state.tick == position_state.upper_tick,
    )]
    pub upper_tick_state: Account<'info, TickState>,

    #[account(
        mut,
        seeds = [b"tick", pool_state.key().as_ref(), current_tick.to_le_bytes().as_ref()],
        bump = current_tick_state.bump,
        constraint = current_tick_state.tick == current_tick,
        constraint = current_tick_state.tick == pool_state.pool_global_state.tick,
    )]
    pub current_tick_state: Account<'info, TickState>,

    #[account(
        mut,
        constraint = lp_token_mint.key() == pool_state.lp_token_mint,
    )]
    pub lp_token_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        constraint = user_token_x.mint == pool_state.token_x_mint,
        constraint = user_token_x.owner == user.key()
    )]
    pub user_token_x: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = user_token_y.mint == pool_state.token_y_mint,
        constraint = user_token_y.owner == user.key()
    )]
    pub user_token_y: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [ TOKEN_VAULT_SEED, pool_state.token_x_mint.as_ref(), lp_token_mint.key().as_ref() ],
        bump,
        constraint = token_x_vault.key() == pool_state.token_x_vault,
    )]
    pub token_x_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [ TOKEN_VAULT_SEED, pool_state.token_y_mint.as_ref(), lp_token_mint.key().as_ref() ],
        bump,
        constraint = token_y_vault.key() == pool_state.token_y_vault,
    )]
    pub token_y_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [ LP_TOKEN_VAULT_SEED, pool_state.key().as_ref(), lp_token_mint.key().as_ref() ],
        bump,
    )]
    pub lp_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = lp_token_to.mint == pool_state.lp_token_mint,
    )]
    pub lp_token_to: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    //+ pub user_account:  Account<'info, UserAccount>, // for PositionList and TempFees
    pub user: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
}

impl<'info> Deposit<'info> {
    pub fn transfer_user_token_x_to_vault(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_token_x.to_account_info(),
            to: self.token_x_vault.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn transfer_user_token_y_to_vault(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_token_y.to_account_info(),
            to: self.token_y_vault.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn mint_lp_tokens_to_user_account(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.lp_token_mint.to_account_info(),
            to: self.lp_token_to.to_account_info(),
            authority: self.pool_state.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn mint_and_lock_lp_tokens_to_pool_state_account(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.lp_token_mint.to_account_info(),
            to: self.lp_token_vault.to_account_info(),
            authority: self.pool_state.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handle(
    ctx: Context<Deposit>,
    lower_tick: u64,
    upper_tick: u64,
    current_tick: u64,
    token_x_amount: u64,
    token_y_amount: u64,
) -> Result<()> {
    let x = Decimal::from_u64(token_x_amount).to_amount();
    let y = Decimal::from_u64(token_y_amount).to_amount();

    let rpa_used = Pool::tick_to_rp(lower_tick as u128);
    let rpb_used = Pool::tick_to_rp(upper_tick as u128);
    let rp_used = Pool::tick_to_rp(current_tick as u128);

    // TODO should we use Oracle price here instead? or real price as param
    // ? only when no liquidity in range?

    let liquidity_delta = Pool::liq_from_x_y_rp_rng(x, y, rp_used, rpa_used, rpb_used);
    // round down to avoid float rounding vulnerabilities
    // TODO choose what precision to round down to
    // if Self::FLOOR_LIQ {
    //     liq = liq.floor().unwrap()
    // };
    if liquidity_delta.negative {
        // emit!(NegativeDepositLiquidity);
        return Err(ErrorCode::NegativeDepositLiquidity.into());
    }

    let x_in = Pool::x_from_l_rp_rng(liquidity_delta, rp_used, rpa_used, rpb_used);

    let y_in = Pool::y_from_l_rp_rng(liquidity_delta, rp_used, rpa_used, rpb_used);

    let zero = Decimal::from_u64(0).to_amount();
    // TODO calculate fees user temp_fees in UserAccount
    let (fees_x, fees_y, adj_x, adj_y) = (zero, zero, zero, zero);

    // update position_state and lower_ & upper_tick_state
    update_position_direct(
        &mut ctx.accounts.position_state,
        &mut ctx.accounts.lower_tick_state,
        &mut ctx.accounts.upper_tick_state,
        liquidity_delta,
        lower_tick,
        upper_tick,
    )
    .unwrap();

    // update global state's liquidity if current tick in within position's range
    let global_state = &mut ctx.accounts.pool_state.pool_global_state;
    let gs_liquidity = Decimal::from_account(global_state.liquidity, 0);

    if current_tick >= lower_tick && current_tick < upper_tick {
        let new_global_liquidity = gs_liquidity.add(liquidity_delta).unwrap();
        // this check may be redundant but just in case
        if new_global_liquidity.negative {
            emit!(NegativeGlobalLiquidity {
                original_liquidity: gs_liquidity.to_int(),
                attempted_removal: liquidity_delta.to_int()
            });
            return Err(ErrorCode::NegativeGlobalLiquidity.into());
        }
        global_state.liquidity = new_global_liquidity.to_account_value();
    }

    // offset fee amounts from deposit amounts: this will be the amount debited from user
    let x_debited = x_in.sub(fees_x).unwrap().sub(adj_x).unwrap();
    let y_debited = y_in.sub(fees_y).unwrap().sub(adj_y).unwrap();

    if x_debited.gt(x).unwrap() {
        emit!(DepositAmountExceeded {
            deposited: x.to_int(),
            attempted_debit: x_debited.to_int(),
        });
        return Err(ErrorCode::DepositAmountExceeded.into());
    }

    if y_debited.gt(y).unwrap() {
        emit!(DepositAmountExceeded {
            deposited: y.to_int(),
            attempted_debit: y_debited.to_int(),
        });
        return Err(ErrorCode::DepositAmountExceeded.into());
    }

    // update state: reserves, fee pot , hmm-adj-fee pot
    let seeds = &[
        POOL_STATE_SEED,
        ctx.accounts.pool_state.lp_token_mint.as_ref(),
        &[ctx.accounts.pool_state.pool_state_bump],
    ];
    let signer = [&seeds[..]];

    // mint lp tokens to users account
    token::mint_to(
        ctx.accounts
            .mint_lp_tokens_to_user_account()
            .with_signer(&signer),
        liquidity_delta.to_int(),
    )?;

    // transfer to vault
    token::transfer(
        ctx.accounts.transfer_user_token_x_to_vault(),
        x_debited.to_int(),
    )?;

    // transfer to vault
    token::transfer(
        ctx.accounts.transfer_user_token_y_to_vault(),
        y_debited.to_int(),
    )?;

    emit!(LiquidityAdded {
        tokens_x_transferred: x_debited.to_int(),
        tokens_y_transferred: y_debited.to_int(),
        lp_tokens_minted: liquidity_delta.to_int(),
    });

    Ok(())
}
