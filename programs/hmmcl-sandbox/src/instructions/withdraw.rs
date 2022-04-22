use std::ops::Neg;

use crate::cl_pool::cl_math::PoolMath;
use crate::constants::*;
use crate::decimal::*;
use crate::instructions::manage_fee::compute_latest_fee;
use crate::instructions::manage_position::update_position;
use crate::state::pool_state::PoolState;
use crate::state::position_state::PositionState;
use crate::state::tick_bitmap::PoolTickBitmap;
use crate::state::tick_state::TickState;

use crate::errors::ErrorCode;
use crate::events::*;

use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Burn, Mint, Token, TokenAccount, Transfer};

pub struct Pool;
impl PoolMath for Pool {}

#[derive(Accounts)]
#[instruction(lower_tick: u64, upper_tick: u64, current_tick: u64)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [ POOL_STATE_SEED, pool_state.lp_token_mint.as_ref() ],
        bump = pool_state.pool_state_bump,
    )]
    pub pool_state: Box<Account<'info, PoolState>>,

    #[account(
        mut,
        seeds = [ b"bitmap", pool_state.key().as_ref() ],
        bump= tick_bitmap.load()?.bump
    )]
    pub tick_bitmap: AccountLoader<'info, PoolTickBitmap>,

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
    pub position_state: Box<Account<'info, PositionState>>,

    #[account(
        mut,
        seeds = [b"tick", pool_state.key().as_ref(), lower_tick.to_le_bytes().as_ref()],
        bump = lower_tick_state.bump,
        constraint = lower_tick_state.tick == position_state.lower_tick,
    )]
    pub lower_tick_state: Box<Account<'info, TickState>>,

    #[account(
        mut,
        seeds = [b"tick", pool_state.key().as_ref(), upper_tick.to_le_bytes().as_ref()],
        bump = upper_tick_state.bump,
        constraint = upper_tick_state.tick == position_state.upper_tick,
    )]
    pub upper_tick_state: Box<Account<'info, TickState>>,

    #[account(
        mut,
        seeds = [b"tick", pool_state.key().as_ref(), current_tick.to_le_bytes().as_ref()],
        bump = current_tick_state.bump,
        constraint = current_tick_state.tick == current_tick,
        constraint = current_tick_state.tick == pool_state.pool_global_state.tick,
    )]
    pub current_tick_state: Box<Account<'info, TickState>>,

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
        constraint = lp_token_user_account.mint == pool_state.lp_token_mint,
    )]
    pub lp_token_user_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    //+ pub user_account:  Account<'info, UserAccount>, // for PositionList and TempFees
    pub user: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
}

