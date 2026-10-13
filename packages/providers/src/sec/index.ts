import type { FundamentalPoint } from '@quantamental/core'
import type { EnrichOptions, FundamentalEnricher, Provenance, ProviderResult } from '../provider'
import { createSecClient, type SecClient, type SecDocument, type SecFilingRef } from './client'
import { extractReitMetrics, perShareFromTotal, type ReportedReit } from './extract'

export * from './client'
export {
	extractReitMetrics,
	perShareFromTotal,
	type ExtractOptions,
	type ReportedReit,
	type ReportedTotals,
} from './extract'

const REIT_FORMS = ['10-K', '10-Q']
/** Earnings releases / supplements are EX-99 exhibits and large. */
const RELEASE_NAME = /ex.?99|supplement|earnings|results/i
const MIN_RELEASE_BYTES = 100_000
const MAX_8KS = 8

export interface SecReitEnricherOptions {
	client: SecClient
	/** Injectable clock, for deterministic provenance in tests. */
	now?: () => Date
}

/**
 * Overlay *reported* FFO/AFFO onto a panel built by a fundamentals provider,
 * replacing the Tier-1 proxy.
 *
 * Reads the latest 10-K/10-Q reconciliation; if a value is still missing it
 * falls back to the earnings release (an EX-99 exhibit on a recent 8-K), where
 * many REITs state "Core FFO per share" in prose. Values are written to the
 * point covering the filing's period.
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
			const sources: string[] = []
			const cloned = points.map((point) => ({ ...point, data: { ...point.data } }))
			const provenance = (estimated: boolean): Provenance => ({
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
			const signal = enrichOptions.signal

			// 1. The annual/quarterly filing.
			const filing = (
				await client.findFilings(symbol, {
					forms: REIT_FORMS,
					limit: 1,
					cik: enrichOptions.cik,
					signal,
				})
			)[0]
			let reported: ReportedReit | undefined
			if (filing) {
				try {
					const html = await client.fetchDocument(filing.url, { signal })
					reported = extractReitMetrics(html)
					sources.push(filing.url)
				} catch {
					warnings.push(`failed to fetch ${filing.form} filed ${filing.filingDate}`)
				}
			} else {
				warnings.push(
					`no 10-K/10-Q found for ${symbol} (EDGAR may lack a ticker; pass a CIK)`,
				)
			}

			const target =
				pointForReportDate(cloned, filing?.reportDate ?? '') ?? cloned[cloned.length - 1]
			const shares = target.data.sharesOutstanding

			let found = false
			let derived = false
			let ffo: number | undefined
			let affo: number | undefined

			if (reported) {
				warnings.push(...reported.warnings)
				found ||= reported.ffoPerShare !== undefined || reported.affoPerShare !== undefined
				const r = resolvePerShare(reported, shares)
				ffo = r.ffo
				affo = r.affo
				derived ||= r.derived
			}

			// 2. Earnings release fallback for whatever is still missing.
			if (ffo === undefined || affo === undefined) {
				const release = await findEarningsRelease(client, symbol, enrichOptions.cik, signal)
				if (release) {
					try {
						const html = await client.fetchDocument(release.url, { signal })
						const fromRelease = extractReitMetrics(html)
						sources.push(release.url)
						warnings.push(...fromRelease.warnings)
						found ||=
							fromRelease.ffoPerShare !== undefined || fromRelease.affoPerShare !== undefined
						const r = resolvePerShare(fromRelease, shares)
						ffo ??= r.ffo
						affo ??= r.affo
						derived ||= r.derived
					} catch {
						warnings.push('failed to fetch earnings release')
					}
				}
			}

			if (ffo !== undefined) target.data.ffoPerShare = ffo
			if (affo !== undefined) {
				target.data.affoPerShare = affo
				const dividend = target.data.forwardAnnualDividendRate
				if (dividend !== undefined && affo > 0) target.data.payoutRatioFfo = dividend / affo
			}
			if (derived) {
				warnings.push('per-share derived: reported total divided by shares outstanding')
			}

			return { data: cloned, provenance: provenance(!found) }
		},
	}
}

/** Per-share from the reported line, else from the reported total ÷ shares. */
function resolvePerShare(
	reported: ReportedReit,
	shares: number | undefined,
): { ffo?: number; affo?: number; derived: boolean } {
	const fromTotal = (total: number | undefined) =>
		total !== undefined && shares !== undefined ? perShareFromTotal(total, shares) : undefined

	const ffo = reported.ffoPerShare ?? fromTotal(reported.totals.ffo)
	const affo = reported.affoPerShare ?? fromTotal(reported.totals.affo)
	const derived =
		(ffo !== undefined && reported.ffoPerShare === undefined) ||
		(affo !== undefined && reported.affoPerShare === undefined)
	return { ffo, affo, derived }
}

/** The largest EX-99-style exhibit across recent 8-Ks, if any. */
async function findEarningsRelease(
	client: SecClient,
	symbol: string,
	cik: number | undefined,
	signal?: AbortSignal,
): Promise<SecDocument | undefined> {
	const filings = await client.findFilings(symbol, { forms: ['8-K'], limit: MAX_8KS, cik, signal })
	let best: SecDocument | undefined
	for (const filing of filings) {
		let documents: SecDocument[]
		try {
			documents = await client.listDocuments(filing as SecFilingRef, { signal })
		} catch {
			continue
		}
		for (const document of documents) {
			if (!/\.html?$/i.test(document.name) || !RELEASE_NAME.test(document.name)) continue
			if (document.size < MIN_RELEASE_BYTES) continue
			if (!best || document.size > best.size) best = document
		}
	}
	return best
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
