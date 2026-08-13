use crate::validation::{validate_arrays, validate_non_empty, validate_period};
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
) -> Result<ChaikinOscillatorResult> {
	validate_arrays(
		&[
			highs.as_ref(),
			lows.as_ref(),
			closings.as_ref(),
			volumes.as_ref(),
		],
		&["highs", "lows", "closings", "volumes"],
	)?;
	let config_obj = config.clone().unwrap_or(ChaikinOscillatorConfig {
		fast_period: None,
		slow_period: None,
	});
	validate_period(config_obj.fast_period.unwrap_or(3), "fast_period")?;
	validate_period(config_obj.slow_period.unwrap_or(10), "slow_period")?;
	Ok(co_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		config,
	))
}

/// Cmo
#[napi]
pub fn cmo(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	config: Option<ChaikinOscillatorConfig>,
) -> Result<ChaikinOscillatorResult> {
	validate_arrays(
		&[
			highs.as_ref(),
			lows.as_ref(),
			closings.as_ref(),
			volumes.as_ref(),
		],
		&["highs", "lows", "closings", "volumes"],
	)?;
	let config_obj = config.clone().unwrap_or(ChaikinOscillatorConfig {
		fast_period: None,
		slow_period: None,
	});
	validate_period(config_obj.fast_period.unwrap_or(3), "fast_period")?;
	validate_period(config_obj.slow_period.unwrap_or(10), "slow_period")?;
	Ok(cmo_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		config,
	))
}

/// Ichimoku Cloud
#[napi]
pub fn ichimoku_cloud(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<IchimokuCloudConfig>,
) -> Result<IchimokuCloudResult> {
	validate_arrays(
		&[highs.as_ref(), lows.as_ref(), closings.as_ref()],
		&["highs", "lows", "closings"],
	)?;
	let config_obj = config.clone().unwrap_or(IchimokuCloudConfig {
		short: None,
		medium: None,
		long: None,
		close: None,
	});
	validate_period(config_obj.short.unwrap_or(9), "short")?;
	validate_period(config_obj.medium.unwrap_or(26), "medium")?;
	validate_period(config_obj.long.unwrap_or(52), "long")?;
	validate_period(config_obj.close.unwrap_or(26), "close")?;
	Ok(ic_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		config,
	))
}

/// Ichimoku
#[napi]
pub fn ichimoku(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<IchimokuCloudConfig>,
) -> Result<IchimokuCloudResult> {
	validate_arrays(
		&[highs.as_ref(), lows.as_ref(), closings.as_ref()],
		&["highs", "lows", "closings"],
	)?;
	let config_obj = config.clone().unwrap_or(IchimokuCloudConfig {
		short: None,
		medium: None,
		long: None,
		close: None,
	});
	validate_period(config_obj.short.unwrap_or(9), "short")?;
	validate_period(config_obj.medium.unwrap_or(26), "medium")?;
	validate_period(config_obj.long.unwrap_or(52), "long")?;
	validate_period(config_obj.close.unwrap_or(26), "close")?;
	Ok(ichimoku_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		config,
	))
}

/// Kst
#[napi]
pub fn kst(prices: Float64Array, config: Option<KSTConfig>) -> Result<KSTResult> {
	validate_non_empty(prices.as_ref(), "prices")?;
	let config_obj = config.clone().unwrap_or(KSTConfig {
		roc1_period: None,
		roc2_period: None,
		roc3_period: None,
		roc4_period: None,
		sma1_period: None,
		sma2_period: None,
		sma3_period: None,
		sma4_period: None,
		signal_period: None,
	});
	for (period, name) in [
		(config_obj.roc1_period.unwrap_or(10), "roc1_period"),
		(config_obj.roc2_period.unwrap_or(15), "roc2_period"),
		(config_obj.roc3_period.unwrap_or(20), "roc3_period"),
		(config_obj.roc4_period.unwrap_or(30), "roc4_period"),
		(config_obj.sma1_period.unwrap_or(10), "sma1_period"),
		(config_obj.sma2_period.unwrap_or(10), "sma2_period"),
		(config_obj.sma3_period.unwrap_or(10), "sma3_period"),
		(config_obj.sma4_period.unwrap_or(15), "sma4_period"),
		(config_obj.signal_period.unwrap_or(9), "signal_period"),
	] {
		validate_period(period, name)?;
	}
	Ok(kst_core(prices.as_ref(), config))
}

/// Larsson
#[napi]
pub fn larsson(highs: Float64Array, lows: Float64Array) -> Result<LarssonResult> {
	validate_arrays(&[highs.as_ref(), lows.as_ref()], &["highs", "lows"])?;
	Ok(larsson_core(highs.as_ref(), lows.as_ref()))
}

