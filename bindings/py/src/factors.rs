use pyo3::prelude::*;

use crate::convert::{records, to_py};
use crate::convert::{Json, PyObject};
use crate::validation::validate_non_empty;
use factors_core::{Bar, EarningsReportPoint, FundamentalPoint, OnChainDataPoint};

type PyResultO = PyResult<PyObject>;

/// Factors computed from a series of `FundamentalPoint` dicts.
macro_rules! factors_fund {
	($($name:ident),* $(,)?) => {
		$(
			#[pyfunction]
			pub fn $name(py: Python<'_>, fundamentals: Vec<Json>) -> PyResultO {
				let fundamentals: Vec<FundamentalPoint> = records(fundamentals, "fundamentals")?;
				validate_non_empty(&fundamentals, "fundamentals")?;
				let out = factors_core::$name(fundamentals);
				to_py(py, &out)
			}
		)*
	};
}

factors_fund!(
	altman_z_score,
	asset_turnover,
	book_value_per_share,
	cash_to_market_cap,
	cost_growth_yo_y,
	current_ratio,
	debt_service_coverage_ratio,
	debt_to_assets,
	debt_to_equity,
	dividend_coverage_ratio,
	dividend_payout_ratio,
	dividend_positive_10_years,
	earnings_yield,
	ebitda_margin,
	ebitdar,
	enterprise_value_to_ebitda,
	epsgrowth,
	eps_growth_10_year,
	eps_growth_qo_q,
	five_y_revenue_growth_per_share,
	free_cash_flow_growth,
	free_cash_flow_margin,
	free_cash_flow_yield,
	gross_margin,
	growth_eps,
	historical_volatility_vs_beta,
	interest_coverage,
	magic_formula,
	market_cap_value,
	net_debt_to_ebitda,
	net_debt_to_ebitdar,
	net_margin,
	operating_profit_margin,
	owner_earnings,
	piotroski_f_score,
	price_to_book,
	price_to_book_ratio,
	price_to_earnings_ratio,
	price_to_free_cash_flow,
	price_to_sales,
	quality_of_earnings_index,
	r_and_d_to_revenue,
	reit_dividend_safety,
	retained_earnings,
	return_on_assets,
	return_on_equity,
	return_on_invested_capital,
	revenue_growth_yo_y,
	revenue_seasonality,
	share_count_growth,
	shareholder_yield,
	tangible_asset_ratio,
	wacc,
	working_capital,
	working_capital_turnover,
);

/// Factors requiring prices alongside fundamentals.
macro_rules! factors_fund_price {
	($($name:ident),* $(,)?) => {
		$(
			#[pyfunction]
			pub fn $name(
				py: Python<'_>,
				fundamentals: Vec<Json>,
				prices: Vec<Json>,
			) -> PyResultO {
				let fundamentals: Vec<FundamentalPoint> = records(fundamentals, "fundamentals")?;
				let prices: Vec<Bar> = records(prices, "prices")?;
				validate_non_empty(&fundamentals, "fundamentals")?;
				validate_non_empty(&prices, "prices")?;
				let out = factors_core::$name(fundamentals, prices);
				to_py(py, &out)
			}
		)*
	};
}

factors_fund_price!(
	dividend_yield,
	margin_of_safety,
	price_to_affo,
	analyst_target_upside,
	price_to_earnings,
);

/// On-chain factors.
macro_rules! factors_onchain {
	($($name:ident),* $(,)?) => {
		$(
			#[pyfunction]
			pub fn $name(py: Python<'_>, on_chain_data: Vec<Json>) -> PyResultO {
				let on_chain_data: Vec<OnChainDataPoint> = records(on_chain_data, "on_chain_data")?;
				validate_non_empty(&on_chain_data, "on_chain_data")?;
				let out = factors_core::$name(on_chain_data);
				to_py(py, &out)
			}
		)*
	};
}

factors_onchain!(nvt_ratio, staking_ratio);

