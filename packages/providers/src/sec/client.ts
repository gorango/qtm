import { ProviderError } from '../provider'

const SEC_BASE = 'https://www.sec.gov'
const DATA_BASE = 'https://data.sec.gov'

export interface SecFilingRef {
	symbol: string
	cik: number
	form: string
	accessionNumber: string
	filingDate: string
	/** Period end the filing covers (YYYY-MM-DD). */
	reportDate: string
	url: string
}

export interface SecFindOptions {
	/** Form types to keep, e.g. `['10-K', '10-Q']`. Defaults to all. */
	forms?: string[]
	limit?: number
	signal?: AbortSignal
}

export interface SecDocument {
	name: string
	url: string
	size: number
}

export interface SecClient {
	findFilings(symbol: string, options?: SecFindOptions): Promise<SecFilingRef[]>
	fetchDocument(url: string, options?: { signal?: AbortSignal }): Promise<string>
	/** List a filing's documents (via its EDGAR index), e.g. to find EX-99.1. */
	listDocuments(filing: SecFilingRef, options?: { signal?: AbortSignal }): Promise<SecDocument[]>
}

export interface SecClientOptions {
	/**
	 * SEC requires a descriptive User-Agent with contact details, e.g.
	 * `"Jane Doe jane@example.com"`. Defaults to `process.env.SEC_USER_AGENT`.
	 */
	userAgent?: string
	/** Injectable for testing; defaults to global `fetch`. */
	fetch?: typeof fetch
	/** Minimum gap between requests. SEC permits 10 req/s; default is ~8 req/s. */
	minIntervalMs?: number
}

/**
 * Minimal EDGAR client: ticker → CIK → submissions → raw filing document.
 * Independent of FMP — EDGAR is free and authoritative, so this works with no
 * `FMP_KEY`. Filings are immutable, so consumers are encouraged to cache.
 */
export function createSecClient(options: SecClientOptions = {}): SecClient {
	const doFetch = options.fetch ?? globalThis.fetch
	const minIntervalMs = options.minIntervalMs ?? 120

	let lastRequest = 0
	let tickers: Map<string, number> | undefined

	function userAgent(): string {
		const ua = options.userAgent ?? process.env.SEC_USER_AGENT
		if (!ua) {
			throw new ProviderError(
				'sec',
				'SEC_USER_AGENT environment variable is required (format: "YourName your@email.com")',
			)
		}
		return ua
	}

	async function get(url: string, signal?: AbortSignal): Promise<Response> {
		const wait = lastRequest + minIntervalMs - Date.now()
		if (wait > 0) await new Promise((resolve) => setTimeout(resolve, wait))
		const response = await doFetch(url, {
			headers: { 'User-Agent': userAgent(), Accept: 'application/json,text/html,*/*' },
			signal,
		})
		lastRequest = Date.now()
		if (!response.ok) {
			throw new ProviderError('sec', `SEC request failed: ${response.status} ${response.statusText} (${url})`)
		}
		return response
	}

	async function cikFor(symbol: string, signal?: AbortSignal): Promise<number | undefined> {
		if (!tickers) {
			const response = await get(`${SEC_BASE}/files/company_tickers.json`, signal)
			const data = (await response.json()) as Record<string, { cik_str: number; ticker: string }>
			tickers = new Map(Object.values(data).map((row) => [row.ticker.toUpperCase(), row.cik_str]))
		}
		return tickers.get(symbol.toUpperCase())
	}

	return {
		async findFilings(symbol, findOptions = {}) {
			const { forms, limit, signal } = findOptions
			const cik = await cikFor(symbol, signal)
			if (cik === undefined) return []

			const response = await get(
				`${DATA_BASE}/submissions/CIK${String(cik).padStart(10, '0')}.json`,
				signal,
			)
			const data = (await response.json()) as {
				filings?: {
					recent?: {
						form: string[]
						accessionNumber: string[]
						filingDate: string[]
						reportDate: string[]
						primaryDocument: string[]
					}
				}
			}
			const recent = data.filings?.recent
			if (!recent) return []

			const refs: SecFilingRef[] = []
			for (let i = 0; i < recent.form.length; i++) {
				const form = recent.form[i]
				if (forms && !forms.includes(form)) continue
				const accessionNumber = recent.accessionNumber[i]
				refs.push({
					symbol: symbol.toUpperCase(),
					cik,
					form,
					accessionNumber,
					filingDate: recent.filingDate[i],
					reportDate: recent.reportDate[i],
					url: `${SEC_BASE}/Archives/edgar/data/${cik}/${accessionNumber.replace(/-/g, '')}/${recent.primaryDocument[i]}`,
				})
			}

			refs.sort((a, b) => b.filingDate.localeCompare(a.filingDate))
			return limit ? refs.slice(0, limit) : refs
		},

		async fetchDocument(url, fetchOptions = {}) {
			const response = await get(url, fetchOptions.signal)
			return response.text()
		},

		async listDocuments(filing, listOptions = {}) {
			const base = `${SEC_BASE}/Archives/edgar/data/${filing.cik}/${filing.accessionNumber.replace(/-/g, '')}/`
			const response = await get(`${base}index.json`, listOptions.signal)
			const data = (await response.json()) as {
				directory?: { item?: Array<{ name: string; size: string }> }
			}
			return (data.directory?.item ?? []).map((item) => ({
				name: item.name,
				url: base + item.name,
				size: Number(item.size),
			}))
		},
	}
}
