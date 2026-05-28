use crate::utils::validation::validate_multiple_arrays;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Elliott Wave
///
/// Detects Elliott Wave 5-wave impulse patterns.
#[napi]
#[allow(clippy::too_many_arguments)]
pub fn elliott_wave(
	opens: Float64Array,
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	wave2_retracement: Option<f64>,
	wave4_retracement: Option<f64>,
	wave3_min_extension: Option<f64>,
	min_wave_separation: Option<u32>,
	lookaround: Option<u32>,
	retracement_tolerance: Option<f64>,
) -> Result<Vec<f64>> {
	validate_multiple_arrays(&[&opens, &highs, &lows, &closes])
		.map_err(|e| napi::Error::from_reason(e.to_string()))?;

	let highs = highs.as_ref();
	let lows = lows.as_ref();
	let closes = closes.as_ref();
	let wave2_retracement = wave2_retracement.unwrap_or(0.618);
	let wave4_retracement = wave4_retracement.unwrap_or(0.382);
	let wave3_min_extension = wave3_min_extension.unwrap_or(1.618);
	let min_wave_separation = min_wave_separation.unwrap_or(5) as usize;
	let lookaround = lookaround.unwrap_or(2) as usize;
	let retracement_tolerance = retracement_tolerance.unwrap_or(0.1);

	let mut results = vec![0.0; highs.len()];

	if highs.len() < 20 {
		return Ok(results);
	}

	let peaks = crate::patterns::helpers::find_peaks_internal(highs, lookaround);
	let troughs = crate::patterns::helpers::find_troughs_internal(lows, lookaround);

	if peaks.len() < 5 && troughs.len() < 5 {
		return Ok(results);
	}

	check_bullish_impulse_wave(
		&peaks,
		&troughs,
		highs,
		lows,
		closes,
		&mut results[..],
		min_wave_separation,
		wave2_retracement,
		wave4_retracement,
		wave3_min_extension,
		retracement_tolerance,
	);

	check_bearish_impulse_wave(
		&peaks,
		&troughs,
		highs,
		lows,
		closes,
		&mut results[..],
		min_wave_separation,
		wave2_retracement,
		wave4_retracement,
		wave3_min_extension,
		retracement_tolerance,
	);

	check_bullish_corrective_wave(
		&peaks,
		&troughs,
		highs,
		lows,
		closes,
		&mut results[..],
		min_wave_separation,
		retracement_tolerance,
	);

	check_bearish_corrective_wave(
		&peaks,
		&troughs,
		highs,
		lows,
		closes,
		&mut results[..],
		min_wave_separation,
		retracement_tolerance,
	);

	Ok(results)
}

