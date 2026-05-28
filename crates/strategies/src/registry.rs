use std::collections::HashMap;
use std::sync::OnceLock;

use crate::StrategyResult;
use crate::*;

#[derive(Clone)]
pub struct StrategyInput {
	pub opens: Option<Vec<f64>>,
	pub highs: Option<Vec<f64>>,
	pub lows: Option<Vec<f64>>,
	pub closes: Vec<f64>,
	pub volumes: Option<Vec<f64>>,
	pub timestamps: Option<Vec<f64>>,
}

#[macro_export]
macro_rules! register_strategy {
	($registry:expr, $id:expr, $function:expr) => {
		$registry.insert(
			$id.to_string(),
			Box::new($function)
				as Box<
					dyn Fn(&StrategyInput, Option<serde_json::Value>) -> StrategyResult<Vec<i8>>
						+ Send
						+ Sync,
				>,
		);
	};
}

pub type StrategyRegistryImpl = HashMap<
	String,
	Box<dyn Fn(&StrategyInput, Option<serde_json::Value>) -> StrategyResult<Vec<i8>> + Send + Sync>,
>;

static STRATEGY_REGISTRY: OnceLock<StrategyRegistryImpl> = OnceLock::new();

pub fn get_strategy_registry_impl() -> &'static StrategyRegistryImpl {
	STRATEGY_REGISTRY.get_or_init(|| {
		let mut registry = HashMap::new();

		// Register strategies using original bindings IDs
		register_strategy!(registry, "rsi", rsi);
		register_strategy!(registry, "macd", macd);
		register_strategy!(registry, "awesomeOscillator", awesome_oscillator);
		register_strategy!(registry, "cci", cci);
		register_strategy!(registry, "roc", roc);
		register_strategy!(registry, "rsi2", rsi2);
		register_strategy!(registry, "stochastic", stochastic);
		register_strategy!(registry, "ultimateOscillator", ultimate_oscillator);
		register_strategy!(registry, "williamsR", williams_r);
		register_strategy!(registry, "dmi", dmi);
		register_strategy!(
			registry,
			"absolutePriceOscillator",
			absolute_price_oscillator
		);
		register_strategy!(registry, "adx", adx);
		register_strategy!(registry, "almaCrossover", alma_crossover);
		register_strategy!(registry, "almaHmaDivergence", alma_hma_divergence);
		register_strategy!(registry, "aroon", aroon);
		register_strategy!(registry, "balanceOfPower", balance_of_power);
		register_strategy!(
			registry,
			"chandeForecastOscillator",
			chande_forecast_oscillator
		);
		register_strategy!(registry, "kdj", kdj);
		register_strategy!(registry, "linRegChannel", lin_reg_channel);
		register_strategy!(registry, "linRegSlope", lin_reg_slope);
		register_strategy!(registry, "larsson", larsson);
		register_strategy!(registry, "maCrossover", ma_crossover);
		register_strategy!(registry, "macdCrossover", macd_crossover);
		register_strategy!(registry, "parabolicSar", parabolic_sar);
		register_strategy!(registry, "pivotPoints", pivot_points);
		register_strategy!(registry, "smaVwapCrossover", sma_vwap_crossover);
		register_strategy!(registry, "superTrend", super_trend);
		register_strategy!(registry, "typicalPrice", typical_price);
		register_strategy!(registry, "vortex", vortex);
		register_strategy!(registry, "vwma", vwma);
		register_strategy!(registry, "wmaConfirmation", wma_confirmation);
		register_strategy!(registry, "wmaMomentum", wma_momentum);
		register_strategy!(registry, "obv_rsi", obv_rsi);
		register_strategy!(registry, "adx_rsi", adx_rsi);
		register_strategy!(registry, "bb_rsi", bb_rsi);
		register_strategy!(registry, "double_top_stochastic", double_top_stochastic);
		register_strategy!(registry, "fibonacciRetracement", fibonacci_retracement);
		register_strategy!(registry, "flag_pennant_macd", flag_pennant_macd);
		register_strategy!(registry, "ma_rsi", ma_rsi);
		register_strategy!(registry, "macd_rsi", macd_rsi);
		register_strategy!(registry, "macd_stochastic", macd_stochastic);
		register_strategy!(registry, "mfi_obv", mfi_obv);
		register_strategy!(registry, "roc_obv_rsi", roc_obv_rsi);
		register_strategy!(registry, "rsi_macd", rsi_macd);
		register_strategy!(registry, "triangle_rsi", triangle_rsi);
		register_strategy!(registry, "volume_profile_rsi", volume_profile_rsi);
		register_strategy!(registry, "vwap_ema_rsi", vwap_ema_rsi);
		register_strategy!(registry, "vwap_macd", vwap_macd);
		register_strategy!(registry, "vwap_rsi", vwap_rsi);
		register_strategy!(registry, "vwap_stochastic", vwap_stochastic);
		register_strategy!(registry, "elliott_wave", elliott_wave);
		register_strategy!(registry, "buyAndHold", buy_and_hold);
		register_strategy!(registry, "triangle", triangle);
		register_strategy!(registry, "cup_and_handle", cup_and_handle);
		register_strategy!(registry, "double_top_bottom", double_top_bottom);
		register_strategy!(registry, "flags_pennants", flags_pennants);
		register_strategy!(registry, "hmaTrend", hma_trend);
		register_strategy!(registry, "head_and_shoulders", head_and_shoulders);
		register_strategy!(registry, "wedge", wedge);
		register_strategy!(registry, "ichimoku", ichimoku);
		register_strategy!(registry, "kst", kst);
		register_strategy!(registry, "momentum", momentum);
		register_strategy!(registry, "cointegration-pair-trading", cointegration);
		register_strategy!(registry, "correlation-pair-trading", correlation_pair);
		register_strategy!(
			registry,
			"correlation-mean-reversion",
			correlation_reversion
		);
		register_strategy!(registry, "percentRank-ranking", percent_rank);
		register_strategy!(registry, "accelerationBands", acceleration_bands);
		register_strategy!(registry, "atrThreshold", atr_threshold);
		register_strategy!(registry, "atrVolatilityThreshold", atr_volatility_threshold);
		register_strategy!(registry, "bollingerBandsBreakout", bollinger_bands_breakout);
		register_strategy!(
			registry,
			"bollingerBandsMeanReversion",
			bollinger_bands_mean_reversion
		);
		register_strategy!(registry, "donchianBreakout", donchian_breakout);
		register_strategy!(registry, "donchianReversion", donchian_reversion);
		register_strategy!(registry, "keltnerChannelBreakout", keltner_channel_breakout);
		register_strategy!(
			registry,
			"keltnerChannelReversion",
			keltner_channel_reversion
		);
		register_strategy!(
			registry,
			"keltnerVolatilityBreakout",
			keltner_volatility_breakout
		);
		register_strategy!(registry, "madReversion", mad_reversion);
		register_strategy!(registry, "openingRangeBreakout", opening_range_breakout);
		register_strategy!(registry, "pairsTrading", pairs_trading);
		register_strategy!(registry, "projectionOscillator", projection_oscillator);
		register_strategy!(registry, "standardDeviation", standard_deviation);
		register_strategy!(registry, "varianceStop", variance_stop);
		register_strategy!(registry, "volatilityAdjusted", volatility_adjusted);
		register_strategy!(registry, "zScoreBreakout", z_score_breakout);
		register_strategy!(registry, "zScoreReversion", z_score_reversion);
		register_strategy!(
			registry,
			"accumulation-distribution",
			accumulation_distribution
		);
		register_strategy!(registry, "chaikin-money-flow", chaikin_money_flow);
		register_strategy!(registry, "ease-of-movement", ease_of_movement);
		register_strategy!(registry, "force-index", force_index);
		register_strategy!(registry, "money-flow-index", money_flow_index);
		register_strategy!(registry, "negative-volume-index", negative_volume_index);
		register_strategy!(registry, "obv", obv);
		register_strategy!(registry, "obv-confirmation", obv_confirmation);
		register_strategy!(registry, "volume-price-trend", volume_price_trend);
		register_strategy!(
			registry,
			"volume-weighted-average-price",
			volume_weighted_average_price
		);
		register_strategy!(registry, "vwap-breakout", vwap_breakout);
		register_strategy!(registry, "vwap-reversion", vwap_reversion);
		registry
	})
}