/// Macd
#[napi]
pub fn macd(closes: Float64Array, config: Option<MACDConfig>) -> Result<MACDResult> {
	macd_core(closes.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Momentum Index
#[napi]
pub fn momentum_index(
	prices: Float64Array,
	config: Option<MomentumIndexConfig>,
) -> Result<Vec<f64>> {
	validate_non_empty(prices.as_ref(), "prices")?;
	let period = config
		.clone()
		.unwrap_or(MomentumIndexConfig { period: None })
		.period
		.unwrap_or(14);
	validate_period(period, "period")?;
	Ok(mi_core(prices.as_ref(), config))
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
) -> Result<PercentageVolumeOscillatorResult> {
	validate_non_empty(volumes.as_ref(), "volumes")?;
	let config_obj = config.clone().unwrap_or(PercentageVolumeOscillatorConfig {
		fast_period: None,
		slow_period: None,
		signal_period: None,
	});
	validate_period(config_obj.fast_period.unwrap_or(12), "fast_period")?;
	validate_period(config_obj.slow_period.unwrap_or(26), "slow_period")?;
	validate_period(config_obj.signal_period.unwrap_or(9), "signal_period")?;
	Ok(pvo_core(volumes.as_ref(), config))
}

/// Pvo
#[napi]
pub fn pvo(
	volumes: Float64Array,
	config: Option<PercentageVolumeOscillatorConfig>,
) -> Result<PercentageVolumeOscillatorResult> {
	validate_non_empty(volumes.as_ref(), "volumes")?;
	let config_obj = config.clone().unwrap_or(PercentageVolumeOscillatorConfig {
		fast_period: None,
		slow_period: None,
		signal_period: None,
	});
	validate_period(config_obj.fast_period.unwrap_or(12), "fast_period")?;
	validate_period(config_obj.slow_period.unwrap_or(26), "slow_period")?;
	validate_period(config_obj.signal_period.unwrap_or(9), "signal_period")?;
	Ok(pvo_alias(volumes.as_ref(), config))
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
pub fn qstick(
	opens: Float64Array,
	closes: Float64Array,
	config: Option<QstickConfig>,
) -> Result<Vec<f64>> {
	validate_arrays(&[opens.as_ref(), closes.as_ref()], &["opens", "closes"])?;
	let period = config
		.clone()
		.unwrap_or(QstickConfig { period: None })
		.period
		.unwrap_or(14);
	validate_period(period, "period")?;
	Ok(qstick_core(opens.as_ref(), closes.as_ref(), config))
}

/// Rsi
#[napi]
pub fn rsi(closings: Float64Array, config: Option<RSIConfig>) -> Result<Vec<f64>> {
	validate_non_empty(closings.as_ref(), "closings")?;
	let period = config
		.clone()
		.unwrap_or(RSIConfig { period: None })
		.period
		.unwrap_or(14);
	validate_period(period, "period")?;
	Ok(rsi_core(closings.as_ref(), config))
}

/// Stochastic Oscillator
#[napi]
pub fn stochastic_oscillator(
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	config: Option<StochConfig>,
) -> Result<StochResult> {
	validate_arrays(
		&[highs.as_ref(), lows.as_ref(), closes.as_ref()],
		&["highs", "lows", "closes"],
	)?;
	let config_obj = config.clone().unwrap_or(StochConfig {
		k_period: None,
		d_period: None,
	});
	validate_period(config_obj.k_period.unwrap_or(14), "k_period")?;
	validate_period(config_obj.d_period.unwrap_or(3), "d_period")?;
	Ok(stoch_core(
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
		config,
	))
}

/// Ultimate Oscillator
#[napi]
pub fn ultimate_oscillator(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<UltimateOscillatorConfig>,
) -> Result<Vec<f64>> {
	validate_arrays(
		&[highs.as_ref(), lows.as_ref(), closings.as_ref()],
		&["highs", "lows", "closings"],
	)?;
	let config_obj = config.clone().unwrap_or(UltimateOscillatorConfig {
		period1: None,
		period2: None,
		period3: None,
	});
	validate_period(config_obj.period1.unwrap_or(7), "period1")?;
	validate_period(config_obj.period2.unwrap_or(14), "period2")?;
	validate_period(config_obj.period3.unwrap_or(28), "period3")?;
	Ok(uo_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		config,
	))
}

/// Uo
#[napi]
pub fn uo(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<UltimateOscillatorConfig>,
) -> Result<Vec<f64>> {
	validate_arrays(
		&[highs.as_ref(), lows.as_ref(), closings.as_ref()],
		&["highs", "lows", "closings"],
	)?;
	let config_obj = config.clone().unwrap_or(UltimateOscillatorConfig {
		period1: None,
		period2: None,
		period3: None,
	});
	validate_period(config_obj.period1.unwrap_or(7), "period1")?;
	validate_period(config_obj.period2.unwrap_or(14), "period2")?;
	validate_period(config_obj.period3.unwrap_or(28), "period3")?;
	Ok(uo_alias(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		config,
	))
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
