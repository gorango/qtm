use pyo3::prelude::*;

use crate::convert::{cfg_u32, deserialize_cfg, err, f64_out, normalize_config, to_py, F64Arr1};
use crate::convert::{Json, PyObject};
use crate::validation::{validate_arrays, validate_non_empty, validate_period};
use indicators_core::{
	absolute_price_oscillator as absolute_price_oscillator_core, adx as adx_core,
	alma as alma_core, annualized_volatility as annualized_volatility_core, aroon as aroon_core,
	awesome_oscillator as awesome_oscillator_core, balance_of_power as balance_of_power_core,
	bearish_engulfing as bearish_engulfing_core, bullish_engulfing as bullish_engulfing_core,
	camarilla_pivot_points as camarilla_pivot_points_core, cci as cci_core,
	chande_forecast_oscillator as chande_forecast_oscillator_core,
	classify_market_trend as classify_market_trend_core, cointegration as cointegration_core,
	correlation as correlation_core, cup_and_handle as cup_and_handle_core, dema as dema_core,
	double_bottom as double_bottom_core, double_top as double_top_core,
	elliott_wave as elliott_wave_core, emv as emv_core,
	fibonacci_pivot_points as fibonacci_pivot_points_core, find_peaks as find_peaks_core,
	find_troughs as find_troughs_core, flags_pennants as flags_pennants_core,
	force_index as force_index_core, head_and_shoulders as head_and_shoulders_core,
	hma as hma_core, ichimoku_cloud as ichimoku_cloud_core, internal::ema::ema_internal,
	internal::moving_sum::moving_sum_internal, internal::sma::sma_internal, kst as kst_core,
	larsson as larsson_core, linear_regression as linear_regression_core, linreg as linreg_core,
	macd as macd_core, market::advance_decline::advance_decline_line as advance_decline_line_core,
	market::mcclellan_oscillator::mcclellan_oscillator as mcclellan_oscillator_core,
	mass_index as mass_index_core, max_drawdown as max_drawdown_core, mfi as mfi_core,
	momentum_index as momentum_index_core, money_flow_index as money_flow_index_core,
	parabolic_sar as parabolic_sar_core, percent_rank as percent_rank_core,
	percentage_price_oscillator as percentage_price_oscillator_core,
	percentage_volume_oscillator as percentage_volume_oscillator_core,
	percentile_linear_interpolation as percentile_linear_interpolation_core,
	percentile_nearest_rank as percentile_nearest_rank_core, pivot_points as pivot_points_core,
	price_rate_of_change as price_rate_of_change_core, qstick as qstick_core,
	random_index as random_index_core, rolling_moving_average as rolling_moving_average_core,
	rsi as rsi_core, smoothed_moving_average as smoothed_moving_average_core, stars as stars_core,
	stochastic_oscillator as stochastic_oscillator_core, super_trend as super_trend_core,
	tema as tema_core, tma as tma_core, trend::moving_max::moving_max_internal,
	trend::moving_min::moving_min_internal, trend::rma::rma_internal, trend::since::since_internal,
	trend::typical_price::typical_price as typical_price_core, triangles as triangles_core,
	trix as trix_core, ulcer_index as ulcer_index_core,
	ultimate_oscillator as ultimate_oscillator_core, uo as uo_core, value_when as value_when_core,
	volatility::acceleration_bands::ab as ab_core,
	volatility::acceleration_bands::acceleration_bands as acceleration_bands_core,
	volatility::average_true_range::atr as atr_core,
	volatility::average_true_range::average_true_range as average_true_range_core,
	volatility::bollinger_bands::bb as bb_core,
	volatility::bollinger_bands::bollinger_bands as bollinger_bands_core,
	volatility::bollinger_bands_width::bbw as bbw_core,
	volatility::bollinger_bands_width::bollinger_bands_width as bollinger_bands_width_core,
	volatility::chandelier_exit::ce as ce_core,
	volatility::chandelier_exit::chandelier_exit as chandelier_exit_core,
	volatility::dev::dev as dev_core,
	volatility::dev::mean_absolute_deviation as mean_absolute_deviation_core,
	volatility::donchian_channel::dc as dc_core,
	volatility::donchian_channel::donchian_channel as donchian_channel_core,
	volatility::keltner_channel::kc as kc_core,
	volatility::keltner_channel::keltner_channel as keltner_channel_core,
	volatility::moving_standard_deviation::moving_standard_deviation as moving_standard_deviation_core,
	volatility::moving_standard_deviation::mstd as mstd_core,
	volatility::projection_oscillator::po as po_core,
	volatility::projection_oscillator::projection_oscillator as projection_oscillator_core,
	volatility::true_range::tr as tr_core, volatility::true_range::true_range as true_range_core,
	volatility::ttm_squeeze::ttm_squeeze as ttm_squeeze_core,
	volatility::variance::rolling_variance as rolling_variance_core,
	volatility::variance::variance as variance_core, volatility::z_score::z_score as z_score_core,
	volume::accumulation_distribution::accumulation_distribution as accumulation_distribution_core,
	volume::accumulation_distribution::ad as ad_core,
	volume::anchored_vwap::anchored_vwap as anchored_vwap_core,
	volume::chaikin_money_flow::chaikin_money_flow as chaikin_money_flow_core,
	volume::chaikin_money_flow::cmf as cmf_core,
	volume::ease_of_movement::ease_of_movement as ease_of_movement_core,
	volume::negative_volume_index::negative_volume_index as negative_volume_index_core,
	volume::negative_volume_index::nvi as nvi_core, volume::obv::obv as obv_core,
	volume::obv::on_balance_volume as on_balance_volume_core,
	volume::volume_price_trend::volume_price_trend as volume_price_trend_core,
	volume::volume_price_trend::vpt as vpt_core, volume_profile as volume_profile_core,
	volume_surge as volume_surge_core,
	volume_weighted_average_price as volume_weighted_average_price_core, vortex as vortex_core,
	vwap as vwap_core, vwma as vwma_core, wedges as wedges_core, williams_r as williams_r_core,
	wma as wma_core, zig_zag_filter as zig_zag_filter_core, ADXConfig, ALMAConfig, AroonConfig,
	AwesomeOscillatorConfig, BBConfig, BBResult, Bar, CCIConfig, ChaikinOscillatorConfig,
	CointegrationConfig, CorrelationConfig, FIConfig, IchimokuCloudConfig, KSTConfig, LinRegConfig,
	MACDConfig, MFIConfig, MSTDConfig, MeanAbsoluteDeviationConfig, MomentumIndexConfig,
	PSARConfig, PercentRankConfig, PercentagePriceOscillatorConfig,
	PercentageVolumeOscillatorConfig, PercentileLinearInterpolationConfig,
	PercentileNearestRankConfig, PriceRateOfChangeConfig, QstickConfig, RSIConfig, StochConfig,
	UltimateOscillatorConfig, VWAPConfig, ValueWhenConfig, VarianceConfig, VolumeSurgeConfig,
	WilliamsRConfig, ZScoreConfig,
};

