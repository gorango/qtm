import type { FundamentalPoint } from '@quantamental/core'
import type { EnrichOptions, FundamentalEnricher, Provenance, ProviderResult } from '../provider'
import { createSecClient, type SecClient } from './client'
import { extractReitMetrics, perShareFromTotal } from './extract'

export * from './client'
export { extractReitMetrics, perShareFromTotal, type ExtractOptions, type ReportedReit, type ReportedTotals } from './extract'

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
			const shares = target.data.sharesOutstanding

			// Preferred: the reported per-share line. Otherwise derive it from the
			// reported total and the period's share count.
			const ffoPerShare =
				reported.ffoPerShare ??
				(reported.totals.ffo !== undefined && shares !== undefined
					? perShareFromTotal(reported.totals.ffo, shares)
					: undefined)
			const affoPerShare =
				reported.affoPerShare ??
				(reported.totals.affo !== undefined && shares !== undefined
					? perShareFromTotal(reported.totals.affo, shares)
					: undefined)

			let derived = false
			if (ffoPerShare !== undefined) {
				if (reported.ffoPerShare === undefined) derived = true
				target.data.ffoPerShare = ffoPerShare
			}
			if (affoPerShare !== undefined) {
				if (reported.affoPerShare === undefined) derived = true
				target.data.affoPerShare = affoPerShare
				const dividend = target.data.forwardAnnualDividendRate
				if (dividend !== undefined && affoPerShare > 0) {
					target.data.payoutRatioFfo = dividend / affoPerShare
				}
			}
			if (derived) {
				warnings.push('per-share derived: reported total divided by shares outstanding')
			}

			const found = reported.ffoPerShare !== undefined || reported.affoPerShare !== undefined
			return { data: cloned, provenance: provenance(!found, [filing.url]) }
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
