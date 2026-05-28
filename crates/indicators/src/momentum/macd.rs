use crate::internal::ema::ema_internal;
use crate::utils::validation::validate_period;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct MACDConfig {
	pub fast_period: Option<u32>,
	pub slow_period: Option<u32>,
	pub signal_period: Option<u32>,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct MACDResult {
	pub macd: Vec<f64>,
	pub signal: Vec<f64>,
	pub histogram: Vec<f64>,
}

/// Moving Average Convergence Divergence (MACD).
///
/// Calculates the MACD line (fast EMA - slow EMA), signal line (EMA of MACD line),
/// and histogram (MACD - signal).
///
/// # Examples
/// ```
/// use indicators_core::{macd, MACDConfig};
///
/// let closes = vec![102.0, 106.0, 110.0, 113.0, 116.0, 118.0, 120.0, 123.0, 126.0, 128.0];
/// let result = macd(&closes, None).unwrap();
/// assert_eq!(result.macd.len(), 10);
/// assert_eq!(result.signal.len(), 10);
/// assert_eq!(result.histogram.len(), 10);
/// ```
pub fn macd(closes: &[f64], config: Option<MACDConfig>) -> Result<MACDResult, String> {
	let config_obj = config.unwrap_or(MACDConfig {
		fast_period: None,
		slow_period: None,
		signal_period: None,
	});

	let fast_period = config_obj.fast_period.unwrap_or(12) as usize;
	let slow_period = config_obj.slow_period.unwrap_or(26) as usize;
	let signal_period = config_obj.signal_period.unwrap_or(9) as usize;

	validate_period(fast_period)?;
	validate_period(slow_period)?;
	validate_period(signal_period)?;

	let len = closes.len();

	let fast_ema = ema_internal(closes, fast_period);
	let slow_ema = ema_internal(closes, slow_period);

	let mut macd_line = vec![f64::NAN; len];

	for i in 0..len {
		let f = fast_ema[i];
		let s = slow_ema[i];
		if !f.is_nan() && !s.is_nan() {
			macd_line[i] = f - s;
		}
	}

	let first_valid_idx = macd_line.iter().position(|&x| !x.is_nan());

	let signal_line = if let Some(idx) = first_valid_idx {
		let valid_macd_data = &macd_line[idx..];

		let calculated_signal = ema_internal(valid_macd_data, signal_period);

		let mut result = vec![f64::NAN; idx];
		result.extend(calculated_signal);
		result
	} else {
		vec![f64::NAN; len]
	};

	let mut histogram = vec![f64::NAN; len];

	for i in 0..len {
		let m = macd_line[i];
		let s = signal_line[i];

		if !m.is_nan() && !s.is_nan() {
			histogram[i] = m - s;
		}
	}

	Ok(MACDResult {
		macd: macd_line,
		signal: signal_line,
		histogram,
	})
}
