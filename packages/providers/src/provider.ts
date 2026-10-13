import type { Bar, FundamentalPoint } from '@quantamental/core'

export type Period = 'annual' | 'quarter'

export interface FetchOptions {
	/** Reporting period of the fundamentals to fetch. Defaults to `quarter`. */
	period?: Period
	/** Maximum number of periods to return, newest first from the source. */
	limit?: number
	signal?: AbortSignal
}

/**
 * Where a result came from, and how much to trust it. The factor/strategy
 * outputs (`FactorPoint`) have nowhere to carry this, so callers that care must
 * read it off the provider result rather than the factor result.
 */
export interface Provenance {
	provider: string
	/** ISO-8601 timestamp of retrieval. */
	retrievedAt: string
	/** True when any value was modelled rather than reported (e.g. proxy AFFO). */
	estimated: boolean
	warnings: string[]
}

export interface ProviderResult<T> {
	data: T
	provenance: Provenance
}

/** Normalizes an external source into quantamental's `FundamentalPoint` contract. */
export interface FundamentalProvider {
	readonly id: string
	fetchFundamentals(
		symbol: string,
		options?: FetchOptions,
	): Promise<ProviderResult<FundamentalPoint[]>>
}

/** Normalizes an external source into quantamental's `Bar` contract. */
export interface PriceProvider {
	readonly id: string
	fetchBars(
		symbol: string,
		options?: { from?: string; to?: string; signal?: AbortSignal },
	): Promise<ProviderResult<Bar[]>>
}

export class ProviderError extends Error {
	readonly provider: string

	constructor(provider: string, message: string, options?: { cause?: unknown }) {
		super(message, options)
		this.name = 'ProviderError'
		this.provider = provider
	}
}
