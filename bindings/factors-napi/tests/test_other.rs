use factors::{
	altman_z_score, current_ratio, debt_to_assets, dividend_coverage_ratio, dividend_payout_ratio,
	dividend_positive_10_years, earnings_surprise, eps_growth_qo_q, eps_positive_count,
	magic_formula, piotroski_f_score, price_to_affo, revenue_growth_cagr, shareholder_yield,
	tangible_asset_ratio, EarningsReportPoint, FundamentalPoint, FundamentalPointData,
};

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

fn make_bar_affo(
	symbol: &str,
	date: f64,
	filing: f64,
	period: &str,
	affo: f64,
	price: f64,
) -> (FundamentalPoint, factors::Bar) {
	(
		make_fp(
			symbol,
			date,
			filing,
			period,
			FundamentalPointData {
				affo_per_share: Some(affo),
				..Default::default()
			},
		),
		factors::Bar {
			time: filing,
			open: price,
			high: price,
			low: price,
			close: price,
			volume: 1000.0,
		},
	)
}

// --- Composite ---

#[test]
fn test_altman_z_score_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			total_assets: Some(1000.0),
			net_income: Some(100.0),
			revenue: Some(500.0),
			market_cap: Some(800.0),
			total_liabilities: Some(400.0),
			..Default::default()
		},
	);
	let result = altman_z_score(vec![fp]);
	assert_eq!(result.len(), 1);
	// z = 3.3*0.1 + 0.999*0.5 + 0.6*2.0 + 1.2*0.1 + 1.4*0.2
	// = 0.33 + 0.4995 + 1.2 + 0.12 + 0.28 = 2.4295
	assert!((result[0].value - 2.4295).abs() < 1e-4);
}

#[test]
fn test_altman_z_score_no_assets() {
	let fp = make_fp("AAPL", 100.0, 100.0, "FY", FundamentalPointData::default());
	let result = altman_z_score(vec![fp]);
	assert!(result.is_empty());
}

#[test]
fn test_magic_formula_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			eps: Some(5.0),
			market_cap: Some(1000.0),
			shares_outstanding: Some(100.0),
			net_income: Some(100.0),
			total_assets: Some(2000.0),
			total_liabilities: Some(1000.0),
			..Default::default()
		},
	);
	let result = magic_formula(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!(result[0].value > 0.0);
}

#[test]
fn test_magic_formula_no_profitable() {
	let fp = make_fp("AAPL", 100.0, 100.0, "FY", FundamentalPointData::default());
	let result = magic_formula(vec![fp]);
	assert!(result.is_empty());
}

#[test]
fn test_piotroski_f_score_happy_path() {
	let prev = make_fp(
		"AAPL",
		0.0,
		0.0,
		"FY",
		FundamentalPointData {
			net_income: Some(50.0),
			operating_cash_flow: Some(60.0),
			total_assets: Some(500.0),
			total_liabilities: Some(300.0),
			revenue: Some(400.0),
			..Default::default()
		},
	);
	let cur = make_fp(
		"AAPL",
		365.0,
		365.0,
		"FY",
		FundamentalPointData {
			net_income: Some(80.0),
			operating_cash_flow: Some(100.0),
			total_assets: Some(600.0),
			total_liabilities: Some(300.0),
			revenue: Some(500.0),
			..Default::default()
		},
	);
	let result = piotroski_f_score(vec![prev, cur]);
	assert_eq!(result.len(), 1);
	// Positive NI +1, Positive OCF +1, ROA up +1, OCF > NI +1, leverage down +1, margin up +1 = 6
	assert!((result[0].value - 6.0).abs() < 1e-6);
}

#[test]
fn test_piotroski_f_score_single_point() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			net_income: Some(80.0),
			..Default::default()
		},
	);
	let result = piotroski_f_score(vec![fp]);
	assert!(result.is_empty());
}

// --- Solvency ---

#[test]
fn test_debt_to_assets_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			total_liabilities: Some(300.0),
			total_assets: Some(1000.0),
			..Default::default()
		},
	);
	let result = debt_to_assets(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.3).abs() < 1e-6);
}

