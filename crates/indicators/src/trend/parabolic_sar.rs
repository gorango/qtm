use crate::types::trend::Trend;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct PSARResult {
	pub trends: Vec<i32>,
	pub psar_result: Vec<f64>,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct PSARConfig {
	pub step: Option<f64>,
	pub max: Option<f64>,
}

pub fn parabolic_sar(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	config: Option<PSARConfig>,
) -> Result<PSARResult, String> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closings])?;

	let config = config.unwrap_or(PSARConfig {
		step: Some(0.02),
		max: Some(0.2),
	});

	let step = config.step.unwrap_or(0.02);
	let max = config.max.unwrap_or(0.2);

	let len = highs.len();
	let mut trends = vec![Trend::Falling as i32; len];
	let mut psar_result = vec![0.0; len];

	if len == 0 {
		return Ok(PSARResult {
			trends,
			psar_result,
		});
	}

	trends[0] = Trend::Falling as i32;
	psar_result[0] = highs[0];

	let mut af = step;
	let mut ep = lows[0];

	for i in 1..len {
		psar_result[i] = psar_result[i - 1] - (psar_result[i - 1] - ep) * af;

		if trends[i - 1] == Trend::Falling as i32 {
			psar_result[i] = psar_result[i].max(highs[i - 1]);
			if i > 1 {
				psar_result[i] = psar_result[i].max(highs[i - 2]);
			}

			if highs[i] >= psar_result[i] {
				psar_result[i] = ep;
			}
		} else {
			psar_result[i] = psar_result[i].min(lows[i - 1]);
			if i > 1 {
				psar_result[i] = psar_result[i].min(lows[i - 2]);
			}

			if lows[i] <= psar_result[i] {
				psar_result[i] = ep;
			}
		}

		let prev_ep = ep;

		if psar_result[i] > closings[i] {
			trends[i] = Trend::Falling as i32;
			ep = ep.min(lows[i]);
		} else {
			trends[i] = Trend::Rising as i32;
			ep = ep.max(highs[i]);
		}

		if trends[i] != trends[i - 1] {
			af = step;
		} else if prev_ep != ep && af < max {
			af += step;
		}
	}

	Ok(PSARResult {
		trends,
		psar_result,
	})
}
