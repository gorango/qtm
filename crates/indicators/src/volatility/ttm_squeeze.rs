use crate::internal::ema::ema_internal;
use crate::internal::moving_std::std_dev_internal;
use crate::internal::sma::sma_internal;
use crate::internal::true_range::tr_internal;
use crate::trend::rma::rma_internal;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct TTMSqueezeResult {
	pub in_squeeze: Vec<bool>,
	pub breakout: Vec<Option<String>>,
}

pub fn ttm_squeeze(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	bb_period: Option<u32>,
	bb_std_dev: Option<f64>,
	kc_period: Option<u32>,
) -> Result<TTMSqueezeResult, String> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closes])?;

	let len = highs.len();

	let bb_period = bb_period.unwrap_or(20) as usize;
	let bb_std_dev = bb_std_dev.unwrap_or(2.0);
	let kc_period = kc_period.unwrap_or(20) as usize;

	crate::utils::validation::validate_period(bb_period)?;
	crate::utils::validation::validate_period(kc_period)?;

	let bb_std_devs = std_dev_internal(closes, bb_period);
	let bb_middles = sma_internal(closes, bb_period);

	let mut bb_upper = vec![f64::NAN; len];
	let mut bb_lower = vec![f64::NAN; len];

	for i in 0..len {
		if !bb_std_devs[i].is_nan() && !bb_middles[i].is_nan() {
			bb_upper[i] = bb_middles[i] + bb_std_devs[i] * bb_std_dev;
			bb_lower[i] = bb_middles[i] - bb_std_devs[i] * bb_std_dev;
		}
	}

	let tr_line = tr_internal(highs, lows, closes);
	let atr_line = rma_internal(&tr_line, kc_period);

	let atr2: Vec<f64> = atr_line.iter().map(|&val| val * 2.0).collect();

	let kc_middles = ema_internal(closes, kc_period);

	let mut kc_upper = vec![f64::NAN; len];
	let mut kc_lower = vec![f64::NAN; len];

	for i in 0..len {
		let mid = kc_middles[i];
		let atr_val = atr2[i];

		if !mid.is_nan() && !atr_val.is_nan() {
			kc_upper[i] = mid + atr_val;
			kc_lower[i] = mid - atr_val;
		}
	}

	let mut in_squeeze = vec![false; len];
	let mut breakout = vec![None; len];

	for i in 0..len {
		let bb_u = bb_upper[i];
		let bb_l = bb_lower[i];
		let kc_u = kc_upper[i];
		let kc_l = kc_lower[i];

		if !bb_u.is_nan() && !bb_l.is_nan() && !kc_u.is_nan() && !kc_l.is_nan() {
			in_squeeze[i] = bb_u <= kc_u && bb_l >= kc_l;

			if i > 0 && in_squeeze[i - 1] && !in_squeeze[i] {
				let prev_bb_u = bb_upper[i - 1];
				let prev_bb_l = bb_lower[i - 1];

				if !prev_bb_u.is_nan() && bb_u > kc_u && prev_bb_u <= kc_u {
					breakout[i] = Some("up".to_string());
				} else if !prev_bb_l.is_nan() && bb_l < kc_l && prev_bb_l >= kc_l {
					breakout[i] = Some("down".to_string());
				}
			}
		}
	}

	Ok(TTMSqueezeResult {
		in_squeeze,
		breakout,
	})
}
