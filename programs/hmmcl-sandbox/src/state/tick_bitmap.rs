use anchor_lang::prelude::*;

pub const TICK_BITMAP_SIZE: usize = 10000; // 221_816;
pub const TICKS_HALFWAY: usize = 5000; // 110_908;

pub const BITMAP_SPACE: usize = 8 + 1 + 221_816;

// (u128::MAX as f64).log(1.0001f64.sqrt()) is approximately 1774545(.50359)
// this would be the maximum tick that allowing the root-price to be inbound of u128 (not overflow)
// To allow for the price itself (square) to be inbound, we'll halve that (and on each side of 0)
// since we also want negative ticks to track very small prices less than 1.
// we'll use here the largest multiple of 32 that is less than 1_774_545 ==> 1_774_528
// that is 887_264 ticks represented on each side of 0.
// Also we'll represent a tick by one bit not the full byte. so we can use an array 8 times smaller:
// Hence array_size = 221_816 = 1_774_528 / 8. Or 110_908 on each side on tick 0
// The 1st half of indices will represent negative ticks

#[account(zero_copy)]
pub struct PoolTickBitmap {
    pub tick_map: [u8; TICK_BITMAP_SIZE],
    pub bump: u8,
}

impl Default for PoolTickBitmap {
    fn default() -> Self {
        PoolTickBitmap {
            tick_map: [0u8; TICK_BITMAP_SIZE],
            bump: 0,
        }
    }
}

impl PoolTickBitmap {
    pub fn activate_tick(&mut self, tick: i32) {
        // mark tick as initialized
        // key is the byte (u8 element of array) where the tick is stored
        let key = (tick >> 3) + TICKS_HALFWAY as i32;
        // ix is the particular bit of that byte that represents the tick; ix = tick % 8
        // we add TICK_BITMAP_SIZE (a multiple of 8) to avoid overflow for negative numbers
        let ix = (tick + TICK_BITMAP_SIZE as i32) % 8;
        self.tick_map[key as usize] = self.tick_map[key as usize] | (1 << ix);
    }

    pub fn unset_tick(&mut self, tick: i32) {
        // mark tick as no longer active, when no position references it anymore
        let key = (tick >> 3) + TICKS_HALFWAY as i32;
        let ix = (tick + TICK_BITMAP_SIZE as i32) % 8;
        self.tick_map[key as usize] = self.tick_map[key as usize] & !(1 << ix);
    }

    pub fn tick_is_active(&self, tick: i32) -> bool {
        // return true if particular is initialized (has some liquidity attached to it)
        let key = (tick >> 3) + TICKS_HALFWAY as i32;
        let ix = (tick + TICK_BITMAP_SIZE as i32) % 8;
        (self.tick_map[key as usize] >> ix & 1) != 0
    }

    pub fn get_next_tick(&self, current_tick: i32, direction: u8) -> Option<i32> {
        let current_key = (current_tick >> 3) + TICKS_HALFWAY as i32;
        let ix = ((current_tick + TICK_BITMAP_SIZE as i32) % 8) as u8;
        println!(
            "the current word is {} and the th {}, current_tick {}",
            current_key, ix, current_tick
        );
        let word_space: i32;
        if direction == 0u8 {
            word_space = -1;
        } else {
            word_space = 1;
        }
        //current word
        let cur = self.tick_map[current_key as usize];
        let res = byte_get_next_tick(cur, direction, ix, true);
        if res != None {
            // println!("the byte_get_next_tick ans is {:?}", res);
            let ans = ((current_key - TICKS_HALFWAY as i32) << 3) + (res.unwrap() as i32);
            return Some(ans);
        }
        // next word
        let iter_word;
        //fix me
        let mut next_ix = (current_key as i32) + word_space;
        if direction == 0 {
            println!("iter current_word is {}", current_key);
            iter_word = &self.tick_map[0..(current_key as usize)];
            for &cur in iter_word.iter().rev() {
                if cur > 0u8 {
                    let res = byte_get_next_tick(cur, direction, ix, false);
                    if res != None {
                        // println!("the next th is {} and the res is {}", next_ix, res.unwrap());
                        let ans = ((next_ix - TICKS_HALFWAY as i32) << 3) + (res.unwrap() as i32);
                        return Some(ans);
                    }
                }
                next_ix = next_ix + word_space;
            }
        } else {
            iter_word = &self.tick_map[(next_ix as usize)..(self.tick_map.len())];
            for &cur in iter_word {
                if cur > 0u8 {
                    let res = byte_get_next_tick(cur, direction, ix, false);
                    if res != None {
                        // println!("the next th is {} and the res is {}", next_ix, res.unwrap());
                        let ans = ((next_ix - TICKS_HALFWAY as i32) << 3) + (res.unwrap() as i32);
                        return Some(ans);
                    }
                }
                // println!("the next th is {}", next_ix);
                next_ix = next_ix + word_space;
            }
        }
        return None;
    }
}