/// Indicator helpers used across modules.
type PyResultO = PyResult<PyObject>;

fn result_or_err<T>(r: indicators_core::IndicatorResult<T>) -> PyResult<T> {
	r.map_err(|e| err(e.to_string()))
}

// ── Shared statistics ─────────────────────────────────────────

#[pyfunction]
#[pyo3(signature = (values1, values2, config = None))]
pub fn cointegration<'py>(
	py: Python<'py>,
	values1: F64Arr1<'py>,
	values2: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let v1 = values1.as_array().to_vec();
	let v2 = values2.as_array().to_vec();
	validate_arrays([(&v1, "values1"), (&v2, "values2")])?;
	let cfg = deserialize_cfg::<CointegrationConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(cointegration_core(&v1, &v2, cfg))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values1, values2, config = None))]
pub fn correlation<'py>(
	py: Python<'py>,
	values1: F64Arr1<'py>,
	values2: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let v1 = values1.as_array().to_vec();
	let v2 = values2.as_array().to_vec();
	validate_arrays([(&v1, "values1"), (&v2, "values2")])?;
	let cfg = deserialize_cfg::<CorrelationConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(correlation_core(&v1, &v2, cfg))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values1, values2, config = None))]
pub fn pearson_correlation<'py>(
	py: Python<'py>,
	values1: F64Arr1<'py>,
	values2: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	correlation(py, values1, values2, config)
}

#[pyfunction]
#[pyo3(signature = (values, config = None))]
pub fn percent_rank<'py>(py: Python<'py>, values: F64Arr1<'py>, config: Option<Json>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let cfg = deserialize_cfg::<PercentRankConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(percent_rank_core(&values, cfg))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, config = None))]
pub fn percentile_linear_interpolation<'py>(
	py: Python<'py>,
	values: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let cfg = deserialize_cfg::<PercentileLinearInterpolationConfig>(
		config.map(|c| normalize_config(c.0)),
	)?;
	let out = result_or_err(percentile_linear_interpolation_core(&values, cfg))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, config = None))]
pub fn percentile_nearest_rank<'py>(
	py: Python<'py>,
	values: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let cfg =
		deserialize_cfg::<PercentileNearestRankConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(percentile_nearest_rank_core(&values, cfg))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (condition, source, config = None))]
pub fn value_when<'py>(
	py: Python<'py>,
	condition: F64Arr1<'py>,
	source: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let condition = condition.as_array().to_vec();
	let source = source.as_array().to_vec();
	validate_arrays([(&condition, "condition"), (&source, "source")])?;
	let cfg = deserialize_cfg::<ValueWhenConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(value_when_core(&condition, &source, cfg))?;
	Ok(f64_out(py, &out))
}

// ── Market breadth ────────────────────────────────────────────

#[pyfunction]
pub fn advance_decline_line<'py>(
	py: Python<'py>,
	advances: F64Arr1<'py>,
	declines: F64Arr1<'py>,
) -> PyResultO {
	let advances = advances.as_array().to_vec();
	let declines = declines.as_array().to_vec();
	validate_arrays([(&advances, "advances"), (&declines, "declines")])?;
	let out = advance_decline_line_core(&advances, &declines);
	Ok(f64_out(py, &out))
}

#[pyfunction]
pub fn mcclellan_oscillator<'py>(
	py: Python<'py>,
	advances: F64Arr1<'py>,
	declines: F64Arr1<'py>,
) -> PyResultO {
	let advances = advances.as_array().to_vec();
	let declines = declines.as_array().to_vec();
	validate_arrays([(&advances, "advances"), (&declines, "declines")])?;
	let out = result_or_err(mcclellan_oscillator_core(&advances, &declines))?;
	Ok(f64_out(py, &out))
}

// ── Momentum ──────────────────────────────────────────────────

#[pyfunction]
#[pyo3(signature = (highs, lows, config = None))]
pub fn awesome_oscillator<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let highs = highs.as_array().to_vec();
	let lows = lows.as_array().to_vec();
	validate_arrays([(&highs, "highs"), (&lows, "lows")])?;
	let cfg = deserialize_cfg::<AwesomeOscillatorConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(awesome_oscillator_core(&highs, &lows, cfg))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, volumes, config = None))]
pub fn chaikin_oscillator<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let (h, l, c, v) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
		volumes.as_array().to_vec(),
	);
	validate_arrays([
		(&h, "highs"),
		(&l, "lows"),
		(&c, "closings"),
		(&v, "volumes"),
	])?;
	let cfg = config.map(|c| normalize_config(c.0));
	let period = cfg_u32(&cfg, "fast_period", 3);
	validate_period(period, "fast_period")?;
	validate_period(cfg_u32(&cfg, "slow_period", 10), "slow_period")?;
	let cfg = deserialize_cfg::<ChaikinOscillatorConfig>(cfg)?;
	let out = indicators_core::chaikin_oscillator(&h, &l, &c, &v, cfg);
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, volumes, config = None))]
pub fn cmo<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let (h, l, c, v) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
		volumes.as_array().to_vec(),
	);
	validate_arrays([
		(&h, "highs"),
		(&l, "lows"),
		(&c, "closings"),
		(&v, "volumes"),
	])?;
	let cfg = config.map(|c| normalize_config(c.0));
	let period = cfg_u32(&cfg, "fast_period", 3);
	validate_period(period, "fast_period")?;
	validate_period(cfg_u32(&cfg, "slow_period", 10), "slow_period")?;
	let cfg = deserialize_cfg::<ChaikinOscillatorConfig>(cfg)?;
	let out = indicators_core::cmo(&h, &l, &c, &v, cfg);
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, config = None))]
pub fn ichimoku_cloud<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let cfg = config.map(|c| normalize_config(c.0));
	for (key, default) in [("short", 9u32), ("medium", 26), ("long", 52), ("close", 26)] {
		validate_period(cfg_u32(&cfg, key, default), key)?;
	}
	let cfg = deserialize_cfg::<IchimokuCloudConfig>(cfg)?;
	let out = ichimoku_cloud_core(&h, &l, &c, cfg);
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, config = None))]
pub fn ichimoku<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let cfg = config.map(|c| normalize_config(c.0));
	for (key, default) in [("short", 9u32), ("medium", 26), ("long", 52), ("close", 26)] {
		validate_period(cfg_u32(&cfg, key, default), key)?;
	}
	let cfg = deserialize_cfg::<IchimokuCloudConfig>(cfg)?;
	let out = indicators_core::ichimoku(&h, &l, &c, cfg);
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (prices, config = None))]
pub fn kst<'py>(py: Python<'py>, prices: F64Arr1<'py>, config: Option<Json>) -> PyResultO {
	let prices = prices.as_array().to_vec();
	validate_non_empty(&prices, "prices")?;
	let cfg = config.map(|c| normalize_config(c.0));
	for (key, default) in [
		("roc1_period", 10u32),
		("roc2_period", 15),
		("roc3_period", 20),
		("roc4_period", 30),
		("sma1_period", 10),
		("sma2_period", 10),
		("sma3_period", 10),
		("sma4_period", 15),
		("signal_period", 9),
	] {
		validate_period(cfg_u32(&cfg, key, default), key)?;
	}
	let cfg = deserialize_cfg::<KSTConfig>(cfg)?;
	let out = kst_core(&prices, cfg);
	to_py(py, &out)
}