#[test]
fn test_current_ratio_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			current_assets: Some(500.0),
			current_liabilities: Some(250.0),
			..Default::default()
		},
	);
	let result = current_ratio(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 2.0).abs() < 1e-6);
}

#[test]
fn test_tangible_asset_ratio_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			property_plant_equipment: Some(400.0),
			total_assets: Some(1000.0),
			..Default::default()
		},
	);
	let result = tangible_asset_ratio(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.4).abs() < 1e-6);
}

// --- Shareholder ---

#[test]
fn test_shareholder_yield_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			dividends_per_share: Some(2.0),
			market_cap: Some(5000.0),
			shares_outstanding: Some(100.0),
			..Default::default()
		},
	);
	let result = shareholder_yield(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.04).abs() < 1e-6);
}

#[test]
fn test_dividend_payout_ratio_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			dividends_per_share: Some(1.0),
			eps: Some(4.0),
			..Default::default()
		},
	);
	let result = dividend_payout_ratio(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.25).abs() < 1e-6);
}

#[test]
fn test_dividend_coverage_ratio_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			net_income: Some(500.0),
			dividends_per_share: Some(2.0),
			shares_outstanding: Some(100.0),
			..Default::default()
		},
	);
	let result = dividend_coverage_ratio(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 2.5).abs() < 1e-6);
}

#[test]
fn test_dividend_positive_10_years_all_positive() {
	let mut fps = Vec::new();
	for i in 0..45 {
		fps.push(make_fp(
			"AAPL",
			i as f64 * 90.0,
			i as f64 * 90.0,
			"Q1",
			FundamentalPointData {
				dividends_per_share: Some(0.5),
				..Default::default()
			},
		));
	}
	let result = dividend_positive_10_years(fps);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 1.0).abs() < 1e-6);
}

#[test]
fn test_dividend_positive_10_years_some_zero() {
	let mut fps = Vec::new();
	for i in 0..45 {
		let div = if i < 30 { Some(0.5) } else { Some(0.0) };
		fps.push(make_fp(
			"AAPL",
			i as f64 * 90.0,
			i as f64 * 90.0,
			"Q1",
			FundamentalPointData {
				dividends_per_share: div,
				..Default::default()
			},
		));
	}
	let result = dividend_positive_10_years(fps);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.0).abs() < 1e-6);
}

// --- REIT ---

#[test]
fn test_price_to_affo_happy_path() {
	let (fp, bar) = make_bar_affo("O", 100.0, 100.0, "Q1", 2.0, 80.0);
	let result = price_to_affo(vec![fp], vec![bar]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 40.0).abs() < 1e-6);
}

// --- Event ---

#[test]
fn test_earnings_surprise_happy_path() {
	let report = EarningsReportPoint {
		date: 100.0,
		symbol: "AAPL".to_string(),
		eps_actual: 1.50,
		eps_estimated: 1.20,
	};
	let result = earnings_surprise(vec![report]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.25).abs() < 1e-6);
}

#[test]
fn test_earnings_surprise_negative_estimate() {
	let report = EarningsReportPoint {
		date: 100.0,
		symbol: "AAPL".to_string(),
		eps_actual: -0.80,
		eps_estimated: -1.00,
	};
	let result = earnings_surprise(vec![report]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.20).abs() < 1e-6);
}

#[test]
fn test_earnings_surprise_zero_estimate() {
	let report = EarningsReportPoint {
		date: 100.0,
		symbol: "AAPL".to_string(),
		eps_actual: 1.0,
		eps_estimated: 0.0,
	};
	let result = earnings_surprise(vec![report]);
	assert!(result.is_empty());
}

#[test]
fn test_growth_factors_empty() {
	let result = revenue_growth_cagr(vec![], None);
	assert!(result.is_empty());

	let result = eps_growth_qo_q(vec![]);
	assert!(result.is_empty());

	let result = eps_positive_count(vec![], None);
	assert!(result.is_empty());
}
