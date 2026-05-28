#[cfg(feature = "napi")]
use napi_derive::napi;

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FundamentalPointData {
	pub revenue: Option<f64>,
	pub cost_of_revenue: Option<f64>,
	pub gross_profit: Option<f64>,
	pub net_income: Option<f64>,
	pub eps: Option<f64>,
	pub total_assets: Option<f64>,
	pub current_assets: Option<f64>,
	pub current_liabilities: Option<f64>,
	pub total_liabilities: Option<f64>,
	pub total_debt: Option<f64>,
	pub shareholders_equity: Option<f64>,
	pub retained_earnings: Option<f64>,
	pub operating_income: Option<f64>,
	pub cost_and_expenses: Option<f64>,
	pub research_and_development_expenses: Option<f64>,
	pub operating_cash_flow: Option<f64>,
	pub capital_expenditure: Option<f64>,
	pub interest_expense: Option<f64>,
	pub shares_outstanding: Option<f64>,
	pub common_stock: Option<f64>,
	pub market_cap: Option<f64>,
	pub enterprise_value: Option<f64>,
	pub ebitda: Option<f64>,
	pub dividends_per_share: Option<f64>,
	pub dividends_paid: Option<f64>,
	pub share_repurchases: Option<f64>,
	pub cash_and_equivalents: Option<f64>,
	pub property_plant_equipment: Option<f64>,
	pub analyst_target_price: Option<f64>,
	pub rating: Option<f64>,
	pub asset_turnover: Option<f64>,
	pub dcf: Option<f64>,
	pub ffo_per_share: Option<f64>,
	pub affo_per_share: Option<f64>,
	pub payout_ratio_ffo: Option<f64>,
	pub forward_annual_dividend_rate: Option<f64>,
	pub dividend_yield: Option<f64>,
	pub dividend_growth_3y: Option<f64>,
	pub dividend_growth_5y: Option<f64>,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct FundamentalPoint {
	pub symbol: String,
	pub date: f64,
	pub filing_date: f64,
	pub period: String,
	pub data: FundamentalPointData,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct Bar {
	pub time: f64,
	pub open: f64,
	pub high: f64,
	pub low: f64,
	pub close: f64,
	pub volume: f64,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct OnChainDataPoint {
	pub time: f64,
	pub metric: String,
	pub value: f64,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionMarketPoint {
	pub time: f64,
	pub market_id: String,
	pub price: f64,
	pub volume: f64,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct EarningsReportPoint {
	pub date: f64,
	pub symbol: String,
	pub eps_actual: f64,
	pub eps_estimated: f64,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct FactorPoint {
	pub date: f64,
	pub value: f64,
}
