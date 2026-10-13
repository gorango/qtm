import type { FundamentalPoint } from '@quantamental/core'
import type { EnrichOptions, FundamentalEnricher, Provenance, ProviderResult } from '../provider'
import { createSecClient, type SecClient } from './client'
import { extractReitMetrics } from './extract'

export * from './client'
export { extractReitMetrics, type ExtractOptions, type ReportedReit } from './extract'

const REIT_FORMS = ['10-K', '10-Q']

export interface SecReitEnricherOptions {
	client: SecClient
	/** Injectable clock, for deterministic provenance in tests. */
	now?: () => Date
}

/**
 * Overlay *reported* FFO/AFFO onto a panel built by a fundamentals provider,
 * replacing the Tier-1 proxy. Finds the latest 10-K/10-Q on EDGAR, reads the
 * reconciliation's per-share lines, and writes them to the point covering the
 * filing's period.
 */
export function createSecReitEnricher(options: SecReitEnricherOptions): FundamentalEnricher {
	const { client, now = () => new Date() } = options
	const id = 'sec'

	return {
		id,
		async enrich(
			points: FundamentalPoint[],
			enrichOptions: EnrichOptions = {},
		): Promise<ProviderResult<FundamentalPoint[]>> {
			const warnings: string[] = []
			const cloned = points.map((point) => ({ ...point, data: { ...point.data } }))
			const provenance = (estimated: boolean, sources?: string[]): Provenance => ({
				provider: id,
				retrievedAt: now().toISOString(),
				estimated,
				sources,
				warnings,
			})

			const symbol = enrichOptions.symbol ?? cloned[0]?.symbol
			if (!symbol || cloned.length === 0) {
				warnings.push('no symbol available to locate a filing')
				return { data: cloned, provenance: provenance(true) }
			}

			const filings = await client.findFilings(symbol, {
				forms: REIT_FORMS,
				limit: 1,
				signal: enrichOptions.signal,
			})
			const filing = filings[0]
			if (!filing) {
				warnings.push(`no 10-K/10-Q found for ${symbol}`)
				return { data: cloned, provenance: provenance(true) }
			}

			let html: string
			try {
				html = await client.fetchDocument(filing.url, { signal: enrichOptions.signal })
			} catch {
				warnings.push(`failed to fetch ${filing.form} filed ${filing.filingDate}`)
				return { data: cloned, provenance: provenance(true, [filing.url]) }
			}

			const reported = extractReitMetrics(html)
			warnings.push(...reported.warnings)

			const target = pointForReportDate(cloned, filing.reportDate) ?? cloned[cloned.length - 1]
			if (reported.ffoPerShare !== undefined) target.data.ffoPerShare = reported.ffoPerShare
			if (reported.affoPerShare !== undefined) {
				target.data.affoPerShare = reported.affoPerShare
				const dividend = target.data.forwardAnnualDividendRate
				if (dividend !== undefined && reported.affoPerShare > 0) {
					target.data.payoutRatioFfo = dividend / reported.affoPerShare
				}
			}

			const estimated = reported.ffoPerShare === undefined && reported.affoPerShare === undefined
			return { data: cloned, provenance: provenance(estimated, [filing.url]) }
		},
	}
}

/** Enricher gated on `SEC_USER_AGENT`; returns `undefined` when unconfigured. */
export function createConfiguredSecReitEnricher(): FundamentalEnricher | undefined {
	if (!process.env.SEC_USER_AGENT) return undefined
	return createSecReitEnricher({ client: createSecClient() })
}

function pointForReportDate(
	points: FundamentalPoint[],
	reportDate: string,
): FundamentalPoint | undefined {
	if (!reportDate) return undefined
	return points.find((point) => new Date(point.date).toISOString().slice(0, 10) === reportDate)
}
