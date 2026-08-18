use std::collections::HashMap;
use std::sync::LazyLock;

pub struct IndicatorDescriptor {
	pub id: &'static str,
	pub name: &'static str,
	pub category: &'static str,
	pub description: &'static str,
	/// JSON Schema describing the config parameters for this indicator.
	/// Empty string `""` means no config parameters.
	pub params_schema: &'static str,
	/// Output type: "signal" (Vec<i8>), "factor" (Vec<f64>), "indicator" (Vec<f64>)
	pub output_type: &'static str,
}

inventory::collect!(IndicatorDescriptor);

pub fn get_indicator_descriptors() -> Vec<&'static IndicatorDescriptor> {
	inventory::iter::<IndicatorDescriptor>.into_iter().collect()
}

pub static INDICATOR_REGISTRY: LazyLock<HashMap<&'static str, &'static IndicatorDescriptor>> =
	LazyLock::new(|| {
		let mut m = HashMap::new();
		for desc in inventory::iter::<IndicatorDescriptor> {
			m.insert(desc.id, desc);
		}
		m
	});

/// Returns the `IndicatorDescriptor` for a given id, or `None` if not found.
pub fn get_indicator(id: &str) -> Option<&'static IndicatorDescriptor> {
	INDICATOR_REGISTRY.get(id).copied()
}

// ── momentum ─────────────────────────────────────

