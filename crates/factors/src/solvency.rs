use crate::types::data::{FactorPoint, FundamentalPoint};

/// Debt-to-Assets ratio: `totalLiabilities / totalAssets`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn debt_to_assets(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let liab = match f.data.total_liabilities {
			Some(v) => v,
			None => continue,
		};
		let assets = match f.data.total_assets {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		results.push(FactorPoint {
			date: f.filing_date,
			value: liab / assets,
		});
	}
	results
}

/// Current Ratio: `currentAssets / currentLiabilities`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn current_ratio(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let ca = match f.data.current_assets {
			Some(v) => v,
			None => continue,
		};
		let cl = match f.data.current_liabilities {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		results.push(FactorPoint {
			date: f.filing_date,
			value: ca / cl,
		});
	}
	results
}

/// Interest Coverage Ratio: `operatingIncome / interestExpense`.
/// Falls back to `netIncome / interestExpense` if operating income unavailable.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn interest_coverage(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let oi = f.data.operating_income.or(f.data.net_income).unwrap_or(0.0);
		if oi == 0.0 {
			continue;
		}
		let interest = match f.data.interest_expense {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		results.push(FactorPoint {
			date: f.filing_date,
			value: oi / interest,
		});
	}
	results
}

/// Tangible Asset Ratio: `propertyPlantEquipment / totalAssets`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn tangible_asset_ratio(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let ppe = match f.data.property_plant_equipment {
			Some(v) => v,
			None => continue,
		};
		let assets = match f.data.total_assets {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		results.push(FactorPoint {
			date: f.filing_date,
			value: ppe / assets,
		});
	}
	results
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::types::data::*;

	fn make_fp(
		symbol: &str,
		date: f64,
		filing: f64,
		period: &str,
		data: FundamentalPointData,
	) -> FundamentalPoint {
		FundamentalPoint {
			symbol: symbol.to_string(),
			date,
			filing_date: filing,
			period: period.to_string(),
			data,
		}
	}

	fn assert_approx_eq(a: f64, b: f64, epsilon: f64) {
		assert!((a - b).abs() < epsilon, "expected {} ≈ {}", a, b);
	}

	#[test]
	fn test_debt_to_assets_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				total_liabilities: Some(400.0),
				total_assets: Some(1000.0),
				..Default::default()
			},
		);
		let result = debt_to_assets(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 0.4, 1e-6);
	}

	#[test]
	fn test_current_ratio_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				current_assets: Some(500.0),
				current_liabilities: Some(200.0),
				..Default::default()
			},
		);
		let result = current_ratio(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 2.5, 1e-6);
	}

	#[test]
	fn test_interest_coverage_happy_with_oi() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				operating_income: Some(300.0),
				interest_expense: Some(50.0),
				..Default::default()
			},
		);
		let result = interest_coverage(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 6.0, 1e-6);
	}

	#[test]
	fn test_interest_coverage_falls_back_to_ni() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				net_income: Some(200.0),
				interest_expense: Some(50.0),
				..Default::default()
			},
		);
		let result = interest_coverage(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 4.0, 1e-6);
	}

	#[test]
	fn test_tangible_asset_ratio_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				property_plant_equipment: Some(300.0),
				total_assets: Some(1000.0),
				..Default::default()
			},
		);
		let result = tangible_asset_ratio(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 0.3, 1e-6);
	}

	#[test]
	fn test_skips_missing_values() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData::default(),
		);
		assert_eq!(debt_to_assets(vec![fp.clone()]).len(), 0);
		assert_eq!(current_ratio(vec![fp.clone()]).len(), 0);
		assert_eq!(interest_coverage(vec![fp.clone()]).len(), 0);
		assert_eq!(tangible_asset_ratio(vec![fp]).len(), 0);
	}

	#[test]
	fn test_skips_zero_assets() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				total_liabilities: Some(100.0),
				total_assets: Some(0.0),
				..Default::default()
			},
		);
		assert_eq!(debt_to_assets(vec![fp]).len(), 0);
	}

	#[test]
	fn test_skips_zero_current_liabilities() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				current_assets: Some(100.0),
				current_liabilities: Some(0.0),
				..Default::default()
			},
		);
		assert_eq!(current_ratio(vec![fp]).len(), 0);
	}

	#[test]
	fn test_empty_fundamentals() {
		assert_eq!(debt_to_assets(vec![]).len(), 0);
		assert_eq!(current_ratio(vec![]).len(), 0);
		assert_eq!(interest_coverage(vec![]).len(), 0);
		assert_eq!(tangible_asset_ratio(vec![]).len(), 0);
	}
}