#[pyfunction]
pub fn larsson<'py>(py: Python<'py>, highs: F64Arr1<'py>, lows: F64Arr1<'py>) -> PyResultO {
	let highs = highs.as_array().to_vec();
	let lows = lows.as_array().to_vec();
	validate_arrays([(&highs, "highs"), (&lows, "lows")])?;
	let out = larsson_core(&highs, &lows);
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (closes, config = None))]
pub fn macd<'py>(py: Python<'py>, closes: F64Arr1<'py>, config: Option<Json>) -> PyResultO {
	let closes = closes.as_array().to_vec();
	validate_non_empty(&closes, "closes")?;
	let cfg = deserialize_cfg::<MACDConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(macd_core(&closes, cfg))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (prices, config = None))]
pub fn momentum_index<'py>(
	py: Python<'py>,
	prices: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let prices = prices.as_array().to_vec();
	validate_non_empty(&prices, "prices")?;
	let cfg = config.map(|c| normalize_config(c.0));
	validate_period(cfg_u32(&cfg, "period", 14), "period")?;
	let cfg = deserialize_cfg::<MomentumIndexConfig>(cfg)?;
	let out = momentum_index_core(&prices, cfg);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (prices, config = None))]
pub fn percentage_price_oscillator<'py>(
	py: Python<'py>,
	prices: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let prices = prices.as_array().to_vec();
	validate_non_empty(&prices, "prices")?;
	let cfg =
		deserialize_cfg::<PercentagePriceOscillatorConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(percentage_price_oscillator_core(&prices, cfg))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (volumes, config = None))]
pub fn percentage_volume_oscillator<'py>(
	py: Python<'py>,
	volumes: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let volumes = volumes.as_array().to_vec();
	validate_non_empty(&volumes, "volumes")?;
	let cfg = config.map(|c| normalize_config(c.0));
	for (key, default) in [
		("fast_period", 12u32),
		("slow_period", 26),
		("signal_period", 9),
	] {
		validate_period(cfg_u32(&cfg, key, default), key)?;
	}
	let cfg = deserialize_cfg::<PercentageVolumeOscillatorConfig>(cfg)?;
	let out = percentage_volume_oscillator_core(&volumes, cfg);
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (volumes, config = None))]
pub fn pvo<'py>(py: Python<'py>, volumes: F64Arr1<'py>, config: Option<Json>) -> PyResultO {
	let volumes = volumes.as_array().to_vec();
	validate_non_empty(&volumes, "volumes")?;
	let cfg = config.map(|c| normalize_config(c.0));
	for (key, default) in [
		("fast_period", 12u32),
		("slow_period", 26),
		("signal_period", 9),
	] {
		validate_period(cfg_u32(&cfg, key, default), key)?;
	}
	let cfg = deserialize_cfg::<PercentageVolumeOscillatorConfig>(cfg)?;
	let out = indicators_core::pvo(&volumes, cfg);
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (values, config = None))]
pub fn price_rate_of_change<'py>(
	py: Python<'py>,
	values: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let cfg = deserialize_cfg::<PriceRateOfChangeConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(price_rate_of_change_core(&values, cfg))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (opens, closes, config = None))]
pub fn qstick<'py>(
	py: Python<'py>,
	opens: F64Arr1<'py>,
	closes: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let opens = opens.as_array().to_vec();
	let closes = closes.as_array().to_vec();
	validate_arrays([(&opens, "opens"), (&closes, "closes")])?;
	let cfg = config.map(|c| normalize_config(c.0));
	validate_period(cfg_u32(&cfg, "period", 14), "period")?;
	let cfg = deserialize_cfg::<QstickConfig>(cfg)?;
	let out = qstick_core(&opens, &closes, cfg);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (closings, config = None))]
pub fn rsi<'py>(py: Python<'py>, closings: F64Arr1<'py>, config: Option<Json>) -> PyResultO {
	let closings = closings.as_array().to_vec();
	validate_non_empty(&closings, "closings")?;
	let cfg = config.map(|c| normalize_config(c.0));
	validate_period(cfg_u32(&cfg, "period", 14), "period")?;
	let cfg = deserialize_cfg::<RSIConfig>(cfg)?;
	let out = rsi_core(&closings, cfg);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closes, config = None))]
pub fn stochastic_oscillator<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closes: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closes.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closes")])?;
	let cfg = config.map(|c| normalize_config(c.0));
	validate_period(cfg_u32(&cfg, "k_period", 14), "k_period")?;
	validate_period(cfg_u32(&cfg, "d_period", 3), "d_period")?;
	let cfg = deserialize_cfg::<StochConfig>(cfg)?;
	let out = stochastic_oscillator_core(&h, &l, &c, cfg);
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, config = None))]
pub fn ultimate_oscillator<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let cfg = config.map(|c| normalize_config(c.0));
	validate_period(cfg_u32(&cfg, "period1", 7), "period1")?;
	validate_period(cfg_u32(&cfg, "period2", 14), "period2")?;
	validate_period(cfg_u32(&cfg, "period3", 28), "period3")?;
	let cfg = deserialize_cfg::<UltimateOscillatorConfig>(cfg)?;
	let out = ultimate_oscillator_core(&h, &l, &c, cfg);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, config = None))]
pub fn uo<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let cfg = config.map(|c| normalize_config(c.0));
	validate_period(cfg_u32(&cfg, "period1", 7), "period1")?;
	validate_period(cfg_u32(&cfg, "period2", 14), "period2")?;
	validate_period(cfg_u32(&cfg, "period3", 28), "period3")?;
	let cfg = deserialize_cfg::<UltimateOscillatorConfig>(cfg)?;
	let out = uo_core(&h, &l, &c, cfg);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, config = None))]
pub fn williams_r<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let cfg = deserialize_cfg::<WilliamsRConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(williams_r_core(&h, &l, &c, cfg))?;
	Ok(f64_out(py, &out))
}

// ── Patterns ──────────────────────────────────────────────────

