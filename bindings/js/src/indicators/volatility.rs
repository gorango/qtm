use indicators_core::{
	annualized_volatility as av_core, max_drawdown as md_core, ulcer_index as ui_core,
	volatility::acceleration_bands::{ab as ab_core, acceleration_bands as ab_alias, ABResult},
	volatility::average_true_range::{
		atr as atr_core, average_true_range as atr_alias, ATRConfig, ATRResult,
	},
	volatility::bollinger_bands::{
		bb as bb_core, bollinger_bands as bbands_core, BBConfig, BBResult,
	},
	volatility::bollinger_bands_width::{
		bbw as bbw_core, bollinger_bands_width as bbw_alias, BBWResult,
	},
	volatility::chandelier_exit::{ce as ce_core, chandelier_exit as ce_alias, CEResult},
	volatility::dev::{
		dev as dev_core, mean_absolute_deviation as mad_core, MeanAbsoluteDeviationConfig,
	},
	volatility::donchian_channel::{dc as dc_core, donchian_channel as dc_alias, DCResult},
	volatility::keltner_channel::{kc as kc_core, keltner_channel as kc_alias, KCResult},
	volatility::moving_standard_deviation::{
		moving_standard_deviation as msd_core, mstd as mstd_core, MSTDConfig,
	},
	volatility::projection_oscillator::{
		po as po_core, projection_oscillator as po_alias, POResult,
	},
	volatility::true_range::{tr as tr_core, true_range as tr_alias, TrueRangeResult},
	volatility::ttm_squeeze::{
		ttm_squeeze as tts_core, TTMSqueezeResult,
	},
	volatility::variance::{rolling_variance as rv_core, variance as var_core, VarianceConfig},
	volatility::z_score::{z_score as zscore_alias, zs as zs_core, ZScoreConfig},
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Ab
#[napi]
pub fn ab(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	period: Option<u32>,
	multiplier: Option<f64>,
) -> Result<ABResult> {
	ab_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		period,
		multiplier,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Acceleration Bands (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn accelerationBands(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	period: Option<u32>,
	multiplier: Option<f64>,
) -> Result<ABResult> {
	ab_alias(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		period,
		multiplier,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Annualized Volatility
#[napi]
pub fn annualized_volatility(returns: Float64Array, periods: Option<u32>) -> Result<Vec<f64>> {
	av_core(returns.as_ref(), periods).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Atr
#[napi]
pub fn atr(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<ATRConfig>,
) -> Result<ATRResult> {
	atr_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Average True Range (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn averageTrueRange(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<ATRConfig>,
) -> Result<ATRResult> {
	atr_alias(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Bollinger Bands
#[napi]
pub fn bb(closings: Float64Array, config: Option<BBConfig>) -> Result<BBResult> {
	bb_core(closings.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Bollinger Bands (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn bollingerBands(closings: Float64Array, config: Option<BBConfig>) -> Result<BBResult> {
	bbands_core(closings.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Bbw
#[napi]
pub fn bbw(bb: BBResult, period: Option<u32>) -> Result<BBWResult> {
	bbw_core(bb, period).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Bollinger Bands Width (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn bollingerBandsWidth(bb: BBResult, period: Option<u32>) -> Result<BBWResult> {
	bbw_alias(bb, period).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Ce
#[napi]
pub fn ce(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	period: Option<u32>,
) -> Result<CEResult> {
	ce_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), period)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Chandelier Exit (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn chandelierExit(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	period: Option<u32>,
) -> Result<CEResult> {
	ce_alias(highs.as_ref(), lows.as_ref(), closings.as_ref(), period)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Dev
#[napi]
pub fn dev(values: Float64Array, config: Option<MeanAbsoluteDeviationConfig>) -> Result<Vec<f64>> {
	dev_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Mean Absolute Deviation (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn meanAbsoluteDeviation(
	values: Float64Array,
	config: Option<MeanAbsoluteDeviationConfig>,
) -> Result<Vec<f64>> {
	mad_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Dc
#[napi]
pub fn dc(closings: Float64Array, period: Option<u32>) -> Result<DCResult> {
	dc_core(closings.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Donchian Channel (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn donchianChannel(closings: Float64Array, period: Option<u32>) -> Result<DCResult> {
	dc_alias(closings.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Kc
#[napi]
pub fn kc(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	period: Option<u32>,
) -> Result<KCResult> {
	kc_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), period)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Keltner Channel (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn keltnerChannel(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	period: Option<u32>,
) -> Result<KCResult> {
	kc_alias(highs.as_ref(), lows.as_ref(), closings.as_ref(), period)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Max Drawdown
#[napi]
pub fn max_drawdown(values: Float64Array, period: u32) -> Result<Vec<f64>> {
	md_core(values.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Mstd
#[napi]
pub fn mstd(values: Float64Array, config: Option<MSTDConfig>) -> Result<Vec<f64>> {
	mstd_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Moving Standard Deviation (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn movingStandardDeviation(
	values: Float64Array,
	config: Option<MSTDConfig>,
) -> Result<Vec<f64>> {
	msd_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Po
#[napi]
pub fn po(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	period: Option<u32>,
	smooth: Option<u32>,
) -> Result<POResult> {
	po_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		period,
		smooth,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Projection Oscillator (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn projectionOscillator(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	period: Option<u32>,
	smooth: Option<u32>,
) -> Result<POResult> {
	po_alias(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		period,
		smooth,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Tr
#[napi]
pub fn tr(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
) -> Result<TrueRangeResult> {
	tr_core(highs.as_ref(), lows.as_ref(), closings.as_ref())
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// True Range (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn trueRange(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
) -> Result<TrueRangeResult> {
	tr_alias(highs.as_ref(), lows.as_ref(), closings.as_ref())
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Ttm Squeeze
#[napi]
pub fn ttm_squeeze(
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	bb_period: Option<u32>,
	bb_std_dev: Option<f64>,
	kc_period: Option<u32>,
) -> Result<TTMSqueezeResult> {
	tts_core(
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
		bb_period,
		bb_std_dev,
		kc_period,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Ulcer Index
#[napi]
pub fn ulcer_index(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	ui_core(values.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Variance
#[napi]
pub fn variance(values: Float64Array, config: Option<VarianceConfig>) -> Result<Vec<f64>> {
	var_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Rolling Variance (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn rollingVariance(values: Float64Array, config: Option<VarianceConfig>) -> Result<Vec<f64>> {
	rv_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Zs
#[napi]
pub fn zs(values: Float64Array, config: Option<ZScoreConfig>) -> Result<Vec<f64>> {
	zs_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Z Score (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn zScore(values: Float64Array, config: Option<ZScoreConfig>) -> Result<Vec<f64>> {
	zscore_alias(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}
