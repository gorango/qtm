use pyo3::prelude::*;

use crate::convert::{deserialize_cfg, err, f64_out, i8_out, json_to_py, normalize_config, records};
use crate::convert::{Json, PyObject};
use crate::validation::validate_non_empty;
use factors_core::{Bar, FactorPoint, FundamentalPoint, OnChainDataPoint};
use strategies_core::*;

type PyResultO = PyResult<PyObject>;

// ── Hand-written dialect strategy wrappers ────────────────────
// Shapes mirror bindings/js: factors | points | points+prices | on-chain |
// prediction | portfolio. Configs accept dicts (snake_case keys are normalized
// to the serde camelCase field names).

/// shape: `factors: Vec<FactorPoint>, config`
macro_rules! strat_factors {
	($($name:ident: $cfg:ty),* $(,)?) => {
		$(
			#[pyfunction]
			#[pyo3(signature = (factors, config = None))]
			pub fn $name(
				py: Python<'_>,
				factors: Vec<Json>,
				config: Option<Json>,
			) -> PyResultO {
				let factors: Vec<FactorPoint> = records(factors, "factors")?;
				validate_non_empty(&factors, "factors")?;
				let cfg = deserialize_cfg::<$cfg>(config.map(|c| normalize_config(c.0)))?;
				let out = strategies_core::$name(factors, cfg);
				Ok(i8_out(py, &out))
			}
		)*
	};
}

/// shape: `points: Vec<FundamentalPoint>, config`
macro_rules! strat_points {
	($($name:ident: $cfg:ty),* $(,)?) => {
		$(
			#[pyfunction]
			#[pyo3(signature = (points, config = None))]
			pub fn $name(
				py: Python<'_>,
				points: Vec<Json>,
				config: Option<Json>,
			) -> PyResultO {
				let points: Vec<FundamentalPoint> = records(points, "points")?;
				validate_non_empty(&points, "points")?;
				let cfg = deserialize_cfg::<$cfg>(config.map(|c| normalize_config(c.0)))?;
				let out = strategies_core::$name(points, cfg);
				Ok(i8_out(py, &out))
			}
		)*
	};
}

/// shape: `points: Vec<FundamentalPoint>, prices: Vec<Bar>, config`
macro_rules! strat_points_prices {
	($($name:ident: $cfg:ty),* $(,)?) => {
		$(
			#[pyfunction]
			#[pyo3(signature = (points, prices, config = None))]
			pub fn $name(
				py: Python<'_>,
				points: Vec<Json>,
				prices: Vec<Json>,
				config: Option<Json>,
			) -> PyResultO {
				let points: Vec<FundamentalPoint> = records(points, "points")?;
				let prices: Vec<Bar> = records(prices, "prices")?;
				validate_non_empty(&points, "points")?;
				validate_non_empty(&prices, "prices")?;
				let cfg = deserialize_cfg::<$cfg>(config.map(|c| normalize_config(c.0)))?;
				let out = strategies_core::$name(points, prices, cfg);
				Ok(i8_out(py, &out))
			}
		)*
	};
}

strat_factors!(
	value_strategy: ValueConfig,
	classic_value_strategy: ClassicValueConfig,
	quality_strategy: QualityConfig,
	peg_strategy: PegConfig,
	altman_z_score_strategy: AltmanZScoreConfig,
	piotroski_strategy: PiotroskiConfig,
);