#[allow(clippy::too_many_arguments)]
fn check_bullish_impulse_wave(
	peaks: &[usize],
	troughs: &[usize],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	results: &mut [f64],
	min_wave_separation: usize,
	wave2_retracement: f64,
	wave4_retracement: f64,
	wave3_min_extension: f64,
	retracement_tolerance: f64,
) {
	for i in 0..troughs.len().saturating_sub(5) {
		let w1_low = troughs[i];
		let w1_high = match peaks.iter().find(|&&p| p > w1_low && p < troughs[i + 1]) {
			Some(&p) => p,
			None => continue,
		};

		let w2_low = troughs[i + 1];
		let w2_high = match peaks.iter().find(|&&p| p > w2_low && p < troughs[i + 2]) {
			Some(&p) => p,
			None => continue,
		};

		let w3_low = troughs[i + 2];
		let w3_high = match peaks.iter().find(|&&p| p > w3_low && p < troughs[i + 3]) {
			Some(&p) => p,
			None => continue,
		};

		let w4_low = troughs[i + 3];
		let w4_high = match peaks.iter().find(|&&p| p > w4_low && p < troughs[i + 4]) {
			Some(&p) => p,
			None => continue,
		};

		let w5_low = troughs[i + 4];

		if w2_high - w1_low < min_wave_separation
			|| w3_low - w2_high < min_wave_separation
			|| w4_high - w3_low < min_wave_separation
			|| w5_low - w4_high < min_wave_separation
		{
			continue;
		}

		let w1_price = lows[w1_low];
		let w1_top_price = highs[w1_high];
		let w2_price = lows[w2_low];
		let w2_top_price = highs[w2_high];
		let w3_price = lows[w3_low];
		let w3_top_price = highs[w3_high];
		let w4_price = lows[w4_low];
		let w4_top_price = highs[w4_high];

		if w2_price >= w1_price || w4_price >= w3_price {
			continue;
		}

		let w1_range = w1_top_price - w1_price;
		let w2_retracement = (w1_top_price - w2_price) / w1_range;
		let w3_range = w3_top_price - w3_price;
		let w4_retracement = (w3_top_price - w4_price) / w3_range;
		let w3_extension = w3_range / w1_range;

		let w2_retracement_expected = wave2_retracement * (1.0 - retracement_tolerance);
		let w2_retracement_max = wave2_retracement * (1.0 + retracement_tolerance);
		let w4_retracement_expected = wave4_retracement * (1.0 - retracement_tolerance);
		let w4_retracement_max = wave4_retracement * (1.0 + retracement_tolerance);

		if w2_retracement < w2_retracement_expected
			|| w2_retracement > w2_retracement_max
			|| w4_retracement < w4_retracement_expected
			|| w4_retracement > w4_retracement_max
		{
			continue;
		}

		if w3_extension < wave3_min_extension {
			continue;
		}

		if w2_top_price >= w1_top_price || w4_top_price >= w3_top_price {
			continue;
		}

		let breakout_level = w1_top_price;

		for j in (w5_low + 1)..closes.len() {
			if closes[j] > breakout_level {
				results[j] = 1.0;
				break;
			}
		}
	}
}

#[allow(clippy::too_many_arguments)]
fn check_bearish_impulse_wave(
	peaks: &[usize],
	troughs: &[usize],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	results: &mut [f64],
	min_wave_separation: usize,
	wave2_retracement: f64,
	wave4_retracement: f64,
	wave3_min_extension: f64,
	retracement_tolerance: f64,
) {
	for i in 0..peaks.len().saturating_sub(5) {
		let w1_high = peaks[i];
		let w1_low = match troughs.iter().find(|&&t| t > w1_high && t < peaks[i + 1]) {
			Some(&t) => t,
			None => continue,
		};

		let w2_high = peaks[i + 1];
		let w2_low = match troughs.iter().find(|&&t| t > w2_high && t < peaks[i + 2]) {
			Some(&t) => t,
			None => continue,
		};

		let w3_high = peaks[i + 2];
		let w3_low = match troughs.iter().find(|&&t| t > w3_high && t < peaks[i + 3]) {
			Some(&t) => t,
			None => continue,
		};

		let w4_high = peaks[i + 3];
		let w4_low = match troughs.iter().find(|&&t| t > w4_high && t < peaks[i + 4]) {
			Some(&t) => t,
			None => continue,
		};

		let w5_high = peaks[i + 4];

		if w2_high - w1_low < min_wave_separation
			|| w3_low - w2_high < min_wave_separation
			|| w4_high - w3_low < min_wave_separation
			|| w5_high - w4_low < min_wave_separation
		{
			continue;
		}

		let w1_price = highs[w1_high];
		let w1_bottom_price = lows[w1_low];
		let w2_price = highs[w2_high];
		let w2_bottom_price = lows[w2_low];
		let w3_price = highs[w3_high];
		let w3_bottom_price = lows[w3_low];
		let w4_price = highs[w4_high];
		let w4_bottom_price = lows[w4_low];

		if w2_price <= w1_price || w4_price <= w3_price {
			continue;
		}

		let w1_range = w1_price - w1_bottom_price;
		let w2_retracement = (w2_price - w1_bottom_price) / w1_range;
		let w3_range = w3_price - w3_bottom_price;
		let w4_retracement = (w4_price - w3_bottom_price) / w3_range;
		let w3_extension = w3_range / w1_range;

		let w2_retracement_expected = wave2_retracement * (1.0 - retracement_tolerance);
		let w2_retracement_max = wave2_retracement * (1.0 + retracement_tolerance);
		let w4_retracement_expected = wave4_retracement * (1.0 - retracement_tolerance);
		let w4_retracement_max = wave4_retracement * (1.0 + retracement_tolerance);

		if w2_retracement < w2_retracement_expected
			|| w2_retracement > w2_retracement_max
			|| w4_retracement < w4_retracement_expected
			|| w4_retracement > w4_retracement_max
		{
			continue;
		}

		if w3_extension < wave3_min_extension {
			continue;
		}

		if w2_bottom_price <= w1_bottom_price || w4_bottom_price <= w3_bottom_price {
			continue;
		}

		let breakdown_level = w1_bottom_price;

		for j in (w5_high + 1)..closes.len() {
			if closes[j] < breakdown_level {
				results[j] = -1.0;
				break;
			}
		}
	}
}

