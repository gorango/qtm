use factors_core;
use napi_derive::napi;

#[napi]
pub fn return_on_equity(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::return_on_equity(fundamentals)
}

#[napi]
pub fn return_on_assets(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::return_on_assets(fundamentals)
}

#[napi]
pub fn return_on_invested_capital(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::return_on_invested_capital(fundamentals)
}

#[napi]
pub fn gross_margin(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::gross_margin(fundamentals)
}

#[napi]
pub fn net_margin(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::net_margin(fundamentals)
}

#[napi]
pub fn operating_profit_margin(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::operating_profit_margin(fundamentals)
}

#[napi]
pub fn ebitda_margin(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::ebitda_margin(fundamentals)
}

#[napi]
pub fn asset_turnover(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::asset_turnover(fundamentals)
}

#[napi]
pub fn working_capital(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::working_capital(fundamentals)
}

#[napi]
pub fn working_capital_turnover(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::working_capital_turnover(fundamentals)
}

#[napi]
pub fn quality_of_earnings_index(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::quality_of_earnings_index(fundamentals)
}

#[napi]
pub fn retained_earnings(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::retained_earnings(fundamentals)
}

#[napi]
pub fn r_and_d_to_revenue(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::r_and_d_to_revenue(fundamentals)
}

#[napi]
pub fn historical_volatility_vs_beta(
	_fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::historical_volatility_vs_beta(_fundamentals)
}

#[napi]
pub fn debt_to_equity(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::debt_to_equity(fundamentals)
}

#[napi]
pub fn ebitdar(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::ebitdar(fundamentals)
}