strat_points!(
	benjamin_graham_strategy: ValueChecklistConfig,
	bill_miller_strategy: ValueChecklistConfig,
	john_templeton_strategy: ValueChecklistConfig,
	walter_schloss_strategy: ValueChecklistConfig,
	free_cash_flow_analysis_strategy: FreeCashFlowAnalysisConfig,
	wacc_vs_roic_spread_strategy: WaccVsRoicSpreadConfig,
	ev_ebitda_fair_value_strategy: EvEbitdaFairValueConfig,
	cash_burn_runway_strategy: CashBurnRunwayConfig,
	debt_ebitdar_stress_test_strategy: DebtEbitdarStressTestConfig,
	ev_fcf_10yr_band_strategy: EvFcf10yrBandConfig,
	ev_revenue_multiples_strategy: EvRevenueMultiplesConfig,
	ev_sales_fair_value_strategy: EvSalesFairValueConfig,
	interest_coverage_buffer_strategy: InterestCoverageBufferConfig,
	net_cash_position_toggle_strategy: NetCashPositionToggleConfig,
	normal_pe_future_fair_value_strategy: NormalPeFutureFairValueConfig,
	ocf_coverage_dividends_strategy: OcfCoverageDividendsConfig,
	price_sales_fair_value_strategy: PriceSalesFairValueConfig,
	price_to_owner_earnings_strategy: PriceToOwnerEarningsConfig,
	quick_ratio_stress_test_strategy: QuickRatioStressTestConfig,
	return_of_capital_vs_growth_strategy: ReturnOfCapitalVsGrowthConfig,
	working_capital_health_strategy: WorkingCapitalHealthConfig,
	capex_discipline_strategy: CapexDisciplineConfig,
	cash_conversion_cycle_analysis_strategy: CccAnalysisConfig,
	cash_conversion_cycle_check_strategy: CccCheckConfig,
	charlie_munger_strategy: QualityChecklistConfig,
	philip_fisher_strategy: QualityChecklistConfig,
	dupont_roe_strategy: DupontRoeConfig,
	return_on_capital_strategy: ReturnOnCapitalConfig,
	operating_margin_strategy: MarginChecklistConfig,
	earnings_quality_analysis_strategy: EarningsQualityConfig,
	ebitda_margin_strategy: EbitdaMarginConfig,
	gross_profit_analysis_strategy: GrossProfitConfig,
	operating_cashflow_analysis_strategy: OcfAnalysisConfig,
	operating_leverage_trend_strategy: OperatingLeverageConfig,
	five_year_margin_expansion_strategy: MarginExpansionConfig,
	ebitda_growth_vs_competition_strategy: EbitdaGrowthVsCompetitionConfig,
	eps_vs_fcf_divergence_strategy: EpsVsFcfDivergenceConfig,
	expense_surprise_detector_strategy: ExpenseSurpriseConfig,
	revenue_assets_efficiency_strategy: RevenueAssetsEfficiencyConfig,
	revenue_diversification_proxy_strategy: RevenueDiversificationConfig,
	revenue_per_employee_strategy: RevenuePerEmployeeConfig,
	rnd_intensity_tracker_strategy: RndIntensityConfig,
	roic_durability_sweep_strategy: RoicDurabilityConfig,
	working_capital_efficiency_strategy: WorkingCapitalEfficiencyConfig,
	earnings_growth_vs_competition_strategy: GrowthVsCompetitionConfig,
	revenue_growth_analysis_strategy: RevenueGrowthAnalysisConfig,
	sustainable_growth_rate_strategy: SustainableGrowthRateConfig,
	earnings_reinvestment_rate_strategy: EarningsReinvestmentRateConfig,
	top_quartile_strategy: TopQuartileConfig,
	qoq_revenue_momentum_strategy: QoqRevenueMomentumConfig,
	revenue_growth_vs_competitors_strategy: RevenueGrowthVsCompetitorsConfig,
	revenue_growth_vs_competition_strategy: GrowthVsCompetitionConfig,
	revenue_volatility_score_strategy: RevenueVolatilityScoreConfig,
	seasonality_index_revenue_strategy: SeasonalityIndexRevenueConfig,
	management_earnings_call_tone_analysis_strategy: MgmtEarningsCallToneConfig,
	earnings_call_revenue_analysis_strategy: EarningsCallRevenueConfig,
	high_yield_reit_strategy: HighYieldReitConfig,
	dividend_strategy: DividendConfig,
	dividend_growth_consistency_strategy: DividendGrowthConsistencyConfig,
	solvency_strategy: SolvencyConfig,
	magic_formula_strategy: MagicFormulaConfig,
	joel_greenblatt_strategy: JoelGreenblattConfig,
	growth_investing_suite_strategy: SuiteConfig,
	quality_investing_suite_strategy: SuiteConfig,
	value_investing_suite_strategy: SuiteConfig,
	multi_factor_suite_strategy: MultiFactorSuiteConfig,
);

strat_points_prices!(
	qarp_strategy: QarpConfig,
	composite_value_momentum_strategy: CompositeValueMomentumConfig,
	quantamental_value_momentum_strategy: QuantamentalValueMomentumConfig,
	growth_quality_strategy: GrowthQualityConfig,
	value_momentum_pattern_strategy: ValueMomentumPatternConfig,
	multi_factor_value_strategy: MultiFactorValueConfig,
	intrinsic_value_multi_metric_strategy: IntrinsicValueMultiMetricConfig,
	margin_of_safety_target_price_strategy: MarginOfSafetyTargetPriceConfig,
);

