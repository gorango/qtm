use std::collections::HashMap;
use std::sync::LazyLock;

pub struct FactorDescriptor {
	pub id: &'static str,
	pub name: &'static str,
	pub category: &'static str,
	pub description: &'static str,
	pub needs_prices: bool,
	/// JSON Schema describing the config parameters for this factor.
	/// Empty string `""` means no config parameters.
	pub params_schema: &'static str,
	/// Output type: "signal" (Vec<i8>), "factor" (Vec<f64>), "indicator" (Vec<f64>)
	pub output_type: &'static str,
}

inventory::collect!(FactorDescriptor);

pub fn get_factor_descriptors() -> Vec<&'static FactorDescriptor> {
	inventory::iter::<FactorDescriptor>.into_iter().collect()
}

pub static FACTOR_REGISTRY: LazyLock<HashMap<&'static str, &'static FactorDescriptor>> =
	LazyLock::new(|| {
		let mut m = HashMap::new();
		for desc in inventory::iter::<FactorDescriptor> {
			m.insert(desc.id, desc);
		}
		m
	});

pub fn get_factor(id: &str) -> Option<&'static FactorDescriptor> {
	FACTOR_REGISTRY.get(id).copied()
}

// ── value ─────────────────────────────────────────

inventory::submit! {
	FactorDescriptor { id: "price_to_earnings", name: "Price to Earnings", category: "value", description: "Price-to-Earnings (P/E) ratio", needs_prices: true, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "price_to_book", name: "Price to Book", category: "value", description: "Price-to-Book ratio: marketCap / shareholdersEquity", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "price_to_sales", name: "Price to Sales", category: "value", description: "Price-to-Sales ratio: marketCap / revenue", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "price_to_free_cash_flow", name: "Price to Free Cash Flow", category: "value", description: "Price-to-Free-Cash-Flow ratio: marketCap / (operatingCashFlow - capitalExpenditure)", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "enterprise_value_to_ebitda", name: "Enterprise Value to EBITDA", category: "value", description: "Enterprise-Value-to-EBITDA ratio: enterpriseValue / ebitda", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "earnings_yield", name: "Earnings Yield", category: "value", description: "Earnings Yield: eps / (marketCap / sharesOutstanding)", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "dividend_yield", name: "Dividend Yield", category: "value", description: "Dividend Yield: dividendsPerShare / price", needs_prices: true, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "free_cash_flow_yield", name: "Free Cash Flow Yield", category: "value", description: "Free Cash Flow Yield: freeCashFlow / marketCap", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "free_cash_flow_margin", name: "Free Cash Flow Margin", category: "value", description: "Free Cash Flow Margin: (operatingCashFlow - capitalExpenditure) / revenue", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "margin_of_safety", name: "Margin of Safety", category: "value", description: "Margin of Safety: dcfValue / price - 1. Positive means undervalued per DCF", needs_prices: true, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "owner_earnings", name: "Owner Earnings", category: "value", description: "Owner Earnings: operatingCashFlow - capitalExpenditure (Buffett's metric)", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "wacc", name: "WACC", category: "value", description: "Weighted Average Cost of Capital (simplified): E/V*0.08 + D/V*0.04*(1-0.21)", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "cash_to_market_cap", name: "Cash to Market Cap", category: "value", description: "Cash-to-Market-Cap ratio: cashAndEquivalents / marketCap", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "market_cap_value", name: "Market Cap Value", category: "value", description: "Extracts raw market capitalization value", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "net_debt_to_ebitda", name: "Net Debt to EBITDA", category: "value", description: "Net Debt to EBITDA: (totalDebt - cashAndEquivalents) / ebitda", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "net_debt_to_ebitdar", name: "Net Debt to EBITDAR", category: "value", description: "Net Debt to EBITDAR: (totalDebt - cash) / (operatingIncome + depreciation)", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "debt_service_coverage_ratio", name: "Debt Service Coverage Ratio", category: "value", description: "Debt Service Coverage Ratio: operatingIncome / interestExpense", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "book_value_per_share", name: "Book Value Per Share", category: "value", description: "Book Value Per Share: shareholdersEquity / sharesOutstanding", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "price_to_book_ratio", name: "Price to Book Ratio", category: "value", description: "Price-to-Book ratio (alternate): marketCap / shareholdersEquity", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "price_to_earnings_ratio", name: "Price to Earnings Ratio", category: "value", description: "Price-to-Earnings ratio (alternate): (marketCap / sharesOutstanding) / eps", needs_prices: false, params_schema: "", output_type: "factor" }
}

// ── quality ───────────────────────────────────────

