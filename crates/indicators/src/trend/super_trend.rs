use crate::internal::true_range::tr_internal;
use crate::trend::rma::rma_internal;
use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
/// SuperTrend result — upper/lower bands and trend direction.
pub struct SuperTrendResult {
	pub super_trend: Vec<f64>,
	pub direction: Vec<i32>,
}

/// SuperTrend — ATR-based trend filter.
///
/// Bands = `(high+low)/2 ± multiplier*ATR`; price closing above/below flips trend.
/// Widely used as trailing stop and trend filter. `NaN` until `period` bars.
///
/// # Errors
/// Returns an error if `period` is 0 or inputs invalid.
pub fn super_trend(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	period: Option<u32>,
	multiplier: Option<f64>,
) -> IndicatorResult<SuperTrendResult> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closes])?;

	let period = period.unwrap_or(14) as usize;
	let multiplier = multiplier.unwrap_or(3.0);

	let len = closes.len();

	if len < period {
		return Ok(SuperTrendResult {
			super_trend: vec![0.0; len],
			direction: vec![0; len],
		});
	}

	let tr_line = tr_internal(highs, lows, closes);
	let atr_line = rma_internal(&tr_line, period);

	let mut super_trend_values = vec![0.0; len];
	let mut direction = vec![0; len];
	let mut final_upper = vec![0.0; len];
	let mut final_lower = vec![0.0; len];

	for i in 0..len {
		if i < period - 1 {
			super_trend_values[i] = 0.0;
			direction[i] = 0;
			continue;
		}

		let hl2 = (highs[i] + lows[i]) / 2.0;
		let basic_upper_band = hl2 + multiplier * atr_line[i];
		let basic_lower_band = hl2 - multiplier * atr_line[i];

		// Ratchet each final band against ITS OWN previous value (canonical
		// supertrend).  Previously both ratcheted against `super_trend_values
		// [i-1]` — a single line — which collapsed the upper band onto the
		// lower band during an uptrend and froze the direction at +1.
		let (fu, fl) = if i == period - 1 {
			(basic_upper_band, basic_lower_band)
		} else {
			let fu = if basic_upper_band < final_upper[i - 1] || closes[i - 1] > final_upper[i - 1]
			{
				basic_upper_band
			} else {
				final_upper[i - 1]
			};

			let fl = if basic_lower_band > final_lower[i - 1] || closes[i - 1] < final_lower[i - 1]
			{
				basic_lower_band
			} else {
				final_lower[i - 1]
			};

			(fu, fl)
		};
		final_upper[i] = fu;
		final_lower[i] = fl;

		// Line = upper band while price was below it (downtrend regime),
		// lower band otherwise; direction flips when close crosses the line.
		let trend_line = if i == period - 1 || closes[i - 1] <= final_upper[i - 1] {
			fu
		} else {
			fl
		};
		super_trend_values[i] = trend_line;
		direction[i] = if closes[i] > trend_line { 1 } else { -1 };
	}

	Ok(SuperTrendResult {
		super_trend: super_trend_values,
		direction,
	})
}
