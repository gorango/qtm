use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

/// Bullish Engulfing — two-candle reversal where a green candle engulfs the prior red.
/// Returns per-bar boolean/score. Direct candlestick definition.
pub fn bullish_engulfing(
	opens: &[f64],
	_highs: &[f64],
	_lows: &[f64],
	closes: &[f64],
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, _highs, _lows, closes])?;

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

/// Bearish Engulfing — inverse of bullish: red engulfs prior green.
pub fn bearish_engulfing(
	opens: &[f64],
	_highs: &[f64],
	_lows: &[f64],
	closes: &[f64],
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, _highs, _lows, closes])?;

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

		let is_prev_bullish = prev_close > prev_open;
		let is_curr_bearish = curr_close < curr_open;
		let engulfs = curr_open > prev_close && curr_close < prev_open;

		if is_prev_bullish && is_curr_bearish && engulfs {
			results[i] = 1.0;
		}
	}

	Ok(results)
}
