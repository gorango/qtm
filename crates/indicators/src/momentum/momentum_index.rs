use crate::utils::validation::validate_period;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MomentumIndexConfig {
	pub period: Option<u32>,
}

pub fn momentum_index(prices: &[f64], config: Option<MomentumIndexConfig>) -> Vec<f64> {
	let config_obj = config.unwrap_or(MomentumIndexConfig { period: None });
	let period = config_obj.period.unwrap_or(14) as usize;

	let _ = validate_period(period);

	let len = prices.len();

	if len < period {
		return vec![f64::NAN; len];
	}

	let mut result = vec![f64::NAN; len];

	for i in 0..len {
		if i >= period - 1 {
			result[i] = prices[i] - prices[i - (period - 1)];
		}
	}

	result
}
