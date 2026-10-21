use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RSIConfig {
	pub period: Option<u32>,
}

/// Relative Strength Index (RSI), Wilder-smoothed.
///
/// The first value is emitted at index `period` from the mean of changes
/// `1..=period`; later values use `avg = avg * (1 - 1/period) + change/period`.
/// Everything before `period` is `NaN`.
///
/// # Examples
/// ```
/// use indicators_core::{rsi, RSIConfig};
///
/// let closings = vec![10.0, 9.0, 11.0, 10.0, 12.0];
/// let result = rsi(&closings, None);
/// assert_eq!(result.len(), 5);
/// for &v in &result {
///     if !v.is_nan() {
///         assert!(v >= 0.0 && v <= 100.0);
///     }
/// }
/// ```
pub fn rsi(closings: &[f64], config: Option<RSIConfig>) -> Vec<f64> {
	let config_obj = config.unwrap_or(RSIConfig { period: None });
	let period = config_obj.period.unwrap_or(14) as usize;

	let len = closings.len();

	if len <= period || period < 2 {
		return vec![f64::NAN; len];
	}

	let mut gains = vec![0.0; len];
	let mut losses = vec![0.0; len];

	for i in 1..len {
		let difference = closings[i] - closings[i - 1];

		gains[i] = if difference > 0.0 { difference } else { 0.0 };
		losses[i] = if difference < 0.0 { -difference } else { 0.0 };
	}

	let mut mean_gains = 0.0;
	let mut mean_losses = 0.0;

	for i in 1..=period {
		mean_gains += gains[i];
		mean_losses += losses[i];
	}

	mean_gains /= period as f64;
	mean_losses /= period as f64;

	let mut result = vec![f64::NAN; len];
	result[period] = rsi_from_means(mean_gains, mean_losses);

	let k = 1.0 / period as f64;
	for i in period + 1..len {
		mean_gains = mean_gains * (1.0 - k) + gains[i] * k;
		mean_losses = mean_losses * (1.0 - k) + losses[i] * k;
		result[i] = rsi_from_means(mean_gains, mean_losses);
	}

	result
}

