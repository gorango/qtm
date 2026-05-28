use crate::utils::validation::validate_multiple_arrays;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Bullish Engulfing
///
/// Detects bullish engulfing candlestick pattern.
#[napi]
pub fn bullish_engulfing(
	opens: Float64Array,
	_highs: Float64Array,
	_lows: Float64Array,
	closes: Float64Array,
) -> Result<Vec<f64>> {
	validate_multiple_arrays(&[&opens, &_highs, &_lows, &closes])
		.map_err(napi::Error::from_reason)?;

	let opens = opens.as_ref();
	let closes = closes.as_ref();
	let mut results = vec![0.0; opens.len()];

	if opens.len() < 2 {
		return Ok(results);
	}

	for i in 1..opens.len() {
		let prev_open = opens[i - 1];
		let prev_close = closes[i - 1];
		let curr_open = opens[i];
		let curr_close = closes[i];

		if !prev_open.is_finite()
			|| !prev_close.is_finite()
			|| !curr_open.is_finite()
			|| !curr_close.is_finite()
		{
			continue;
		}

		let is_prev_bearish = prev_close < prev_open;
		let is_curr_bullish = curr_close > curr_open;
		let engulfs = curr_open < prev_close && curr_close > prev_open;

		if is_prev_bearish && is_curr_bullish && engulfs {
			results[i] = 1.0;
		}
	}

	Ok(results)
}

/// Bearish Engulfing
///
/// Detects bearish engulfing candlestick pattern.
#[napi]
pub fn bearish_engulfing(
	opens: Float64Array,
	_highs: Float64Array,
	_lows: Float64Array,
	closes: Float64Array,
) -> Result<Vec<f64>> {
	validate_multiple_arrays(&[&opens, &_highs, &_lows, &closes])
		.map_err(napi::Error::from_reason)?;

	let opens = opens.as_ref();
	let closes = closes.as_ref();
	let mut results = vec![0.0; opens.len()];

	if opens.len() < 2 {
		return Ok(results);
	}

	for i in 1..opens.len() {
		let prev_open = opens[i - 1];
		let prev_close = closes[i - 1];
		let curr_open = opens[i];
		let curr_close = closes[i];

		// Skip if any value is not finite
		if !prev_open.is_finite()
			|| !prev_close.is_finite()
			|| !curr_open.is_finite()
			|| !curr_close.is_finite()
		{
			continue;
		}

		// Previous candle is bullish (close > open)
		let is_prev_bullish = prev_close > prev_open;
		// Current candle is bearish (close < open)
		let is_curr_bearish = curr_close < curr_open;
		// Current candle's body engulfs the previous candle's body
		let engulfs = curr_open > prev_close && curr_close < prev_open;

		if is_prev_bullish && is_curr_bearish && engulfs {
			results[i] = 1.0;
		}
	}

	Ok(results)
}
