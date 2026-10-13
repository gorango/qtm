import type { FetchOptions, FundamentalProvider, ProviderResult } from '../provider'
import { ProviderError } from '../provider'
import { toFundamentalPoints } from './map'
import type { FmpBundle, FmpClient } from './types'

export * from './types'
export { createFmpClient, type FmpClientOptions } from './client'
export { toFundamentalPoints, type MapResult } from './map'
export {
	deriveReitMetrics,
	forwardAnnualDividendRate,
	frequencyMultiplier,
	trailingAnnualDividend,
	type ReitInputs,
	type ReitMetrics,
} from './enrich'

export interface FmpProviderOptions {
	client: FmpClient
	/** Injectable clock, for deterministic provenance in tests. */
	now?: () => Date
}

/**
 * Build a `FundamentalProvider` backed by FMP.
 *
 * The income statement is required; everything else is best-effort — a failed
 * snapshot endpoint degrades the panel rather than failing the call, and the
 * reason is recorded in `provenance.warnings`.
 */
export function createFmpProvider(options: FmpProviderOptions): FundamentalProvider {
	const { client, now = () => new Date() } = options
	const id = 'fmp'

	return {
		id,
		async fetchFundamentals(
			symbol: string,
			fetchOptions: FetchOptions = {},
		): Promise<ProviderResult<import('@quantamental/core').FundamentalPoint[]>> {
			if (!symbol) throw new ProviderError(id, 'symbol is required')

			const { period = 'quarter', limit, signal } = fetchOptions
			const statementOptions = { period, limit, signal }
			const warnings: string[] = []

			async function optional<T>(label: string, run: () => Promise<T>): Promise<T | undefined> {
				try {
					return await run()
				} catch {
					warnings.push(`${label} unavailable`)
					return undefined
				}
			}

			let income
			try {
				income = await client.incomeStatement(symbol, statementOptions)
			} catch (cause) {
				throw new ProviderError(id, `failed to fetch income statement for ${symbol}`, { cause })
			}

			const [balance, cashFlow, keyMetrics, ratios, ownerEarnings] = await Promise.all([
				optional('balance sheet', () => client.balanceSheetStatement(symbol, statementOptions)),
				optional('cash flow', () => client.cashFlowStatement(symbol, statementOptions)),
				optional('key metrics', () => client.keyMetrics(symbol, statementOptions)),
				optional('ratios', () => client.financialRatios(symbol, statementOptions)),
				optional('owner earnings', () => client.ownerEarnings(symbol, statementOptions)),
			])

			const [quote, profile, priceTarget, rating, dcf, dividends] = await Promise.all([
				optional('quote', () => client.stockQuote(symbol)),
				optional('profile', () => client.companyProfile(symbol)),
				optional('price target', () => client.priceTargetConsensus(symbol)),
				optional('ratings', () => client.ratingsSnapshot(symbol)),
				optional('dcf', () => client.dcfValuation(symbol)),
				optional('dividends', () => client.companyDividends(symbol)),
			])

			const bundle: FmpBundle = {
				income,
				balance,
				cashFlow,
				keyMetrics,
				ratios,
				ownerEarnings,
				quote,
				profile,
				priceTarget,
				rating,
				dcf,
				dividends,
			}

			const result = toFundamentalPoints(bundle)

			return {
				data: result.points,
				provenance: {
					provider: id,
					retrievedAt: now().toISOString(),
					estimated: result.estimated,
					warnings: [...result.warnings, ...warnings],
				},
			}
		},
	}
}
