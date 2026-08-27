use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct KAMAConfig {
	/// Lookback period for the efficiency ratio (default 10). Valid range 2..=100.
	pub period: Option<u32>,
	/// Fast smoothing constant input (classic value: 2)
	pub fast: Option<u32>,
	/// Slow smoothing constant input (classic value: 30)
	pub slow: Option<u32>,
}

/// Kaufman's Adaptive Moving Average.
///
/// The smoothing constant is derived from the efficiency ratio over
/// `period` bars, scaled between `2/(slow+1)` and `2/(fast+1)` and squared.
/// In a clean trend it tracks price closely; in chop it goes almost flat.
///
/// The recursion is seeded at bar `period` with the SMA of the window the
/// efficiency ratio looks at, so output before index `period` is NaN.
pub fn kama(values: &[f64], config: Option<KAMAConfig>) -> IndicatorResult<Vec<f64>> {
	let config = config.unwrap_or(KAMAConfig {
		period: Some(10),
		fast: Some(2),
		slow: Some(30),
	});

	let period = config.period.unwrap_or(10) as usize;
	let fast = config.fast.unwrap_or(2) as usize;
	let slow = config.slow.unwrap_or(30) as usize;

	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_period(fast)?;
	crate::utils::validation::validate_period(slow)?;
	crate::utils::validation::validate_finite(&[values])?;

	let len = values.len();
	let mut result = vec![f64::NAN; len];
	if len <= period {
		return Ok(result);
	}

	let fast_sc = 2.0 / (fast as f64 + 1.0);
	let slow_sc = 2.0 / (slow as f64 + 1.0);

	let er = crate::trend::kaufman_efficiency_ratio::er_internal(values, period);

	// seed with the mean of the same window the first ER covers
	let mut prev = values[1..=period].iter().sum::<f64>() / period as f64;
	result[period] = prev;

	for i in period + 1..len {
		let sc = (er[i] * (fast_sc - slow_sc) + slow_sc).powi(2);
		prev += sc * (values[i] - prev);
		result[i] = prev;
	}

	Ok(result)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn warmup_then_finite() {
		let values: Vec<f64> = (0..30).map(|i| 100.0 + i as f64).collect();
		let result = kama(&values, None).unwrap();
		assert!(result[..10].iter().all(|v| v.is_nan()));
		assert!(result[10..].iter().all(|v| v.is_finite()));
	}

	#[test]
	fn flat_input_stays_flat() {
		let values = vec![42.0; 20];
		let result = kama(&values, None).unwrap();
		for &v in &result[10..] {
			assert!((v - 42.0).abs() < 1e-12);
		}
	}

	#[test]
	fn hand_computed_small_case() {
		// period=2, fast=2, slow=3 on [1,2,4,5]:
		// ER at i=2 is |4-1|/(1+2) = 1; ER at i=3 is |5-2|/(2+1) = 1
		// fast_sc = 2/3, slow_sc = 2/4 = 1/2
		// seed = (2+4)/2 = 3; then sc = (1*(2/3-1/2)+1/2)^2 = (2/3)^2 = 4/9
		// kama[3] = 3 + 4/9 * (5-3) = 35/9
		let values = vec![1.0, 2.0, 4.0, 5.0];
		let result = kama(
			&values,
			Some(KAMAConfig {
				period: Some(2),
				fast: Some(2),
				slow: Some(3),
			}),
		)
		.unwrap();
		assert_eq!(result[2], 3.0);
		let expected = 3.0 + (2.0f64 / 3.0).powi(2) * 2.0;
		assert!((result[3] - expected).abs() < 1e-12);
	}

	#[test]
	fn trending_series_lags_below_price() {
		let values: Vec<f64> = (0..60).map(|i| 100.0 + i as f64).collect();
		let result = kama(&values, None).unwrap();
		for i in 11..values.len() {
			assert!(
				result[i] > result[i - 1],
				"kama must rise on a rising series"
			);
			assert!(
				result[i] < values[i],
				"adaptive MA must lag a straight-line rally"
			);
		}
	}

	#[test]
	fn short_input_is_all_nan() {
		let values = vec![1.0];
		let result = kama(&values, None).unwrap();
		assert!(result.iter().all(|v| v.is_nan()));
	}
}
