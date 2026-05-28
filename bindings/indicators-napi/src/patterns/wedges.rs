use crate::utils::validation::validate_multiple_arrays;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Wedges
///
/// Detects rising/falling wedge patterns.
#[napi]
pub fn wedges(
	opens: Float64Array,
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	min_points: Option<u32>,
	slope_tolerance: Option<f64>,
) -> Result<Vec<f64>> {
	validate_multiple_arrays(&[&opens, &highs, &lows, &closes])
		.map_err(napi::Error::from_reason)?;

	let highs = highs.as_ref();
	let lows = lows.as_ref();
	let closes = closes.as_ref();
	let min_points = min_points.unwrap_or(4) as usize;
	let slope_tolerance = slope_tolerance.unwrap_or(0.0001);

	let mut results = vec![0.0; highs.len()];

	if highs.len() < 20 {
		return Ok(results);
	}

	let peaks = crate::patterns::helpers::find_peaks_internal(highs, 1);
	let troughs = crate::patterns::helpers::find_troughs_internal(lows, 1);

	if peaks.len() < 2 || troughs.len() < 2 {
		return Ok(results);
	}

	let filtered_peaks: Vec<usize> = peaks
		.iter()
		.copied()
		.filter(|&p| p > (highs.len() as f64 * 0.3) as usize)
		.collect();
	let recent_peaks: Vec<usize> = filtered_peaks
		.iter()
		.rev()
		.take(min_points)
		.copied()
		.collect();

	let filtered_troughs: Vec<usize> = troughs
		.iter()
		.copied()
		.filter(|&t| t > (lows.len() as f64 * 0.3) as usize)
		.collect();
	let recent_troughs: Vec<usize> = filtered_troughs
		.iter()
		.rev()
		.take(min_points)
		.copied()
		.collect();

	if recent_peaks.len() < 2 || recent_troughs.len() < 2 {
		return Ok(results);
	}

	let mut peak_points = Vec::new();
	for &p in &recent_peaks {
		peak_points.push(p as f64);
		peak_points.push(highs[p]);
	}

	let mut trough_points = Vec::new();
	for &t in &recent_troughs {
		trough_points.push(t as f64);
		trough_points.push(lows[t]);
	}

	let high_line = crate::patterns::helpers::linear_regression_internal(&peak_points);
	let low_line = crate::patterns::helpers::linear_regression_internal(&trough_points);

	let wedge_type = if high_line[0] > low_line[0]
		&& high_line[0] > slope_tolerance
		&& low_line[0] > slope_tolerance
	{
		Some("rising")
	} else if high_line[0] < low_line[0]
		&& high_line[0] < -slope_tolerance
		&& low_line[0] < -slope_tolerance
	{
		Some("falling")
	} else {
		None
	};

	let wedge_type = match wedge_type {
		Some(t) => t,
		None => return Ok(results),
	};

	let end_index = *recent_peaks
		.last()
		.unwrap_or(&0)
		.max(recent_troughs.last().unwrap_or(&0));

	for i in (end_index + 1)..highs.len() {
		let close = closes[i];

		let breakout = match wedge_type {
			"rising" => {
				let support = low_line[1] + low_line[0] * i as f64;
				if close < support {
					Some(-1.0)
				} else {
					None
				}
			}
			"falling" => {
				let resistance = high_line[1] + high_line[0] * i as f64;
				if close > resistance {
					Some(1.0)
				} else {
					None
				}
			}
			_ => None,
		};

		if let Some(signal) = breakout {
			results[i] = signal;
			break;
		}
	}

	Ok(results)
}
