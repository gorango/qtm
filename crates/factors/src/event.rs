use crate::types::data::{EarningsReportPoint, FactorPoint};

/// Earnings Surprise: `(actualEPS - estimatedEPS) / |estimatedEPS|`.
pub fn earnings_surprise(reports: Vec<EarningsReportPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for r in &reports {
		let est = r.eps_estimated;
		if est.abs() > 0.0 {
			let surprise = (r.eps_actual - est) / est.abs();
			results.push(FactorPoint {
				date: r.date,
				value: surprise,
			});
		}
	}
	results
}