#[pyfunction]
#[allow(clippy::too_many_arguments)]
pub fn cup_and_handle<'py>(
	py: Python<'py>,
	opens: F64Arr1<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closes: F64Arr1<'py>,
	cup_depth: Option<f64>,
	handle_retracement: Option<f64>,
	min_duration: Option<u32>,
) -> PyResultO {
	let (o, h, l, c) = (
		opens.as_array().to_vec(),
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closes.as_array().to_vec(),
	);
	validate_arrays([(&o, "opens"), (&h, "highs"), (&l, "lows"), (&c, "closes")])?;
	let out = result_or_err(cup_and_handle_core(
		&o,
		&h,
		&l,
		&c,
		cup_depth,
		handle_retracement,
		min_duration,
	))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[allow(clippy::too_many_arguments)]
pub fn double_bottom<'py>(
	py: Python<'py>,
	opens: F64Arr1<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closes: F64Arr1<'py>,
	tolerance: Option<f64>,
	min_separation: Option<u32>,
	lookaround: Option<u32>,
) -> PyResultO {
	let (o, h, l, c) = (
		opens.as_array().to_vec(),
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closes.as_array().to_vec(),
	);
	validate_arrays([(&o, "opens"), (&h, "highs"), (&l, "lows"), (&c, "closes")])?;
	let out = result_or_err(double_bottom_core(
		&o,
		&h,
		&l,
		&c,
		tolerance,
		min_separation,
		lookaround,
	))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[allow(clippy::too_many_arguments)]
pub fn double_top<'py>(
	py: Python<'py>,
	opens: F64Arr1<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closes: F64Arr1<'py>,
	tolerance: Option<f64>,
	min_separation: Option<u32>,
	lookaround: Option<u32>,
) -> PyResultO {
	let (o, h, l, c) = (
		opens.as_array().to_vec(),
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closes.as_array().to_vec(),
	);
	validate_arrays([(&o, "opens"), (&h, "highs"), (&l, "lows"), (&c, "closes")])?;
	let out = result_or_err(double_top_core(
		&o,
		&h,
		&l,
		&c,
		tolerance,
		min_separation,
		lookaround,
	))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[allow(clippy::too_many_arguments)]
pub fn elliott_wave<'py>(
	py: Python<'py>,
	opens: F64Arr1<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closes: F64Arr1<'py>,
	wave2_retracement: Option<f64>,
	wave4_retracement: Option<f64>,
	wave3_min_extension: Option<f64>,
	min_wave_separation: Option<u32>,
	lookaround: Option<u32>,
	retracement_tolerance: Option<f64>,
) -> PyResultO {
	let (o, h, l, c) = (
		opens.as_array().to_vec(),
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closes.as_array().to_vec(),
	);
	validate_arrays([(&o, "opens"), (&h, "highs"), (&l, "lows"), (&c, "closes")])?;
	let out = result_or_err(elliott_wave_core(
		&o,
		&h,
		&l,
		&c,
		wave2_retracement,
		wave4_retracement,
		wave3_min_extension,
		min_wave_separation,
		lookaround,
		retracement_tolerance,
	))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
pub fn bullish_engulfing<'py>(
	py: Python<'py>,
	opens: F64Arr1<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closes: F64Arr1<'py>,
) -> PyResultO {
	let (o, h, l, c) = (
		opens.as_array().to_vec(),
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closes.as_array().to_vec(),
	);
	validate_arrays([(&o, "opens"), (&h, "highs"), (&l, "lows"), (&c, "closes")])?;
	let out = result_or_err(bullish_engulfing_core(&o, &h, &l, &c))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
pub fn bearish_engulfing<'py>(
	py: Python<'py>,
	opens: F64Arr1<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closes: F64Arr1<'py>,
) -> PyResultO {
	let (o, h, l, c) = (
		opens.as_array().to_vec(),
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closes.as_array().to_vec(),
	);
	validate_arrays([(&o, "opens"), (&h, "highs"), (&l, "lows"), (&c, "closes")])?;
	let out = result_or_err(bearish_engulfing_core(&o, &h, &l, &c))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[allow(clippy::too_many_arguments)]
pub fn flags_pennants<'py>(
	py: Python<'py>,
	opens: F64Arr1<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closes: F64Arr1<'py>,
	pole_length: Option<u32>,
	consolidation_bars: Option<u32>,
	breakout_threshold: Option<f64>,
) -> PyResultO {
	let (o, h, l, c) = (
		opens.as_array().to_vec(),
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closes.as_array().to_vec(),
	);
	validate_arrays([(&o, "opens"), (&h, "highs"), (&l, "lows"), (&c, "closes")])?;
	let out = result_or_err(flags_pennants_core(
		&o,
		&h,
		&l,
		&c,
		pole_length,
		consolidation_bars,
		breakout_threshold,
	))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[allow(clippy::too_many_arguments)]
pub fn head_and_shoulders<'py>(
	py: Python<'py>,
	opens: F64Arr1<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closes: F64Arr1<'py>,
	min_distance: Option<u32>,
	tolerance: Option<f64>,
	deviation: Option<f64>,
) -> PyResultO {
	let (o, h, l, c) = (
		opens.as_array().to_vec(),
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closes.as_array().to_vec(),
	);
	validate_arrays([(&o, "opens"), (&h, "highs"), (&l, "lows"), (&c, "closes")])?;
	let out = result_or_err(head_and_shoulders_core(
		&o,
		&h,
		&l,
		&c,
		min_distance,
		tolerance,
		deviation,
	))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
pub fn find_peaks<'py>(py: Python<'py>, values: F64Arr1<'py>, lookaround: u32) -> PyResultO {
	let values = values.as_array().to_vec();
	let out = find_peaks_core(&values, lookaround);
	Ok(u32_out_owned(py, out))
}

#[pyfunction]
pub fn find_troughs<'py>(py: Python<'py>, values: F64Arr1<'py>, lookaround: u32) -> PyResultO {
	let values = values.as_array().to_vec();
	let out = find_troughs_core(&values, lookaround);
	Ok(u32_out_owned(py, out))
}

fn u32_out_owned(py: Python<'_>, v: Vec<u32>) -> PyObject {
	crate::convert::u32_out(py, &v)
}

#[pyfunction]
pub fn linear_regression<'py>(py: Python<'py>, points: F64Arr1<'py>) -> PyResultO {
	let points = points.as_array().to_vec();
	let out = linear_regression_core(points);
	Ok(f64_out(py, &out))
}

#[pyfunction]
pub fn zig_zag_filter<'py>(py: Python<'py>, values: F64Arr1<'py>, deviation: f64) -> PyResultO {
	let values = values.as_array().to_vec();
	let out = zig_zag_filter_core(&values, deviation);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[allow(clippy::too_many_arguments)]
pub fn stars<'py>(
	py: Python<'py>,
	opens: F64Arr1<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closes: F64Arr1<'py>,
	body_ratio_threshold: Option<f64>,
	gap_threshold: Option<f64>,
) -> PyResultO {
	let (o, h, l, c) = (
		opens.as_array().to_vec(),
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closes.as_array().to_vec(),
	);
	validate_arrays([(&o, "opens"), (&h, "highs"), (&l, "lows"), (&c, "closes")])?;
	let out = result_or_err(stars_core(
		&o,
		&h,
		&l,
		&c,
		body_ratio_threshold,
		gap_threshold,
	))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[allow(clippy::too_many_arguments)]
pub fn triangles<'py>(
	py: Python<'py>,
	opens: F64Arr1<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closes: F64Arr1<'py>,
	min_points: Option<u32>,
	tolerance: Option<f64>,
	convergence_tolerance: Option<f64>,
) -> PyResultO {
	let (o, h, l, c) = (
		opens.as_array().to_vec(),
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closes.as_array().to_vec(),
	);
	validate_arrays([(&o, "opens"), (&h, "highs"), (&l, "lows"), (&c, "closes")])?;
	let out = result_or_err(triangles_core(
		&o,
		&h,
		&l,
		&c,
		min_points,
		tolerance,
		convergence_tolerance,
	))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (opens, highs, lows, closes, min_points = None, slope_tolerance = None, lookback = None))]
