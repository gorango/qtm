use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

/// Stars — doji-like indecision patterns (morning/evening stars).
/// Returns per-bar scores. Direct candlestick definition.
pub fn stars(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	body_ratio_threshold: Option<f64>,
	gap_threshold: Option<f64>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let body_ratio_threshold = body_ratio_threshold.unwrap_or(0.3);
	let gap_threshold = gap_threshold.unwrap_or(0.001);

	let mut results = vec![0.0; opens.len()];

	if opens.len() < 3 {
		return Ok(results);
	}

	for i in 2..opens.len() {
		let prev_open = opens[i - 2];
		let prev_close = closes[i - 2];
		let star_open = opens[i - 1];
		let star_close = closes[i - 1];
		let confirm_open = opens[i];
		let confirm_close = closes[i];

		let prev_body = (prev_close - prev_open).abs();
		let star_body = (star_close - star_open).abs();
		let confirm_body = (confirm_close - confirm_open).abs();

		if star_body > prev_body * body_ratio_threshold {
			continue;
		}

		let is_morning_star = prev_close < prev_open
			&& star_body < prev_body * body_ratio_threshold
			&& confirm_close > confirm_open
			&& confirm_body > star_body
			&& star_open.min(star_close) > highs[i - 2] * (1.0 - gap_threshold);

		let is_evening_star = prev_close > prev_open
			&& star_body < prev_body * body_ratio_threshold
			&& confirm_close < confirm_open
			&& confirm_body > star_body
			&& star_open.max(star_close) < lows[i - 2] * (1.0 + gap_threshold);

		if is_morning_star {
			results[i] = 1.0;
		} else if is_evening_star {
			results[i] = -1.0;
		}
	}

	Ok(results)
}
