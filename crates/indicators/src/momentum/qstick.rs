use crate::internal::sma::sma_internal;
use crate::utils::arrays::validate_arrays_equal_length;
use crate::utils::validation::validate_period;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QstickConfig {
	pub period: Option<u32>,
}

pub fn qstick(opens: &[f64], closes: &[f64], config: Option<QstickConfig>) -> Vec<f64> {
	let config_obj = config.unwrap_or(QstickConfig { period: None });
	let period = config_obj.period.unwrap_or(14) as usize;

	let _ = validate_period(period);
	let _ = validate_arrays_equal_length(&[opens, closes]);

	let len = opens.len();

	let mut result = vec![f64::NAN; len];

	for i in 0..len {
		result[i] = closes[i] - opens[i];
	}

	sma_internal(&result, period)
}