/// Prediction-market factors.
macro_rules! factors_prediction {
	($($name:ident),* $(,)?) => {
		$(
			#[pyfunction]
			pub fn $name(py: Python<'_>, prediction_data: Vec<Json>) -> PyResultO {
				let prediction_data: Vec<factors_core::PredictionMarketPoint> =
					records(prediction_data, "prediction_data")?;
				validate_non_empty(&prediction_data, "prediction_data")?;
				let out = factors_core::$name(prediction_data);
				to_py(py, &out)
			}
		)*
	};
}

factors_prediction!(prediction_market_odds);

#[pyfunction]
pub fn earnings_surprise(py: Python<'_>, reports: Vec<Json>) -> PyResultO {
	let reports: Vec<EarningsReportPoint> = records(reports, "reports")?;
	validate_non_empty(&reports, "reports")?;
	let out = factors_core::earnings_surprise(reports);
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (fundamentals, periods = None))]
pub fn eps_avg(py: Python<'_>, fundamentals: Vec<Json>, periods: Option<u32>) -> PyResultO {
	let fundamentals: Vec<FundamentalPoint> = records(fundamentals, "fundamentals")?;
	validate_non_empty(&fundamentals, "fundamentals")?;
	let out = factors_core::eps_avg(fundamentals, periods);
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (fundamentals, period = None))]
pub fn eps_growth_cagr(py: Python<'_>, fundamentals: Vec<Json>, period: Option<u32>) -> PyResultO {
	let fundamentals: Vec<FundamentalPoint> = records(fundamentals, "fundamentals")?;
	validate_non_empty(&fundamentals, "fundamentals")?;
	let out = factors_core::eps_growth_cagr(fundamentals, period);
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (fundamentals, periods = None))]
pub fn eps_positive_count(
	py: Python<'_>,
	fundamentals: Vec<Json>,
	periods: Option<u32>,
) -> PyResultO {
	let fundamentals: Vec<FundamentalPoint> = records(fundamentals, "fundamentals")?;
	validate_non_empty(&fundamentals, "fundamentals")?;
	let out = factors_core::eps_positive_count(fundamentals, periods);
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (on_chain_data, period = None))]
pub fn active_address_growth(
	py: Python<'_>,
	on_chain_data: Vec<Json>,
	period: Option<f64>,
) -> PyResultO {
	let on_chain_data: Vec<OnChainDataPoint> = records(on_chain_data, "on_chain_data")?;
	validate_non_empty(&on_chain_data, "on_chain_data")?;
	let out = factors_core::active_address_growth(on_chain_data, period);
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (on_chain_data, period = None))]
pub fn exchange_flow_momentum(
	py: Python<'_>,
	on_chain_data: Vec<Json>,
	period: Option<f64>,
) -> PyResultO {
	let on_chain_data: Vec<OnChainDataPoint> = records(on_chain_data, "on_chain_data")?;
	validate_non_empty(&on_chain_data, "on_chain_data")?;
	let out = factors_core::exchange_flow_momentum(on_chain_data, period);
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (prediction_data, period = None))]
pub fn odds_momentum(py: Python<'_>, prediction_data: Vec<Json>, period: Option<u32>) -> PyResultO {
	let prediction_data: Vec<factors_core::PredictionMarketPoint> =
		records(prediction_data, "prediction_data")?;
	validate_non_empty(&prediction_data, "prediction_data")?;
	let out = factors_core::odds_momentum(prediction_data, period);
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (fundamentals, period = None))]
pub fn analyst_rating_momentum(
	py: Python<'_>,
	fundamentals: Vec<Json>,
	period: Option<u32>,
) -> PyResultO {
	let fundamentals: Vec<FundamentalPoint> = records(fundamentals, "fundamentals")?;
	validate_non_empty(&fundamentals, "fundamentals")?;
	let out = factors_core::analyst_rating_momentum(fundamentals, period);
	to_py(py, &out)
}

#[pyfunction]
#[pyo3(signature = (fundamentals, period = None))]
pub fn revenue_growth_cagr(
	py: Python<'_>,
	fundamentals: Vec<Json>,
	period: Option<u32>,
) -> PyResultO {
	let fundamentals: Vec<FundamentalPoint> = records(fundamentals, "fundamentals")?;
	validate_non_empty(&fundamentals, "fundamentals")?;
	let out = factors_core::revenue_growth_cagr(fundamentals, period);
	to_py(py, &out)
}