inventory::submit! {
	IndicatorDescriptor { id: "awesome_oscillator", name: "Awesome Oscillator", category: "momentum", description: "Measures market momentum by comparing SMA of median price over two periods", params_schema: "{\"type\":\"object\",\"properties\":{\"fastPeriod\":{\"type\":\"integer\",\"default\":5},\"slowPeriod\":{\"type\":\"integer\",\"default\":34}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "chaikin_oscillator", name: "Chaikin Oscillator", category: "momentum", description: "Measures accumulation/distribution momentum as EMA difference of ADL", params_schema: "{\"type\":\"object\",\"properties\":{\"fastPeriod\":{\"type\":\"integer\",\"default\":3},\"slowPeriod\":{\"type\":\"integer\",\"default\":10}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "cmo", name: "Chaikin Oscillator", category: "momentum", description: "Alias for Chaikin Oscillator", params_schema: "{\"type\":\"object\",\"properties\":{\"fastPeriod\":{\"type\":\"integer\",\"default\":3},\"slowPeriod\":{\"type\":\"integer\",\"default\":10}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "ichimoku_cloud", name: "Ichimoku Cloud", category: "momentum", description: "Comprehensive indicator showing support, resistance, and momentum", params_schema: "{\"type\":\"object\",\"properties\":{\"short\":{\"type\":\"integer\",\"default\":9},\"medium\":{\"type\":\"integer\",\"default\":26},\"long\":{\"type\":\"integer\",\"default\":52},\"close\":{\"type\":\"integer\",\"default\":26}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "ichimoku", name: "Ichimoku", category: "momentum", description: "Alias for Ichimoku Cloud", params_schema: "{\"type\":\"object\",\"properties\":{\"short\":{\"type\":\"integer\",\"default\":9},\"medium\":{\"type\":\"integer\",\"default\":26},\"long\":{\"type\":\"integer\",\"default\":52},\"close\":{\"type\":\"integer\",\"default\":26}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "kst", name: "Know Sure Thing", category: "momentum", description: "KST oscillator based on four rate-of-change periods", params_schema: "{\"type\":\"object\",\"properties\":{\"roc1Period\":{\"type\":\"integer\",\"default\":10},\"roc2Period\":{\"type\":\"integer\",\"default\":15},\"roc3Period\":{\"type\":\"integer\",\"default\":20},\"roc4Period\":{\"type\":\"integer\",\"default\":30},\"sma1Period\":{\"type\":\"integer\",\"default\":10},\"sma2Period\":{\"type\":\"integer\",\"default\":10},\"sma3Period\":{\"type\":\"integer\",\"default\":10},\"sma4Period\":{\"type\":\"integer\",\"default\":15},\"signalPeriod\":{\"type\":\"integer\",\"default\":9}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "larsson", name: "Larsson", category: "momentum", description: "Larsson signal indicator using SMMA crossovers", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "macd", name: "MACD", category: "momentum", description: "Moving Average Convergence Divergence", params_schema: "{\"type\":\"object\",\"properties\":{\"fastPeriod\":{\"type\":\"integer\",\"default\":12},\"slowPeriod\":{\"type\":\"integer\",\"default\":26},\"signalPeriod\":{\"type\":\"integer\",\"default\":9}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "momentum_index", name: "Momentum Index", category: "momentum", description: "Simple price momentum over a lookback period", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "percentage_price_oscillator", name: "Percentage Price Oscillator", category: "momentum", description: "Percentage-based MACD variant", params_schema: "{\"type\":\"object\",\"properties\":{\"fastPeriod\":{\"type\":\"integer\",\"default\":12},\"slowPeriod\":{\"type\":\"integer\",\"default\":26},\"signalPeriod\":{\"type\":\"integer\",\"default\":9}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "percentage_volume_oscillator", name: "Percentage Volume Oscillator", category: "momentum", description: "Percentage-based MACD applied to volume", params_schema: "{\"type\":\"object\",\"properties\":{\"fastPeriod\":{\"type\":\"integer\",\"default\":12},\"slowPeriod\":{\"type\":\"integer\",\"default\":26},\"signalPeriod\":{\"type\":\"integer\",\"default\":9}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "pvo", name: "Percentage Volume Oscillator", category: "momentum", description: "Alias for Percentage Volume Oscillator", params_schema: "{\"type\":\"object\",\"properties\":{\"fastPeriod\":{\"type\":\"integer\",\"default\":12},\"slowPeriod\":{\"type\":\"integer\",\"default\":26},\"signalPeriod\":{\"type\":\"integer\",\"default\":9}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "price_rate_of_change", name: "Price Rate of Change", category: "momentum", description: "Percentage price change over a period", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":3}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "qstick", name: "Qstick", category: "momentum", description: "SMA of the close-open differential", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "rsi", name: "RSI", category: "momentum", description: "Relative Strength Index", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"Lookback period for RSI calculation\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "stochastic_oscillator", name: "Stochastic Oscillator", category: "momentum", description: "Compares close to the high-low range over a period", params_schema: "{\"type\":\"object\",\"properties\":{\"kPeriod\":{\"type\":\"integer\",\"default\":14},\"dPeriod\":{\"type\":\"integer\",\"default\":3}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "ultimate_oscillator", name: "Ultimate Oscillator", category: "momentum", description: "Multi-timeframe momentum oscillator", params_schema: "{\"type\":\"object\",\"properties\":{\"period1\":{\"type\":\"integer\",\"default\":7},\"period2\":{\"type\":\"integer\",\"default\":14},\"period3\":{\"type\":\"integer\",\"default\":28}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "uo", name: "Ultimate Oscillator", category: "momentum", description: "Alias for Ultimate Oscillator", params_schema: "{\"type\":\"object\",\"properties\":{\"period1\":{\"type\":\"integer\",\"default\":7},\"period2\":{\"type\":\"integer\",\"default\":14},\"period3\":{\"type\":\"integer\",\"default\":28}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "williams_r", name: "Williams %R", category: "momentum", description: "Overbought/oversold indicator based on highest high", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14}}}", output_type: "indicator" }
}

// ── trend ─────────────────────────────────────────

inventory::submit! {
	IndicatorDescriptor { id: "absolute_price_oscillator", name: "Absolute Price Oscillator", category: "trend", description: "Absolute difference between fast and slow EMA", params_schema: "{\"type\":\"object\",\"properties\":{\"fastPeriod\":{\"type\":\"integer\",\"default\":14},\"slowPeriod\":{\"type\":\"integer\",\"default\":30}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "adx", name: "ADX", category: "trend", description: "Average Directional Index for trend strength", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "alma", name: "ALMA", category: "trend", description: "Arnaud Legoux Moving Average", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":9},\"offset\":{\"type\":\"number\",\"default\":0.85},\"sigma\":{\"type\":\"number\",\"default\":6}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "aroon", name: "Aroon", category: "trend", description: "Aroon indicator for trend direction and strength", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":25}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "balance_of_power", name: "Balance of Power", category: "trend", description: "Measures the strength of buyers vs sellers", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "cci", name: "CCI", category: "trend", description: "Commodity Channel Index", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":20}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "chande_forecast_oscillator", name: "Chande Forecast Oscillator", category: "trend", description: "Forecast oscillator using linear regression", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "moving_chande_forecast_oscillator", name: "Moving Chande Forecast Oscillator", category: "trend", description: "Moving variant of the Chande Forecast Oscillator", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":4}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "classify_market_trend", name: "Classify Market Trend", category: "trend", description: "Classifies market as trending, ranging, or volatile", params_schema: "{\"type\":\"object\",\"properties\":{\"trailingPeriodLength\":{\"type\":\"integer\",\"description\":\"Trailing period length\",\"default\":0}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "dema", name: "DEMA", category: "trend", description: "Double Exponential Moving Average", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":12}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "ema", name: "EMA", category: "trend", description: "Exponential Moving Average", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":12}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "fibonacci_pivot_points", name: "Fibonacci Pivot Points", category: "trend", description: "Fibonacci-based pivot point levels", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "camarilla_pivot_points", name: "Camarilla Pivot Points", category: "trend", description: "Camarilla-style pivot point levels", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "hma", name: "HMA", category: "trend", description: "Hull Moving Average", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":16}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "linreg", name: "Linear Regression", category: "trend", description: "Linear regression line over a period", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14},\"offset\":{\"type\":\"integer\",\"default\":0}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "mass_index", name: "Mass Index", category: "trend", description: "Mass Index for trend reversal detection", params_schema: "{\"type\":\"object\",\"properties\":{\"emaPeriod\":{\"type\":\"integer\",\"default\":9},\"miPeriod\":{\"type\":\"integer\",\"default\":25}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "moving_max", name: "Moving Max", category: "trend", description: "Rolling maximum over a window", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":4}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "moving_min", name: "Moving Min", category: "trend", description: "Rolling minimum over a window", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":4}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "moving_sum", name: "Moving Sum", category: "trend", description: "Rolling sum over a window", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":4}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "parabolic_sar", name: "Parabolic SAR", category: "trend", description: "Parabolic Stop and Reverse", params_schema: "{\"type\":\"object\",\"properties\":{\"step\":{\"type\":\"number\",\"description\":\"Acceleration factor increment\",\"default\":0.02},\"max\":{\"type\":\"number\",\"description\":\"Maximum acceleration factor\",\"default\":0.2}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "pivot_points", name: "Pivot Points", category: "trend", description: "Standard pivot point levels", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "random_index", name: "Random Index", category: "trend", description: "Random walk index for mean reversion", params_schema: "{\"type\":\"object\",\"properties\":{\"rPeriod\":{\"type\":\"integer\",\"default\":9},\"kPeriod\":{\"type\":\"integer\",\"default\":3},\"dPeriod\":{\"type\":\"integer\",\"default\":3}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "rma", name: "RMA", category: "trend", description: "Rolling Moving Average", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":4}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "rolling_moving_average", name: "Rolling Moving Average", category: "trend", description: "Alias for RMA", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":4}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "since", name: "Since", category: "trend", description: "Counts periods since last condition", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "sma", name: "SMA", category: "trend", description: "Simple Moving Average", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":2}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "smoothed_moving_average", name: "Smoothed Moving Average", category: "trend", description: "Smoothed variant of SMMA", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "super_trend", name: "Super Trend", category: "trend", description: "Super Trend for trend direction and stop levels", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14},\"multiplier\":{\"type\":\"number\",\"description\":\"ATR multiplier for band width\",\"default\":3}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "tema", name: "TEMA", category: "trend", description: "Triple Exponential Moving Average", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":2}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "tma", name: "TMA", category: "trend", description: "Triangular Moving Average", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":4}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "trix", name: "TRIX", category: "trend", description: "Triple-smoothed EMA oscillator", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":4}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "typical_price", name: "Typical Price", category: "trend", description: "Average of high, low, and close", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "vortex", name: "Vortex", category: "trend", description: "Vortex indicator for trend direction", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "vwma", name: "VWMA", category: "trend", description: "Volume Weighted Moving Average", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":20}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "wma", name: "WMA", category: "trend", description: "Weighted Moving Average", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14}}}", output_type: "indicator" }
}