pub fn byte_get_next_tick(byte: u8, direction: u8, ix: u8, is_contain: bool) -> Option<u8> {
    let mut start = 8u8;
    let mut end = 0u8;
    let mut ans: Option<u8> = None;
    if byte == 0 {
        return None;
    }
    if !is_contain {
        if direction == 0 {
            end = 8
        } else {
            start = 0
        }
    } else {
        if direction == 1 {
            start = ix + 1;
        } else {
            end = ix;
        }
    }
    if direction == 0u8 {
        for i in 0..end {
            let seq = end - i - 1;
            if byte >> seq & 0x1 == 1u8 {
                ans = Some(seq);
                break;
            }
        }
    } else {
        for i in start..8 {
            if byte >> i & 0x1 == 1u8 {
                ans = Some(i);
                break;
            }
        }
    }
    return ans;
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_bitmap() {
        let n = 400;
        let out = TICK_BITMAP_SIZE as i32 + 100;
        let mut b = PoolTickBitmap::default();

        assert_eq!(b.tick_is_active(0), false);
        assert_eq!(b.tick_is_active(-10), false);
        assert_eq!(b.tick_is_active(n), false);

        b.activate_tick(-10);
        b.activate_tick(n);
        b.activate_tick(n + 10);
        b.activate_tick(n + 100);

        assert_eq!(b.tick_is_active(-10), true);
        assert_eq!(b.tick_is_active(n), true);

        assert_eq!(b.get_next_tick(0, 0).unwrap_or(-out), -10);
        assert_eq!(b.get_next_tick(n, 0).unwrap_or(-out), -10);
        assert_eq!(b.get_next_tick(-10, 0).unwrap_or(-out), -out);
        assert_eq!(b.get_next_tick(-11, 0).unwrap_or(-out), -out);

        assert_eq!(b.get_next_tick(0, 1).unwrap_or(out), n);

        assert_eq!(b.get_next_tick(n + 1, 1).unwrap_or(out), n + 10);
        assert_eq!(b.get_next_tick(n + 20, 1).unwrap_or(out), n + 100);

        b.unset_tick(n);
        b.unset_tick(n + 1);
        assert_eq!(b.tick_is_active(n), false);
        assert_eq!(b.get_next_tick(0, 1).unwrap_or(out), n + 10);
        assert_eq!(b.get_next_tick(n + 1, 0).unwrap_or(-out), -10);

        assert_eq!(b.get_next_tick(n + 200, 0).unwrap_or(-out), n + 100);
    }

    #[test]
    fn test_fixed_bitset() {
        use fixedbitset::FixedBitSet;
        // let bs = FixedBitSet::with_capacity_and_blocks(4, vec![3]);
        let mut bs = FixedBitSet::with_capacity(1774545);
        // let mut bs = FixedBitSet::new();
        // bs.grow(8); //74592);
        bs.insert(0);
        bs.insert(887272);
        bs.insert(1774544);
        bs.set_range(42..46, true);

        let s: Vec<usize> = bs.ones().collect();
        println!("collected ones {:?}", s);
        // println!("0th ones {:}", bs.ones().nth(0).unwrap());
        let (bwd, fwd): (Vec<usize>, Vec<usize>) = s.iter().partition(|&n| n <= &43);
        println!("indices under {:?}", bwd);
        println!("indices over {:?}", fwd);

        let mut rev_bwd = bwd.clone();
        rev_bwd.reverse();
        let tick_bwd: Vec<&usize> = rev_bwd.iter().take(5).collect();
        let tick_fwd: Vec<&usize> = fwd.iter().take(5).collect();
        println!("under {:?}", tick_bwd);
        println!("over {:?}", tick_fwd);

        // println!("fbs {:}", bs);
        // println!("fbs slice {:?}", bs.as_slice().len());
        // println!("fbs len {:}", bs.len());
        // println!("fbs size {:}", std::mem::size_of_val(&bs));
    }

    #[test]
    fn test_bitvec() {
        use bitvec::prelude::*;
        let data = 2u64;
        let bitsl = data.view_bits::<Lsb0>();
        println!("{:}", bitsl);
        let bitsm = data.view_bits::<Msb0>();
        println!("{:}", bitsm);
        println!("bits l {:}", std::mem::size_of_val(&bitsl));
    }

    #[test]
    fn test_boundaries() {
        // let space: i32 = 1000;
        // let large: i32 = i32::MAX / space;
        let large: i32 = 10_u32.pow(4) as i32;
        println!("large: {:#?}", large);
        println!("large log 10: {:#?}", (large as f64).log10());

        let u_bound = 1.0001_f64.powi(large);
        println!("u_bound: {:#?}", u_bound);
        println!("u_bound log 10: {:#?}", u_bound.log10().floor());

        let u_bound_100 = 1.0001_f64.powi(large * 100);
        println!("u_bound_100: {:#?}", u_bound_100);
        println!("u_bound_100 log 10: {:#?}", u_bound_100.log10().floor());
        println!("u_bound_100 bit_length: {:#?}", u_bound_100.log2());

        let max_tick = (u128::MAX as f64).log(1.0001f64.sqrt());
        let max_price_digits = (u128::MAX as f64).log(10.0);
        println!("max tick :{:?}", max_tick);
        println!("max_price_digits :{:?}", max_price_digits);
    }
}