inventory::submit! {
	FactorDescriptor { id: "return_on_equity", name: "Return on Equity", category: "quality", description: "Return on Equity: netIncome / shareholdersEquity", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "return_on_assets", name: "Return on Assets", category: "quality", description: "Return on Assets: netIncome / totalAssets", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "return_on_invested_capital", name: "Return on Invested Capital", category: "quality", description: "Return on Invested Capital: (netIncome - totalDividends) / (totalLiabilities + shareholdersEquity)", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "gross_margin", name: "Gross Margin", category: "quality", description: "Gross Margin: (revenue - costOfRevenue) / revenue", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "net_margin", name: "Net Profit Margin", category: "quality", description: "Net Profit Margin: netIncome / revenue", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "operating_profit_margin", name: "Operating Profit Margin", category: "quality", description: "Operating Profit Margin: operatingIncome / revenue", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "ebitda_margin", name: "EBITDA Margin", category: "quality", description: "EBITDA Margin: ebitda / revenue. Falls back to operatingIncome / revenue", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "asset_turnover", name: "Asset Turnover", category: "quality", description: "Asset Turnover: revenue / totalAssets", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "working_capital", name: "Working Capital", category: "quality", description: "Working Capital: currentAssets - currentLiabilities", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "working_capital_turnover", name: "Working Capital Turnover", category: "quality", description: "Working Capital Turnover: revenue / (currentAssets - currentLiabilities)", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "quality_of_earnings_index", name: "Quality of Earnings Index", category: "quality", description: "Quality of Earnings Index (0-1): composite of accruals, cash flow consistency, revenue quality, and balance sheet strength", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "retained_earnings", name: "Retained Earnings", category: "quality", description: "Retained Earnings Per Share: retainedEarnings / sharesOutstanding", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "r_and_d_to_revenue", name: "R&D to Revenue", category: "quality", description: "R&D-to-Revenue ratio: researchAndDevelopmentExpenses / revenue", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "historical_volatility_vs_beta", name: "Historical Volatility vs Beta", category: "quality", description: "Placeholder — requires price data not available in current data", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "debt_to_equity", name: "Debt to Equity", category: "quality", description: "Debt-to-Equity ratio: totalLiabilities / shareholdersEquity", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "ebitdar", name: "EBITDAR", category: "quality", description: "EBITDAR: operatingIncome + depreciationAndAmortization (adds back rent proxy)", needs_prices: false, params_schema: "", output_type: "factor" }
}

// ── growth ────────────────────────────────────────

inventory::submit! {
	FactorDescriptor { id: "revenue_growth_yo_y", name: "Revenue Growth YoY", category: "growth", description: "Year-over-Year Revenue Growth: (currentRevenue - previousRevenue) / |previousRevenue|", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "revenue_growth_cagr", name: "Revenue Growth CAGR", category: "growth", description: "Revenue CAGR over a specified number of filings", needs_prices: false, params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"Number of periods for CAGR calculation\",\"default\":5}}}", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "revenue_seasonality", name: "Revenue Seasonality", category: "growth", description: "Revenue Seasonality Index: (maxQuarterRevenue - minQuarterRevenue) / annualAvgRevenue", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "five_y_revenue_growth_per_share", name: "5Y Revenue Growth per Share", category: "growth", description: "5-Year Revenue Per Share Growth: (rpsEnd / rpsStart)^(1/5) - 1", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "epsgrowth", name: "EPS Growth", category: "growth", description: "Year-over-Year EPS Growth", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "eps_growth_qo_q", name: "EPS Growth QoQ", category: "growth", description: "Quarter-over-Quarter EPS Growth", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "eps_growth_10_year", name: "EPS Growth 10 Year", category: "growth", description: "10-Year EPS Growth: point-to-point CAGR", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "eps_growth_cagr", name: "EPS Growth CAGR", category: "growth", description: "EPS CAGR over a specified number of filings", needs_prices: false, params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"Number of periods for CAGR calculation\",\"default\":5}}}", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "eps_avg", name: "EPS Average", category: "growth", description: "Rolling Average EPS over a specified number of filings", needs_prices: false, params_schema: "{\"type\":\"object\",\"properties\":{\"periods\":{\"type\":\"integer\",\"description\":\"Number of filings for rolling average\",\"default\":4}}}", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "eps_positive_count", name: "EPS Positive Count", category: "growth", description: "Count of quarters with positive EPS over a specified number of filings", needs_prices: false, params_schema: "{\"type\":\"object\",\"properties\":{\"periods\":{\"type\":\"integer\",\"description\":\"Number of filings to check for positive EPS\",\"default\":10}}}", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "growth_eps", name: "Growth EPS", category: "growth", description: "Sequential EPS Growth: (currentEPS - previousEPS) / |previousEPS|", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "free_cash_flow_growth", name: "Free Cash Flow Growth", category: "growth", description: "Sequential Free Cash Flow Growth", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "cost_growth_yo_y", name: "Cost Growth YoY", category: "growth", description: "Year-over-Year Cost Growth", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "share_count_growth", name: "Share Count Growth", category: "growth", description: "Share Count CAGR: (sharesLast / sharesFirst)^(1/years) - 1", needs_prices: false, params_schema: "", output_type: "factor" }
}

// ── solvency ──────────────────────────────────────