#[pyfunction]
#[pyo3(signature = (on_chain_data, prediction_data, config = None))]
pub fn alternative_data_strategy(
	py: Python<'_>,
	on_chain_data: Vec<Json>,
	prediction_data: Vec<Json>,
	config: Option<Json>,
) -> PyResultO {
	let on_chain_data: Vec<OnChainDataPoint> = records(on_chain_data, "on_chain_data")?;
	let prediction_data: Vec<factors_core::PredictionMarketPoint> =
		records(prediction_data, "prediction_data")?;
	validate_non_empty(&on_chain_data, "on_chain_data")?;
	validate_non_empty(&prediction_data, "prediction_data")?;
	let cfg = deserialize_cfg::<AlternativeDataConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = strategies_core::alternative_data_strategy(on_chain_data, prediction_data, cfg);
	Ok(i8_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (prediction_data, prices, config = None))]
pub fn event_driven_strategy(
	py: Python<'_>,
	prediction_data: Vec<Json>,
	prices: Vec<Json>,
	config: Option<Json>,
) -> PyResultO {
	let prediction_data: Vec<factors_core::PredictionMarketPoint> =
		records(prediction_data, "prediction_data")?;
	let prices: Vec<Bar> = records(prices, "prices")?;
	validate_non_empty(&prediction_data, "prediction_data")?;
	validate_non_empty(&prices, "prices")?;
	let cfg = deserialize_cfg::<EventDrivenConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = strategies_core::event_driven_strategy(prediction_data, prices, cfg);
	Ok(i8_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (on_chain_data, prices, config = None))]
pub fn on_chain_confirmation_strategy(
	py: Python<'_>,
	on_chain_data: Vec<Json>,
	prices: Vec<Json>,
	config: Option<Json>,
) -> PyResultO {
	let on_chain_data: Vec<OnChainDataPoint> = records(on_chain_data, "on_chain_data")?;
	let prices: Vec<Bar> = records(prices, "prices")?;
	validate_non_empty(&on_chain_data, "on_chain_data")?;
	validate_non_empty(&prices, "prices")?;
	let cfg = deserialize_cfg::<OnChainConfirmationConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = strategies_core::on_chain_confirmation_strategy(on_chain_data, prices, cfg);
	Ok(i8_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (value_factors, quality_factors, momentum_factors, prices, config = None))]
pub fn multi_factor_strategy(
	py: Python<'_>,
	value_factors: Vec<Json>,
	quality_factors: Vec<Json>,
	momentum_factors: Vec<Json>,
	prices: Vec<Json>,
	config: Option<Json>,
) -> PyResultO {
	let value_factors: Vec<FactorPoint> = records(value_factors, "value_factors")?;
	let quality_factors: Vec<FactorPoint> = records(quality_factors, "quality_factors")?;
	let momentum_factors: Vec<FactorPoint> = records(momentum_factors, "momentum_factors")?;
	let prices: Vec<Bar> = records(prices, "prices")?;
	validate_non_empty(&value_factors, "value_factors")?;
	validate_non_empty(&quality_factors, "quality_factors")?;
	validate_non_empty(&momentum_factors, "momentum_factors")?;
	validate_non_empty(&prices, "prices")?;
	let cfg = deserialize_cfg::<MultiFactorConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = strategies_core::multi_factor_strategy(value_factors, quality_factors, momentum_factors, prices, cfg);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (closes, config = None))]
pub fn risk_parity_strategy(
	py: Python<'_>,
	closes: crate::convert::F64Arr2<'_>,
	config: Option<Json>,
) -> PyResultO {
	let closes = crate::convert::f64_matrix(&closes, "closes")?;
	validate_non_empty(&closes, "closes")?;
	let cfg = deserialize_cfg::<RiskParityConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = strategies_core::risk_parity_strategy(closes, cfg);
	Ok(f64_out(py, &out))
}

#[pyfunction]
#[pyo3(signature = (closes, config = None))]
pub fn dual_momentum_strategy(
	py: Python<'_>,
	closes: crate::convert::F64Arr2<'_>,
	config: Option<Json>,
) -> PyResultO {
	let closes = crate::convert::f64_matrix(&closes, "closes")?;
	validate_non_empty(&closes, "closes")?;
	let cfg = deserialize_cfg::<DualMomentumConfig>(config.map(|c| normalize_config(c.0)))?;
	let out = strategies_core::dual_momentum_strategy(closes, cfg);
	Ok(f64_out(py, &out))
}

// ── Metadata / defaults lookup ────────────────────────────────
// strategies_core exposes `{name}_strategy_metadata()` / `{name}_strategy_defaults()`
// for the hand-written dialect. Python calls `strategy_metadata("value_strategy")`.

