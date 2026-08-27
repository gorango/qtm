use crate::internal::true_range::tr_internal;
use crate::trend::rma::rma_internal;
use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct ATRResult {
	/// True range per bar.
	pub tr_line: Vec<f64>,
	/// ATR — RMA of true range over `period` bars.
	pub atr_line: Vec<f64>,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ATRConfig {
	/// ATR period (default 14). Valid 2..=100.
	pub period: Option<u32>,
}

impl Default for ATRConfig {
	fn default() -> Self {
		Self { period: Some(14) }
	}
}

/// Average True Range (ATR) — Wilder's volatility measure.
///
/// TR = max(high-low, |high-prev_close|, |low-prev_close|); ATR = RMA(TR, period).
/// Period defaults to 14. `NaN` until `period` bars. Direct Wilder definition.
///
/// # Errors
/// Returns an error if `period` is 0 or inputs mismatched.
pub fn atr(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	config: Option<ATRConfig>,
) -> IndicatorResult<ATRResult> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closings])?;

	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14) as usize;
	crate::utils::validation::validate_period(period)?;

	let tr_line = tr_internal(highs, lows, closings);
	let atr_line = rma_internal(&tr_line, period);

	Ok(ATRResult { tr_line, atr_line })
}

/// Alias for `atr` — Average True Range (full name).
pub fn average_true_range(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	config: Option<ATRConfig>,
) -> IndicatorResult<ATRResult> {
	atr(highs, lows, closings, config)
}
