# Quantamental Providers

Data providers that normalize external sources into quantamental's contracts —
`FundamentalPoint` and `Bar` — so the factor, indicator, and strategy tools can
consume them without any tool-side changes.

This package is pure I/O + normalization. It depends on `@quantamental/core` for
the contract *types* only; all computation still happens in the native core.

> Naming: these are **providers**, not "adapters". `@quantamental/tools` already
> uses "adapters" for its LLM-framework bindings (Anthropic/OpenAI/etc.).

## Install

Workspace package — `bun install` at the repo root links it.

## Usage

```ts
import { createFmpClient, createFmpProvider } from '@quantamental/providers/fmp'

const provider = createFmpProvider({ client: createFmpClient() }) // reads FMP_KEY
const { data, provenance } = await provider.fetchFundamentals('PLD', {
	period: 'quarter',
	limit: 40,
})

// `data` is FundamentalPoint[] — feed straight into any factor or strategy.
const points = await core.factorPriceToAffo(data)
```

The client is injectable, so if you already depend on `@financialmodelingprep/sdk`
you can pass that in instead — it satisfies the `FmpClient` interface structurally:

```ts
import * as fmp from '@financialmodelingprep/sdk'
const provider = createFmpProvider({ client: fmp })
```

## What it maps

| Source rows | Contract fields |
| --- | --- |
| `income-statement` | revenue, costOfRevenue, grossProfit, operatingIncome, netIncome, eps, ebitda, costAndExpenses, researchAndDevelopmentExpenses, interestExpense |
| `balance-sheet-statement` | totalAssets, currentAssets, currentLiabilities, totalLiabilities, totalDebt, shareholdersEquity, retainedEarnings, commonStock, cashAndEquivalents, propertyPlantEquipment |
| `cash-flow-statement` | operatingCashFlow, capitalExpenditure, dividendsPaid, shareRepurchases |
| `key-metrics` | marketCap, enterpriseValue |
| `ratios` | assetTurnover, dividendYield |
| `quote` / `profile` | sharesOutstanding, marketCap |
| `price-target-consensus` | analystTargetPrice |
| `ratings-snapshot` | rating |
| `discounted-cash-flow` | dcf |
| `dividends` | dividendsPerShare (trailing 365d), forwardAnnualDividendRate |
| `owner-earnings` | maintenance capex (input to AFFO) |

Per-period rows are joined on `date`. Point-in-time snapshots (`quote`,
`profile`, `price-target-consensus`, `ratings-snapshot`, `discounted-cash-flow`)
are applied to the **latest** period only.

## Derived REIT metrics — approximated

FMP's structured feed omits the FFO/AFFO components (impairment, gain on sale,
the maintenance-vs-growth capex split, straight-line rent), so `ffoPerShare`,
`affoPerShare`, and `payoutRatioFfo` are **proxies**:

```
FFO  = netIncome + depreciationAndAmortization − preferredDividendsPaid
AFFO = FFO − maintenanceCapex            # owner-earnings model value
```

Every result carries `provenance.estimated` and human-readable
`provenance.warnings`. `affoPerShare` is **annual** (matching `price_to_affo`,
`reit_dividend_safety`, and `high_yield_reit_strategy`).

## Reported FFO/AFFO (SEC, "Tier-2")

The proxies above can be replaced with the figures a REIT actually reported, by
reading the FFO/AFFO reconciliation from its latest 10-K/10-Q on EDGAR:

```ts
import { createSecClient, createSecReitEnricher } from '@quantamental/providers/sec'

const enricher = createSecReitEnricher({ client: createSecClient() }) // reads SEC_USER_AGENT
const { data, provenance } = await enricher.enrich(points)
// provenance.estimated === false when reported values were found
// provenance.sources === [filing url]
```

`SEC_USER_AGENT` is required (`"YourName your@email.com"`) and EDGAR is rate-limited
to ~10 req/s — filings are immutable, so cache them. Extraction is heuristic
(filings have no standard machine format for the reconciliation): it reads the
diluted per-share FFO/AFFO lines and reports what it matched, warning rather than
guessing when a value is absent.

## Dates

`date`, `filingDate`, and `Bar.time` are epoch **milliseconds**. Quantamental
only compares/sorts these, so any `prices` supplied alongside must use the same
unit.

## Development

```bash
bun run typecheck
bun run test
```