// ── volatility ────────────────────────────────────

inventory::submit! {
	IndicatorDescriptor { id: "ab", name: "Acceleration Bands", category: "volatility", description: "Alias for Acceleration Bands", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":20},\"multiplier\":{\"type\":\"number\",\"default\":4}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "acceleration_bands", name: "Acceleration Bands", category: "volatility", description: "Volatility bands based on price acceleration", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":20},\"multiplier\":{\"type\":\"number\",\"default\":4}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "annualized_volatility", name: "Annualized Volatility", category: "volatility", description: "Annualized standard deviation of returns", params_schema: "{\"type\":\"object\",\"properties\":{\"tradingDays\":{\"type\":\"integer\",\"description\":\"Trading days per year\",\"default\":252}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "atr", name: "ATR", category: "volatility", description: "Alias for Average True Range", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"RMA lookback period\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "average_true_range", name: "Average True Range", category: "volatility", description: "Average True Range", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"RMA lookback period\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "bb", name: "Bollinger Bands", category: "volatility", description: "Bollinger Bands", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":20},\"stdDev\":{\"type\":\"number\",\"default\":2}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "bollinger_bands", name: "Bollinger Bands", category: "volatility", description: "Bollinger Bands", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":20},\"stdDev\":{\"type\":\"number\",\"default\":2}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "bbw", name: "Bollinger Bands Width", category: "volatility", description: "Alias for Bollinger Bands Width", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"EMA period for width smoothing\",\"default\":90}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "bollinger_bands_width", name: "Bollinger Bands Width", category: "volatility", description: "Normalized width of Bollinger Bands", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"EMA period for width smoothing\",\"default\":90}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "ce", name: "Chandelier Exit", category: "volatility", description: "Alias for Chandelier Exit", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"ATR and lookback period\",\"default\":22}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "chandelier_exit", name: "Chandelier Exit", category: "volatility", description: "ATR-based trailing stop", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"ATR and lookback period\",\"default\":22}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "dc", name: "Donchian Channel", category: "volatility", description: "Alias for Donchian Channel", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":4}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "donchian_channel", name: "Donchian Channel", category: "volatility", description: "Channel based on highest high and lowest low", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":4}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "dev", name: "Mean Absolute Deviation", category: "volatility", description: "Alias for Mean Absolute Deviation", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "mean_absolute_deviation", name: "Mean Absolute Deviation", category: "volatility", description: "Mean absolute deviation from the mean", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "kc", name: "Keltner Channel", category: "volatility", description: "Alias for Keltner Channel", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":20}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "keltner_channel", name: "Keltner Channel", category: "volatility", description: "ATR-based volatility channel", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":20}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "max_drawdown", name: "Max Drawdown", category: "volatility", description: "Maximum peak-to-trough decline", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"Lookback window period\",\"default\":0}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "mstd", name: "Moving Standard Deviation", category: "volatility", description: "Alias for Moving Standard Deviation", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":4}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "moving_standard_deviation", name: "Moving Standard Deviation", category: "volatility", description: "Rolling standard deviation over a window", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":4}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "po", name: "Projection Oscillator", category: "volatility", description: "Alias for Projection Oscillator", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"Lookback period for linear regression\",\"default\":14},\"smooth\":{\"type\":\"integer\",\"description\":\"EMA smoothing period\",\"default\":3}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "projection_oscillator", name: "Projection Oscillator", category: "volatility", description: "Oscillator based on linear regression projection", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"Lookback period for linear regression\",\"default\":14},\"smooth\":{\"type\":\"integer\",\"description\":\"EMA smoothing period\",\"default\":3}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "rolling_variance", name: "Rolling Variance", category: "volatility", description: "Rolling variance over a window", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "tr", name: "True Range", category: "volatility", description: "Alias for True Range", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "true_range", name: "True Range", category: "volatility", description: "True Range", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "ttm_squeeze", name: "TTM Squeeze", category: "volatility", description: "Detects Bollinger/Keltner squeeze setups", params_schema: "{\"type\":\"object\",\"properties\":{\"bbPeriod\":{\"type\":\"integer\",\"default\":20},\"bbStdDev\":{\"type\":\"number\",\"default\":2},\"kcPeriod\":{\"type\":\"integer\",\"default\":20}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "ui", name: "Ulcer Index", category: "volatility", description: "Alias for Ulcer Index", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "ulcer_index", name: "Ulcer Index", category: "volatility", description: "Drawdown-based volatility measure", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "variance", name: "Variance", category: "volatility", description: "Variance over a period", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "zs", name: "Z-Score", category: "volatility", description: "Alias for Z-Score", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":20}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "z_score", name: "Z-Score", category: "volatility", description: "Number of standard deviations from the mean", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":20}}}", output_type: "indicator" }
}

