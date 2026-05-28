#[allow(dead_code)]
pub fn sum(values: &[f64]) -> f64 {
	values.iter().sum()
}

#[allow(dead_code)]
pub fn mean(values: &[f64]) -> f64 {
	if values.is_empty() {
		return f64::NAN;
	}
	sum(values) / values.len() as f64
}

#[allow(dead_code)]
pub fn max(values: &[f64]) -> f64 {
	values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
}

#[allow(dead_code)]
pub fn min(values: &[f64]) -> f64 {
	values.iter().fold(f64::INFINITY, |a, &b| a.min(b))
}

#[allow(dead_code)]
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
	value.min(max).max(min)
}
