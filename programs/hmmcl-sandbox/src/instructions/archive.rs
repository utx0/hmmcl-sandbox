// *********  TICK *********

//+ lib_rs
// /// update a tick ( this is done by pool - triggered by deposits or swaps)
// pub fn update_tick(
//     ctx: Context<UpdateTick>,
//     tick: u64,
//     liq: Decimal,
//     upper: bool,
// ) -> Result<()> {
//     instructions::manage_tick::update_tick(ctx, tick, liq, upper)
// }

// /// unset tick: flags a tick inactive when no position is referencing it
// pub fn unset_tick(ctx: Context<UnsetTick>, tick: u64) -> Result<()> {
//     instructions::manage_tick::unset_tick(ctx, tick)
// }

// /// crossing a tick during the swap process
// pub fn cross_tick(
//     ctx: Context<CrossTick>,
//     provided_tick: u64,
//     left_to_right: bool,
// ) -> Result<()> {
//     instructions::manage_tick::cross_tick(ctx, provided_tick, left_to_right)
// }

//+ update_tick handler when it is instruction
// pub fn _update_tick(
//     ctx: Context<UpdateTick>,
//     tick: u64,
//     liquidity_delta: Decimal,
//     upper: bool,
// ) -> Result<()> {
//     let tick_state = &mut ctx.accounts.tick_state;

// let applied_net_liquidity = match upper {
//     false => liquidity_delta,
//     true => Decimal::flip_sign(liquidity_delta),
// };
// tick_state.liq_net = tick_state.liq_net.add(applied_net_liquidity).unwrap();

// let new_gross_liquidity = tick_state.liq_gross.add(liquidity_delta).unwrap();
// if new_gross_liquidity.negative {
//     emit!(NegativeTickGrossLiquidity {
//         original_liquidity: tick_state.liq_gross.to_int(),
//         attempted_removal: liquidity_delta.to_int(),
//     });
//     return Err(ErrorCode::NegativeTickGrossLiquidity.into());
// }

// Ok(())
//     update_tick_direct(tick_state, tick, liquidity_delta, upper)
// }

//+ unset_tick handler
// pub fn unset_tick(ctx: Context<UnsetTick>, tick: u64) -> Result<()> {
//     msg!("{}", tick);
//     msg!("{}", ctx.program_id);

//     Ok(())
// }

//+ cross_tick handler
// pub fn cross_tick(ctx: Context<CrossTick>, provided_tick: u64, left_to_right: bool) -> Result<()> {
//     msg!("{}", provided_tick);
//     msg!("{}", left_to_right);
//     msg!("{}", ctx.program_id);
//     Ok(())
// }

//+ UpdateTick Context
// #[derive(Accounts)]
// #[instruction(tick: u64)]
// pub struct UpdateTick<'info> {
//     #[account(
//         // mut,
//         seeds = [ POOL_STATE_SEED, pool_state.lp_token_mint.as_ref() ],
//         bump = pool_state.pool_state_bump,
//     )]
//     pub pool_state: Account<'info, PoolState>,

//     #[account(
//         mut,
//         seeds = [b"tick", pool_state.key().as_ref(), tick.to_le_bytes().as_ref()],
//         bump = tick_state.bump,
//         // has_one = authority,
//         constraint = tick_state.authority == pool_state.key(),
//         constraint = tick_state.tick == tick,
//     )]
//     pub tick_state: Account<'info, TickState>,
//     //+ pub tick_list: Account<'info, TickList>,
//     // pub authority: Signer<'info>,
// }

//+ UnsetTick context
// #[derive(Accounts)]
// #[instruction(tick: u64)]
// pub struct UnsetTick<'info> {
//     #[account(
//         // mut,
//         seeds = [ POOL_STATE_SEED, pool_state.lp_token_mint.as_ref() ],
//         bump = pool_state.pool_state_bump,
//     )]
//     pub pool_state: Account<'info, PoolState>,

//     #[account(
//         mut,
//         seeds = [b"tick", pool_state.key().as_ref(), tick.to_le_bytes().as_ref()],
//         bump = tick_state.bump,
//         // has_one = authority,
//         constraint = tick_state.authority == pool_state.key(),
//         constraint = tick_state.tick == tick,
//     )]
//     pub tick_state: Account<'info, TickState>,
//     //+ pub tick_list: Account<'info, TickList>,
//     // pub authority: Signer<'info>,
// }

// +CrossTick context
// #[derive(Accounts)]
// #[instruction(provided_tick: u64)]
// pub struct CrossTick<'info> {
//     #[account(
//         // mut,
//         seeds = [ POOL_STATE_SEED, pool_state.lp_token_mint.as_ref() ],
//         bump = pool_state.pool_state_bump,
//     )]
//     pub pool_state: Account<'info, PoolState>,

//     #[account(
//         mut,
//         seeds = [b"tick", pool_state.key().as_ref(), provided_tick.to_le_bytes().as_ref()],
//         bump = tick_state.bump,
//         // has_one = authority,
//         constraint = tick_state.authority == pool_state.key(),
//     )]
//     pub tick_state: Account<'info, TickState>,
//     // pub authority: Signer<'info>,
// }

