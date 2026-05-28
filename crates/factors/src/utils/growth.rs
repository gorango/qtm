use crate::types::data::{FactorPoint, FundamentalPoint};

const YEAR_MS: f64 = 365.25 * 24.0 * 60.0 * 60.0 * 1000.0;
const YEAR_TOLERANCE_MS: f64 = 30.0 * 24.0 * 60.0 * 60.0 * 1000.0;

fn get_field(point: &FundamentalPoint, metric: &str) -> Option<f64> {
	let d = &point.data;
	match metric {
		"revenue" => d.revenue,
		"costAndExpenses" | "cost_and_expenses" => d.cost_and_expenses,
		"eps" => d.eps,
		_ => None,
	}
}

pub fn create_growth_factor(fundamentals: &[FundamentalPoint], metric: &str) -> Vec<FactorPoint> {
	let mut results: Vec<FactorPoint> = Vec::new();

	let mut symbol_groups: std::collections::HashMap<&str, Vec<&FundamentalPoint>> =
		std::collections::HashMap::new();
	for f in fundamentals {
		symbol_groups.entry(&f.symbol).or_default().push(f);
	}

	for group in symbol_groups.values_mut() {
		group.sort_by(|a, b| a.date.partial_cmp(&b.date).unwrap());

		let mut period_groups: std::collections::HashMap<&str, Vec<&&FundamentalPoint>> =
			std::collections::HashMap::new();
		for p in group.iter() {
			period_groups.entry(&p.period).or_default().push(p);
		}

		for points in period_groups.values() {
			let mut sorted = points.clone();
			sorted.sort_by(|a, b| a.date.partial_cmp(&b.date).unwrap());

			for i in 1..sorted.len() {
				let current = sorted[i];
				let previous = sorted[i - 1];

				let diff_ms = current.date - previous.date;
				if !(YEAR_MS - YEAR_TOLERANCE_MS..=YEAR_MS + YEAR_TOLERANCE_MS).contains(&diff_ms) {
					continue;
				}

				let current_value = get_field(current, metric);
				let previous_value = get_field(previous, metric);

				if let (Some(cv), Some(pv)) = (current_value, previous_value) {
					if pv != 0.0 {
						let growth = (cv - pv) / pv.abs();
						results.push(FactorPoint {
							date: current.filing_date,
							value: growth,
						});
					}
				}
			}
		}
	}

	results
}
