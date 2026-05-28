use crate::utils::validation::validate_multiple_arrays;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Stars
///
/// Detects evening/morning star patterns.
#[napi]
pub fn stars(
	opens: Float64Array,
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	body_ratio_threshold: Option<f64>,
	gap_threshold: Option<f64>,
) -> Result<Vec<f64>> {
	validate_multiple_arrays(&[&opens, &highs, &lows, &closes])
		.map_err(napi::Error::from_reason)?;

	let opens = opens.as_ref();
	let highs = highs.as_ref();
	let lows = lows.as_ref();
	let closes = closes.as_ref();
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
