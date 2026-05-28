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
						format!("unknown attribute key: {}", key),
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
		_ => panic!("unknown OHLCV param: {}", name),
	}
}

/// Marks a function as a strategy, auto-registering it in the strategy registry.
///
/// # Attributes
/// - `id` — unique strategy identifier (e.g. `"rsi"`)
/// - `name` — human-readable name
/// - `category` — strategy category (e.g. `"momentum"`, `"trend"`)
/// - `default_timeframes` — array of default timeframe strings
/// - `description` — brief description
/// - `opt_params` — optional JSON string for optimization bounds
///
/// The annotated function must have OHLCV `&[f64]` params followed by
/// `config: Option<ConfigType>`. Recognized OHLCV names: `opens`, `highs`,
/// `lows`, `closes`, `volumes`.
#[proc_macro_attribute]
pub fn strategy(attr: TokenStream, item: TokenStream) -> TokenStream {
	let attr = parse_macro_input!(attr as StrategyAttr);
	let input_fn = parse_macro_input!(item as ItemFn);

	let fn_name = extract_fn_name(&input_fn);
	let fn_name_str = fn_name.to_string();
	let wrapped_fn_name = Ident::new(&format!("{}_wrapped", fn_name_str), fn_name.span());
	let metadata_fn_name = Ident::new(&format!("{}_metadata", fn_name_str), fn_name.span());
	let defaults_fn_name = Ident::new(&format!("{}_defaults", fn_name_str), fn_name.span());

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
				.map(|c| serde_json::from_value::<#config_type>(c).unwrap_or_default());
			#fn_name(#(#ohlcv_args),*, config)
		}

		inventory::submit! {
			crate::registry::StrategyDescriptor {
				id: #id,
				name: #name,
				category: #category,
				default_timeframes: &[#(#default_timeframes),*],
				description: #description,
				handler: #wrapped_fn_name,
			}
		}
	};

	TokenStream::from(expanded)
}