#[allow(clippy::too_many_arguments)]
pub fn wedges<'py>(
	py: Python<'py>,
	opens: F64Arr1<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closes: F64Arr1<'py>,
	min_points: Option<u32>,
	slope_tolerance: Option<f64>,
	lookback: Option<u32>,
) -> PyResultO {
	let (o, h, l, c) = (
		opens.as_array().to_vec(),
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closes.as_array().to_vec(),
	);
	validate_arrays([(&o, "opens"), (&h, "highs"), (&l, "lows"), (&c, "closes")])?;
	let out = result_or_err(wedges_core(
		&o,
		&h,
		&l,
		&c,
		min_points,
		slope_tolerance,
		lookback,
	))?;
	Ok(f64_out(py, &out))
}

// ── Trend ─────────────────────────────────────────────────────

#[pyfunction]
#[pyo3(signature = (closes, fast_period = None, slow_period = None))]
pub fn absolute_price_oscillator<'py>(
	py: Python<'py>,
	closes: F64Arr1<'py>,
	fast_period: Option<u32>,
	slow_period: Option<u32>,
) -> PyResultO {
	let closes = closes.as_array().to_vec();
	validate_non_empty(&closes, "closes")?;
	let out = result_or_err(absolute_price_oscillator_core(
		&closes,
		fast_period,
		slow_period,
	))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, config = None))]
pub fn adx<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let cfg = deserialize_cfg::<ADXConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(adx_core(&h, &l, &c, cfg))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (values, config = None))]
pub fn alma<'py>(py: Python<'py>, values: F64Arr1<'py>, config: Option<Json>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let cfg = deserialize_cfg::<ALMAConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(alma_core(&values, cfg))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, config = None))]
pub fn aroon<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let highs = highs.as_array().to_vec();
	let lows = lows.as_array().to_vec();
	validate_arrays([(&highs, "highs"), (&lows, "lows")])?;
	let cfg = deserialize_cfg::<AroonConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(aroon_core(&highs, &lows, cfg))?;
	to_py(py, &out)
}

#[pyfunction]
pub fn balance_of_power<'py>(
	py: Python<'py>,
	opens: F64Arr1<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closes: F64Arr1<'py>,
) -> PyResultO {
	let (o, h, l, c) = (
		opens.as_array().to_vec(),
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closes.as_array().to_vec(),
	);
	validate_arrays([(&o, "opens"), (&h, "highs"), (&l, "lows"), (&c, "closes")])?;
	let out = result_or_err(balance_of_power_core(&o, &h, &l, &c))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, config = None))]
pub fn cci<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let cfg = deserialize_cfg::<CCIConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(cci_core(&h, &l, &c, cfg))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
pub fn chande_forecast_oscillator<'py>(py: Python<'py>, values: F64Arr1<'py>) -> PyResultO {
	let values = values.as_array().to_vec();
	let out = result_or_err(chande_forecast_oscillator_core(&values))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, period = None))]
pub fn dema<'py>(py: Python<'py>, values: F64Arr1<'py>, period: Option<u32>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let out = result_or_err(dema_core(&values, period))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, period = None))]
pub fn ema<'py>(py: Python<'py>, values: F64Arr1<'py>, period: Option<u32>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let period = period.unwrap_or(12);
	validate_period(period, "period")?;
	let out = ema_internal(&values, period as usize);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, period = None))]
pub fn hma<'py>(py: Python<'py>, values: F64Arr1<'py>, period: Option<u32>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let out = result_or_err(hma_core(&values, period))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, config = None))]
pub fn linreg<'py>(py: Python<'py>, values: F64Arr1<'py>, config: Option<Json>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let cfg = deserialize_cfg::<LinRegConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(linreg_core(&values, cfg))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, ema_period = None, mi_period = None))]
pub fn mass_index<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	ema_period: Option<u32>,
	mi_period: Option<u32>,
) -> PyResultO {
	let highs = highs.as_array().to_vec();
	let lows = lows.as_array().to_vec();
	validate_arrays([(&highs, "highs"), (&lows, "lows")])?;
	let out = result_or_err(mass_index_core(&highs, &lows, ema_period, mi_period))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, period = None))]
pub fn moving_max<'py>(py: Python<'py>, values: F64Arr1<'py>, period: Option<u32>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let period = period.unwrap_or(4);
	validate_period(period, "period")?;
	let out = moving_max_internal(&values, period as usize);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, period = None))]
pub fn moving_min<'py>(py: Python<'py>, values: F64Arr1<'py>, period: Option<u32>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let period = period.unwrap_or(4);
	validate_period(period, "period")?;
	let out = moving_min_internal(&values, period as usize);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, period = None))]
pub fn moving_sum<'py>(py: Python<'py>, values: F64Arr1<'py>, period: Option<u32>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let period = period.unwrap_or(4);
	validate_period(period, "period")?;
	let out = moving_sum_internal(&values, period as usize);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, config = None))]
pub fn parabolic_sar<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let cfg = deserialize_cfg::<PSARConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(parabolic_sar_core(&h, &l, &c, cfg))?;
	to_py(py, &out)
}

#[pyfunction]
pub fn pivot_points<'py>(py: Python<'py>, high: f64, low: f64, close: f64) -> PyResultO {
	to_py(py, &pivot_points_core(high, low, close))
}

#[pyfunction]
pub fn fibonacci_pivot_points<'py>(py: Python<'py>, high: f64, low: f64, close: f64) -> PyResultO {
	to_py(py, &fibonacci_pivot_points_core(high, low, close))
}

#[pyfunction]
pub fn camarilla_pivot_points<'py>(py: Python<'py>, high: f64, low: f64, close: f64) -> PyResultO {
	to_py(py, &camarilla_pivot_points_core(high, low, close))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, r_period = None, k_period = None, d_period = None))]
pub fn random_index<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	r_period: Option<u32>,
	k_period: Option<u32>,
	d_period: Option<u32>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let out = result_or_err(random_index_core(&h, &l, &c, r_period, k_period, d_period))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (values, period = None))]
pub fn rma<'py>(py: Python<'py>, values: F64Arr1<'py>, period: Option<u32>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let period = period.unwrap_or(4);
	validate_period(period, "period")?;
	let out = rma_internal(&values, period as usize);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, period = None))]