// ── Wrapper functions ──────────

pub fn buy_and_hold(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<BuyAndHoldConfig>(c).unwrap_or_default());
	crate::buy_and_hold_strategy(&input.closes, config)
}

pub fn cup_and_handle(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<CupAndHandleConfig>(c).unwrap_or_default());
	crate::cup_and_handle_strategy(
		input.opens.as_ref().unwrap_or(&input.closes),
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn double_top_bottom(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<DoubleTopBottomConfig>(c).unwrap_or_default());
	crate::double_top_bottom_strategy(
		input.opens.as_ref().unwrap_or(&input.closes),
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn wedge(input: &StrategyInput, config: Option<serde_json::Value>) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<WedgeConfig>(c).unwrap_or_default());
	crate::wedge_strategy(
		input.opens.as_ref().unwrap_or(&input.closes),
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn head_and_shoulders(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<HeadAndShouldersConfig>(c).unwrap_or_default());
	crate::head_and_shoulders_strategy(
		input.opens.as_ref().unwrap_or(&input.closes),
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn triangle(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<TriangleConfig>(c).unwrap_or_default());
	crate::triangle_strategy(
		input.opens.as_ref().unwrap_or(&input.closes),
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn elliott_wave(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let _ = config;
	let opens = input.opens.as_ref().unwrap_or(&input.closes);
	let highs = input.highs.as_ref().unwrap_or(&input.closes);
	let lows = input.lows.as_ref().unwrap_or(&input.closes);
	crate::elliott_wave_strategy(
		opens,
		highs,
		lows,
		&input.closes,
		0.5,
		0.38,
		1.618,
		5,
		2,
		0.05,
	)
}

pub fn flags_pennants(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<FlagsPennantsConfig>(c).unwrap_or_default());
	crate::flags_pennants_strategy(
		input.opens.as_ref().unwrap_or(&input.closes),
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn bb_rsi(input: &StrategyInput, config: Option<serde_json::Value>) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<BbRsiConfig>(c).unwrap_or_default());
	crate::bb_rsi_strategy(&input.closes, config)
}

pub fn ma_rsi(input: &StrategyInput, config: Option<serde_json::Value>) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<MaRsiConfig>(c).unwrap_or_default());
	crate::ma_rsi_strategy(&input.closes, config)
}

pub fn obv_rsi(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<RSIConfig>(c).unwrap_or_default());
	crate::obv_rsi_strategy(
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn macd_stochastic(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<MacdStochasticConfig>(c).unwrap_or_default());
	crate::macd_stochastic_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn volume_profile_rsi(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<VolumeProfileRsiConfig>(c).unwrap_or_default());
	crate::volume_profile_rsi_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn double_top_stochastic(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<DoubleTopStochasticConfig>(c).unwrap_or_default());
	crate::double_top_stochastic_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn roc_obv_rsi(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<RocObvRsiConfig>(c).unwrap_or_default());
	crate::roc_obv_rsi_strategy(
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn macd_rsi(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<MACDConfig>(c).unwrap_or_default());
	crate::macd_rsi_strategy(&input.closes, config, None)
}

pub fn adx_rsi(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<AdxRsiConfig>(c).unwrap_or_default());
	crate::adx_rsi_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn rsi_macd(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<RsiMacdConfig>(c).unwrap_or_default());
	crate::rsi_macd_strategy(&input.closes, config)
}

pub fn mfi_obv(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<MfiObvConfig>(c).unwrap_or_default());
	crate::mfi_obv_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn vwap_ema_rsi(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<VwapEmaRsiConfig>(c).unwrap_or_default());
	crate::vwap_ema_rsi_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn triangle_rsi(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<TriangleRsiConfig>(c).unwrap_or_default());
	crate::triangle_rsi_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn vwap_macd(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<VwapMacdConfig>(c).unwrap_or_default());
	crate::vwap_macd_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn vwap_stochastic(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<VwapStochasticConfig>(c).unwrap_or_default());
	crate::vwap_stochastic_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn vwap_rsi(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<VwapRsiConfig>(c).unwrap_or_default());
	crate::vwap_rsi_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn flag_pennant_macd(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<FlagsPennantsConfig>(c).unwrap_or_default());
	crate::flag_pennant_macd_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
		None,
	)
}

