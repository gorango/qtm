import type {
	FmpBalanceSheet,
	FmpCashFlowStatement,
	FmpClient,
	FmpCompanyProfile,
	FmpDcfValuation,
	FmpDividend,
	FmpFinancialRatios,
	FmpIncomeStatement,
	FmpKeyMetrics,
	FmpOwnerEarnings,
	FmpPriceTargetConsensus,
	FmpQuote,
	FmpRatingSnapshot,
	FmpStatementOptions,
} from './types'

export interface FmpClientOptions {
	/** Defaults to `process.env.FMP_KEY`. */
	apiKey?: string
	/** Defaults to the FMP stable API. */
	baseUrl?: string
	/** Injectable for testing; defaults to global `fetch`. */
	fetch?: typeof fetch
}

const DEFAULT_BASE_URL = 'https://financialmodelingprep.com/stable/'

/**
 * Minimal FMP HTTP client covering exactly the endpoints the provider reads.
 *
 * This exists so the package is usable standalone; if you already depend on
 * `@financialmodelingprep/sdk`, pass that client into `createFmpProvider`
 * instead — it satisfies `FmpClient` structurally.
 */
export function createFmpClient(options: FmpClientOptions = {}): FmpClient {
	const doFetch = options.fetch ?? globalThis.fetch
	const baseUrl = options.baseUrl ?? DEFAULT_BASE_URL

	function apiKey(): string {
		const key = options.apiKey ?? process.env.FMP_KEY
		if (!key) throw new Error('FMP_KEY environment variable is required')
		return key
	}

	async function get<T>(
		path: string,
		params: Record<string, string | number>,
		signal?: AbortSignal,
	): Promise<T> {
		const url = new URL(path, baseUrl)
		for (const [key, value] of Object.entries(params)) {
			url.searchParams.set(key, String(value))
		}
		url.searchParams.set('apikey', apiKey())

		const response = await doFetch(url, { signal })
		if (!response.ok) {
			// FMP puts the reason (plan limits, bad symbol) in the body.
			const detail = await response.text().catch(() => '')
			throw new Error(
				`FMP ${path} failed: ${response.status} ${response.statusText}${detail ? ` — ${detail.slice(0, 200)}` : ''}`,
			)
		}
		return (await response.json()) as T
	}

	// FMP's stable API accepts `period=quarter|annual` directly and returns
	// newest-first; only `limit` is needed to bound the history.
	async function statement<T extends { date: string }>(
		path: string,
		symbol: string,
		options: FmpStatementOptions = {},
	): Promise<T[]> {
		if (!symbol) throw new Error(`symbol is required for FMP ${path}`)
		const { period = 'quarter', limit, signal } = options

		const params: Record<string, string | number> = { symbol, period }
		if (limit) params.limit = limit
		const rows = await get<T[]>(path, params, signal)
		rows.sort((a, b) => b.date.localeCompare(a.date))
		return limit ? rows.slice(0, limit) : rows
	}

	return {
		incomeStatement: (symbol, options) =>
			statement<FmpIncomeStatement>('income-statement', symbol, options),
		balanceSheetStatement: (symbol, options) =>
			statement<FmpBalanceSheet>('balance-sheet-statement', symbol, options),
		cashFlowStatement: (symbol, options) =>
			statement<FmpCashFlowStatement>('cash-flow-statement', symbol, options),
		keyMetrics: (symbol, options) => statement<FmpKeyMetrics>('key-metrics', symbol, options),
		financialRatios: (symbol, options) => statement<FmpFinancialRatios>('ratios', symbol, options),
		ownerEarnings: (symbol, options) =>
			statement<FmpOwnerEarnings>('owner-earnings', symbol, options),
		companyProfile: (symbol) => get<FmpCompanyProfile[]>('profile', { symbol }),
		stockQuote: (symbol) => get<FmpQuote[]>('quote', { symbol }),
		priceTargetConsensus: (symbol) =>
			get<FmpPriceTargetConsensus[]>('price-target-consensus', { symbol }),
		ratingsSnapshot: (symbol) => get<FmpRatingSnapshot[]>('ratings-snapshot', { symbol }),
		dcfValuation: (symbol) => get<FmpDcfValuation[]>('discounted-cash-flow', { symbol }),
		companyDividends: (symbol) => get<FmpDividend[]>('dividends', { symbol }),
	}
}