pub fn rolling_moving_average<'py>(
	py: Python<'py>,
	values: F64Arr1<'py>,
	period: Option<u32>,
) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let out = result_or_err(rolling_moving_average_core(&values, period))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
pub fn since<'py>(py: Python<'py>, values: F64Arr1<'py>) -> PyResultO {
	let values = values.as_array().to_vec();
	let out = since_internal(&values);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, period = None))]
pub fn sma<'py>(py: Python<'py>, values: F64Arr1<'py>, period: Option<u32>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let period = period.unwrap_or(2);
	validate_period(period, "period")?;
	let out = sma_internal(&values, period as usize);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, period = None))]
pub fn smoothed_moving_average<'py>(
	py: Python<'py>,
	values: F64Arr1<'py>,
	period: Option<u32>,
) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let out = result_or_err(smoothed_moving_average_core(&values, period))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closes, period = None, multiplier = None))]
pub fn super_trend<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closes: F64Arr1<'py>,
	period: Option<u32>,
	multiplier: Option<f64>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closes.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closes")])?;
	let out = result_or_err(super_trend_core(&h, &l, &c, period, multiplier))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (values, period = None))]
pub fn tema<'py>(py: Python<'py>, values: F64Arr1<'py>, period: Option<u32>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let out = result_or_err(tema_core(&values, period))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, period = None))]
pub fn tma<'py>(py: Python<'py>, values: F64Arr1<'py>, period: Option<u32>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let out = result_or_err(tma_core(&values, period))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (market_data, trailing_period_length = None))]
pub fn classify_market_trend<'py>(
	py: Python<'py>,
	market_data: Vec<Json>,
	trailing_period_length: Option<u32>,
) -> PyResultO {
	let market_data: Vec<Bar> = crate::convert::records(market_data, "market_data")?;
	validate_non_empty(&market_data, "market_data")?;
	let out = classify_market_trend_core(market_data, trailing_period_length);
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (values, period = None))]
pub fn trix<'py>(py: Python<'py>, values: F64Arr1<'py>, period: Option<u32>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let out = result_or_err(trix_core(&values, period))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
pub fn typical_price<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closes: F64Arr1<'py>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closes.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closes")])?;
	let out = result_or_err(typical_price_core(&h, &l, &c))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, period = None))]
pub fn vortex<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	period: Option<u32>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let out = result_or_err(vortex_core(&h, &l, &c, period))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (closes, volumes, period = None))]
pub fn vwma<'py>(
	py: Python<'py>,
	closes: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
	period: Option<u32>,
) -> PyResultO {
	let closes = closes.as_array().to_vec();
	let volumes = volumes.as_array().to_vec();
	validate_arrays([(&closes, "closes"), (&volumes, "volumes")])?;
	let out = result_or_err(vwma_core(&closes, &volumes, period))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, period = None))]
pub fn wma<'py>(py: Python<'py>, values: F64Arr1<'py>, period: Option<u32>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let out = result_or_err(wma_core(&values, period))?;
	Ok(f64_out(py, &out))
}

// ── Volatility ────────────────────────────────────────────────

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, period = None, multiplier = None))]
pub fn ab<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	period: Option<u32>,
	multiplier: Option<f64>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let out = result_or_err(ab_core(&h, &l, &c, period, multiplier))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, period = None, multiplier = None))]
pub fn acceleration_bands<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	period: Option<u32>,
	multiplier: Option<f64>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let out = result_or_err(acceleration_bands_core(&h, &l, &c, period, multiplier))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (returns, periods = None))]
pub fn annualized_volatility<'py>(
	py: Python<'py>,
	returns: F64Arr1<'py>,
	periods: Option<u32>,
) -> PyResultO {
	let returns = returns.as_array().to_vec();
	let out = result_or_err(annualized_volatility_core(&returns, periods))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, config = None))]
pub fn atr<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let cfg = deserialize_cfg::<indicators_core::ATRConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(atr_core(&h, &l, &c, cfg))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, config = None))]
pub fn average_true_range<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let cfg = deserialize_cfg::<indicators_core::ATRConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(average_true_range_core(&h, &l, &c, cfg))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (closings, config = None))]
pub fn bb<'py>(py: Python<'py>, closings: F64Arr1<'py>, config: Option<Json>) -> PyResultO {
	let closings = closings.as_array().to_vec();
	validate_non_empty(&closings, "closings")?;
	let cfg = deserialize_cfg::<BBConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(bb_core(&closings, cfg))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (closings, config = None))]
pub fn bollinger_bands<'py>(
	py: Python<'py>,
	closings: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let closings = closings.as_array().to_vec();
	validate_non_empty(&closings, "closings")?;
	let cfg = deserialize_cfg::<BBConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(bollinger_bands_core(&closings, cfg))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (bb, period = None))]
pub fn bbw<'py>(py: Python<'py>, bb: Json, period: Option<u32>) -> PyResultO {
	let bb: BBResult = crate::convert::from_value(bb.0, "bb")?;
	let out = result_or_err(bbw_core(bb, period))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (bb, period = None))]
pub fn bollinger_bands_width<'py>(py: Python<'py>, bb: Json, period: Option<u32>) -> PyResultO {
	let bb: BBResult = crate::convert::from_value(bb.0, "bb")?;
	let out = result_or_err(bollinger_bands_width_core(bb, period))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, period = None))]
pub fn ce<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	period: Option<u32>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let out = result_or_err(ce_core(&h, &l, &c, period))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, period = None))]
pub fn chandelier_exit<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	period: Option<u32>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let out = result_or_err(chandelier_exit_core(&h, &l, &c, period))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (values, config = None))]
pub fn dev<'py>(py: Python<'py>, values: F64Arr1<'py>, config: Option<Json>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let cfg =
		deserialize_cfg::<MeanAbsoluteDeviationConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(dev_core(&values, cfg))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, config = None))]
pub fn mean_absolute_deviation<'py>(
	py: Python<'py>,
	values: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let cfg =
		deserialize_cfg::<MeanAbsoluteDeviationConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(mean_absolute_deviation_core(&values, cfg))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (closings, period = None))]
pub fn dc<'py>(py: Python<'py>, closings: F64Arr1<'py>, period: Option<u32>) -> PyResultO {
	let closings = closings.as_array().to_vec();
	validate_non_empty(&closings, "closings")?;
	let out = result_or_err(dc_core(&closings, period))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (closings, period = None))]
pub fn donchian_channel<'py>(
	py: Python<'py>,
	closings: F64Arr1<'py>,
	period: Option<u32>,
) -> PyResultO {
	let closings = closings.as_array().to_vec();
	validate_non_empty(&closings, "closings")?;
	let out = result_or_err(donchian_channel_core(&closings, period))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, period = None))]
pub fn kc<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	period: Option<u32>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let out = result_or_err(kc_core(&h, &l, &c, period))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, period = None))]
pub fn keltner_channel<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	period: Option<u32>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let out = result_or_err(keltner_channel_core(&h, &l, &c, period))?;
	to_py(py, &out)
}

