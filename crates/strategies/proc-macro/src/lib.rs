use proc_macro::TokenStream;
use quote::quote;
use syn::{
	parse::{Parse, ParseStream},
	parse_macro_input,
	punctuated::Punctuated,
	Expr, Ident, ItemFn, Token, Type,
};

const OHLCV_NAMES: &[&str] = &["opens", "highs", "lows", "closes", "volumes"];

struct KvPair {
	key: Ident,
	_eq_token: Token![=],
	value: Expr,
}

impl Parse for KvPair {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(KvPair {
			key: input.parse()?,
			_eq_token: input.parse()?,
			value: input.parse()?,
		})
	}
}

struct StrategyAttr {
	id: String,
	name: String,
	category: String,
	default_timeframes: Vec<String>,
	description: String,
	opt_params: Option<String>,
}

impl Parse for StrategyAttr {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let pairs = Punctuated::<KvPair, Token![,]>::parse_terminated(input)?;
		let mut id = None;
		let mut name = None;
		let mut category = None;
		let mut default_timeframes: Option<Vec<String>> = None;
		let mut description = None;
		let mut opt_params = None;

		for pair in pairs {
			let key = pair.key.to_string();
			match key.as_str() {
				"id" => id = Some(expr_to_string(&pair.value)?),
				"name" => name = Some(expr_to_string(&pair.value)?),
				"category" => category = Some(expr_to_string(&pair.value)?),
				"default_timeframes" => {
					default_timeframes = Some(expr_to_string_array(&pair.value)?);
				}
				"description" => description = Some(expr_to_string(&pair.value)?),
				"opt_params" => opt_params = Some(expr_to_string(&pair.value)?),
				_ => {
					return Err(syn::Error::new_spanned(
						&pair.key,
						format!("unknown attribute key: {key}"),
					));
				}
			}
		}

		Ok(StrategyAttr {
			id: id.ok_or_else(|| input.error("missing required attribute: id"))?,
			name: name.ok_or_else(|| input.error("missing required attribute: name"))?,
			category: category
				.ok_or_else(|| input.error("missing required attribute: category"))?,
			default_timeframes: default_timeframes
				.ok_or_else(|| input.error("missing required attribute: default_timeframes"))?,
			description: description
				.ok_or_else(|| input.error("missing required attribute: description"))?,
			opt_params,
		})
	}
}

fn expr_to_string(expr: &Expr) -> syn::Result<String> {
	match expr {
		Expr::Lit(lit) => match &lit.lit {
			syn::Lit::Str(s) => Ok(s.value()),
			_ => Err(syn::Error::new_spanned(expr, "expected string literal")),
		},
		_ => Err(syn::Error::new_spanned(expr, "expected string literal")),
	}
}

fn expr_to_string_array(expr: &Expr) -> syn::Result<Vec<String>> {
	match expr {
		Expr::Array(arr) => {
			let mut values = Vec::new();
			for elem in &arr.elems {
				values.push(expr_to_string(elem)?);
			}
			Ok(values)
		}
		_ => Err(syn::Error::new_spanned(expr, "expected array literal")),
	}
}

/// Extract OHLCV parameter names from the function signature in declaration order.
fn extract_ohlcv_param_names(input_fn: &ItemFn) -> Vec<String> {
	let mut names = Vec::new();
	for arg in &input_fn.sig.inputs {
		if let syn::FnArg::Typed(pat_type) = arg {
			if let syn::Pat::Ident(pat) = pat_type.pat.as_ref() {
				let name = pat.ident.to_string();
				if OHLCV_NAMES.contains(&name.as_str()) {
					names.push(name);
				}
			}
		}
	}
	names
}

