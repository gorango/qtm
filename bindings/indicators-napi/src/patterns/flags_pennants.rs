use crate::utils::validation::validate_multiple_arrays;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Flags Pennants
///
/// Detects flag and pennant continuation patterns.
#[napi]
pub fn flags_pennants(
	opens: Float64Array,
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	pole_length: Option<u32>,
	consolidation_bars: Option<u32>,
	breakout_threshold: Option<f64>,
) -> Result<Vec<f64>> {
	validate_multiple_arrays(&[&opens, &highs, &lows, &closes])
		.map_err(napi::Error::from_reason)?;

	let highs = highs.as_ref();
	let lows = lows.as_ref();
	let closes = closes.as_ref();
	let pole_length = pole_length.unwrap_or(10) as usize;
	let consolidation_bars = consolidation_bars.unwrap_or(10) as usize;
	let breakout_threshold = breakout_threshold.unwrap_or(0.02);

	let mut results = vec![0.0; highs.len()];

	if highs.len() < pole_length + consolidation_bars + 5 {
		return Ok(results);
	}

	for i in (pole_length + consolidation_bars)..highs.len() {
		let pole_start = i - consolidation_bars - pole_length;
		let pole_end = i - consolidation_bars;
		let consolidation_start = pole_end;
		let consolidation_end = i;

		let pole_high = highs[pole_start..=pole_end]
			.iter()
			.fold(f64::NEG_INFINITY, |a, &b| a.max(b));
		let pole_low = lows[pole_start..=pole_end]
			.iter()
			.fold(f64::INFINITY, |a, &b| a.min(b));
		let pole_range = pole_high - pole_low;

		if pole_range / pole_low < 0.05 {
			continue;
		}

		let direction = if closes[pole_end] > closes[pole_start] {
			"up"
		} else {
			"down"
		};

		let consolidation_high = highs[consolidation_start..=consolidation_end]
			.iter()
			.fold(f64::NEG_INFINITY, |a, &b| a.max(b));
		let consolidation_low = lows[consolidation_start..=consolidation_end]
			.iter()
			.fold(f64::INFINITY, |a, &b| a.min(b));
		let consolidation_range = consolidation_high - consolidation_low;

		if consolidation_range > pole_range * 0.5 {
			continue;
		}

		let consolidation_highs_slice = &highs[consolidation_start..=consolidation_end];
		let consolidation_lows_slice = &lows[consolidation_start..=consolidation_end];

		let mut high_points = Vec::new();
		let mut low_points = Vec::new();

		for (idx, &h) in consolidation_highs_slice.iter().enumerate() {
			high_points.push((consolidation_start + idx) as f64);
			high_points.push(h);
		}

		for (idx, &l) in consolidation_lows_slice.iter().enumerate() {
			low_points.push((consolidation_start + idx) as f64);
			low_points.push(l);
		}

		let high_line = crate::patterns::helpers::linear_regression_internal(&high_points);
		let low_line = crate::patterns::helpers::linear_regression_internal(&low_points);

		let is_pennant = high_line[0].abs() > 0.0001
			&& low_line[0].abs() > 0.0001
			&& ((high_line[0] < 0.0 && low_line[0] > 0.0)
				|| (high_line[0] > 0.0 && low_line[0] < 0.0));

		let is_flag = high_line[0].abs() < 0.0001 && low_line[0].abs() < 0.0001;

		if !is_pennant && !is_flag {
			continue;
		}

		let breakout_level = match direction {
			"up" => consolidation_high,
			_ => consolidation_low,
		};

		let end_breakout = (consolidation_end + 5).min(highs.len());

		for k in (consolidation_end + 1)..end_breakout {
			let close = closes[k];
			if direction == "up" && close > breakout_level * (1.0 + breakout_threshold) {
				results[k] = 1.0;
				break;
			} else if direction == "down" && close < breakout_level * (1.0 - breakout_threshold) {
				results[k] = -1.0;
				break;
			}
		}
	}

	Ok(results)
}