#[pyfunction]
pub fn max_drawdown<'py>(py: Python<'py>, values: F64Arr1<'py>, period: u32) -> PyResultO {
	let values = values.as_array().to_vec();
	let out = result_or_err(max_drawdown_core(&values, period))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, config = None))]
pub fn mstd<'py>(py: Python<'py>, values: F64Arr1<'py>, config: Option<Json>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let cfg = deserialize_cfg::<MSTDConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(mstd_core(&values, cfg))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, config = None))]
pub fn moving_standard_deviation<'py>(
	py: Python<'py>,
	values: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let cfg = deserialize_cfg::<MSTDConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(moving_standard_deviation_core(&values, cfg))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, period = None, smooth = None))]
pub fn po<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	period: Option<u32>,
	smooth: Option<u32>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let out = result_or_err(po_core(&h, &l, &c, period, smooth))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, period = None, smooth = None))]
pub fn projection_oscillator<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	period: Option<u32>,
	smooth: Option<u32>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let out = result_or_err(projection_oscillator_core(&h, &l, &c, period, smooth))?;
	to_py(py, &out)
}

#[pyfunction]
pub fn tr<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let out = result_or_err(tr_core(&h, &l, &c))?;
	to_py(py, &out)
}

#[pyfunction]
pub fn true_range<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closings")])?;
	let out = result_or_err(true_range_core(&h, &l, &c))?;
	to_py(py, &out)
}

#[pyfunction]
#[allow(clippy::too_many_arguments)]
pub fn ttm_squeeze<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closes: F64Arr1<'py>,
	bb_period: Option<u32>,
	bb_std_dev: Option<f64>,
	kc_period: Option<u32>,
) -> PyResultO {
	let (h, l, c) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closes.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&c, "closes")])?;
	let out = result_or_err(ttm_squeeze_core(
		&h, &l, &c, bb_period, bb_std_dev, kc_period,
	))?;
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (values, period = None))]
pub fn ulcer_index<'py>(py: Python<'py>, values: F64Arr1<'py>, period: Option<u32>) -> PyResultO {
	let values = values.as_array().to_vec();
	let out = result_or_err(ulcer_index_core(&values, period))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, config = None))]
pub fn variance<'py>(py: Python<'py>, values: F64Arr1<'py>, config: Option<Json>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let cfg = deserialize_cfg::<VarianceConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(variance_core(&values, cfg))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, config = None))]
pub fn rolling_variance<'py>(
	py: Python<'py>,
	values: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let cfg = deserialize_cfg::<VarianceConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(rolling_variance_core(&values, cfg))?;
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (values, config = None))]
pub fn z_score<'py>(py: Python<'py>, values: F64Arr1<'py>, config: Option<Json>) -> PyResultO {
	let values = values.as_array().to_vec();
	validate_non_empty(&values, "values")?;
	let cfg = deserialize_cfg::<ZScoreConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = result_or_err(z_score_core(&values, cfg))?;
	Ok(f64_out(py, &out))
}

// ── Volume ────────────────────────────────────────────────────

#[pyfunction]
pub fn accumulation_distribution<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
) -> PyResultO {
	let (h, l, c, v) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
		volumes.as_array().to_vec(),
	);
	validate_arrays([
		(&h, "highs"),
		(&l, "lows"),
		(&c, "closings"),
		(&v, "volumes"),
	])?;
	let out = accumulation_distribution_core(&h, &l, &c, &v);
	Ok(f64_out(py, &out))
}

#[pyfunction]
pub fn ad<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
) -> PyResultO {
	let (h, l, c, v) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
		volumes.as_array().to_vec(),
	);
	validate_arrays([
		(&h, "highs"),
		(&l, "lows"),
		(&c, "closings"),
		(&v, "volumes"),
	])?;
	let out = ad_core(&h, &l, &c, &v);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, volumes, anchor_index = None))]
pub fn anchored_vwap<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
	anchor_index: Option<u32>,
) -> PyResultO {
	let (h, l, c, v) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
		volumes.as_array().to_vec(),
	);
	validate_arrays([
		(&h, "highs"),
		(&l, "lows"),
		(&c, "closings"),
		(&v, "volumes"),
	])?;
	let anchor = anchor_index.unwrap_or(0);
	let out = anchored_vwap_core(&h, &l, &c, &v, anchor);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, volumes, period = None))]
pub fn chaikin_money_flow<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
	period: Option<u32>,
) -> PyResultO {
	let (h, l, c, v) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
		volumes.as_array().to_vec(),
	);
	validate_arrays([
		(&h, "highs"),
		(&l, "lows"),
		(&c, "closings"),
		(&v, "volumes"),
	])?;
	let period = period.unwrap_or(20);
	validate_period(period, "period")?;
	let out = chaikin_money_flow_core(&h, &l, &c, &v, period);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, volumes, period = None))]
pub fn cmf<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
	period: Option<u32>,
) -> PyResultO {
	let (h, l, c, v) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
		volumes.as_array().to_vec(),
	);
	validate_arrays([
		(&h, "highs"),
		(&l, "lows"),
		(&c, "closings"),
		(&v, "volumes"),
	])?;
	let period = period.unwrap_or(20);
	validate_period(period, "period")?;
	let out = cmf_core(&h, &l, &c, &v, period);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, volumes, period = None))]
pub fn ease_of_movement<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
	period: Option<u32>,
) -> PyResultO {
	let (h, l, v) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		volumes.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&v, "volumes")])?;
	let period = period.unwrap_or(14);
	validate_period(period, "period")?;
	let out = ease_of_movement_core(&h, &l, &v, period);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, volumes, period = None))]
pub fn emv<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
	period: Option<u32>,
) -> PyResultO {
	let (h, l, v) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		volumes.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&v, "volumes")])?;
	let period = period.unwrap_or(14);
	validate_period(period, "period")?;
	let out = emv_core(&h, &l, &v, period);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (closings, volumes, config = None))]
pub fn force_index<'py>(
	py: Python<'py>,
	closings: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let closings = closings.as_array().to_vec();
	let volumes = volumes.as_array().to_vec();
	validate_arrays([(&closings, "closings"), (&volumes, "volumes")])?;
	let cfg = config.map(|c| normalize_config(c.0));
	validate_period(cfg_u32(&cfg, "period", 13), "period")?;
	let cfg = deserialize_cfg::<FIConfig>(cfg)?;
	let out = force_index_core(&closings, &volumes, cfg);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (closings, volumes, config = None))]
pub fn fi<'py>(
	py: Python<'py>,
	closings: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let closings = closings.as_array().to_vec();
	let volumes = volumes.as_array().to_vec();
	validate_arrays([(&closings, "closings"), (&volumes, "volumes")])?;
	let cfg = config.map(|c| normalize_config(c.0));
	validate_period(cfg_u32(&cfg, "period", 13), "period")?;
	let cfg = deserialize_cfg::<FIConfig>(cfg)?;
	let out = force_index_core(&closings, &volumes, cfg);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, volumes, config = None))]