/// Extract the config type from the last parameter (`Option<ConfigType>`).
fn extract_config_type(input_fn: &ItemFn) -> syn::Result<Option<Type>> {
	for arg in input_fn.sig.inputs.iter().rev() {
		if let syn::FnArg::Typed(pat_type) = arg {
			if let syn::Pat::Ident(pat) = pat_type.pat.as_ref() {
				if pat.ident == "config" || OHLCV_NAMES.contains(&pat.ident.to_string().as_str()) {
					// Skip OHLCV params — look for Option<...>
					if OHLCV_NAMES.contains(&pat.ident.to_string().as_str()) {
						continue;
					}
				}
			}
			if let Type::Path(type_path) = pat_type.ty.as_ref() {
				if let Some(seg) = type_path.path.segments.last() {
					if seg.ident == "Option" {
						if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
							if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
								return Ok(Some(inner.clone()));
							}
						}
					}
				}
			}
		}
	}
	Ok(None)
}

fn extract_fn_name(input_fn: &ItemFn) -> Ident {
	input_fn.sig.ident.clone()
}

/// Generate the field-access expression for a given OHLCV parameter name.
fn ohlcv_accessor(name: &str) -> proc_macro2::TokenStream {
	match name {
		"closes" => quote! { &input.closes },
		"highs" => quote! { input.highs.as_deref().unwrap_or(&input.closes) },
		"lows" => quote! { input.lows.as_deref().unwrap_or(&input.closes) },
		"opens" => quote! { input.opens.as_deref().unwrap_or(&input.closes) },
		"volumes" => quote! { input.volumes.as_deref().unwrap_or(&input.closes) },
		_ => panic!("unknown OHLCV param: {name}"),
	}
}

fn snake_to_camel(s: &str) -> String {
	let mut result = String::new();
	let mut capitalize_next = false;
	for c in s.chars() {
		if c == '_' {
			capitalize_next = true;
		} else if capitalize_next {
			result.push(c.to_ascii_uppercase());
			capitalize_next = false;
		} else {
			result.push(c);
		}
	}
	result
}

