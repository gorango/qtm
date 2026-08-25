use crate::validation::{validate_non_empty, validate_period};
use indicators_core::{
	absolute_price_oscillator as apo_core, adx as adx_core, alma as alma_core, aroon as aroon_core,
	balance_of_power as bop_core, camarilla_pivot_points as cpp_core, cci as cci_core,
	chande_forecast_oscillator as cfo_core, classify_market_trend as cmt_core, dema as dema_core,
	fibonacci_pivot_points as fpp_core, hma as hma_core, internal::ema::ema_internal,
	internal::moving_sum::moving_sum_internal, internal::sma::sma_internal, kama as kama_core,
	kaufman_efficiency_ratio as ker_core, linreg as linreg_core, mass_index as mi_core,
	parabolic_sar as psar_core, pivot_points as pp_core, random_index as ri_core,
	rolling_moving_average as rma_core, smoothed_moving_average as smma_core,
	super_trend as st_core, tema as tema_core, tma as tma_core,
	trend::moving_max::moving_max_internal, trend::moving_min::moving_min_internal,
	trend::rma::rma_internal, trend::since::since_internal,
	trend::typical_price::typical_price as typical_price_core, trix as trix_core,
	vortex as vortex_core, vwma as vwma_core, wma as wma_core, ADXConfig, ADXResult, ALMAConfig,
	AroonConfig, AroonResult, Bar, CCIConfig, FibonacciPivotPointsResult, KAMAConfig, KDJResult,
	LinRegConfig, PSARConfig, PSARResult, PivotPointsResult, SuperTrendResult, TrendAnalysis,
	VortexResult,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Absolute Price Oscillator
#[napi]
pub fn absolute_price_oscillator(
	closes: Float64Array,
	fast_period: Option<u32>,
	slow_period: Option<u32>,
) -> Result<Vec<f64>> {
	apo_core(closes.as_ref(), fast_period, slow_period)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Adx
#[napi]
pub fn adx(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<ADXConfig>,
) -> Result<ADXResult> {
	adx_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Alma
#[napi]
pub fn alma(values: Float64Array, config: Option<ALMAConfig>) -> Result<Vec<f64>> {
	alma_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Aroon
#[napi]
pub fn aroon(
	highs: Float64Array,
	lows: Float64Array,
	config: Option<AroonConfig>,
) -> Result<AroonResult> {
	aroon_core(highs.as_ref(), lows.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Balance Of Power
#[napi]
pub fn balance_of_power(
	opens: Float64Array,
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
) -> Result<Vec<f64>> {
	bop_core(
		opens.as_ref(),
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Cci
#[napi]
pub fn cci(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<CCIConfig>,
) -> Result<Vec<f64>> {
	cci_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Chande Forecast Oscillator
#[napi]
pub fn chande_forecast_oscillator(values: Float64Array) -> Result<Vec<f64>> {
	cfo_core(values.as_ref()).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Dema
#[napi]
pub fn dema(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	dema_core(values.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Ema
#[napi]
pub fn ema(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	validate_non_empty(values.as_ref(), "values")?;
	let period = period.unwrap_or(12);
	validate_period(period, "period")?;
	Ok(ema_internal(values.as_ref(), period as usize))
}

/// Hma
#[napi]
pub fn hma(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	hma_core(values.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Kaufman's Adaptive Moving Average
#[napi]
pub fn kama(values: Float64Array, config: Option<KAMAConfig>) -> Result<Vec<f64>> {
	kama_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Kaufman Efficiency Ratio
#[napi]
pub fn kaufman_efficiency_ratio(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	ker_core(values.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Linreg
#[napi]
pub fn linreg(values: Float64Array, config: Option<LinRegConfig>) -> Result<Vec<f64>> {
	linreg_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Mass Index
#[napi]
pub fn mass_index(
	highs: Float64Array,
	lows: Float64Array,
	ema_period: Option<u32>,
	mi_period: Option<u32>,
) -> Result<Vec<f64>> {
	mi_core(highs.as_ref(), lows.as_ref(), ema_period, mi_period)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Moving Max
#[napi]
pub fn moving_max(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	validate_non_empty(values.as_ref(), "values")?;
	let period = period.unwrap_or(4);
	validate_period(period, "period")?;
	Ok(moving_max_internal(values.as_ref(), period as usize))
}

/// Moving Min
#[napi]
pub fn moving_min(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	validate_non_empty(values.as_ref(), "values")?;
	let period = period.unwrap_or(4);
	validate_period(period, "period")?;
	Ok(moving_min_internal(values.as_ref(), period as usize))
}

/// Moving Sum
#[napi]
pub fn moving_sum(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	validate_non_empty(values.as_ref(), "values")?;
	let period = period.unwrap_or(4);
	validate_period(period, "period")?;
	Ok(moving_sum_internal(values.as_ref(), period as usize))
}

/// Parabolic Sar
#[napi]
pub fn parabolic_sar(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<PSARConfig>,
) -> Result<PSARResult> {
	psar_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Pivot Points
#[napi]
pub fn pivot_points(high: f64, low: f64, close: f64) -> PivotPointsResult {
	pp_core(high, low, close)
}

/// Fibonacci Pivot Points
#[napi]
pub fn fibonacci_pivot_points(high: f64, low: f64, close: f64) -> FibonacciPivotPointsResult {
	fpp_core(high, low, close)
}

/// Camarilla Pivot Points
#[napi]
pub fn camarilla_pivot_points(high: f64, low: f64, close: f64) -> PivotPointsResult {
	cpp_core(high, low, close)
}

/// Random Index
#[napi]
pub fn random_index(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	r_period: Option<u32>,
	k_period: Option<u32>,
	d_period: Option<u32>,
) -> Result<KDJResult> {
	ri_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		r_period,
		k_period,
		d_period,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Rma
#[napi]
pub fn rma(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	validate_non_empty(values.as_ref(), "values")?;
	let period = period.unwrap_or(4);
	validate_period(period, "period")?;
	Ok(rma_internal(values.as_ref(), period as usize))
}

/// Rolling Moving Average
#[napi]
pub fn rolling_moving_average(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	rma_core(values.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Since
#[napi]
pub fn since(values: Float64Array) -> Vec<f64> {
	since_internal(values.as_ref())
}

/// Sma
#[napi]
pub fn sma(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	validate_non_empty(values.as_ref(), "values")?;
	let period = period.unwrap_or(2);
	validate_period(period, "period")?;
	Ok(sma_internal(values.as_ref(), period as usize))
}

/// Smoothed Moving Average
#[napi]
pub fn smoothed_moving_average(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	smma_core(values.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Super Trend
#[napi]
pub fn super_trend(
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	period: Option<u32>,
	multiplier: Option<f64>,
) -> Result<SuperTrendResult> {
	st_core(
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
		period,
		multiplier,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Tema
#[napi]
pub fn tema(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	tema_core(values.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Tma
#[napi]
pub fn tma(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	tma_core(values.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Classify Market Trend
#[napi]
pub fn classify_market_trend(
	market_data: Vec<Bar>,
	trailing_period_length: Option<u32>,
) -> TrendAnalysis {
	cmt_core(market_data, trailing_period_length)
}

/// Trix
#[napi]
pub fn trix(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	trix_core(values.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Typical Price
#[napi]
pub fn typical_price(
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
) -> Result<Vec<f64>> {
	typical_price_core(highs.as_ref(), lows.as_ref(), closes.as_ref())
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Vortex
#[napi]
pub fn vortex(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	period: Option<u32>,
) -> Result<VortexResult> {
	vortex_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), period)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Vwma
#[napi]
pub fn vwma(closes: Float64Array, volumes: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	vwma_core(closes.as_ref(), volumes.as_ref(), period)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Wma
#[napi]
pub fn wma(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	wma_core(values.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}