inventory::submit! {
	FactorDescriptor { id: "debt_to_assets", name: "Debt to Assets", category: "solvency", description: "Debt-to-Assets ratio: totalLiabilities / totalAssets", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "current_ratio", name: "Current Ratio", category: "solvency", description: "Current Ratio: currentAssets / currentLiabilities", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "interest_coverage", name: "Interest Coverage", category: "solvency", description: "Interest Coverage Ratio: operatingIncome / interestExpense. Falls back to netIncome / interestExpense", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "tangible_asset_ratio", name: "Tangible Asset Ratio", category: "solvency", description: "Tangible Asset Ratio: propertyPlantEquipment / totalAssets", needs_prices: false, params_schema: "", output_type: "factor" }
}

// ── shareholder ───────────────────────────────────

inventory::submit! {
	FactorDescriptor { id: "shareholder_yield", name: "Shareholder Yield", category: "shareholder", description: "Shareholder Yield: (dividendsPerShare * sharesOutstanding) / marketCap", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "dividend_payout_ratio", name: "Dividend Payout Ratio", category: "shareholder", description: "Dividend Payout Ratio: dividendsPerShare / eps", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "dividend_coverage_ratio", name: "Dividend Coverage Ratio", category: "shareholder", description: "Dividend Coverage Ratio: netIncome / (dividendsPerShare * sharesOutstanding)", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "dividend_positive_10_years", name: "Dividend Positive 10 Years", category: "shareholder", description: "Dividend Positive for 10 Years (binary): 1 if all 40 trailing quarters had positive dividends", needs_prices: false, params_schema: "", output_type: "factor" }
}

// ── reit ──────────────────────────────────────────

inventory::submit! {
	FactorDescriptor { id: "price_to_affo", name: "Price to AFFO", category: "reit", description: "Price to AFFO (Adjusted Funds From Operations): price / affoPerShare", needs_prices: true, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "reit_dividend_safety", name: "REIT Dividend Safety", category: "reit", description: "REIT Dividend Safety: forwardAnnualDividendRate / (affoPerShare * 4)", needs_prices: false, params_schema: "", output_type: "factor" }
}

// ── composite ─────────────────────────────────────

inventory::submit! {
	FactorDescriptor { id: "altman_z_score", name: "Altman Z-Score", category: "composite", description: "Altman Z-Score (simplified): predicts bankruptcy risk", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "magic_formula", name: "Magic Formula", category: "composite", description: "Greenblatt Magic Formula score: earningsYield + returnOnCapital", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "piotroski_f_score", name: "Piotroski F-Score", category: "composite", description: "Piotroski F-Score (0-6): scores profitability, leverage, and efficiency criteria", needs_prices: false, params_schema: "", output_type: "factor" }
}

// ── event ─────────────────────────────────────────

inventory::submit! {
	FactorDescriptor { id: "earnings_surprise", name: "Earnings Surprise", category: "event", description: "Earnings Surprise: (actualEPS - estimatedEPS) / |estimatedEPS|", needs_prices: false, params_schema: "", output_type: "factor" }
}

// ── expectations ──────────────────────────────────

inventory::submit! {
	FactorDescriptor { id: "analyst_rating_momentum", name: "Analyst Rating Momentum", category: "expectations", description: "Analyst Rating Momentum: currentRating - avgPastRating over period filings", needs_prices: false, params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"Number of days for rating momentum window\",\"default\":90}}}", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "analyst_target_upside", name: "Analyst Target Upside", category: "expectations", description: "Analyst Target Upside: (targetPrice - price) / price", needs_prices: true, params_schema: "", output_type: "factor" }
}

// ── onchain ───────────────────────────────────────

inventory::submit! {
	FactorDescriptor { id: "active_address_growth", name: "Active Address Growth", category: "onchain", description: "Active Address Growth over period days. Measures blockchain network adoption growth", needs_prices: false, params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"number\",\"description\":\"Number of days for growth comparison\",\"default\":30}}}", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "exchange_flow_momentum", name: "Exchange Flow Momentum", category: "onchain", description: "Exchange Flow Momentum. Positive = inflow (accumulation), negative = outflow (distribution)", needs_prices: false, params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"number\",\"description\":\"Number of days for flow momentum window\",\"default\":30}}}", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "nvt_ratio", name: "NVT Ratio", category: "onchain", description: "Network Value to Transactions (NVT) Ratio: marketCap / transactionVolume", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "staking_ratio", name: "Staking Ratio", category: "onchain", description: "Staking Ratio: stakedSupply / totalSupply. Measures network security and participant commitment", needs_prices: false, params_schema: "", output_type: "factor" }
}

// ── prediction ────────────────────────────────────

inventory::submit! {
	FactorDescriptor { id: "prediction_market_odds", name: "Prediction Market Odds", category: "prediction", description: "Prediction Market Odds: extracts implied probability price from each prediction market point", needs_prices: false, params_schema: "", output_type: "factor" }
}
inventory::submit! {
	FactorDescriptor { id: "odds_momentum", name: "Odds Momentum", category: "prediction", description: "Prediction Market Odds Momentum: (currentPrice - priceNPeriodsBack) / priceNPeriodsBack", needs_prices: false, params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"Number of periods back for odds change\",\"default\":1}}}", output_type: "factor" }
}