impl<'info> Withdraw<'info> {
    pub fn transfer_vault_token_x_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.token_x_vault.to_account_info(),
            to: self.user_token_x.to_account_info(),
            authority: self.pool_state.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn transfer_vault_token_y_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.token_y_vault.to_account_info(),
            to: self.user_token_y.to_account_info(),
            authority: self.pool_state.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn burn_lp_tokens(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_accounts = Burn {
            mint: self.lp_token_mint.to_account_info(),
            to: self.lp_token_user_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handle(
    ctx: Context<Withdraw>,
    lower_tick: u64,
    upper_tick: u64,
    current_tick: u64,
    liquidity_to_withdraw: u64,
) -> Result<()> {
    // TODO conversion from lp_tokens to liquidity using mint decimals (and vice-versa)

    let liquidity_delta = Decimal::from_u64(liquidity_to_withdraw)
        .to_compute_scale()
        .neg();

    let rpa_used = Pool::tick_to_rp(lower_tick as u128);
    let rpb_used = Pool::tick_to_rp(upper_tick as u128);
    let rp_used = Pool::tick_to_rp(current_tick as u128);

    // Deal with fee accounting
    let old_fee = ctx.accounts.position_state.last_collected_fee;
    let lwr_fee = ctx.accounts.lower_tick_state.tick_fee;
    let upr_fee = ctx.accounts.upper_tick_state.tick_fee;
    let glbl_fee = ctx.accounts.pool_state.pool_global_state.global_fee;
    let new_fee = compute_latest_fee(
        lower_tick,
        upper_tick,
        current_tick,
        glbl_fee,
        lwr_fee,
        upr_fee,
        old_fee.fee_scale,
    );

    // update position_state and lower_ & upper_tick_state
    update_position(
        &mut ctx.accounts.position_state,
        &mut ctx.accounts.lower_tick_state,
        &mut ctx.accounts.upper_tick_state,
        liquidity_delta,
        lower_tick,
        upper_tick,
        new_fee,
    )
    .unwrap();

    // after operation, if lower or upper_tick has 0 gross liquidity then unset tick (from tickmap)
    if ctx.accounts.lower_tick_state.liq_gross == 0 {
        msg!("tick map needs updating");
        let tick_bitmap = &mut ctx.accounts.tick_bitmap.load_mut()?;
        tick_bitmap.unset_tick(0); // just mock number, to be fixed after 'spacing' implemented
    }
    if ctx.accounts.upper_tick_state.liq_gross == 0 {
        msg!("tick map needs updating");
        let tick_bitmap = &mut ctx.accounts.tick_bitmap.load_mut()?;
        tick_bitmap.unset_tick(4000); // just mock number, to be fixed after 'spacing' implemented
    }

    // update global state's liquidity if current tick in within position's range
    let global_state = &mut ctx.accounts.pool_state.pool_global_state;
    let gs_liquidity = Decimal::new(global_state.liquidity, global_state.liq_scale, false);

    if current_tick >= lower_tick && current_tick < upper_tick {
        let new_global_liquidity = gs_liquidity.add(liquidity_delta).unwrap();
        // this check may be redundant but just in case
        if new_global_liquidity.negative {
            emit!(NegativeGlobalLiquidity {
                original_liquidity: gs_liquidity.abs(),
                attempted_removal: liquidity_delta.abs()
            });
            return Err(ErrorCode::NegativeGlobalLiquidity.into());
        }

        global_state.liquidity = new_global_liquidity.value;
        global_state.liq_scale = new_global_liquidity.scale;
    }

    let x_out = Pool::x_from_l_rp_rng(liquidity_delta.neg(), rp_used, rpa_used, rpb_used);
    let y_out = Pool::y_from_l_rp_rng(liquidity_delta.neg(), rp_used, rpa_used, rpb_used);

    // TODO round down amount withdrawn if necessary, as precation

    // add fees on top of what user would receive
    let x_credited = x_out
        .add(
            Decimal::new(new_fee.f_x + new_fee.h_x, new_fee.fee_scale, false).to_scale(x_out.scale),
        )
        .unwrap();
    let y_credited = y_out
        .add(
            Decimal::new(new_fee.f_y + new_fee.h_y, new_fee.fee_scale, false).to_scale(y_out.scale),
        )
        .unwrap();

    // TODO first check x_out and y_out are not larger than reserves

    // update reserves and carry out transfers
    let seeds = &[
        POOL_STATE_SEED,
        ctx.accounts.pool_state.lp_token_mint.as_ref(),
        &[ctx.accounts.pool_state.pool_state_bump],
    ];
    let signer = [&seeds[..]];

    // burn lp tokens
    token::burn(ctx.accounts.burn_lp_tokens(), liquidity_to_withdraw)?;

    // transfer user_token_a to vault
    token::transfer(
        ctx.accounts
            .transfer_vault_token_x_to_user()
            .with_signer(&signer),
        x_credited.abs(),
    )?;

    // transfer user_token_b to vault
    token::transfer(
        ctx.accounts
            .transfer_vault_token_y_to_user()
            .with_signer(&signer),
        y_credited.abs(),
    )?;

    emit!(LiquidityRemoved {
        tokens_x_credited: x_credited.abs(),
        tokens_y_credited: y_credited.abs(),
        lp_tokens_burnt: liquidity_to_withdraw,
    });

    Ok(())
}
