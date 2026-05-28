use crate::utils::validation::validate_multiple_arrays;
use serde::{Deserialize, Serialize};

fn average_price_reducer(
	highs: &[f64],
	lows: &[f64],
	period: usize,
	projection: usize,
) -> Vec<f64> {
	let len = highs.len();
	let mut result = vec![0.0; len + projection];

	for i in 0..len {
		if i < period - 1 {
			result[i + projection] = 0.0;
			continue;
		}

		let from = i + 1 - period;
		let to = std::cmp::min(i + 1, len);

		let max_val = highs[from..to]
			.iter()
			.copied()
			.fold(f64::NEG_INFINITY, f64::max);
		let min_val = lows[from..to].iter().copied().fold(f64::INFINITY, f64::min);

		result[i + projection] = (max_val + min_val) / 2.0;
	}

	result
}

fn calculate_tenkan_sen(highs: &[f64], lows: &[f64], short: usize) -> Vec<f64> {
	average_price_reducer(highs, lows, short, 0)
}

fn calculate_kijun_sen(highs: &[f64], lows: &[f64], medium: usize) -> Vec<f64> {
	average_price_reducer(highs, lows, medium, 0)
}

fn calculate_senkou_span_a(tenkan_sen: &[f64], kijun_sen: &[f64], medium: usize) -> Vec<f64> {
	let len = kijun_sen.len();
	let mut ssa = vec![0.0; len + medium];

	for (i, &k) in kijun_sen.iter().enumerate() {
		if k != 0.0 {
			ssa[i + medium] = (k + tenkan_sen.get(i).copied().unwrap_or(0.0)) / 2.0;
		}
	}

	ssa
}

fn calculate_senkou_span_b(highs: &[f64], lows: &[f64], long: usize, medium: usize) -> Vec<f64> {
	let len = highs.len();
	let mut ssb = vec![0.0; len + medium];

	for i in 0..len {
		if i + 1 < long {
			ssb[i + medium] = 0.0;
			continue;
		}

		let from = i + 1 - long;
		let to = std::cmp::min(i + 1, len);

		let max_val = highs[from..to]
			.iter()
			.copied()
			.fold(f64::NEG_INFINITY, f64::max);
		let min_val = lows[from..to].iter().copied().fold(f64::INFINITY, f64::min);

		ssb[i + medium] = (max_val + min_val) / 2.0;
	}

	ssb
}

fn shift_left_by(n: usize, values: &[f64]) -> Vec<f64> {
	let len = values.len();
	let mut result = vec![0.0; len];

	for (i, &val) in values.iter().enumerate().skip(n).take(len - n) {
		let new_index = (i - n + len) % len;
		result[new_index] = val;
	}

	result
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct IchimokuCloudConfig {
	pub short: Option<u32>,
	pub medium: Option<u32>,
	pub long: Option<u32>,
	pub close: Option<u32>,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct IchimokuCloudResult {
	pub tenkan: Vec<f64>,
	pub kijun: Vec<f64>,
	pub ssa: Vec<f64>,
	pub ssb: Vec<f64>,
	pub lagging_span: Vec<f64>,
}

pub fn ichimoku_cloud(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	config: Option<IchimokuCloudConfig>,
) -> IchimokuCloudResult {
	validate_multiple_arrays(&[highs, lows, closings]).unwrap();

	let config_obj = config.unwrap_or(IchimokuCloudConfig {
		short: None,
		medium: None,
		long: None,
		close: None,
	});
	let short = config_obj.short.unwrap_or(9) as usize;
	let medium = config_obj.medium.unwrap_or(26) as usize;
	let long = config_obj.long.unwrap_or(52) as usize;
	let close = config_obj.close.unwrap_or(26) as usize;

	if highs.is_empty() {
		return IchimokuCloudResult {
			tenkan: vec![],
			kijun: vec![],
			ssa: vec![],
			ssb: vec![],
			lagging_span: vec![],
		};
	}

	let tenkan = calculate_tenkan_sen(highs, lows, short);
	let kijun = calculate_kijun_sen(highs, lows, medium);
	let ssa = calculate_senkou_span_a(&tenkan, &kijun, medium);
	let ssb = calculate_senkou_span_b(highs, lows, long, medium);
	let lagging_span = shift_left_by(close, closings);

	IchimokuCloudResult {
		tenkan,
		kijun,
		ssa,
		ssb,
		lagging_span,
	}
}

pub fn ichimoku(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	config: Option<IchimokuCloudConfig>,
) -> IchimokuCloudResult {
	ichimoku_cloud(highs, lows, closings, config)
}