// *********  POSITION *********

//+ lib_rs
// /// user sets a position ( this will be used by deposits and withdrawals by user)
// pub fn update_position(
//     ctx: Context<UpdatePosition>,
//     liquidity_abs_value: u64,
//     liquidity_negative: bool,
//     lower_tick: u64,
//     upper_tick: u64,
// ) -> Result<()> {
//     instructions::manage_position::update_position(
//         ctx,
//         liquidity_abs_value,
//         liquidity_negative,
//         lower_tick,
//         upper_tick,
//     )
// }

//+ UpdatePosition context when it is an instruction
// #[derive(Accounts)]
// #[instruction(liquidity: u64, negative: bool, lower_tick: u64, upper_tick: u64)]
// pub struct UpdatePosition<'info> {
//     #[account(
//         mut,
//         seeds = [ POOL_STATE_SEED, pool_state.lp_token_mint.as_ref() ],
//         bump = pool_state.pool_state_bump,
//     )]
//     pub pool_state: Account<'info, PoolState>,

//     #[account(
//         mut,
//         seeds = [
//             b"position",
//             pool_state.key().as_ref(),
//             user.key().as_ref(),
//             lower_tick.to_le_bytes().as_ref(),
//             upper_tick.to_le_bytes().as_ref(),
//         ],
//         bump = position_state.bump,
//         constraint = lower_tick < upper_tick,
//     )]
//     pub position_state: Account<'info, PositionState>,

//     #[account(
//         mut,
//         seeds = [b"tick", pool_state.key().as_ref(), lower_tick.to_le_bytes().as_ref()],
//         bump = lower_tick_state.bump,
//         constraint = lower_tick_state.tick == position_state.lower_tick,
//     )]
//     pub lower_tick_state: Account<'info, TickState>,

//     #[account(
//         mut,
//         seeds = [b"tick", pool_state.key().as_ref(), upper_tick.to_le_bytes().as_ref()],
//         bump = upper_tick_state.bump,
//         constraint = upper_tick_state.tick == position_state.upper_tick,
//     )]
//     pub upper_tick_state: Account<'info, TickState>,

//     // pub user_account:  Account<'info, UserAccount>, // for PositionList and TempFees
//     pub user: Signer<'info>,

//     #[account(mut)]
//     pub payer: Signer<'info>,
// /// CHECK: only used as a signing PDA
// pub authority: UncheckedAccount<'info>,
// }

// +update_position handler when it is an instruction
// pub fn update_position(
//     ctx: Context<UpdatePosition>,
//     liquidity_abs_value: u64,
//     liquidity_negative: bool,
//     lower_tick: u64,
//     upper_tick: u64,
// ) -> Result<()> {
//     let position_state = &mut ctx.accounts.position_state;
//     let lower_tick_state = &mut ctx.accounts.lower_tick_state;
//     let upper_tick_state = &mut ctx.accounts.upper_tick_state;

//     // Update position liquidity
//     let mut liquidity_delta = Decimal::from_u64(liquidity_abs_value);
//     if liquidity_negative {
//         liquidity_delta = Decimal::flip_sign(liquidity_delta);
//     }

//     let new_liquidity = position_state.liquidity.add(liquidity_delta).unwrap();
//     if new_liquidity.negative {
//         emit!(InsufficientPositionLiquidity {
//             original_liquidity: position_state.liquidity.to_int(),
//             attempted_removal: liquidity_abs_value,
//         });
//         return Err(ErrorCode::InsufficientPositionLiquidity.into());
//     }

//     position_state.liquidity = new_liquidity;

//     // let &mut ctx_lt_accounts = &mut UpdateTick {
//     //     tick_state: ctx.accounts.lower_tick_state.clone(),
//     //     pool_state: &ctx.accounts.pool_state.clone(),
//     // };

//     // let update_lower_ctx: Context<UpdateTick> = Context {
//     //     accounts: &mut ctx_lt_accounts,
//     //     program_id: ctx.program_id,
//     //     remaining_accounts: ctx.remaining_accounts,
//     //     bumps: ctx.bumps,
//     // };
//     // update_tick(update_lower_ctx, lower_tick, liquidity_delta, false).unwrap();

//     // Update liquidity on respective tick_states
//     update_tick_direct(lower_tick_state, lower_tick, liquidity_delta, false).unwrap();
//     update_tick_direct(upper_tick_state, upper_tick, liquidity_delta, true).unwrap();

//     Ok(())
// }