macro_rules! meta_lookup {
	($func:ident, $suffix:literal, $($name:ident),* $(,)?) => {
		#[pyfunction]
		pub fn $func(py: Python<'_>, name: &str) -> PyResultO {
			let entries: Vec<(&str, fn() -> serde_json::Value)> = vec![
				$((stringify!($name).trim_end_matches($suffix), strategies_core::$name),)*
			];
			let Some((_, f)) = entries.into_iter().find(|(n, _)| *n == name) else {
				return Err(err(format!("Unknown strategy: {name}")));
			};
			let v = f();
			json_to_py(py, &v)
		}
	};
}

meta_lookup!(
	strategy_metadata,
	"_metadata",
	alternative_data_strategy_metadata,
	altman_z_score_strategy_metadata,
	benjamin_graham_strategy_metadata,
	bill_miller_strategy_metadata,
	capex_discipline_strategy_metadata,
	cash_burn_runway_strategy_metadata,
	cash_conversion_cycle_analysis_strategy_metadata,
	cash_conversion_cycle_check_strategy_metadata,
	charlie_munger_strategy_metadata,
	classic_value_strategy_metadata,
	composite_value_momentum_strategy_metadata,
	debt_ebitdar_stress_test_strategy_metadata,
	dividend_growth_consistency_strategy_metadata,
	dividend_strategy_metadata,
	dual_momentum_strategy_metadata,
	dupont_roe_strategy_metadata,
	earnings_call_revenue_analysis_strategy_metadata,
	earnings_growth_vs_competition_strategy_metadata,
	earnings_quality_analysis_strategy_metadata,
	earnings_reinvestment_rate_strategy_metadata,
	ebitda_growth_vs_competition_strategy_metadata,
	ebitda_margin_strategy_metadata,
	eps_vs_fcf_divergence_strategy_metadata,
	ev_ebitda_fair_value_strategy_metadata,
	event_driven_strategy_metadata,
	ev_fcf_10yr_band_strategy_metadata,
	ev_revenue_multiples_strategy_metadata,
	ev_sales_fair_value_strategy_metadata,
	expense_surprise_detector_strategy_metadata,
	five_year_margin_expansion_strategy_metadata,
	free_cash_flow_analysis_strategy_metadata,
	gross_profit_analysis_strategy_metadata,
	growth_investing_suite_strategy_metadata,
	growth_quality_strategy_metadata,
	high_yield_reit_strategy_metadata,
	interest_coverage_buffer_strategy_metadata,
	intrinsic_value_multi_metric_strategy_metadata,
	joel_greenblatt_strategy_metadata,
	john_templeton_strategy_metadata,
	magic_formula_strategy_metadata,
	management_earnings_call_tone_analysis_strategy_metadata,
	margin_of_safety_target_price_strategy_metadata,
	multi_factor_strategy_metadata,
	multi_factor_suite_strategy_metadata,
	multi_factor_value_strategy_metadata,
	net_cash_position_toggle_strategy_metadata,
	normal_pe_future_fair_value_strategy_metadata,
	ocf_coverage_dividends_strategy_metadata,
	on_chain_confirmation_strategy_metadata,
	operating_cashflow_analysis_strategy_metadata,
	operating_leverage_trend_strategy_metadata,
	operating_margin_strategy_metadata,
	peg_strategy_metadata,
	philip_fisher_strategy_metadata,
	piotroski_strategy_metadata,
	price_sales_fair_value_strategy_metadata,
	price_to_owner_earnings_strategy_metadata,
	qarp_strategy_metadata,
	qoq_revenue_momentum_strategy_metadata,
	quality_investing_suite_strategy_metadata,
	quality_strategy_metadata,
	quantamental_value_momentum_strategy_metadata,
	quick_ratio_stress_test_strategy_metadata,
	return_of_capital_vs_growth_strategy_metadata,
	return_on_capital_strategy_metadata,
	revenue_assets_efficiency_strategy_metadata,
	revenue_diversification_proxy_strategy_metadata,
	revenue_growth_analysis_strategy_metadata,
	revenue_growth_vs_competition_strategy_metadata,
	revenue_growth_vs_competitors_strategy_metadata,
	revenue_per_employee_strategy_metadata,
	revenue_volatility_score_strategy_metadata,
	risk_parity_strategy_metadata,
	rnd_intensity_tracker_strategy_metadata,
	roic_durability_sweep_strategy_metadata,
	seasonality_index_revenue_strategy_metadata,
	solvency_strategy_metadata,
	sustainable_growth_rate_strategy_metadata,
	top_quartile_strategy_metadata,
	value_investing_suite_strategy_metadata,
	value_momentum_pattern_strategy_metadata,
	value_strategy_metadata,
	wacc_vs_roic_spread_strategy_metadata,
	walter_schloss_strategy_metadata,
	working_capital_efficiency_strategy_metadata,
	working_capital_health_strategy_metadata,
);