pub fn correlation_pair(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<CorrelationPairConfig>(c).unwrap_or_default());
	crate::correlation_pair_strategy(&input.closes, config)
}

pub fn percent_rank(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<PercentRankConfig>(c).unwrap_or_default());
	crate::percent_rank_strategy(&input.closes, config)
}

pub fn cointegration(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<CointegrationConfig>(c).unwrap_or_default());
	crate::cointegration_strategy(&input.closes, config)
}

pub fn correlation_reversion(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<CorrelationReversionConfig>(c).unwrap_or_default());
	crate::correlation_reversion_strategy(&input.closes, config)
}

pub fn roc(input: &StrategyInput, config: Option<serde_json::Value>) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<RocConfig>(c).unwrap_or_default());
	crate::roc_strategy(&input.closes, config)
}

pub fn kst(input: &StrategyInput, config: Option<serde_json::Value>) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<KSTConfig>(c).unwrap_or_default());
	crate::kst_strategy(&input.closes, config)
}

pub fn stochastic(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<StochasticConfig>(c).unwrap_or_default());
	crate::stochastic_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn cci(input: &StrategyInput, config: Option<serde_json::Value>) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<CciConfig>(c).unwrap_or_default());
	crate::cci_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn ultimate_oscillator(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<UltimateOscillatorConfig>(c).unwrap_or_default());
	crate::ultimate_oscillator_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn momentum(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<MomentumConfig>(c).unwrap_or_default());
	crate::momentum_strategy(&input.closes, config)
}

