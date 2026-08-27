use crate::utils::validation;
use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PriceRateOfChangeConfig {
	pub period: Option<u32>,
}

/// Price Rate of Change — `100*(close - close[n])/close[n]` over `period` bars.
/// Percentage change; 0 = no change. Period defaults to 12. `NaN` for first `period` bars.
///
/// # Errors
/// Returns an error if `period` is 0 or inputs invalid.
pub fn price_rate_of_change(
	values: &[f64],
	config: Option<PriceRateOfChangeConfig>,
) -> IndicatorResult<Vec<f64>> {
	let config_obj = config.unwrap_or(PriceRateOfChangeConfig { period: None });
	let period = config_obj.period.unwrap_or(3) as usize;

	validation::validate_period(period)?;

	let len = values.len();

	let mut result = vec![0.0; len];

	for i in 0..len {
		if i < period {
			result[i] = 0.0;
		} else {
			result[i] = (values[i] / values[i - period] - 1.0) * 100.0;
		}
	}

	Ok(result)
}