#[allow(clippy::too_many_arguments)]
fn check_bullish_corrective_wave(
	peaks: &[usize],
	troughs: &[usize],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	results: &mut [f64],
	min_wave_separation: usize,
	retracement_tolerance: f64,
) {
	for i in 0..peaks.len().saturating_sub(3) {
		let wave_a_high = peaks[i];
		let wave_a_low = match troughs.iter().find(|&&t| t < wave_a_high) {
			Some(&t) => t,
			None => continue,
		};

		let wave_b_high = match peaks.iter().find(|&&p| p > wave_a_high && p < peaks[i + 1]) {
			Some(&p) => p,
			None => continue,
		};

		let wave_c_low = match troughs
			.iter()
			.find(|&&t| t > wave_b_high && t < peaks[i + 1])
		{
			Some(&t) => t,
			None => continue,
		};

		if wave_b_high - wave_a_high < min_wave_separation
			|| wave_c_low - wave_b_high < min_wave_separation
		{
			continue;
		}

		let a_price = highs[wave_a_high];
		let b_price = highs[wave_b_high];
		let c_price = lows[wave_c_low];

		if b_price <= a_price {
			continue;
		}

		let a_range = a_price - lows[wave_a_low];
		let c_range = b_price - c_price;
		let c_retracement = c_range / a_range;

		if c_retracement < 0.618 * (1.0 - retracement_tolerance)
			|| c_retracement > 1.0 * (1.0 + retracement_tolerance)
		{
			continue;
		}

		let breakout_level = b_price;

		for j in (wave_c_low + 1)..closes.len() {
			if closes[j] > breakout_level {
				results[j] = 2.0;
				break;
			}
		}
	}
}

#[allow(clippy::too_many_arguments)]
fn check_bearish_corrective_wave(
	peaks: &[usize],
	troughs: &[usize],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	results: &mut [f64],
	min_wave_separation: usize,
	retracement_tolerance: f64,
) {
	for i in 0..troughs.len().saturating_sub(3) {
		let wave_a_low = troughs[i];
		let wave_a_high = match peaks.iter().find(|&&p| p > wave_a_low) {
			Some(&p) => p,
			None => continue,
		};

		let wave_b_low = match troughs
			.iter()
			.find(|&&t| t < wave_a_low && t > troughs[i + 1])
		{
			Some(&t) => t,
			None => continue,
		};

		let wave_c_high = match peaks.iter().find(|&&p| p > wave_a_high && p < peaks[i + 1]) {
			Some(&p) => p,
			None => continue,
		};

		if wave_a_high - wave_a_low < min_wave_separation
			|| wave_c_high - wave_b_low < min_wave_separation
		{
			continue;
		}

		let a_price = lows[wave_a_low];
		let b_price = lows[wave_b_low];
		let c_price = highs[wave_c_high];

		if b_price >= a_price {
			continue;
		}

		let a_range = highs[wave_a_high] - a_price;
		let c_range = c_price - b_price;
		let c_retracement = c_range / a_range;

		if c_retracement < 0.618 * (1.0 - retracement_tolerance)
			|| c_retracement > 1.0 * (1.0 + retracement_tolerance)
		{
			continue;
		}

		let breakdown_level = b_price;

		for j in (wave_c_high + 1)..closes.len() {
			if closes[j] < breakdown_level {
				results[j] = -2.0;
				break;
			}
		}
	}
}