pub fn rsi(input: &StrategyInput, config: Option<serde_json::Value>) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<RSIConfig>(c).unwrap_or_default());
	crate::rsi_strategy(&input.closes, config)
}

pub fn rsi2(input: &StrategyInput, config: Option<serde_json::Value>) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<Rsi2Config>(c).unwrap_or_default());
	crate::rsi2_strategy(&input.closes, config)
}

pub fn awesome_oscillator(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<AwesomeOscillatorConfig>(c).unwrap_or_default());
	crate::awesome_oscillator_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		config,
	)
}

pub fn williams_r(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<WilliamsRConfig>(c).unwrap_or_default());
	crate::williams_r_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn ichimoku(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<IchimokuCloudConfig>(c).unwrap_or_default());
	crate::ichimoku_strategy(
		&input.closes,
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		config,
	)
}

pub fn lin_reg_channel(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<LinRegChannelConfig>(c).unwrap_or_default());
	crate::lin_reg_channel_strategy(&input.closes, config)
}

pub fn balance_of_power(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<BalanceOfPowerConfig>(c).unwrap_or_default());
	crate::balance_of_power_strategy(
		input.opens.as_ref().unwrap_or(&input.closes),
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn alma_crossover(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<AlmacrossoverConfig>(c).unwrap_or_default());
	crate::alma_crossover_strategy(&input.closes, config)
}

pub fn pivot_points(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<PivotPointsConfig>(c).unwrap_or_default());
	crate::pivot_points_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn wma_momentum(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<WmaMomentumConfig>(c).unwrap_or_default());
	crate::wma_momentum_strategy(&input.closes, config)
}

pub fn vwma(input: &StrategyInput, config: Option<serde_json::Value>) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<VwmaConfig>(c).unwrap_or_default());
	crate::vwma_strategy(
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn dmi(input: &StrategyInput, config: Option<serde_json::Value>) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<DmiConfig>(c).unwrap_or_default());
	crate::dmi_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn aroon(input: &StrategyInput, config: Option<serde_json::Value>) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<AroonConfig>(c).unwrap_or_default());
	crate::aroon_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		config,
	)
}

pub fn typical_price(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<TypicalPriceConfig>(c).unwrap_or_default());
	crate::typical_price_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn larsson(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<LarssonConfig>(c).unwrap_or_default());
	crate::larsson_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn macd(input: &StrategyInput, config: Option<serde_json::Value>) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<MACDConfig>(c).unwrap_or_default());
	crate::macd_strategy(&input.closes, config)
}

pub fn fibonacci_retracement(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<FibonacciRetracementConfig>(c).unwrap_or_default());
	crate::fibonacci_retracement_strategy(&input.closes, config)
}

pub fn adx(input: &StrategyInput, config: Option<serde_json::Value>) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<ADXConfig>(c).unwrap_or_default());
	crate::adx_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn lin_reg_slope(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<LinregSlopeConfig>(c).unwrap_or_default());
	crate::lin_reg_slope_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn absolute_price_oscillator(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config
		.map(|c| serde_json::from_value::<AbsolutePriceOscillatorConfig>(c).unwrap_or_default());
	crate::absolute_price_oscillator_strategy(&input.closes, config)
}