pub fn mfi<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let (h, l, c, v) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
		volumes.as_array().to_vec(),
	);
	validate_arrays([
		(&h, "highs"),
		(&l, "lows"),
		(&c, "closings"),
		(&v, "volumes"),
	])?;
	let cfg = config.map(|c| normalize_config(c.0));
	validate_period(cfg_u32(&cfg, "period", 14), "period")?;
	let cfg = deserialize_cfg::<MFIConfig>(cfg)?;
	let out = mfi_core(&h, &l, &c, &v, cfg);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, volumes, config = None))]
pub fn money_flow_index<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let (h, l, c, v) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
		volumes.as_array().to_vec(),
	);
	validate_arrays([
		(&h, "highs"),
		(&l, "lows"),
		(&c, "closings"),
		(&v, "volumes"),
	])?;
	let cfg = config.map(|c| normalize_config(c.0));
	validate_period(cfg_u32(&cfg, "period", 14), "period")?;
	let cfg = deserialize_cfg::<MFIConfig>(cfg)?;
	let out = money_flow_index_core(&h, &l, &c, &v, cfg);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (closings, volumes, start = None))]
pub fn negative_volume_index<'py>(
	py: Python<'py>,
	closings: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
	start: Option<f64>,
) -> PyResultO {
	let closings = closings.as_array().to_vec();
	let volumes = volumes.as_array().to_vec();
	validate_arrays([(&closings, "closings"), (&volumes, "volumes")])?;
	let out = negative_volume_index_core(&closings, &volumes, start);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (closings, volumes, start = None))]
pub fn nvi<'py>(
	py: Python<'py>,
	closings: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
	start: Option<f64>,
) -> PyResultO {
	let closings = closings.as_array().to_vec();
	let volumes = volumes.as_array().to_vec();
	validate_arrays([(&closings, "closings"), (&volumes, "volumes")])?;
	let out = nvi_core(&closings, &volumes, start);
	Ok(f64_out(py, &out))
}

#[pyfunction]
pub fn obv<'py>(py: Python<'py>, closings: F64Arr1<'py>, volumes: F64Arr1<'py>) -> PyResultO {
	let closings = closings.as_array().to_vec();
	let volumes = volumes.as_array().to_vec();
	validate_arrays([(&closings, "closings"), (&volumes, "volumes")])?;
	let out = obv_core(&closings, &volumes);
	Ok(f64_out(py, &out))
}

#[pyfunction]
pub fn on_balance_volume<'py>(
	py: Python<'py>,
	closings: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
) -> PyResultO {
	let closings = closings.as_array().to_vec();
	let volumes = volumes.as_array().to_vec();
	validate_arrays([(&closings, "closings"), (&volumes, "volumes")])?;
	let out = on_balance_volume_core(&closings, &volumes);
	Ok(f64_out(py, &out))
}

#[pyfunction]
pub fn volume_price_trend<'py>(
	py: Python<'py>,
	closings: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
) -> PyResultO {
	let closings = closings.as_array().to_vec();
	let volumes = volumes.as_array().to_vec();
	validate_arrays([(&closings, "closings"), (&volumes, "volumes")])?;
	let out = volume_price_trend_core(&closings, &volumes);
	Ok(f64_out(py, &out))
}

#[pyfunction]
pub fn vpt<'py>(py: Python<'py>, closings: F64Arr1<'py>, volumes: F64Arr1<'py>) -> PyResultO {
	let closings = closings.as_array().to_vec();
	let volumes = volumes.as_array().to_vec();
	validate_arrays([(&closings, "closings"), (&volumes, "volumes")])?;
	let out = vpt_core(&closings, &volumes);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, volumes, bins = None))]
pub fn volume_profile<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
	bins: Option<u32>,
) -> PyResultO {
	let (h, l, v) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		volumes.as_array().to_vec(),
	);
	validate_arrays([(&h, "highs"), (&l, "lows"), (&v, "volumes")])?;
	let bins = bins.unwrap_or(50);
	validate_period(bins, "bins")?;
	let out = volume_profile_core(&h, &l, &v, Some(bins));
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (volumes, config = None))]
pub fn volume_surge<'py>(
	py: Python<'py>,
	volumes: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let volumes = volumes.as_array().to_vec();
	validate_non_empty(&volumes, "volumes")?;
	let cfg = config.map(|c| normalize_config(c.0));
	validate_period(cfg_u32(&cfg, "period", 20), "period")?;
	let cfg = deserialize_cfg::<VolumeSurgeConfig>(cfg)?;
	let out = volume_surge_core(&volumes, cfg);
	Ok(crate::convert::bool_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (volumes, config = None))]
pub fn vs<'py>(py: Python<'py>, volumes: F64Arr1<'py>, config: Option<Json>) -> PyResultO {
	let volumes = volumes.as_array().to_vec();
	validate_non_empty(&volumes, "volumes")?;
	let cfg = config.map(|c| normalize_config(c.0));
	validate_period(cfg_u32(&cfg, "period", 20), "period")?;
	let cfg = deserialize_cfg::<VolumeSurgeConfig>(cfg)?;
	let out = volume_surge_core(&volumes, cfg);
	Ok(crate::convert::bool_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, volumes, config = None))]
pub fn vwap<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let (h, l, c, v) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
		volumes.as_array().to_vec(),
	);
	validate_arrays([
		(&h, "highs"),
		(&l, "lows"),
		(&c, "closings"),
		(&v, "volumes"),
	])?;
	let cfg = config.map(|c| normalize_config(c.0));
	let period = cfg_u32(&cfg, "period", 0);
	if period != 0 {
		validate_period(period, "period")?;
	}
	let session = cfg_u32(&cfg, "session_length", 0);
	if session != 0 {
		validate_period(session, "session_length")?;
	}
	let cfg = deserialize_cfg::<VWAPConfig>(cfg)?;
	let out = vwap_core(&h, &l, &c, &v, cfg);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (highs, lows, closings, volumes, config = None))]
pub fn volume_weighted_average_price<'py>(
	py: Python<'py>,
	highs: F64Arr1<'py>,
	lows: F64Arr1<'py>,
	closings: F64Arr1<'py>,
	volumes: F64Arr1<'py>,
	config: Option<Json>,
) -> PyResultO {
	let (h, l, c, v) = (
		highs.as_array().to_vec(),
		lows.as_array().to_vec(),
		closings.as_array().to_vec(),
		volumes.as_array().to_vec(),
	);
	validate_arrays([
		(&h, "highs"),
		(&l, "lows"),
		(&c, "closings"),
		(&v, "volumes"),
	])?;
	let cfg = config.map(|c| normalize_config(c.0));
	let period = cfg_u32(&cfg, "period", 0);
	if period != 0 {
		validate_period(period, "period")?;
	}
	let session = cfg_u32(&cfg, "session_length", 0);
	if session != 0 {
		validate_period(session, "session_length")?;
	}
	let cfg = deserialize_cfg::<VWAPConfig>(cfg)?;
	let out = volume_weighted_average_price_core(&h, &l, &c, &v, cfg);
	Ok(f64_out(py, &out))
}
