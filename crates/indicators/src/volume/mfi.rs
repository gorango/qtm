use crate::internal::moving_sum::moving_sum_internal;
use crate::trend::typical_price::typical_price_internal;
use crate::utils::arrays::validate_arrays_equal_length;
use crate::utils::validation::validate_period;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct MFIConfig {
	pub period: Option<u32>,
	pub price_source: Option<String>,
}

pub fn mfi(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	volumes: &[f64],
	config: Option<MFIConfig>,
) -> Vec<f64> {
	validate_arrays_equal_length(&[highs, lows, closings, volumes]).unwrap();

	let len = highs.len();

	let cfg = config.unwrap_or(MFIConfig {
		period: Some(14),
		price_source: Some("typical".to_string()),
	});

	let period = cfg.period.unwrap_or(14) as usize;
	let _price_source = cfg.price_source.unwrap_or("typical".to_string());

	if period > 0 {
		validate_period(period).unwrap();
	}

	let typical_prices = typical_price_internal(highs, lows, closings);
	let mut raw_money_flow = Vec::with_capacity(len);
	let mut positive_money_flow = Vec::with_capacity(len);
	let mut negative_money_flow = Vec::with_capacity(len);

	for i in 0..len {
		raw_money_flow.push(typical_prices[i] * volumes[i]);

		if i == 0 || typical_prices[i] >= typical_prices[i - 1] {
			positive_money_flow.push(raw_money_flow[i]);
			negative_money_flow.push(0.0);
		} else {
			positive_money_flow.push(0.0);
			negative_money_flow.push(raw_money_flow[i]);
		}
	}

	let sum_positive = moving_sum_internal(&positive_money_flow, period);
	let sum_negative = moving_sum_internal(&negative_money_flow, period);
	let mut result = vec![f64::NAN; len];

	for i in 0..len {
		let sp = sum_positive[i];
		let sn = sum_negative[i];

		let money_ratio = if sn == 0.0 {
			if sp == 0.0 {
				0.0
			} else {
				f64::INFINITY
			}
		} else {
			sp / sn
		};

		result[i] = 100.0 - 100.0 / (1.0 + money_ratio);
	}

	result
}

pub fn money_flow_index(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	volumes: &[f64],
	config: Option<MFIConfig>,
) -> Vec<f64> {
	mfi(highs, lows, closings, volumes, config)
}