pub fn parabolic_sar(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<ParabolicSarConfig>(c).unwrap_or_default());
	crate::parabolic_sar_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn super_trend(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<SuperTrendConfig>(c).unwrap_or_default());
	crate::super_trend_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn sma_vwap_crossover(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<SmaVwapCrossoverConfig>(c).unwrap_or_default());
	crate::sma_vwap_crossover_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn wma_confirmation(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<WmaConfirmationConfig>(c).unwrap_or_default());
	crate::wma_confirmation_strategy(&input.closes, config)
}

pub fn kdj(input: &StrategyInput, config: Option<serde_json::Value>) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<KdjConfig>(c).unwrap_or_default());
	crate::kdj_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn alma_hma_divergence(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<AlmahmaDivergenceConfig>(c).unwrap_or_default());
	crate::alma_hma_divergence_strategy(&input.closes, config)
}

pub fn ma_crossover(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<MaCrossoverConfig>(c).unwrap_or_default());
	crate::ma_crossover_strategy(&input.closes, config)
}

pub fn hma_trend(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<HmaTrendConfig>(c).unwrap_or_default());
	crate::hma_trend_strategy(&input.closes, config)
}

pub fn macd_crossover(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<MacdCrossoverConfig>(c).unwrap_or_default());
	crate::macd_crossover_strategy(&input.closes, config)
}

pub fn chande_forecast_oscillator(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config
		.map(|c| serde_json::from_value::<ChandeForecastOscillatorConfig>(c).unwrap_or_default());
	crate::chande_forecast_oscillator_strategy(&input.closes, config)
}

pub fn vortex(input: &StrategyInput, config: Option<serde_json::Value>) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<VortexConfig>(c).unwrap_or_default());
	crate::vortex_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn atr_threshold(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<AtrThresholdConfig>(c).unwrap_or_default());
	crate::atr_threshold_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn pairs_trading(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<PairsTradingConfig>(c).unwrap_or_default());
	crate::pairs_trading_strategy(&input.closes, config)
}

pub fn bollinger_bands_breakout(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<BollingerBandsConfig>(c).unwrap_or_default());
	crate::bollinger_bands_breakout_strategy(&input.closes, config)
}

pub fn z_score_breakout(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<ZScoreConfig>(c).unwrap_or_default());
	crate::z_score_breakout_strategy(&input.closes, config)
}

pub fn donchian_reversion(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<DonchianTurtleConfig>(c).unwrap_or_default());
	crate::donchian_reversion_strategy(&input.closes, config)
}

pub fn bollinger_bands_mean_reversion(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<BollingerBandsConfig>(c).unwrap_or_default());
	crate::bollinger_bands_mean_reversion_strategy(&input.closes, config)
}

pub fn mad_reversion(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<MadReversionConfig>(c).unwrap_or_default());
	crate::mad_reversion_strategy(&input.closes, config)
}

pub fn acceleration_bands(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<AccelerationBandsConfig>(c).unwrap_or_default());
	crate::acceleration_bands_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn projection_oscillator(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<ProjectionOscillatorConfig>(c).unwrap_or_default());
	crate::projection_oscillator_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn z_score_reversion(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<ZScoreReversionConfig>(c).unwrap_or_default());
	crate::z_score_reversion_strategy(&input.closes, config)
}

pub fn keltner_channel_reversion(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<KeltnerChannelConfig>(c).unwrap_or_default());
	crate::keltner_channel_reversion_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn donchian_breakout(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<DonchianTurtleConfig>(c).unwrap_or_default());
	crate::donchian_breakout_strategy(&input.closes, config)
}

pub fn atr_volatility_threshold(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config
		.map(|c| serde_json::from_value::<AtrVolatilityThresholdConfig>(c).unwrap_or_default());
	crate::atr_volatility_threshold_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn opening_range_breakout(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<OpeningRangeBreakoutConfig>(c).unwrap_or_default());
	crate::opening_range_breakout_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn keltner_channel_breakout(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<KeltnerChannelConfig>(c).unwrap_or_default());
	crate::keltner_channel_breakout_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn volatility_adjusted(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<VolatilityAdjustedConfig>(c).unwrap_or_default());
	crate::volatility_adjusted_strategy(&input.closes, config)
}

pub fn standard_deviation(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<StandardDeviationConfig>(c).unwrap_or_default());
	crate::standard_deviation_strategy(&input.closes, config)
}

