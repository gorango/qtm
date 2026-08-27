use crate::internal::sma::sma_internal;
use crate::utils::validation::validate_period;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct KSTConfig {
	pub roc1_period: Option<u32>,
	pub roc2_period: Option<u32>,
	pub roc3_period: Option<u32>,
	pub roc4_period: Option<u32>,
	pub sma1_period: Option<u32>,
	pub sma2_period: Option<u32>,
	pub sma3_period: Option<u32>,
	pub sma4_period: Option<u32>,
	pub signal_period: Option<u32>,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct KSTResult {
	pub kst: Vec<f64>,
	pub signal: Vec<f64>,
}

/// Know Sure Thing (KST) — weighted sum of four ROCs (Martin Pring).
/// `KST = Σ weight_i * ROC(period_i)`, signal = SMA(KST, signal_period). Bounded roughly -100..100.
///
/// # Errors
/// Returns an error if inputs invalid.
pub fn kst(prices: &[f64], config: Option<KSTConfig>) -> KSTResult {
	let config_obj = config.unwrap_or(KSTConfig {
		roc1_period: None,
		roc2_period: None,
		roc3_period: None,
		roc4_period: None,
		sma1_period: None,
		sma2_period: None,
		sma3_period: None,
		sma4_period: None,
		signal_period: None,
	});

	let roc1 = config_obj.roc1_period.unwrap_or(10) as usize;
	let roc2 = config_obj.roc2_period.unwrap_or(15) as usize;
	let roc3 = config_obj.roc3_period.unwrap_or(20) as usize;
	let roc4 = config_obj.roc4_period.unwrap_or(30) as usize;
	let sma1 = config_obj.sma1_period.unwrap_or(10) as usize;
	let sma2 = config_obj.sma2_period.unwrap_or(10) as usize;
	let sma3 = config_obj.sma3_period.unwrap_or(10) as usize;
	let sma4 = config_obj.sma4_period.unwrap_or(15) as usize;
	let signal_period = config_obj.signal_period.unwrap_or(9) as usize;

	let _ = validate_period(signal_period);

	let len = prices.len();
	let _rc1 = vec![f64::NAN; len];
	let _rc2 = vec![f64::NAN; len];
	let _rc3 = vec![f64::NAN; len];
	let _rc4 = vec![f64::NAN; len];

	fn roc(values: &[f64], period: usize) -> Vec<f64> {
		let len = values.len();
		let mut result = vec![f64::NAN; len];

		for i in period..len {
			let current = values[i];
			let past = values[i - period];
			if !past.is_nan() && past != 0.0 {
				result[i] = ((current - past) / past.abs()) * 100.0;
			}
		}

		result
	}

	let roc1_vals = roc(prices, roc1);
	let roc2_vals = roc(prices, roc2);
	let roc3_vals = roc(prices, roc3);
	let roc4_vals = roc(prices, roc4);

	let rcma1 = sma_internal(&roc1_vals, sma1);
	let rcma2 = sma_internal(&roc2_vals, sma2);
	let rcma3 = sma_internal(&roc3_vals, sma3);
	let rcma4 = sma_internal(&roc4_vals, sma4);

	let mut kst_value = vec![f64::NAN; len];

	let max_len = rcma1.len();
	for i in 0..max_len {
		let v1 = rcma1[i];
		let v2 = rcma2[i];
		let v3 = rcma3[i];
		let v4 = rcma4[i];

		if v1.is_nan() || v2.is_nan() || v3.is_nan() || v4.is_nan() {
			kst_value[i] = f64::NAN;
		} else {
			kst_value[i] = v1 + v2 + v3 + v4;
		}
	}

	let signal_value = sma_internal(&kst_value, signal_period);

	KSTResult {
		kst: kst_value,
		signal: signal_value,
	}
}
