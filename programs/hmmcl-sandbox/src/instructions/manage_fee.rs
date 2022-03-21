use crate::decimal::{Add, Decimal, Sub};
use crate::state::fees::Fee;

pub fn fee_earned_below_above(
    tick: u64,
    glbl_tick: u64,
    fg: Decimal,
    f0: Decimal,
) -> (Decimal, Decimal) {
    // Fees earned in a token below and above tick, as tuple.
    // can compute for either token, for either swap fees or hmm fees
    let f_below = match glbl_tick >= tick {
        true => f0,
        false => fg.sub(f0).unwrap(),
    }; // [6.18]
    let f_above = match glbl_tick >= tick {
        true => fg.sub(f0).unwrap(),
        false => f0,
    }; // [6.17]
    (f_below, f_above)
}

pub fn fee_earned_within_range(
    lower_tick: u64,
    upper_tick: u64,
    glbl_tick: u64,
    fg: Decimal,
    f0_lwr: Decimal,
    f0_upr: Decimal,
) -> Decimal {
    // Fees earned (per unit of liq) within a range of ticks (e.g. by a position)
    let (f_blw_lwr, f_abv_lwr) = fee_earned_below_above(lower_tick, glbl_tick, fg, f0_lwr);
    let (f_blw_upr, f_abv_upr) = fee_earned_below_above(upper_tick, glbl_tick, fg, f0_upr);

    // retrieve fg by summing up either tuple, they should match
    let sum_lwr = f_blw_lwr.add(f_abv_lwr).unwrap();
    let sum_upr = f_blw_upr.add(f_abv_upr).unwrap();

    // TODO proper error handling with own error code
    assert_eq!(sum_lwr, sum_upr);

    fg.sub(f_blw_lwr).unwrap().sub(f_abv_upr).unwrap()
}

pub fn compute_latest_fee(
    lower_tick: u64,
    upper_tick: u64,
    current_tick: u64,
    glbl_fee: Fee,
    lwr_fee: Fee,
    upr_fee: Fee,
    scale: u8,
) -> Fee {
    let new_fr_x = fee_earned_within_range(
        lower_tick,
        upper_tick,
        current_tick,
        Decimal::new(glbl_fee.f_x, glbl_fee.fee_scale, false),
        Decimal::new(lwr_fee.f_x, lwr_fee.fee_scale, false),
        Decimal::new(upr_fee.f_x, upr_fee.fee_scale, false),
    );
    let new_fr_y = fee_earned_within_range(
        lower_tick,
        upper_tick,
        current_tick,
        Decimal::new(glbl_fee.f_y, glbl_fee.fee_scale, false),
        Decimal::new(lwr_fee.f_y, lwr_fee.fee_scale, false),
        Decimal::new(upr_fee.f_y, upr_fee.fee_scale, false),
    );
    let new_hr_x = fee_earned_within_range(
        lower_tick,
        upper_tick,
        current_tick,
        Decimal::new(glbl_fee.h_x, glbl_fee.fee_scale, false),
        Decimal::new(lwr_fee.h_x, lwr_fee.fee_scale, false),
        Decimal::new(upr_fee.h_x, upr_fee.fee_scale, false),
    );
    let new_hr_y = fee_earned_within_range(
        lower_tick,
        upper_tick,
        current_tick,
        Decimal::new(glbl_fee.h_y, glbl_fee.fee_scale, false),
        Decimal::new(lwr_fee.h_y, lwr_fee.fee_scale, false),
        Decimal::new(upr_fee.h_y, upr_fee.fee_scale, false),
    );
    Fee {
        fee_scale: scale,
        f_x: new_fr_x.to_scale(scale).value,
        f_y: new_fr_y.to_scale(scale).value,
        h_x: new_hr_x.to_scale(scale).value,
        h_y: new_hr_y.to_scale(scale).value,
    }
}
