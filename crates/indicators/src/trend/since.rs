pub fn since_internal(values: &[f64]) -> Vec<f64> {
	let len = values.len();
	let mut result = vec![0.0; len];

	if len > 0 {
		let mut last: Option<f64> = None;
		let mut count = 0.0;

		for i in 0..len {
			let current = values[i];

			if last != Some(current) {
				last = Some(current);
				count = 0.0;
			} else {
				count += 1.0;
			}

			result[i] = count;
		}
	}

	result
}

pub fn since(values: &[f64]) -> Result<Vec<f64>, String> {
	Ok(since_internal(values))
}
