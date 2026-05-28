use crate::internal::moving_sum::moving_sum_internal;
use crate::trend::typical_price::typical_price_internal;
use crate::utils::arrays::validate_arrays_equal_length;
use crate::utils::validation::validate_period;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct VWAPConfig {
	pub period: Option<u32>,
	pub price_source: Option<String>,
	pub anchored: Option<bool>,
	pub session_length: Option<u32>,
}

pub fn vwap(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	volumes: &[f64],
	config: Option<VWAPConfig>,
) -> Vec<f64> {
	validate_arrays_equal_length(&[highs, lows, closings, volumes]).unwrap();

	let len = highs.len();
	let mut result = vec![f64::NAN; len];

	let cfg = config.unwrap_or(VWAPConfig {
		period: Some(14),
		price_source: Some("close".to_string()),
		anchored: Some(false),
		session_length: Some(0),
	});

	let period = cfg.period.unwrap_or(14) as usize;
	let price_source = cfg.price_source.unwrap_or("close".to_string());
	let anchored = cfg.anchored.unwrap_or(false);
	let session_length = cfg.session_length.unwrap_or(0) as usize;

	if period > 0 {
		validate_period(period).unwrap();
	}

	let prices = if price_source == "hlc3" {
		typical_price_internal(highs, lows, closings)
	} else {
		closings.to_vec()
	};

	if anchored {
		let mut cum_pv = Vec::with_capacity(len);
		let mut cum_v = Vec::with_capacity(len);
		let mut cpv = 0.0;
		let mut cv = 0.0;

		for i in 0..len {
			cpv += prices[i] * volumes[i];
			cv += volumes[i];
			cum_pv.push(cpv);
			cum_v.push(cv);
		}

		let sl = if session_length > 0 {
			session_length
		} else {
			len
		};

		for i in 0..len {
			let session_start = (i / sl) * sl;
			let start_pv = if session_start > 0 {
				cum_pv[session_start - 1]
			} else {
				0.0
			};
			let start_v = if session_start > 0 {
				cum_v[session_start - 1]
			} else {
				0.0
			};
			let current_pv = cum_pv[i];
			let current_v = cum_v[i];
			let pv = current_pv - start_pv;
			let v = current_v - start_v;
			result[i] = if v != 0.0 { pv / v } else { 0.0 };
		}
	} else {
		let mut price_volume = Vec::with_capacity(len);
		for i in 0..len {
			price_volume.push(prices[i] * volumes[i]);
		}

		let sum_price_volume = moving_sum_internal(&price_volume, period);
		let sum_volume = moving_sum_internal(volumes, period);

		for i in 0..len {
			let sv = sum_volume[i];
			result[i] = if sv != 0.0 {
				sum_price_volume[i] / sv
			} else {
				0.0
			};
		}
	}

	result
}

pub fn volume_weighted_average_price(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	volumes: &[f64],
	config: Option<VWAPConfig>,
) -> Vec<f64> {
	vwap(highs, lows, closings, volumes, config)
}
