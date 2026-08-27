use crate::internal::ema::ema_internal;
use crate::{IndicatorError, IndicatorResult};

/// McClellan Oscillator — `EMA19(advances-declines) - EMA39(advances-declines)` normalized.
/// Breadth momentum; positive = bullish breadth. Defined by Sherman & Marian McClellan.
pub fn mcclellan_oscillator(advances: &[f64], declines: &[f64]) -> IndicatorResult<Vec<f64>> {
	if advances.is_empty() || declines.is_empty() {
		return Ok(vec![]);
	}

	if advances.len() != declines.len() {
		return Err(IndicatorError::Custom(
			"Advances and declines arrays must be equal length".into(),
		));
	}

	if advances.len() < 39 {
		return Err(IndicatorError::Custom(
			"Advances and declines arrays must have at least 39 data points".into(),
		));
	}

	crate::utils::validation::validate_finite(&[advances, declines])?;

	let len = advances.len();
	let mut ad_diff = vec![0.0; len];

	for i in 0..len {
		ad_diff[i] = advances[i] - declines[i];
	}

	let ema19 = ema_internal(&ad_diff, 19);
	let ema39 = ema_internal(&ad_diff, 39);

	let mut result = vec![f64::NAN; len];

	for i in 38..len {
		result[i] = ema19[i] - ema39[i];
	}

	Ok(result)
}
