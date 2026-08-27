use crate::trend::rma::rma_internal;
use crate::{IndicatorError, IndicatorResult};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct ADXResult {
	/// +DI — positive directional index (0..100).
	pub plus_di: Vec<f64>,
	/// -DI — negative directional index (0..100).
	pub minus_di: Vec<f64>,
	/// ADX — average directional index, smoothed DX (0..100). Higher = stronger trend.
	pub adx: Vec<f64>,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ADXConfig {
	/// Smoothing period for directional movement (default 14). Valid 2..=100.
	pub period: Option<u32>,
}

/// Average Directional Index (ADX) with +DI and -DI.
///
/// Wilder's definition: TR/DM via RMA, DI = 100*DM/ATR, DX = 100*|+DI - -DI|/(+DI+ -DI), ADX = RMA(DX).
/// Range 0..100; >25 often taken as trending. `adx` here also returns the two DIs.
///
/// # Errors
/// Returns an error if `period` is 0 or inputs are mismatched/too short.
pub fn adx(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	config: Option<ADXConfig>,
) -> IndicatorResult<ADXResult> {
	let config = config.unwrap_or(ADXConfig { period: Some(14) });
	let period = config.period.unwrap_or(14) as usize;

	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closings])?;
	crate::utils::validation::validate_period(period)?;

	if highs.len() < period + 1 {
		return Err(IndicatorError::Custom(format!(
			"Not enough data points. Need at least {}, got {}",
			period + 1,
			highs.len()
		)));
	}

	let len = highs.len();
	let mut plus_di = vec![0.0; len];
	let mut minus_di = vec![0.0; len];
	let mut adx_result = vec![0.0; len];

	let mut tr = vec![0.0; len];
	let mut plus_dm = vec![0.0; len];
	let mut minus_dm = vec![0.0; len];

	for i in 0..len {
		if i == 0 {
			tr[i] = highs[i] - lows[i];
		} else {
			let tr1 = highs[i] - lows[i];
			let tr2 = (highs[i] - closings[i - 1]).abs();
			let tr3 = (lows[i] - closings[i - 1]).abs();
			tr[i] = tr1.max(tr2).max(tr3);
		}

		if i == 0 {
			plus_dm[i] = 0.0;
			minus_dm[i] = 0.0;
		} else {
			let up_move = highs[i] - highs[i - 1];
			let down_move = lows[i - 1] - lows[i];

			if up_move > down_move && up_move > 0.0 {
				plus_dm[i] = up_move;
				minus_dm[i] = 0.0;
			} else if down_move > up_move && down_move > 0.0 {
				plus_dm[i] = 0.0;
				minus_dm[i] = down_move;
			} else {
				plus_dm[i] = 0.0;
				minus_dm[i] = 0.0;
			}
		}
	}

	let atr = rma_internal(&tr, period);
	let adx_plus_dm = rma_internal(&plus_dm, period);
	let adx_minus_dm = rma_internal(&minus_dm, period);

	for i in 0..len {
		if atr[i] > 0.0 {
			plus_di[i] = (adx_plus_dm[i] / atr[i]) * 100.0;
			minus_di[i] = (adx_minus_dm[i] / atr[i]) * 100.0;
		}
	}

	let mut dx = vec![0.0; len];
	for i in 0..len {
		if plus_di[i] + minus_di[i] > 0.0 {
			dx[i] = ((plus_di[i] - minus_di[i]).abs() / (plus_di[i] + minus_di[i])) * 100.0;
		}
	}

	let adx_values = rma_internal(&dx, period);
	adx_result[..len].copy_from_slice(&adx_values[..len]);

	Ok(ADXResult {
		plus_di,
		minus_di,
		adx: adx_result,
	})
}