pub fn variance_stop(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<VarianceStopConfig>(c).unwrap_or_default());
	crate::variance_stop_strategy(&input.closes, config)
}

pub fn keltner_volatility_breakout(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config
		.map(|c| serde_json::from_value::<KeltnerVolatilityBreakoutConfig>(c).unwrap_or_default());
	crate::keltner_volatility_breakout_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn money_flow_index(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<MoneyFlowIndexConfig>(c).unwrap_or_default());
	crate::money_flow_index_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn volume_weighted_average_price(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config
		.map(|c| serde_json::from_value::<VolumeWeightedAveragePriceConfig>(c).unwrap_or_default());
	crate::volume_weighted_average_price_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn accumulation_distribution(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config
		.map(|c| serde_json::from_value::<AccumulationDistributionConfig>(c).unwrap_or_default());
	crate::accumulation_distribution_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn chaikin_money_flow(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<ChaikinMoneyFlowConfig>(c).unwrap_or_default());
	crate::chaikin_money_flow_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn obv_confirmation(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<ObvConfirmationConfig>(c).unwrap_or_default());
	crate::obv_confirmation_strategy(
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn vwap_reversion(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<VwapReversionConfig>(c).unwrap_or_default());
	crate::vwap_reversion_strategy(
		&input.closes,
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn ease_of_movement(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<EaseOfMovementConfig>(c).unwrap_or_default());
	crate::ease_of_movement_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn vwap_breakout(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<VwapBreakoutConfig>(c).unwrap_or_default());
	crate::vwap_breakout_strategy(
		&input.closes,
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn obv(input: &StrategyInput, config: Option<serde_json::Value>) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<OBVConfig>(c).unwrap_or_default());
	crate::obv_strategy(
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn negative_volume_index(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<NegativeVolumeIndexConfig>(c).unwrap_or_default());
	crate::negative_volume_index_strategy(
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn force_index(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<ForceIndexConfig>(c).unwrap_or_default());
	crate::force_index_strategy(
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn volume_price_trend(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<VolumePriceTrendConfig>(c).unwrap_or_default());
	crate::volume_price_trend_strategy(
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

// ── Metadata registry ─────────────────────────────

pub fn get_strategy_registry() -> crate::types::results::StrategyRegistry {
	let _ = get_strategy_registry_impl();
	let mut strategies = HashMap::new();

	strategies.insert(
		"buy-and-hold".to_string(),
		serde_json::from_value(crate::buy_and_hold_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"adx-rsi-trend-momentum".to_string(),
		serde_json::from_value(crate::adx_rsi_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"bb-rsi-breakout".to_string(),
		serde_json::from_value(crate::bb_rsi_strategy_metadata()).expect("valid strategy metadata"),
	);
	strategies.insert(
		"double-top-stochastic-reversal".to_string(),
		serde_json::from_value(crate::double_top_stochastic_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"flag-pennant-macd-continuation".to_string(),
		serde_json::from_value(crate::flag_pennant_macd_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"ma-rsi-trend-following".to_string(),
		serde_json::from_value(crate::ma_rsi_strategy_metadata()).expect("valid strategy metadata"),
	);
	strategies.insert(
		"macd-rsi-momentum".to_string(),
		serde_json::from_value(crate::macd_rsi_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"macd-stochastic-confirmation".to_string(),
		serde_json::from_value(crate::macd_stochastic_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"mfi-obv-volume-flow".to_string(),
		serde_json::from_value(crate::mfi_obv_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"obv-rsi-volume-confirmation".to_string(),
		serde_json::from_value(crate::obv_rsi_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"roc-obv-rsi-momentum".to_string(),
		serde_json::from_value(crate::roc_obv_rsi_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"rsi-macd-confirmation".to_string(),
		serde_json::from_value(crate::rsi_macd_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"triangle-rsi-breakout".to_string(),
		serde_json::from_value(crate::triangle_rsi_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"volume-profile-rsi".to_string(),
		serde_json::from_value(crate::volume_profile_rsi_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"vwap-ema-rsi-trend".to_string(),
		serde_json::from_value(crate::vwap_ema_rsi_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"vwap-macd-momentum".to_string(),
		serde_json::from_value(crate::vwap_macd_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"vwap-rsi-breakout".to_string(),
		serde_json::from_value(crate::vwap_rsi_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"vwap-stochastic-confirmation".to_string(),
		serde_json::from_value(crate::vwap_stochastic_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"awesomeOscillator".to_string(),
		serde_json::from_value(crate::awesome_oscillator_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"cci".to_string(),
		serde_json::from_value(crate::cci_strategy_metadata()).expect("valid strategy metadata"),
	);
	strategies.insert(
		"ichimoku".to_string(),
		serde_json::from_value(crate::ichimoku_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"kst".to_string(),
		serde_json::from_value(crate::kst_strategy_metadata()).expect("valid strategy metadata"),
	);
	strategies.insert(
		"momentum".to_string(),
		serde_json::from_value(crate::momentum_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"roc".to_string(),
		serde_json::from_value(crate::roc_strategy_metadata()).expect("valid strategy metadata"),
	);
	strategies.insert(
		"rsi".to_string(),
		serde_json::from_value(crate::rsi_strategy_metadata()).expect("valid strategy metadata"),
	);
	strategies.insert(
		"rsi2".to_string(),
		serde_json::from_value(crate::rsi2_strategy_metadata()).expect("valid strategy metadata"),
	);
	strategies.insert(
		"stochastic".to_string(),
		serde_json::from_value(crate::stochastic_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"ultimateOscillator".to_string(),
		serde_json::from_value(crate::ultimate_oscillator_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"williamsR".to_string(),
		serde_json::from_value(crate::williams_r_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"cup-and-handle-breakout".to_string(),
		serde_json::from_value(crate::cup_and_handle_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"double-top-bottom-reversal".to_string(),
		serde_json::from_value(crate::double_top_bottom_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"elliott-wave-pattern".to_string(),
		serde_json::from_value(crate::percent_rank_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"flags-pennants-continuation".to_string(),
		serde_json::from_value(crate::flags_pennants_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"head-and-shoulders-reversal".to_string(),
		serde_json::from_value(crate::head_and_shoulders_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"triangle-breakout".to_string(),
		serde_json::from_value(crate::triangle_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"wedge-breakout".to_string(),
		serde_json::from_value(crate::wedge_strategy_metadata()).expect("valid strategy metadata"),
	);
	strategies.insert(
		"cointegration".to_string(),
		serde_json::from_value(crate::cointegration_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"correlation-pair-trading".to_string(),
		serde_json::from_value(crate::correlation_pair_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"correlation-mean-reversion".to_string(),
		serde_json::from_value(crate::correlation_reversion_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"percentRank-ranking".to_string(),
		serde_json::from_value(crate::percent_rank_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"absolutePriceOscillator".to_string(),
		serde_json::from_value(crate::absolute_price_oscillator_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"adx".to_string(),
		serde_json::from_value(crate::adx_strategy_metadata()).expect("valid strategy metadata"),
	);
	strategies.insert(
		"almaCrossover".to_string(),
		serde_json::from_value(crate::alma_crossover_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"almaHmaDivergence".to_string(),
		serde_json::from_value(crate::alma_hma_divergence_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"aroon".to_string(),
		serde_json::from_value(crate::aroon_strategy_metadata()).expect("valid strategy metadata"),
	);
	strategies.insert(
		"balanceOfPower".to_string(),
		serde_json::from_value(crate::balance_of_power_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"chandeForecastOscillator".to_string(),
		serde_json::from_value(crate::chande_forecast_oscillator_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"dmi".to_string(),
		serde_json::from_value(crate::dmi_strategy_metadata()).expect("valid strategy metadata"),
	);
	strategies.insert(
		"fibonacciRetracement".to_string(),
		serde_json::from_value(crate::fibonacci_retracement_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"hmaTrend".to_string(),
		serde_json::from_value(crate::hma_trend_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"kdj".to_string(),
		serde_json::from_value(crate::kdj_strategy_metadata()).expect("valid strategy metadata"),
	);
	strategies.insert(
		"larsson".to_string(),
		serde_json::from_value(crate::larsson_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"linRegChannel".to_string(),
		serde_json::from_value(crate::lin_reg_channel_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"linRegSlope".to_string(),
		serde_json::from_value(crate::lin_reg_slope_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"maCrossover".to_string(),
		serde_json::from_value(crate::ma_crossover_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"macd".to_string(),
		serde_json::from_value(crate::macd_strategy_metadata()).expect("valid strategy metadata"),
	);
	strategies.insert(
		"macdCrossover".to_string(),
		serde_json::from_value(crate::macd_crossover_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"parabolicSar".to_string(),
		serde_json::from_value(crate::parabolic_sar_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"pivotPoints".to_string(),
		serde_json::from_value(crate::pivot_points_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"smaVwapCrossover".to_string(),
		serde_json::from_value(crate::sma_vwap_crossover_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"superTrend".to_string(),
		serde_json::from_value(crate::super_trend_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"typicalPrice".to_string(),
		serde_json::from_value(crate::typical_price_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"vortex".to_string(),
		serde_json::from_value(crate::vortex_strategy_metadata()).expect("valid strategy metadata"),
	);
	strategies.insert(
		"vwma".to_string(),
		serde_json::from_value(crate::vwma_strategy_metadata()).expect("valid strategy metadata"),
	);
	strategies.insert(
		"wmaConfirmation".to_string(),
		serde_json::from_value(crate::wma_confirmation_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"wmaMomentum".to_string(),
		serde_json::from_value(crate::wma_momentum_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"accelerationBands".to_string(),
		serde_json::from_value(crate::acceleration_bands_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"atrThreshold".to_string(),
		serde_json::from_value(crate::atr_threshold_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"atrVolatilityThreshold".to_string(),
		serde_json::from_value(crate::atr_volatility_threshold_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"bollingerBandsBreakout".to_string(),
		serde_json::from_value(crate::bollinger_bands_breakout_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"bollingerBandsMeanReversion".to_string(),
		serde_json::from_value(crate::bollinger_bands_mean_reversion_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"donchianBreakout".to_string(),
		serde_json::from_value(crate::donchian_breakout_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"donchianReversion".to_string(),
		serde_json::from_value(crate::donchian_reversion_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"keltnerChannelBreakout".to_string(),
		serde_json::from_value(crate::keltner_channel_breakout_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"keltnerChannelReversion".to_string(),
		serde_json::from_value(crate::keltner_channel_reversion_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"keltnerVolatilityBreakout".to_string(),
		serde_json::from_value(crate::keltner_volatility_breakout_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"madReversion".to_string(),
		serde_json::from_value(crate::mad_reversion_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"openingRangeBreakout".to_string(),
		serde_json::from_value(crate::opening_range_breakout_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"pairsTrading".to_string(),
		serde_json::from_value(crate::pairs_trading_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"projectionOscillator".to_string(),
		serde_json::from_value(crate::projection_oscillator_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"standardDeviation".to_string(),
		serde_json::from_value(crate::standard_deviation_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"varianceStop".to_string(),
		serde_json::from_value(crate::variance_stop_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"volatilityAdjusted".to_string(),
		serde_json::from_value(crate::volatility_adjusted_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"zScoreBreakout".to_string(),
		serde_json::from_value(crate::z_score_breakout_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"zScoreReversion".to_string(),
		serde_json::from_value(crate::z_score_reversion_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"accumulation-distribution".to_string(),
		serde_json::from_value(crate::accumulation_distribution_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"chaikin-money-flow".to_string(),
		serde_json::from_value(crate::chaikin_money_flow_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"ease-of-movement".to_string(),
		serde_json::from_value(crate::ease_of_movement_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"force-index".to_string(),
		serde_json::from_value(crate::force_index_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"money-flow-index".to_string(),
		serde_json::from_value(crate::money_flow_index_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"negative-volume-index".to_string(),
		serde_json::from_value(crate::negative_volume_index_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"obv".to_string(),
		serde_json::from_value(crate::obv_strategy_metadata()).expect("valid strategy metadata"),
	);
	strategies.insert(
		"obv-confirmation".to_string(),
		serde_json::from_value(crate::obv_confirmation_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"volume-price-trend".to_string(),
		serde_json::from_value(crate::volume_price_trend_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"volumeWeightedAveragePrice".to_string(),
		serde_json::from_value(crate::volume_weighted_average_price_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"vwap-breakout".to_string(),
		serde_json::from_value(crate::vwap_breakout_strategy_metadata())
			.expect("valid strategy metadata"),
	);
	strategies.insert(
		"vwap-reversion".to_string(),
		serde_json::from_value(crate::vwap_reversion_strategy_metadata())
			.expect("valid strategy metadata"),
	);

	crate::types::results::StrategyRegistry { strategies }
}
