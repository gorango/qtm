use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RSIConfig {
	pub period: Option<u32>,
}

/// Relative Strength Index (RSI).
///
/// Measures the magnitude of recent price changes to evaluate overbought (>70)
/// or oversold (<30) conditions. Uses Wilder's smoothing method.
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

	if len < period || period < 2 {
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

	if period == 1 {
		result.fill(100.0);
		return result;
	}

	let k = 1.0 / period as f64;

	for i in period - 1..len {
		mean_gains = mean_gains * (1.0 - k) + gains[i] * k;
		mean_losses = mean_losses * (1.0 - k) + losses[i] * k;

		let rs = if mean_losses == 0.0 {
			100.0
		} else {
			mean_gains / mean_losses
		};

		result[i] = 100.0 - (100.0 / (1.0 + rs));
	}

	result
}
