pub fn tr_internal(highs: &[f64], lows: &[f64], closes: &[f64]) -> Vec<f64> {
	let len = highs.len();
	if len == 0 {
		return vec![];
	}

	let mut result = vec![0.0; len];

	for i in 0..len {
		let tr1 = highs[i] - lows[i];
		let tr2 = if i > 0 {
			(highs[i] - closes[i - 1]).max(0.0)
		} else {
			0.0
		};
		let tr3 = if i > 0 {
			(closes[i - 1] - lows[i]).max(0.0)
		} else {
			0.0
		};

		if i == 0 {
			result[i] = highs[i] - lows[i];
		} else {
			result[i] = tr1.max(tr2).max(tr3);
		}
	}

	result
}
