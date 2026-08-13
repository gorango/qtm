use pyo3::prelude::*;

use crate::convert::{err, normalize_config, to_py};
use crate::convert::{Json, PyObject};

#[pyfunction]
pub fn calculate_rsi_warmup(period: u32) -> u32 {
	indicators_core::calculate_rsi_warmup(period)
}

#[pyfunction]
pub fn calculate_ema_warmup(period: u32) -> u32 {
	indicators_core::calculate_ema_warmup(period)
}

#[pyfunction]
pub fn calculate_macd_warmup(fast_period: u32, slow_period: u32, signal_period: u32) -> u32 {
	indicators_core::calculate_macd_warmup(fast_period, slow_period, signal_period)
}

#[pyfunction]
pub fn calculate_stochastic_warmup(k_period: u32, d_period: u32) -> u32 {
	indicators_core::calculate_stochastic_warmup(k_period, d_period)
}

#[pyfunction]
pub fn calculate_williams_rwarmup(period: u32) -> u32 {
	indicators_core::calculate_williams_rwarmup(period)
}

#[pyfunction]
pub fn calculate_ichimoku_warmup(short: u32, medium: u32, long: u32) -> u32 {
	indicators_core::calculate_ichimoku_warmup(short, medium, long)
}

#[pyfunction]
pub fn calculate_adx_warmup(period: u32) -> u32 {
	indicators_core::calculate_adx_warmup(period)
}

#[pyfunction]
pub fn calculate_supertrend_warmup(period: u32) -> u32 {
	indicators_core::calculate_supertrend_warmup(period)
}

#[pyfunction]
pub fn calculate_chaikin_oscillator_warmup(fast_period: u32, slow_period: u32) -> u32 {
	indicators_core::calculate_chaikin_oscillator_warmup(fast_period, slow_period)
}

#[pyfunction]
pub fn calculate_vwap_warmup(period: u32) -> u32 {
	indicators_core::calculate_vwap_warmup(period)
}

#[pyfunction]
pub fn calculate_obv_warmup(period: u32) -> u32 {
	indicators_core::calculate_obv_warmup(period)
}

#[pyfunction]
pub fn calculate_accumulation_distribution_warmup() -> u32 {
	indicators_core::calculate_accumulation_distribution_warmup()
}

#[pyfunction]
#[allow(clippy::too_many_arguments)]
pub fn calculate_kst_warmup(
	roc1: u32,
	roc2: u32,
	roc3: u32,
	roc4: u32,
	sma1: u32,
	sma2: u32,
	sma3: u32,
	sma4: u32,
	signal_period: u32,
) -> u32 {
	indicators_core::calculate_kst_warmup(roc1, roc2, roc3, roc4, sma1, sma2, sma3, sma4, signal_period)
}

#[pyfunction]
pub fn calculate_mfi_warmup(period: u32) -> u32 {
	indicators_core::calculate_mfi_warmup(period)
}

#[pyfunction]
pub fn calculate_keltner_channel_warmup(period: u32) -> u32 {
	indicators_core::calculate_keltner_channel_warmup(period)
}

#[pyfunction]
pub fn calculate_projection_oscillator_warmup(period: u32, smooth: u32) -> u32 {
	indicators_core::calculate_projection_oscillator_warmup(period, smooth)
}

#[pyfunction]
pub fn calculate_chandelier_exit_warmup(period: u32) -> u32 {
	indicators_core::calculate_chandelier_exit_warmup(period)
}

#[pyfunction]
pub fn calculate_parabolic_sar_warmup() -> u32 {
	indicators_core::calculate_parabolic_sar_warmup()
}

#[pyfunction]
pub fn calculate_bollinger_bands_warmup(period: u32) -> u32 {
	indicators_core::calculate_bollinger_bands_warmup(period)
}

#[pyfunction]
pub fn calculate_atr_warmup(period: u32) -> u32 {
	indicators_core::calculate_atr_warmup(period)
}

#[pyfunction]
pub fn calculate_alma_warmup(period: u32) -> u32 {
	indicators_core::calculate_alma_warmup(period)
}

#[pyfunction]
pub fn calculate_hma_warmup(period: u32) -> u32 {
	indicators_core::calculate_hma_warmup(period)
}

#[pyfunction]
pub fn calculate_wma_warmup(period: u32) -> u32 {
	indicators_core::calculate_wma_warmup(period)
}

#[pyfunction]
pub fn calculate_linreg_warmup(period: u32) -> u32 {
	indicators_core::calculate_linreg_warmup(period)
}

#[pyfunction]
pub fn calculate_mad_warmup(period: u32) -> u32 {
	indicators_core::calculate_mad_warmup(period)
}

#[pyfunction]
pub fn calculate_variance_warmup(period: u32) -> u32 {
	indicators_core::calculate_variance_warmup(period)
}

#[pyfunction]
pub fn calculate_correlation_warmup(period: u32) -> u32 {
	indicators_core::calculate_correlation_warmup(period)
}

#[pyfunction]
pub fn calculate_percent_rank_warmup(period: u32) -> u32 {
	indicators_core::calculate_percent_rank_warmup(period)
}

/// Warmup required for an arbitrary indicator, resolved by name + params dict.
#[pyfunction]
#[pyo3(signature = (indicator_type, params = None))]
pub fn calculate_indicator_warmup(
	py: Python<'_>,
	indicator_type: String,
	params: Option<Json>,
) -> PyResult<PyObject> {
	let params = params.map(|c| normalize_config(c.0))
		.unwrap_or(serde_json::Value::Object(Default::default()));
	let out = indicators_core::calculate_indicator_warmup(indicator_type, params)
		.map_err(|e| err(e.to_string()))?;
	to_py(py, &out)
}
