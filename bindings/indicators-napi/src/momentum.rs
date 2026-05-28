use indicators_core::{
	awesome_oscillator as ao_core, chaikin_oscillator as co_core, cmo as cmo_core,
	ichimoku as ichimoku_core, ichimoku_cloud as ic_core, kst as kst_core, larsson as larsson_core,
	macd as macd_core, momentum_index as mi_core, percentage_price_oscillator as ppo_core,
	percentage_volume_oscillator as pvo_core, price_rate_of_change as proc_core, pvo as pvo_alias,
	qstick as qstick_core, rsi as rsi_core, stochastic_oscillator as stoch_core,
	ultimate_oscillator as uo_core, uo as uo_alias, williams_r as wr_core, AwesomeOscillatorConfig,
	ChaikinOscillatorConfig, ChaikinOscillatorResult, IchimokuCloudConfig, IchimokuCloudResult,
	KSTConfig, KSTResult, LarssonResult, MACDConfig, MACDResult, MomentumIndexConfig,
	PercentagePriceOscillatorConfig, PercentagePriceOscillatorResult,
	PercentageVolumeOscillatorConfig, PercentageVolumeOscillatorResult, PriceRateOfChangeConfig,
	QstickConfig, RSIConfig, StochConfig, StochResult, UltimateOscillatorConfig, WilliamsRConfig,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Awesome Oscillator
#[napi]
pub fn awesome_oscillator(
	highs: Float64Array,
	lows: Float64Array,
	config: Option<AwesomeOscillatorConfig>,
) -> Result<Vec<f64>> {
	ao_core(highs.as_ref(), lows.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Chaikin Oscillator
#[napi]
pub fn chaikin_oscillator(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	config: Option<ChaikinOscillatorConfig>,
) -> ChaikinOscillatorResult {
	co_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		config,
	)
}

/// Cmo
#[napi]
pub fn cmo(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	config: Option<ChaikinOscillatorConfig>,
) -> ChaikinOscillatorResult {
	cmo_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		config,
	)
}

/// Ichimoku Cloud
#[napi]
pub fn ichimoku_cloud(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<IchimokuCloudConfig>,
) -> IchimokuCloudResult {
	ic_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
}

/// Ichimoku
#[napi]
pub fn ichimoku(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<IchimokuCloudConfig>,
) -> IchimokuCloudResult {
	ichimoku_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
}

/// Kst
#[napi]
pub fn kst(prices: Float64Array, config: Option<KSTConfig>) -> KSTResult {
	kst_core(prices.as_ref(), config)
}

/// Larsson
#[napi]
pub fn larsson(highs: Float64Array, lows: Float64Array) -> LarssonResult {
	larsson_core(highs.as_ref(), lows.as_ref())
}

/// Macd
#[napi]
pub fn macd(closes: Float64Array, config: Option<MACDConfig>) -> Result<MACDResult> {
	macd_core(closes.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Momentum Index
#[napi]
pub fn momentum_index(prices: Float64Array, config: Option<MomentumIndexConfig>) -> Vec<f64> {
	mi_core(prices.as_ref(), config)
}

/// Percentage Price Oscillator
#[napi]
pub fn percentage_price_oscillator(
	prices: Float64Array,
	config: Option<PercentagePriceOscillatorConfig>,
) -> Result<PercentagePriceOscillatorResult> {
	ppo_core(prices.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Percentage Volume Oscillator
#[napi]
pub fn percentage_volume_oscillator(
	volumes: Float64Array,
	config: Option<PercentageVolumeOscillatorConfig>,
) -> PercentageVolumeOscillatorResult {
	pvo_core(volumes.as_ref(), config)
}

/// Pvo
#[napi]
pub fn pvo(
	volumes: Float64Array,
	config: Option<PercentageVolumeOscillatorConfig>,
) -> PercentageVolumeOscillatorResult {
	pvo_alias(volumes.as_ref(), config)
}

/// Price Rate Of Change
#[napi]
pub fn price_rate_of_change(
	values: Float64Array,
	config: Option<PriceRateOfChangeConfig>,
) -> Result<Vec<f64>> {
	proc_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Qstick
#[napi]
pub fn qstick(opens: Float64Array, closes: Float64Array, config: Option<QstickConfig>) -> Vec<f64> {
	qstick_core(opens.as_ref(), closes.as_ref(), config)
}

/// Rsi
#[napi]
pub fn rsi(closings: Float64Array, config: Option<RSIConfig>) -> Vec<f64> {
	rsi_core(closings.as_ref(), config)
}

/// Stochastic Oscillator
#[napi]
pub fn stochastic_oscillator(
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	config: Option<StochConfig>,
) -> StochResult {
	stoch_core(highs.as_ref(), lows.as_ref(), closes.as_ref(), config)
}

/// Ultimate Oscillator
#[napi]
pub fn ultimate_oscillator(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<UltimateOscillatorConfig>,
) -> Vec<f64> {
	uo_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
}

/// Uo
#[napi]
pub fn uo(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<UltimateOscillatorConfig>,
) -> Vec<f64> {
	uo_alias(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
}

/// Williams %R
#[napi]
pub fn williams_r(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<WilliamsRConfig>,
) -> Result<Vec<f64>> {
	wr_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}
