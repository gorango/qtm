use crate::IndicatorResult;

/// Kaufman Efficiency Ratio over each rolling window.
///
/// `ER = |close[i] - close[i-period]| / Σ|close[j] - close[j-1]|`
///
/// Ranges 0..=1: 1 means every move went the same direction (pure trend),
/// 0 means the price traveled a lot and got nowhere (pure noise). The first
/// `period` bars have no full window and are NaN. A completely flat window
/// has zero path length, which we report as 0 rather than dividing by zero.
/// Efficiency Ratio kernel — `|close[i]-close[i-period]| / Σ|close[j]-close[j-1]|`.
pub fn er_internal(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	let mut result = vec![f64::NAN; len];
	if len <= period {
		return result;
	}

	for i in period..len {
		let mut volatility = 0.0;
		for j in i - period + 1..=i {
			volatility += (values[j] - values[j - 1]).abs();
		}

		result[i] = if volatility != 0.0 {
			(values[i] - values[i - period]).abs() / volatility
		} else {
			0.0
		};
	}

	result
}

/// Kaufman Efficiency Ratio — `0..1`; 1 = perfect trend, 0 = noise.
/// Numerator = net change over period, denominator = sum of absolute bar changes.
/// Used inside KAMA. Period defaults to 10. Direct Kaufman definition.
pub fn kaufman_efficiency_ratio(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(10) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;
	Ok(er_internal(values, period))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn monotonic_series_has_er_of_one() {
		let values: Vec<f64> = (0..20).map(|i| 100.0 + i as f64).collect();
		let result = kaufman_efficiency_ratio(&values, Some(10)).unwrap();
		for &v in &result[10..] {
			assert!((v - 1.0).abs() < 1e-12);
		}
	}

	#[test]
	fn perfect_zigzag_has_er_of_zero() {
		// every move is +1 or -1 over the same distance, so the net change is 0
		let values: Vec<f64> = (0..20).map(|i| 100.0 + (i % 2) as f64).collect();
		let result = kaufman_efficiency_ratio(&values, Some(10)).unwrap();
		for &v in &result[10..] {
			assert!(v.abs() < 1e-12);
		}
	}

	#[test]
	fn flat_series_reports_zero_not_nan() {
		let values = vec![50.0; 15];
		let result = kaufman_efficiency_ratio(&values, Some(10)).unwrap();
		for &v in &result[10..] {
			assert_eq!(v, 0.0);
		}
	}

	#[test]
	fn known_window_values() {
		let values = vec![1.0, 3.0, 2.0, 5.0];
		let result = kaufman_efficiency_ratio(&values, Some(3)).unwrap();
		assert!(result[0..3].iter().all(|v| v.is_nan()));
		// net |5 - 1| = 4, path |3-1| + |2-3| + |5-2| = 6
		assert!((result[3] - 4.0 / 6.0).abs() < 1e-12);
	}

	#[test]
	fn short_input_is_all_nan() {
		let values = vec![1.0, 2.0];
		let result = kaufman_efficiency_ratio(&values, Some(10)).unwrap();
		assert_eq!(result.len(), 2);
		assert!(result.iter().all(|v| v.is_nan()));
	}
}