meta_lookup!(
	strategy_defaults,
	"_defaults",
	alternative_data_strategy_defaults,
	altman_z_score_strategy_defaults,
	benjamin_graham_strategy_defaults,
	bill_miller_strategy_defaults,
	capex_discipline_strategy_defaults,
	cash_burn_runway_strategy_defaults,
	cash_conversion_cycle_analysis_strategy_defaults,
	cash_conversion_cycle_check_strategy_defaults,
	charlie_munger_strategy_defaults,
	classic_value_strategy_defaults,
	composite_value_momentum_strategy_defaults,
	debt_ebitdar_stress_test_strategy_defaults,
	dividend_growth_consistency_strategy_defaults,
	dividend_strategy_defaults,
	dual_momentum_strategy_defaults,
	dupont_roe_strategy_defaults,
	earnings_call_revenue_analysis_strategy_defaults,
	earnings_growth_vs_competition_strategy_defaults,
	earnings_quality_analysis_strategy_defaults,
	earnings_reinvestment_rate_strategy_defaults,
	ebitda_growth_vs_competition_strategy_defaults,
	ebitda_margin_strategy_defaults,
	eps_vs_fcf_divergence_strategy_defaults,
	ev_ebitda_fair_value_strategy_defaults,
	event_driven_strategy_defaults,
	ev_fcf_10yr_band_strategy_defaults,
	ev_revenue_multiples_strategy_defaults,
	ev_sales_fair_value_strategy_defaults,
	expense_surprise_detector_strategy_defaults,
	five_year_margin_expansion_strategy_defaults,
	free_cash_flow_analysis_strategy_defaults,
	gross_profit_analysis_strategy_defaults,
	growth_investing_suite_strategy_defaults,
	growth_quality_strategy_defaults,
	high_yield_reit_strategy_defaults,
	interest_coverage_buffer_strategy_defaults,
	intrinsic_value_multi_metric_strategy_defaults,
	joel_greenblatt_strategy_defaults,
	john_templeton_strategy_defaults,
	magic_formula_strategy_defaults,
	management_earnings_call_tone_analysis_strategy_defaults,
	margin_of_safety_target_price_strategy_defaults,
	multi_factor_strategy_defaults,
	multi_factor_suite_strategy_defaults,
	multi_factor_value_strategy_defaults,
	net_cash_position_toggle_strategy_defaults,
	normal_pe_future_fair_value_strategy_defaults,
	ocf_coverage_dividends_strategy_defaults,
	on_chain_confirmation_strategy_defaults,
	operating_cashflow_analysis_strategy_defaults,
	operating_leverage_trend_strategy_defaults,
	operating_margin_strategy_defaults,
	peg_strategy_defaults,
	philip_fisher_strategy_defaults,
	piotroski_strategy_defaults,
	price_sales_fair_value_strategy_defaults,
	price_to_owner_earnings_strategy_defaults,
	qarp_strategy_defaults,
	qoq_revenue_momentum_strategy_defaults,
	quality_investing_suite_strategy_defaults,
	quality_strategy_defaults,
	quantamental_value_momentum_strategy_defaults,
	quick_ratio_stress_test_strategy_defaults,
	return_of_capital_vs_growth_strategy_defaults,
	return_on_capital_strategy_defaults,
	revenue_assets_efficiency_strategy_defaults,
	revenue_diversification_proxy_strategy_defaults,
	revenue_growth_analysis_strategy_defaults,
	revenue_growth_vs_competition_strategy_defaults,
	revenue_growth_vs_competitors_strategy_defaults,
	revenue_per_employee_strategy_defaults,
	revenue_volatility_score_strategy_defaults,
	risk_parity_strategy_defaults,
	rnd_intensity_tracker_strategy_defaults,
	roic_durability_sweep_strategy_defaults,
	seasonality_index_revenue_strategy_defaults,
	solvency_strategy_defaults,
	sustainable_growth_rate_strategy_defaults,
	top_quartile_strategy_defaults,
	value_investing_suite_strategy_defaults,
	value_momentum_pattern_strategy_defaults,
	value_strategy_defaults,
	wacc_vs_roic_spread_strategy_defaults,
	walter_schloss_strategy_defaults,
	working_capital_efficiency_strategy_defaults,
	working_capital_health_strategy_defaults,
);
