use crate::types::data::{FactorPoint, FundamentalPoint};

/// Altman Z-Score (simplified): predicts bankruptcy risk.
/// Z = 1.2*(0.1) + 1.4*(0.2) + 3.3*(NI/TA) + 0.6*(MktCap/Liab) + 0.999*(Sales/TA).
/// >3 safe, 1.8-3 grey, <1.8 distress.
pub fn altman_z_score(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let ta = match f.data.total_assets {
			Some(v) if v > 0.0 && v.is_finite() => v,
			_ => continue,
		};

		let mut z = 0.0;

		if let Some(ni) = f.data.net_income {
			if ni.is_finite() {
				z += 3.3 * (ni / ta);
			}
		}

		if let Some(revenue) = f.data.revenue {
			if revenue.is_finite() {
				z += 0.999 * (revenue / ta);
			}
		}

		if let (Some(mcap), Some(liab)) = (f.data.market_cap, f.data.total_liabilities) {
			if mcap.is_finite() && liab.is_finite() && liab > 0.0 {
				z += 0.6 * (mcap / liab);
			}
		}

		z += 1.2 * 0.1;
		z += 1.4 * 0.2;

		if !z.is_finite() {
			continue;
		}

		results.push(FactorPoint {
			date: f.date,
			value: z,
		});
	}
	results
}

/// Greenblatt Magic Formula score: `earningsYield + returnOnCapital`.
/// Higher scores indicate more attractive value+quality combination.
pub fn magic_formula(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let mut earnings_yield = 0.0;
		if let (Some(eps), Some(mcap), Some(shares)) =
			(f.data.eps, f.data.market_cap, f.data.shares_outstanding)
		{
			if shares > 0.0 {
				let price = mcap / shares;
				if price > 0.0 {
					earnings_yield = eps / price;
				}
			}
		}

		let mut roc = 0.0;
		if let (Some(ni), Some(ta), Some(tl)) = (
			f.data.net_income,
			f.data.total_assets,
			f.data.total_liabilities,
		) {
			if ta > 0.0 {
				let invested = ta - tl * 0.5;
				if invested > 0.0 {
					roc = ni / invested;
				}
			}
		}

		let score = earnings_yield + roc;
		if score > 0.0 {
			results.push(FactorPoint {
				date: f.date,
				value: score,
			});
		}
	}
	results
}

/// Piotroski F-Score (0-6): scores profitability, leverage, and efficiency criteria
/// by comparing current vs prior period for each symbol.
pub fn piotroski_f_score(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();

	let mut symbol_groups: std::collections::HashMap<String, Vec<&FundamentalPoint>> =
		std::collections::HashMap::new();
	for f in &fundamentals {
		symbol_groups.entry(f.symbol.clone()).or_default().push(f);
	}

	for group in symbol_groups.values_mut() {
		group.sort_by(|a, b| a.date.partial_cmp(&b.date).unwrap());
		for i in 1..group.len() {
			let cur = group[i];
			let prev = group[i - 1];
			let mut score: i32 = 0;

			if cur.data.net_income.is_some_and(|v| v > 0.0) {
				score += 1;
			}

			if cur.data.operating_cash_flow.is_some_and(|v| v > 0.0) {
				score += 1;
			}

			let cur_roa = match (cur.data.net_income, cur.data.total_assets) {
				(Some(ni), Some(ta)) if ta > 0.0 => ni / ta,
				_ => 0.0,
			};
			let prev_roa = match (prev.data.net_income, prev.data.total_assets) {
				(Some(ni), Some(ta)) if ta > 0.0 => ni / ta,
				_ => 0.0,
			};
			if cur_roa > prev_roa {
				score += 1;
			}

			let cur_ocf = cur.data.operating_cash_flow.unwrap_or(0.0);
			let cur_ni = cur.data.net_income.unwrap_or(0.0);
			if cur_ocf > cur_ni {
				score += 1;
			}

			let cur_dr = match (cur.data.total_liabilities, cur.data.total_assets) {
				(Some(tl), Some(ta)) if ta > 0.0 => tl / ta,
				_ => 0.0,
			};
			let prev_dr = match (prev.data.total_liabilities, prev.data.total_assets) {
				(Some(tl), Some(ta)) if ta > 0.0 => tl / ta,
				_ => 0.0,
			};
			if cur_dr < prev_dr {
				score += 1;
			}

			let cur_margin = match (cur.data.net_income, cur.data.revenue) {
				(Some(ni), Some(rev)) if rev > 0.0 => ni / rev,
				_ => 0.0,
			};
			let prev_margin = match (prev.data.net_income, prev.data.revenue) {
				(Some(ni), Some(rev)) if rev > 0.0 => ni / rev,
				_ => 0.0,
			};
			if cur_margin > prev_margin {
				score += 1;
			}

			results.push(FactorPoint {
				date: cur.date,
				value: score as f64,
			});
		}
	}
	results
}