fn rsi_from_means(mean_gains: f64, mean_losses: f64) -> f64 {
	if mean_losses == 0.0 {
		100.0
	} else {
		let rs = mean_gains / mean_losses;
		100.0 - (100.0 / (1.0 + rs))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn reference_wilder(closes: &[f64], period: usize) -> Vec<f64> {
		let n = closes.len();
		let mut out = vec![f64::NAN; n];
		if n <= period {
			return out;
		}
		let mut ag = (1..=period)
			.map(|i| (closes[i] - closes[i - 1]).max(0.0))
			.sum::<f64>()
			/ period as f64;
		let mut al = (1..=period)
			.map(|i| (closes[i - 1] - closes[i]).max(0.0))
			.sum::<f64>()
			/ period as f64;
		let val = |ag: f64, al: f64| {
			if al == 0.0 {
				100.0
			} else {
				100.0 - 100.0 / (1.0 + ag / al)
			}
		};
		out[period] = val(ag, al);
		for i in period + 1..n {
			ag += ((closes[i] - closes[i - 1]).max(0.0) - ag) / period as f64;
			al += ((closes[i - 1] - closes[i]).max(0.0) - al) / period as f64;
			out[i] = val(ag, al);
		}
		out
	}

	#[test]
	fn golden_values_match_wilder() {
		let closes = [
			100.0, 101.5, 100.75, 102.25, 103.0, 101.5, 100.25, 99.5, 101.0, 102.5, 103.25, 102.0,
			101.25, 102.75, 104.0, 103.5, 105.0, 104.25, 103.0, 105.5, 106.25, 105.0, 104.5, 106.0,
			107.5, 106.75, 108.0, 107.25, 109.0, 108.5,
		];
		let expected = [
			62.121_212_121_212_1,
			60.158_013_544_018_3,
			63.849_062_549_191_9,
			60.815_334_710_924_9,
			56.036_454_203_743_5,
			62.400_205_774_692_4,
			64.080_024_709_229_9,
			59.323_018_909_207_4,
			57.484_752_911_928_5,
			61.353_750_035_495_4,
			64.803_140_057_687_6,
			61.831_479_040_876_4,
			64.734_107_994_735_4,
			61.702_155_040_306_5,
			65.734_916_661_212_1,
			63.671_946_343_075_6,
		];
		let got = rsi(&closes, Some(RSIConfig { period: Some(14) }));
		assert!(got.iter().take(14).all(|v| v.is_nan()));
		for (i, want) in expected.iter().enumerate() {
			let g = got[14 + i];
			assert!(
				(g - want).abs() < 1e-9,
				"index {}: got {g}, want {want}",
				14 + i
			);
		}
	}

	#[test]
	fn first_value_is_the_seed() {
		let rising: Vec<f64> = (0..30).map(|i| 100.0 + i as f64).collect();
		let got = rsi(&rising, Some(RSIConfig { period: Some(14) }));
		assert!(got[..14].iter().all(|v| v.is_nan()));
		assert_eq!(got[14], 100.0);

		let falling: Vec<f64> = (0..30).map(|i| 200.0 - i as f64).collect();
		let got = rsi(&falling, Some(RSIConfig { period: Some(14) }));
		assert_eq!(got[14], 0.0);
	}

	#[test]
	fn no_output_depends_on_a_later_close() {
		let closes: Vec<f64> = (0..40)
			.map(|i| 100.0 + (i as f64 * 0.7).sin() * 3.0 + (i % 5) as f64 * 0.3)
			.collect();
		let base = rsi(&closes, Some(RSIConfig { period: Some(14) }));
		assert!(!base[13].is_finite());
		for cut in 14..closes.len() {
			let mut perturbed = closes.clone();
			for v in perturbed.iter_mut().skip(cut) {
				*v += 25.0;
			}
			let moved = rsi(&perturbed, Some(RSIConfig { period: Some(14) }));
			for i in 0..cut {
				let (a, b) = (base[i], moved[i]);
				assert!(
					(a.is_nan() && b.is_nan()) || a == b,
					"output[{i}] changed when closes[{cut}..] moved: {a} -> {b}"
				);
			}
		}
	}

	#[test]
	fn too_little_history_is_all_nan() {
		for n in 0..=14 {
			let closes: Vec<f64> = (0..n).map(|i| 100.0 + i as f64).collect();
			let got = rsi(&closes, Some(RSIConfig { period: Some(14) }));
			assert_eq!(got.len(), n);
			assert!(got.iter().all(|v| v.is_nan()));
		}
	}

	#[test]
	fn period_below_two_is_all_nan() {
		let closes: Vec<f64> = (0..30).map(|i| 100.0 + i as f64).collect();
		for period in [0u32, 1] {
			let got = rsi(
				&closes,
				Some(RSIConfig {
					period: Some(period),
				}),
			);
			assert!(got.iter().all(|v| v.is_nan()));
		}
	}

	#[test]
	fn matches_a_naive_reference_for_every_period() {
		let mut state = 0x2545_F491_4F6C_DD1Du64;
		let mut next = || {
			state ^= state << 13;
			state ^= state >> 7;
			state ^= state << 17;
			(state as f64 / u64::MAX as f64) - 0.5
		};
		let closes: Vec<f64> = (0..200).fold(Vec::with_capacity(200), |mut acc, i| {
			let prev = acc.last().copied().unwrap_or(100.0);
			acc.push(prev * (1.0 + next() * 0.02 + 0.001 * i as f64));
			acc
		});
		for period in [2usize, 3, 5, 14, 20, 50] {
			let got = rsi(
				&closes,
				Some(RSIConfig {
					period: Some(period as u32),
				}),
			);
			let want = reference_wilder(&closes, period);
			for i in 0..closes.len() {
				let (a, b) = (got[i], want[i]);
				assert!(
					(a.is_nan() && b.is_nan()) || (a - b).abs() < 1e-9,
					"period {period} index {i}: got {a}, want {b}"
				);
			}
		}
	}
}
