use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn cup_and_handle(
	opens: Float64Array,
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	cup_depth: Option<f64>,
	handle_retracement: Option<f64>,
	min_duration: Option<u32>,
) -> Result<Vec<f64>> {
	indicators_core::cup_and_handle(
		opens.as_ref(),
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
		cup_depth,
		handle_retracement,
		min_duration,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn double_bottom(
	opens: Float64Array,
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	tolerance: Option<f64>,
	min_separation: Option<u32>,
	lookaround: Option<u32>,
) -> Result<Vec<f64>> {
	indicators_core::double_bottom(
		opens.as_ref(),
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
		tolerance,
		min_separation,
		lookaround,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn double_top(
	opens: Float64Array,
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	tolerance: Option<f64>,
	min_separation: Option<u32>,
	lookaround: Option<u32>,
) -> Result<Vec<f64>> {
	indicators_core::double_top(
		opens.as_ref(),
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
		tolerance,
		min_separation,
		lookaround,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

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
	indicators_core::elliott_wave(
		opens.as_ref(),
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
		wave2_retracement,
		wave4_retracement,
		wave3_min_extension,
		min_wave_separation,
		lookaround,
		retracement_tolerance,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn bullish_engulfing(
	opens: Float64Array,
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
) -> Result<Vec<f64>> {
	indicators_core::bullish_engulfing(
		opens.as_ref(),
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn bearish_engulfing(
	opens: Float64Array,
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
) -> Result<Vec<f64>> {
	indicators_core::bearish_engulfing(
		opens.as_ref(),
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

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
	indicators_core::flags_pennants(
		opens.as_ref(),
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
		pole_length,
		consolidation_bars,
		breakout_threshold,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn head_and_shoulders(
	opens: Float64Array,
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	min_distance: Option<u32>,
	tolerance: Option<f64>,
	deviation: Option<f64>,
) -> Result<Vec<f64>> {
	indicators_core::head_and_shoulders(
		opens.as_ref(),
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
		min_distance,
		tolerance,
		deviation,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn find_peaks(values: Float64Array, lookaround: u32) -> Vec<u32> {
	indicators_core::find_peaks(values.as_ref(), lookaround)
}

#[napi]
pub fn find_troughs(values: Float64Array, lookaround: u32) -> Vec<u32> {
	indicators_core::find_troughs(values.as_ref(), lookaround)
}

#[napi]
pub fn linear_regression(points: Vec<f64>) -> Vec<f64> {
	indicators_core::linear_regression(points)
}

#[napi]
pub fn zig_zag_filter(values: Float64Array, deviation: f64) -> Vec<f64> {
	indicators_core::zig_zag_filter(values.as_ref(), deviation)
}

#[napi]
pub fn stars(
	opens: Float64Array,
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	body_ratio_threshold: Option<f64>,
	gap_threshold: Option<f64>,
) -> Result<Vec<f64>> {
	indicators_core::stars(
		opens.as_ref(),
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
		body_ratio_threshold,
		gap_threshold,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn triangles(
	opens: Float64Array,
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	min_points: Option<u32>,
	tolerance: Option<f64>,
	convergence_tolerance: Option<f64>,
) -> Result<Vec<f64>> {
	indicators_core::triangles(
		opens.as_ref(),
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
		min_points,
		tolerance,
		convergence_tolerance,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn wedges(
	opens: Float64Array,
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	min_points: Option<u32>,
	slope_tolerance: Option<f64>,
	lookback: Option<u32>,
) -> Result<Vec<f64>> {
	indicators_core::wedges(
		opens.as_ref(),
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
		min_points,
		slope_tolerance,
		lookback,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn break_of_structure(
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	lookaround: Option<u32>,
	mode: Option<String>,
	trendline_points: Option<u32>,
) -> Result<Vec<f64>> {
	indicators_core::break_of_structure(
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
		lookaround,
		mode,
		trendline_points,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn power_of_three(
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	accumulation_period: Option<u32>,
	accumulation_threshold: Option<f64>,
	manipulation_threshold: Option<f64>,
	manipulation_bars: Option<u32>,
) -> Result<Vec<f64>> {
	indicators_core::power_of_three(
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
		accumulation_period,
		accumulation_threshold,
		manipulation_threshold,
		manipulation_bars,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}