// ── volume ────────────────────────────────────────

inventory::submit! {
	IndicatorDescriptor { id: "accumulation_distribution", name: "Accumulation Distribution", category: "volume", description: "Accumulation/distribution line", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "ad", name: "Accumulation Distribution", category: "volume", description: "Alias for Accumulation Distribution", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "anchored_vwap", name: "Anchored VWAP", category: "volume", description: "VWAP from a specified start point", params_schema: "{\"type\":\"object\",\"properties\":{\"anchorIndex\":{\"type\":\"integer\",\"description\":\"Index from which to start VWAP calculation\",\"default\":0}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "chaikin_money_flow", name: "Chaikin Money Flow", category: "volume", description: "Money flow volume over a period", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"Lookback period\",\"default\":20}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "cmf", name: "Chaikin Money Flow", category: "volume", description: "Alias for Chaikin Money Flow", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"Lookback period\",\"default\":20}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "ease_of_movement", name: "Ease of Movement", category: "volume", description: "Relates price movement to volume", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"SMA smoothing period\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "emv", name: "Ease of Movement", category: "volume", description: "Alias for Ease of Movement", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"SMA smoothing period\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "force_index", name: "Force Index", category: "volume", description: "Price change multiplied by volume", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"EMA smoothing period\",\"default\":13}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "fi", name: "Force Index", category: "volume", description: "Alias for Force Index", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"EMA smoothing period\",\"default\":13}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "mfi", name: "Money Flow Index", category: "volume", description: "Alias for Money Flow Index", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"Lookback period\",\"default\":14},\"priceSource\":{\"type\":\"number\",\"description\":\"Price source\",\"default\":\"typical\"}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "money_flow_index", name: "Money Flow Index", category: "volume", description: "Volume-weighted RSI variant", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"Lookback period\",\"default\":14},\"priceSource\":{\"type\":\"number\",\"description\":\"Price source\",\"default\":\"typical\"}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "negative_volume_index", name: "Negative Volume Index", category: "volume", description: "Tracks price changes on lower volume days", params_schema: "{\"type\":\"object\",\"properties\":{\"start\":{\"type\":\"number\",\"description\":\"Starting value for the NVI\",\"default\":1000}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "nvi", name: "Negative Volume Index", category: "volume", description: "Alias for Negative Volume Index", params_schema: "{\"type\":\"object\",\"properties\":{\"start\":{\"type\":\"number\",\"description\":\"Starting value for the NVI\",\"default\":1000}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "obv", name: "OBV", category: "volume", description: "On-Balance Volume", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "on_balance_volume", name: "On-Balance Volume", category: "volume", description: "On-Balance Volume", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "volume_price_trend", name: "Volume Price Trend", category: "volume", description: "Cumulative volume-weighted price trend", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "vpt", name: "Volume Price Trend", category: "volume", description: "Alias for Volume Price Trend", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "volume_profile", name: "Volume Profile", category: "volume", description: "Volume distribution by price level", params_schema: "{\"type\":\"object\",\"properties\":{\"bins\":{\"type\":\"integer\",\"description\":\"Number of price bins for volume distribution\",\"default\":50}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "volume_surge", name: "Volume Surge", category: "volume", description: "Detects abnormal volume spikes", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"SMA period for volume baseline\",\"default\":20},\"multiplier\":{\"type\":\"number\",\"description\":\"Threshold multiplier\",\"default\":2}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "vs", name: "Volume Surge", category: "volume", description: "Alias for Volume Surge", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"SMA period for volume baseline\",\"default\":20},\"multiplier\":{\"type\":\"number\",\"description\":\"Threshold multiplier\",\"default\":2}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "vwap", name: "VWAP", category: "volume", description: "Volume-Weighted Average Price", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14},\"priceSource\":{\"type\":\"number\",\"description\":\"Price source: close or hlc3\",\"default\":\"close\"},\"anchored\":{\"type\":\"boolean\",\"description\":\"Anchored VWAP calculation\",\"default\":false},\"sessionLength\":{\"type\":\"integer\",\"description\":\"Session length for anchored mode\",\"default\":0}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "volume_weighted_average_price", name: "Volume Weighted Average Price", category: "volume", description: "VWAP", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14},\"priceSource\":{\"type\":\"number\",\"description\":\"Price source: close or hlc3\",\"default\":\"close\"},\"anchored\":{\"type\":\"boolean\",\"description\":\"Anchored VWAP calculation\",\"default\":false},\"sessionLength\":{\"type\":\"integer\",\"description\":\"Session length for anchored mode\",\"default\":0}}}", output_type: "indicator" }
}

// ── patterns ──────────────────────────────────────

inventory::submit! {
	IndicatorDescriptor { id: "bullish_engulfing", name: "Bullish Engulfing", category: "patterns", description: "Bullish engulfing candlestick pattern", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "bearish_engulfing", name: "Bearish Engulfing", category: "patterns", description: "Bearish engulfing candlestick pattern", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "cup_and_handle", name: "Cup and Handle", category: "patterns", description: "Cup and handle pattern recognition", params_schema: "{\"type\":\"object\",\"properties\":{\"cupDepth\":{\"type\":\"number\",\"default\":0.15},\"handleRetracement\":{\"type\":\"number\",\"default\":0.3},\"minDuration\":{\"type\":\"integer\",\"default\":20}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "double_bottom", name: "Double Bottom", category: "patterns", description: "Double bottom reversal pattern", params_schema: "{\"type\":\"object\",\"properties\":{\"tolerance\":{\"type\":\"number\",\"default\":0.03},\"minSeparation\":{\"type\":\"integer\",\"default\":10},\"lookaround\":{\"type\":\"integer\",\"default\":2}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "double_top", name: "Double Top", category: "patterns", description: "Double top reversal pattern", params_schema: "{\"type\":\"object\",\"properties\":{\"tolerance\":{\"type\":\"number\",\"default\":0.03},\"minSeparation\":{\"type\":\"integer\",\"default\":10},\"lookaround\":{\"type\":\"integer\",\"default\":2}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "elliott_wave", name: "Elliott Wave", category: "patterns", description: "Elliott Wave pattern detection", params_schema: "{\"type\":\"object\",\"properties\":{\"wave2Retracement\":{\"type\":\"number\",\"default\":0.618},\"wave4Retracement\":{\"type\":\"number\",\"default\":0.382},\"wave3MinExtension\":{\"type\":\"number\",\"default\":1.618},\"minWaveSeparation\":{\"type\":\"integer\",\"default\":5},\"lookaround\":{\"type\":\"integer\",\"default\":2},\"retracementTolerance\":{\"type\":\"number\",\"default\":0.1}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "find_peaks", name: "Find Peaks", category: "patterns", description: "Locates local maxima in price data", params_schema: "{\"type\":\"object\",\"properties\":{\"lookaround\":{\"type\":\"integer\",\"description\":\"Bars on each side to confirm peak\",\"default\":2}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "find_troughs", name: "Find Troughs", category: "patterns", description: "Locates local minima in price data", params_schema: "{\"type\":\"object\",\"properties\":{\"lookaround\":{\"type\":\"integer\",\"description\":\"Bars on each side to confirm trough\",\"default\":2}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "linear_regression", name: "Linear Regression", category: "patterns", description: "Linear regression slope and intercept", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "flags_pennants", name: "Flags and Pennants", category: "patterns", description: "Flag and pennant pattern recognition", params_schema: "{\"type\":\"object\",\"properties\":{\"poleLength\":{\"type\":\"integer\",\"default\":10},\"consolidationBars\":{\"type\":\"integer\",\"default\":10},\"breakoutThreshold\":{\"type\":\"number\",\"default\":0.02}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "head_and_shoulders", name: "Head and Shoulders", category: "patterns", description: "Head and shoulders pattern recognition", params_schema: "{\"type\":\"object\",\"properties\":{\"minDistance\":{\"type\":\"integer\",\"default\":5},\"tolerance\":{\"type\":\"number\",\"default\":0.02},\"deviation\":{\"type\":\"number\",\"default\":0}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "stars", name: "Stars", category: "patterns", description: "Morning star and evening star patterns", params_schema: "{\"type\":\"object\",\"properties\":{\"bodyRatioThreshold\":{\"type\":\"number\",\"default\":0.3},\"gapThreshold\":{\"type\":\"number\",\"default\":0.001}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "triangles", name: "Triangles", category: "patterns", description: "Triangle pattern recognition", params_schema: "{\"type\":\"object\",\"properties\":{\"minPoints\":{\"type\":\"integer\",\"default\":4},\"tolerance\":{\"type\":\"number\",\"default\":0.01},\"convergenceTolerance\":{\"type\":\"number\",\"default\":0.001}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "wedges", name: "Wedges", category: "patterns", description: "Wedge pattern recognition", params_schema: "{\"type\":\"object\",\"properties\":{\"minPoints\":{\"type\":\"integer\",\"default\":4},\"slopeTolerance\":{\"type\":\"number\",\"default\":0.0001}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "zig_zag_filter", name: "Zig Zag Filter", category: "patterns", description: "Filters out noise by percentage deviation", params_schema: "{\"type\":\"object\",\"properties\":{\"deviation\":{\"type\":\"number\",\"description\":\"Minimum deviation to register a pivot\",\"default\":0.01}}}", output_type: "indicator" }
}

// ── chart patterns (added) ───────────────────────

inventory::submit! {
	IndicatorDescriptor { id: "diamond_top", name: "Diamond Top", category: "patterns", description: "Diamond top reversal pattern: broadening then contracting range with downside breakout", params_schema: "{\"type\":\"object\",\"properties\":{\"minPoints\":{\"type\":\"integer\",\"default\":2},\"tolerance\":{\"type\":\"number\",\"default\":0.0005},\"breakoutThreshold\":{\"type\":\"number\",\"default\":0},\"lookback\":{\"type\":\"integer\",\"default\":150}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "diamond_bottom", name: "Diamond Bottom", category: "patterns", description: "Diamond bottom reversal pattern: broadening then contracting range with upside breakout", params_schema: "{\"type\":\"object\",\"properties\":{\"minPoints\":{\"type\":\"integer\",\"default\":2},\"tolerance\":{\"type\":\"number\",\"default\":0.0005},\"breakoutThreshold\":{\"type\":\"number\",\"default\":0},\"lookback\":{\"type\":\"integer\",\"default\":150}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "triple_top", name: "Triple Top", category: "patterns", description: "Triple top reversal pattern: three peaks at similar levels followed by breakdown", params_schema: "{\"type\":\"object\",\"properties\":{\"tolerance\":{\"type\":\"number\",\"default\":0.03},\"minSeparation\":{\"type\":\"integer\",\"default\":8},\"lookaround\":{\"type\":\"integer\",\"default\":2}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "triple_bottom", name: "Triple Bottom", category: "patterns", description: "Triple bottom reversal pattern: three troughs at similar levels followed by breakout", params_schema: "{\"type\":\"object\",\"properties\":{\"tolerance\":{\"type\":\"number\",\"default\":0.03},\"minSeparation\":{\"type\":\"integer\",\"default\":8},\"lookaround\":{\"type\":\"integer\",\"default\":2}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "rectangle", name: "Rectangle", category: "patterns", description: "Rectangle continuation pattern: horizontal range breakout in the direction of the prior trend", params_schema: "{\"type\":\"object\",\"properties\":{\"minPoints\":{\"type\":\"integer\",\"default\":3},\"slopeTolerance\":{\"type\":\"number\",\"default\":0.0002},\"minSpread\":{\"type\":\"number\",\"default\":0.01},\"lookback\":{\"type\":\"integer\",\"default\":120},\"trendBars\":{\"type\":\"integer\",\"default\":30},\"minTrend\":{\"type\":\"number\",\"default\":0.03}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "channels", name: "Price Channels", category: "patterns", description: "Rising/falling parallel price channels with breakout in channel direction", params_schema: "{\"type\":\"object\",\"properties\":{\"minPoints\":{\"type\":\"integer\",\"default\":3},\"minSlope\":{\"type\":\"number\",\"default\":0.0005},\"parallelismTolerance\":{\"type\":\"number\",\"default\":0.5},\"lookback\":{\"type\":\"integer\",\"default\":120}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "broadening", name: "Broadening Pattern", category: "patterns", description: "Broadening/megaphone pattern: diverging trendlines with breakout in either direction", params_schema: "{\"type\":\"object\",\"properties\":{\"minPoints\":{\"type\":\"integer\",\"default\":3},\"tolerance\":{\"type\":\"number\",\"default\":0.0005},\"lookback\":{\"type\":\"integer\",\"default\":120}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "rounding_bottom", name: "Rounding Bottom", category: "patterns", description: "Rounding bottom/saucer reversal pattern with breakout above the rim", params_schema: "{\"type\":\"object\",\"properties\":{\"curvatureTolerance\":{\"type\":\"number\",\"default\":0.01},\"lookback\":{\"type\":\"integer\",\"default\":120}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "rounding_top", name: "Rounding Top", category: "patterns", description: "Rounding top reversal pattern with breakdown below the floor", params_schema: "{\"type\":\"object\",\"properties\":{\"curvatureTolerance\":{\"type\":\"number\",\"default\":0.01},\"lookback\":{\"type\":\"integer\",\"default\":120}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "bump_and_run", name: "Bump and Run Reversal", category: "patterns", description: "Bump-and-run reversal: steep lead-in trendline, bump away, and return through the line", params_schema: "{\"type\":\"object\",\"properties\":{\"leadInBars\":{\"type\":\"integer\",\"default\":20},\"minSlope\":{\"type\":\"number\",\"default\":0.001},\"bumpThreshold\":{\"type\":\"number\",\"default\":0.03},\"lookback\":{\"type\":\"integer\",\"default\":80}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "island_reversal", name: "Island Reversal", category: "patterns", description: "Island reversal: gap away, isolated cluster, then covering gap back", params_schema: "{\"type\":\"object\",\"properties\":{\"minIslandBars\":{\"type\":\"integer\",\"default\":2},\"maxIslandBars\":{\"type\":\"integer\",\"default\":15}}}", output_type: "indicator" }
}

// ── candlestick patterns (added) ─────────────────

inventory::submit! {
	IndicatorDescriptor { id: "hammer", name: "Hammer", category: "patterns", description: "Hammer candlestick: long lower shadow after a decline", params_schema: "{\"type\":\"object\",\"properties\":{\"bodyRatio\":{\"type\":\"number\",\"default\":0.3},\"shadowMultiplier\":{\"type\":\"number\",\"default\":2},\"trendBars\":{\"type\":\"integer\",\"default\":5}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "inverted_hammer", name: "Inverted Hammer", category: "patterns", description: "Inverted hammer candlestick: long upper shadow after a decline", params_schema: "{\"type\":\"object\",\"properties\":{\"bodyRatio\":{\"type\":\"number\",\"default\":0.3},\"shadowMultiplier\":{\"type\":\"number\",\"default\":2},\"trendBars\":{\"type\":\"integer\",\"default\":5}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "hanging_man", name: "Hanging Man", category: "patterns", description: "Hanging man candlestick: hammer shape after an advance", params_schema: "{\"type\":\"object\",\"properties\":{\"bodyRatio\":{\"type\":\"number\",\"default\":0.3},\"shadowMultiplier\":{\"type\":\"number\",\"default\":2},\"trendBars\":{\"type\":\"integer\",\"default\":5}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "shooting_star", name: "Shooting Star", category: "patterns", description: "Shooting star candlestick: inverted-hammer shape after an advance", params_schema: "{\"type\":\"object\",\"properties\":{\"bodyRatio\":{\"type\":\"number\",\"default\":0.3},\"shadowMultiplier\":{\"type\":\"number\",\"default\":2},\"trendBars\":{\"type\":\"integer\",\"default\":5}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "spinning_top", name: "Spinning Top", category: "patterns", description: "Spinning top candlestick: indecision with small body and both shadows", params_schema: "{\"type\":\"object\",\"properties\":{\"bodyRatio\":{\"type\":\"number\",\"default\":0.3},\"trendBars\":{\"type\":\"integer\",\"default\":5}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "long_legged_doji", name: "Long Legged Doji", category: "patterns", description: "Doji with long shadows on both sides, direction follows prior trend", params_schema: "{\"type\":\"object\",\"properties\":{\"bodyRatio\":{\"type\":\"number\",\"default\":0.1},\"shadowMultiplier\":{\"type\":\"number\",\"default\":2},\"trendBars\":{\"type\":\"integer\",\"default\":5}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "dragonfly_doji", name: "Dragonfly Doji", category: "patterns", description: "Doji with long lower shadow after a decline", params_schema: "{\"type\":\"object\",\"properties\":{\"bodyRatio\":{\"type\":\"number\",\"default\":0.1},\"shadowMultiplier\":{\"type\":\"number\",\"default\":2},\"trendBars\":{\"type\":\"integer\",\"default\":5}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "gravestone_doji", name: "Gravestone Doji", category: "patterns", description: "Doji with long upper shadow after an advance", params_schema: "{\"type\":\"object\",\"properties\":{\"bodyRatio\":{\"type\":\"number\",\"default\":0.1},\"shadowMultiplier\":{\"type\":\"number\",\"default\":2},\"trendBars\":{\"type\":\"integer\",\"default\":5}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "bullish_harami", name: "Bullish Harami", category: "patterns", description: "Small bullish candle inside a prior bearish body", params_schema: "{\"type\":\"object\",\"properties\":{\"bodyRatio\":{\"type\":\"number\",\"default\":0.5}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "bearish_harami", name: "Bearish Harami", category: "patterns", description: "Small bearish candle inside a prior bullish body", params_schema: "{\"type\":\"object\",\"properties\":{\"bodyRatio\":{\"type\":\"number\",\"default\":0.5}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "piercing_line", name: "Piercing Line", category: "patterns", description: "Bullish candle closes above midpoint of prior bearish body after gapping below", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "dark_cloud_cover", name: "Dark Cloud Cover", category: "patterns", description: "Bearish candle closes below midpoint of prior bullish body after gapping above", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "tweezer_bottom", name: "Tweezer Bottom", category: "patterns", description: "Two candles with equal lows after a decline", params_schema: "{\"type\":\"object\",\"properties\":{\"shadowTolerance\":{\"type\":\"number\",\"default\":0.001}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "tweezer_top", name: "Tweezer Top", category: "patterns", description: "Two candles with equal highs after an advance", params_schema: "{\"type\":\"object\",\"properties\":{\"shadowTolerance\":{\"type\":\"number\",\"default\":0.001}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "three_white_soldiers", name: "Three White Soldiers", category: "patterns", description: "Three consecutive strong bullish candles with rising closes", params_schema: "{\"type\":\"object\",\"properties\":{\"minBodyRatio\":{\"type\":\"number\",\"default\":0.5}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "three_black_crows", name: "Three Black Crows", category: "patterns", description: "Three consecutive strong bearish candles with falling closes", params_schema: "{\"type\":\"object\",\"properties\":{\"minBodyRatio\":{\"type\":\"number\",\"default\":0.5}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "three_inside_up", name: "Three Inside Up", category: "patterns", description: "Bearish candle, small bullish inside, then bullish close above first open", params_schema: "{\"type\":\"object\",\"properties\":{\"minBodyRatio\":{\"type\":\"number\",\"default\":0.3}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "three_inside_down", name: "Three Inside Down", category: "patterns", description: "Bullish candle, small bearish inside, then bearish close below first close", params_schema: "{\"type\":\"object\",\"properties\":{\"minBodyRatio\":{\"type\":\"number\",\"default\":0.3}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "three_outside_up", name: "Three Outside Up", category: "patterns", description: "Bearish candle engulfed by bullish, then bullish close higher", params_schema: "{\"type\":\"object\",\"properties\":{\"minBodyRatio\":{\"type\":\"number\",\"default\":0.5}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "three_outside_down", name: "Three Outside Down", category: "patterns", description: "Bullish candle engulfed by bearish, then bearish close lower", params_schema: "{\"type\":\"object\",\"properties\":{\"minBodyRatio\":{\"type\":\"number\",\"default\":0.5}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "abandoned_baby", name: "Abandoned Baby", category: "patterns", description: "Gap, doji star, and covering gap in the opposite direction", params_schema: "{\"type\":\"object\",\"properties\":{\"bodyRatio\":{\"type\":\"number\",\"default\":0.1},\"minBodyRatio\":{\"type\":\"number\",\"default\":0.3}}}", output_type: "indicator" }
}

// ── market ────────────────────────────────────────

inventory::submit! {
	IndicatorDescriptor { id: "advance_decline_line", name: "Advance-Decline Line", category: "market", description: "Cumulative breadth indicator", params_schema: "", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "mcclellan_oscillator", name: "McClellan Oscillator", category: "market", description: "Market breadth oscillator from advance-decline data", params_schema: "", output_type: "indicator" }
}

// ── shared ────────────────────────────────────────

inventory::submit! {
	IndicatorDescriptor { id: "cointegration", name: "Cointegration", category: "shared", description: "Cointegration test between two series", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":20},\"betaPeriod\":{\"type\":\"integer\",\"default\":60}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "correlation", name: "Correlation", category: "shared", description: "Correlation coefficient between two series", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"Rolling window for correlation\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "pearson_correlation", name: "Pearson Correlation", category: "shared", description: "Pearson product-moment correlation", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"description\":\"Rolling window for correlation\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "percent_rank", name: "Percent Rank", category: "shared", description: "Percent rank of current value in lookback window", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "percentile_linear_interpolation", name: "Percentile Linear Interpolation", category: "shared", description: "Percentile using linear interpolation", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14},\"percentage\":{\"type\":\"number\",\"default\":50}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "percentile_nearest_rank", name: "Percentile Nearest Rank", category: "shared", description: "Percentile using nearest rank", params_schema: "{\"type\":\"object\",\"properties\":{\"period\":{\"type\":\"integer\",\"default\":14},\"percentage\":{\"type\":\"number\",\"default\":50}}}", output_type: "indicator" }
}
inventory::submit! {
	IndicatorDescriptor { id: "value_when", name: "Value When", category: "shared", description: "Returns value from one series when a condition is met in another", params_schema: "{\"type\":\"object\",\"properties\":{\"occurrence\":{\"type\":\"integer\",\"default\":1}}}", output_type: "indicator" }
}
