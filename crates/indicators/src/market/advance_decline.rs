/// Advance-Decline Line — cumulative `advances - declines` or derived from price.
/// Breadth measure; rising = more advancers. Direct definition.
pub fn advance_decline_line(opens: &[f64], closes: &[f64]) -> Vec<f64> {
	if opens.is_empty() || closes.is_empty() {
		return vec![];
	}

	let mut result = Vec::with_capacity(closes.len());
	let mut cumulative = 0.0;

	for i in 0..closes.len() {
		if i >= opens.len() {
			break;
		}

		let open_val = opens[i];
		let close_val = closes[i];

		if close_val > open_val {
			cumulative += 1.0;
		} else if close_val < open_val {
			cumulative -= 1.0;
		}

		result.push(cumulative);
	}

	result
}