fn generate_napi_binding(
	fn_name: &Ident,
	fn_name_str: &str,
	ohlcv_params: &[String],
	config_type: &Type,
) -> proc_macro2::TokenStream {
	let napi_fn_name = Ident::new(&format!("{fn_name_str}_napi_binding"), fn_name.span());
	let js_name = snake_to_camel(fn_name_str);

	let napi_params: Vec<_> = ohlcv_params
		.iter()
		.map(|name| {
			let ident = Ident::new(name, fn_name.span());
			quote! { #ident: Vec<f64> }
		})
		.collect();

	let param_refs: Vec<_> = ohlcv_params
		.iter()
		.map(|name| {
			let ident = Ident::new(name, fn_name.span());
			quote! { &#ident }
		})
		.collect();

	quote! {
		#[cfg(feature = "napi")]
		#[::napi_derive::napi(js_name = #js_name)]
		pub fn #napi_fn_name(
			#(#napi_params),*,
			config: Option<#config_type>,
		) -> ::napi::Result<Vec<i8>> {
			#fn_name(#(#param_refs),*, config)
				.map_err(|e| ::napi::Error::new(
					::napi::Status::InvalidArg,
					e.to_string(),
				))
		}
	}
}

/// Annotates a function as a trading strategy, auto-registering it in the strategy
/// registry and generating NAPI bindings, metadata, defaults, and JSON Schema.
///
/// This is the primary extension point for adding new strategies. A single
/// `#[strategy(...)]` annotation replaces what previously required manual
/// registry entries, binding wrappers, and schema maintenance.
///
/// # Generated infrastructure
///
/// For a function `fn my_strategy(...)` annotated with `#[strategy(...)]`, this
/// macro generates:
///
/// | Generated item | Purpose |
/// |---|---|
/// | `my_strategy_metadata()` | Returns `{ id, name, category, default_timeframes, description }` as JSON |
/// | `my_strategy_defaults()` | Returns `{ params: <ConfigType::default()>, optimization_bounds: [...] }` as JSON |
/// | `my_strategy_params_schema()` | Lazily returns JSON Schema for `ConfigType` via `schemars` |
/// | `my_strategy_wrapped()` | Adapter that converts `StrategyInput` + `Option<serde_json::Value>` into the native function signature |
/// | `my_strategy_napi_binding()` | `#[napi]`-exported JS binding (behind `feature = "napi"`) |
/// | `inventory::submit!` | Registers a `StrategyDescriptor` into the global registry |
///
/// All generated items are `pub` in the defining module and collected at
/// link time by `inventory`.
///
/// # Attributes
///
/// | Attribute | Required | Description |
/// |---|---|---|
/// | `id` | ✅ | Unique strategy identifier (e.g. `"rsi"`, `"macd-crossover"`). Used as key in registries and LLM tool names. |
/// | `name` | ✅ | Human-readable display name (e.g. `"RSI Momentum Strategy"`) |
/// | `category` | ✅ | Grouping category (`"momentum"`, `"trend"`, `"volatility"`, `"volume"`, `"patterns"`, `"composite"`, `"statistics"`, `"special"`) |
/// | `default_timeframes` | ✅ | Default timeframes as a string array (e.g. `["15m", "1h", "4h"]`) |
/// | `description` | ✅ | Brief description of what the strategy does. This is passed directly to LLM tool definitions, so be descriptive. |
/// | `opt_params` | ❌ | JSON array of optimization parameter bounds for hyperparameter tuning. Each entry: `{"param_name": "...", "min": ..., "max": ..., "step": ...}` |
///
/// # Function signature requirements
///
/// The annotated function **must** follow this pattern:
///
/// ```ignore
/// #[strategy(
///     id = "my-strategy",
///     name = "My Strategy",
///     category = "momentum",
///     default_timeframes = ["15m", "1h", "4h"],
///     description = "Brief description for LLM tool definitions",
/// )]
/// pub fn my_strategy(
///     closes: &[f64],           // required — closing prices
///     highs: Option<&[f64]>,    // optional — high prices
///     lows: Option<&[f64]>,     // optional — low prices
///     opens: Option<&[f64]>,    // optional — open prices
///     volumes: Option<&[f64]>,  // optional — volume data
///     config: Option<MyConfig>, // required — config type, must impl Default + Serialize + schemars::JsonSchema
/// ) -> StrategyResult<Vec<i8>> // returns signals: 1=buy, -1=sell, 0=hold
/// ```
///
/// Recognized OHLCV parameter names: `opens`, `highs`, `lows`, `closes`,
/// `volumes`. Include only the ones your strategy needs — unused ones can be
/// omitted. The `wrapped` adapter fills missing data with `closes` as fallback.
///
/// # Config type requirements
///
/// The `ConfigType` (last `Option<...>` parameter) must:
/// - Derive `Default`, `Serialize`, `Deserialize`, `schemars::JsonSchema`
/// - Use `Option<...>` for all fields so that `Default::default()` produces
///   a valid "unset" state
///
/// See `crates/strategies/src/types/configs.rs` for examples.
///
/// # Example
///
/// ```ignore
/// use strategies_proc_macro::strategy;
///
/// #[strategy(
///     id = "rsi",
///     name = "RSI Momentum Strategy",
///     category = "momentum",
///     default_timeframes = ["15m", "1h", "4h"],
///     description = "Generates buy signals when RSI crosses above oversold and sell when RSI crosses below overbought",
///     opt_params = r#"[
///         {"param_name": "period", "min": 5.0, "max": 30.0, "step": 1.0},
///         {"param_name": "oversold", "min": 10.0, "max": 40.0, "step": 5.0},
///         {"param_name": "overbought", "min": 60.0, "max": 90.0, "step": 5.0}
///     ]"#
/// )]
/// pub fn rsi_strategy(closes: &[f64], config: Option<RSIConfig>) -> StrategyResult<Vec<i8>> {
///     // ... implementation ...
/// }
/// ```
#[proc_macro_attribute]
pub fn strategy(attr: TokenStream, item: TokenStream) -> TokenStream {
	let attr = parse_macro_input!(attr as StrategyAttr);
	let input_fn = parse_macro_input!(item as ItemFn);

	let fn_name = extract_fn_name(&input_fn);
	let fn_name_str = fn_name.to_string();
	let wrapped_fn_name = Ident::new(&format!("{fn_name_str}_wrapped"), fn_name.span());
	let metadata_fn_name = Ident::new(&format!("{fn_name_str}_metadata"), fn_name.span());
	let defaults_fn_name = Ident::new(&format!("{fn_name_str}_defaults"), fn_name.span());
	let params_schema_fn_name = Ident::new(&format!("{fn_name_str}_params_schema"), fn_name.span());

	let id = &attr.id;
	let name = &attr.name;
	let category = &attr.category;
	let description = &attr.description;
	let default_timeframes: Vec<_> = attr
		.default_timeframes
		.iter()
		.map(|s| quote! { #s })
		.collect();

	// Detect OHLCV params
	let ohlcv_params = extract_ohlcv_param_names(&input_fn);
	let ohlcv_args: Vec<_> = ohlcv_params.iter().map(|n| ohlcv_accessor(n)).collect();

	// Detect config type (last Option<...> param)
	let config_type = match extract_config_type(&input_fn) {
		Ok(Some(ty)) => ty,
		_ => {
			return syn::Error::new_spanned(
				&input_fn.sig,
				"strategy function must have a `config: Option<ConfigType>` parameter",
			)
			.to_compile_error()
			.into();
		}
	};

	// NAPI binding (auto-generated #[napi] wrapper)
	let napi_binding = generate_napi_binding(&fn_name, &fn_name_str, &ohlcv_params, &config_type);

	// Optimization bounds JSON string (or empty array)
	let opt_bounds = match &attr.opt_params {
		Some(json) => {
			quote! {
				serde_json::from_str::<serde_json::Value>(#json).unwrap_or_default()
			}
		}
		None => {
			quote! { serde_json::Value::Array(Default::default()) }
		}
	};

	let expanded = quote! {
		#input_fn

		pub fn #metadata_fn_name() -> serde_json::Value {
			serde_json::json!({
				"id": #id,
				"name": #name,
				"category": #category,
				"default_timeframes": [#(#default_timeframes),*],
				"description": #description,
			})
		}

		pub fn #defaults_fn_name() -> serde_json::Value {
			serde_json::json!({
				"params": serde_json::to_value(
					<#config_type as core::default::Default>::default()
				).unwrap_or(serde_json::Value::Null),
				"optimization_bounds": #opt_bounds,
			})
		}

		fn #wrapped_fn_name(
			input: &crate::registry::StrategyInput,
			config: Option<serde_json::Value>,
		) -> crate::StrategyResult<Vec<i8>> {
			let config = config
				.map(|c| {
					serde_json::from_value::<#config_type>(c).map_err(|e| {
						crate::StrategyError::ConfigError(format!(
							"Invalid config for {}: {e}",
							stringify!(#config_type)
						))
					})
				})
				.transpose()?;
			#fn_name(#(#ohlcv_args),*, config)
		}

		pub fn #params_schema_fn_name() -> &'static str {
			use ::std::sync::OnceLock;
			static SCHEMA: OnceLock<String> = OnceLock::new();
			SCHEMA.get_or_init(|| {
				::serde_json::to_string(
					&::schemars::schema_for!(#config_type)
				).expect("valid JSON Schema")
			})
		}

		#napi_binding

		inventory::submit! {
			crate::registry::StrategyDescriptor {
				id: #id,
				name: #name,
				category: #category,
				default_timeframes: &[#(#default_timeframes),*],
				description: #description,
				handler: #wrapped_fn_name,
				defaults_fn: #defaults_fn_name,
				params_schema_fn: #params_schema_fn_name,
				output_type: "signal",
			}
		}
	};

	TokenStream::from(expanded)
}
