use factors_core;
use napi_derive::napi;

#[napi]
pub fn revenue_growth_yo_y(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::revenue_growth_yo_y(fundamentals)
}

#[napi]
pub fn revenue_growth_cagr(
	fundamentals: Vec<factors_core::FundamentalPoint>,
	period: Option<u32>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::revenue_growth_cagr(fundamentals, period)
}

#[napi]
pub fn revenue_seasonality(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::revenue_seasonality(fundamentals)
}

#[napi]
pub fn five_y_revenue_growth_per_share(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::five_y_revenue_growth_per_share(fundamentals)
}

#[napi]
pub fn epsgrowth(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::epsgrowth(fundamentals)
}

#[napi]
pub fn eps_growth_qo_q(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::eps_growth_qo_q(fundamentals)
}

#[napi]
pub fn eps_growth_10_year(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::eps_growth_10_year(fundamentals)
}

#[napi]
pub fn eps_growth_cagr(
	fundamentals: Vec<factors_core::FundamentalPoint>,
	period: Option<u32>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::eps_growth_cagr(fundamentals, period)
}

#[napi]
pub fn eps_avg(
	fundamentals: Vec<factors_core::FundamentalPoint>,
	periods: Option<u32>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::eps_avg(fundamentals, periods)
}

#[napi]
pub fn eps_positive_count(
	fundamentals: Vec<factors_core::FundamentalPoint>,
	periods: Option<u32>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::eps_positive_count(fundamentals, periods)
}

#[napi]
pub fn growth_eps(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::growth_eps(fundamentals)
}

#[napi]
pub fn free_cash_flow_growth(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::free_cash_flow_growth(fundamentals)
}

#[napi]
pub fn cost_growth_yo_y(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::cost_growth_yo_y(fundamentals)
}

#[napi]
pub fn share_count_growth(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::share_count_growth(fundamentals)
}
