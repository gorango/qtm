# Quantamental MCP Server

[MCP](https://modelcontextprotocol.io) server exposing quantamental analysis tools — technical indicators, fundamental factors, and trading strategies — over the Model Context Protocol. Built on `@quantamental/tools` and the native Rust core (`@quantamental/core`); all tools (indicators, factors, strategies) are available to any MCP client.

## Installation

```bash
bun add -g @quantamental/mcp
```

Or run directly without installing:

```bash
bunx @quantamental/mcp
```

## Setup

After a global install, the `quantamental-mcp` binary is on your PATH. Configure it in your AI client (e.g., Claude Desktop, VS Code settings, `opencode.json`):

```json
{
	"mcpServers": {
		"quantamental": {
			"command": "quantamental-mcp"
		}
	}
}
```

Or, without a global install, run it on demand:

```json
{
	"mcpServers": {
		"quantamental": {
			"command": "bunx",
			"args": ["@quantamental/mcp"]
		}
	}
}
```

No environment variables are required — all computation runs locally against the native Rust core.

## Tools

Tool names are prefixed by category (`strategy_`, `indicator_`, `factor_`) because 35 ids are shared by the indicator and strategy categories (e.g. `rsi` is both an indicator and a strategy).

| Category   | Count | Input                               | Dispatch                              |
| ---------- | ----- | ----------------------------------- | ------------------------------------- |
| Indicators | 177   | `values: number[]`                  | `core[id]` or `core[camelCase(id)]`   |
| Factors    | 72    | `fundamentals: FundamentalPoint[]`  | `core[id]` or `core[camelCase(id)]`   |
| Strategies | 112   | `closes: number[]` + OHLCV + config | `core.runStrategy(id, input, config)` |

### Strategies

Take OHLCV arrays (`closes` required; `highs`, `lows`, `volumes` optional) and an optional `config` object whose keys match the strategy's params schema (camelCase, e.g. `{ fastPeriod: 10, slowPeriod: 20 }`). Return a signal array (`1` = buy, `-1` = sell, `0` = hold).

### Indicators

Take a `values` array (oldest first) and dispatch to the matching native export by id (e.g. `rsi` → `core.rsi`, `awesome_oscillator` → `core.awesomeOscillator`). Return indicator-specific result objects.

### Factors

Take an array of `FundamentalPoint` objects and return `FactorPoint[]` (`{ symbol, date, value }`). Each fundamental point has `symbol`, `date`, `filingDate`, `period`, and a `data` object with financial fields (all camelCase, e.g. `marketCap`, `shareholdersEquity`, `eps`, `revenue`).

Most factors follow that shape, but a few expect different inputs, passed through the same `fundamentals` argument:

- **5 factors also accept a `prices` array** of `Bar` objects (`{ time, open, high, low, close, volume }`): `analyst_target_upside`, `dividend_yield`, `margin_of_safety`, `price_to_affo`, `price_to_earnings`.
- **4 on-chain factors** take an array of `OnChainDataPoint` objects (`{ time, metric, value }`) as `fundamentals`: `active_address_growth`, `exchange_flow_momentum`, `nvt_ratio`, `staking_ratio`.
- **2 prediction-market factors** take an array of `PredictionMarketPoint` objects (`{ time, marketId, price, volume }`) as `fundamentals`: `odds_momentum`, `prediction_market_odds`.

Example:

```json
{
	"name": "factor_price_to_book",
	"arguments": {
		"fundamentals": [
			{
				"symbol": "AAPL",
				"date": 1000,
				"filingDate": 1000,
				"period": "Q1",
				"data": { "marketCap": 300000, "shareholdersEquity": 150000 }
			}
		]
	}
}
```

## Development

```bash
bun run typecheck
bun run test
```