//+ INIT_IF_NEEDED version
// #[derive(Accounts)]
// #[instruction(lower_tick: u64, upper_tick: u64)]
// pub struct SetPosition<'info> {
//     #[account(
//         init_if_needed,
//         seeds = [
//             b"position",
//             pool_state.key().as_ref(),
//             user.key().as_ref(),
//             lower_tick.to_le_bytes().as_ref(),
//             upper_tick.to_le_bytes().as_ref()
//         ],
//         bump,
//         payer=payer,
//         // constraint = position_state.upper_tick == upper_tick,
//         // constraint = position_state.lower_tick == lower_tick,
//     )]
//     pub position_state: Account<'info, PositionState>,
//     pub pool_state: Account<'info, PoolState>,
//     // pub user_account:  Account<'info, UserAccount>, // for PositionList and TempFees
//     pub user: Signer<'info>,

//     #[account(mut)]
//     pub payer: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

//* ******** Tick bitmap

// #[derive(Debug, Clone)]
// pub struct BitMap {
//     pub fixed_bitset: FixedBitSet,
// }

// impl Default for BitMap {
//     fn default() -> Self {
//         Self {
//             fixed_bitset: FixedBitSet::with_capacity(TICK_BITMAP_SIZE),
//         }
//     }
// }

// impl AnchorSerialize for BitMap {
//     fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
//         writer.write_all(&self.fixed_bitset.as_slice().try_to_vec()?[..])
//     }
// }

// impl AnchorDeserialize for BitMap {
//     fn deserialize(_buf: &mut &[u8]) -> std::io::Result<Self> {
//         let mut fixed_bitset =
//             FixedBitSet::with_capacity_and_blocks(TICK_BITMAP_SIZE, _buf.try_to_vec());

//         Ok(Self { fixed_bitset })
//     }
// }

// #[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
// pub struct PoolTickBitmap(Vec<u8>);

// impl AnchorSerialize for PoolTickBitmap {
//     fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
//         self.0.serialize(writer)
//     }
// }

// impl AnchorDeserialize for PoolTickBitmap {
//     fn deserialize(_buf: &mut &[u8]) -> std::io::Result<Self> {
//         Ok(Self { 0: _buf.to_vec() })
//     }
// }

// impl Default for PoolTickBitmap {
//     fn default() -> Self {
//         let mut v: Vec<u8> = Vec::with_capacity(TICK_BITMAP_SIZE);
//         for _i in 0..TICK_BITMAP_SIZE {
//             v.push(0);
//         }
//         PoolTickBitmap { 0: v }
//     }
// }

// #[derive(Clone, Debug)]
// // pub struct PoolTickBitmap([u8; TICK_BITMAP_SIZE]);
// impl AnchorSerialize for PoolTickBitmap {
//     fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
//         writer.write_all(&self.tick_map)
//     }
// }

// impl AnchorDeserialize for PoolTickBitmap {
//     fn deserialize(_buf: &mut &[u8]) -> std::io::Result<Self> {
//         Ok(Self {
//             tick_map: [0u8; TICK_BITMAP_SIZE],
//         })

//         // Ok(Self(
//         //     _buf.to_vec()
//         //         .rsplit_array_ref::<TICK_BITMAP_SIZE>()
//         //         .1
//         //         .clone(),
//         // ))
//     }
// }

//** other
// use anchor_lang::solana_program::log::sol_log_compute_units;
// use anchor_lang::solana_program::sysvar;

// let now = sysvar::clock::Clock::get().unwrap().unix_timestamp as u64;
// msg!("Now = {:?}", now);
// sol_log_compute_units();

// let mut ticks = bitvec![u64, Lsb0; 0; 100_000];

// // set some ticks as active
// ticks.set(69, true);
// ticks.set(420, true);

// // get a particular tick index value
// assert_eq!(*ticks.get_mut(69).unwrap(), true);
// assert_eq!(*ticks.get_mut(420).unwrap(), true);
// assert_eq!(*ticks.get_mut(42069).unwrap(), false);
// assert_eq!(ticks.is_empty(), false);

// let mut iter = ticks.iter();

// // grab ticks automatically (we'd wrap this in a function much like get_next_tick)
// let next_tick = iter.position(|tick| tick == true).unwrap();
// assert_eq!(next_tick, 69);
// let last_tick = next_tick;
// let next_tick = iter.position(|tick| tick == true).unwrap();
// assert_eq!(next_tick + last_tick + 1, 420);

// // iterate over all the ticks left to right
// for (i, tick) in enumerate(&ticks) {
//     if *tick {
//         println!("tick {} is active", i);
//     }
// }

// // iterate over all the ticks right to left
// for (i, tick) in enumerate(&ticks).rev() {
//     if *tick {
//         println!("tick {} is active", i);
//     }
// }

// sol_log_compute_units();
// let mut ticks_big = bitvec![u8, Lsb0; 0; 250_000];
// sol_log_compute_units();
// Ok(())

// pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//     // recreate from an array of bits
//     let bits = bits![0, 1, 0, 0, 1];
//     let mut ticks = BitVec::from_bitslice(bits);

//     // set some ticks as active
//     ticks.set(0, true);
//     ticks.set(1, false);

//     // grab a tick
//     let tick = *ticks.get_mut(1).unwrap();
//     msg!("tick is false {:?}", tick);

//     Ok(())
// }
