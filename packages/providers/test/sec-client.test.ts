import { describe, expect, it } from 'vitest'
import { createSecClient } from '../src/sec/client'

const TICKERS = { '0': { cik_str: 1045609, ticker: 'PLD', title: 'Prologis, Inc.' } }

const SUBMISSIONS = {
	filings: {
		recent: {
			form: ['10-Q', '8-K', '10-K'],
			accessionNumber: ['0001045609-24-000010', '0001045609-24-000011', '0001045609-24-000001'],
			filingDate: ['2024-04-20', '2024-04-19', '2024-02-01'],
			reportDate: ['2024-03-31', '2024-04-15', '2023-12-31'],
			primaryDocument: ['pld-20240331.htm', 'pld-8k.htm', 'pld-20231231.htm'],
		},
	},
}

const INDEX = {
	directory: {
		item: [
			{ name: 'pld-20240331.htm', size: '12345' },
			{ name: 'ex99-1.htm', size: '500000' },
		],
	},
}

interface LogEntry {
	url: string
	userAgent: string | undefined
}

function stubFetch(log: LogEntry[]): typeof fetch {
	return (async (input: unknown, init?: { headers?: Record<string, string> }) => {
		const url = String(input)
		log.push({ url, userAgent: init?.headers?.['User-Agent'] })
		const body: unknown = url.includes('company_tickers')
			? TICKERS
			: url.includes('index.json')
				? INDEX
				: SUBMISSIONS
		return {
			ok: true,
			status: 200,
			statusText: 'OK',
			json: async () => body,
			text: async () => (url.includes('Archives') ? '<html>doc</html>' : JSON.stringify(body)),
		} as unknown as Response
	}) as typeof fetch
}

describe('createSecClient', () => {
	it('resolves the ticker to a CIK and builds filing URLs', async () => {
		const log: LogEntry[] = []
		const client = createSecClient({
			userAgent: 'Test test@example.com',
			minIntervalMs: 0,
			fetch: stubFetch(log),
		})

		const filings = await client.findFilings('pld', { forms: ['10-K', '10-Q'] })

		expect(filings).toHaveLength(2)
		expect(filings[0]).toMatchObject({ form: '10-Q', cik: 1045609, reportDate: '2024-03-31' })
		expect(filings[0].url).toBe(
			'https://www.sec.gov/Archives/edgar/data/1045609/000104560924000010/pld-20240331.htm',
		)
		expect(log[0].url).toContain('company_tickers.json')
		expect(log[0].userAgent).toBe('Test test@example.com')
	})

	it('honours the limit', async () => {
		const client = createSecClient({
			userAgent: 'x y@z.co',
			minIntervalMs: 0,
			fetch: stubFetch([]),
		})
		const filings = await client.findFilings('PLD', { forms: ['10-K', '10-Q'], limit: 1 })
		expect(filings).toHaveLength(1)
		expect(filings[0].form).toBe('10-Q')
	})

	it('returns an empty list for an unknown ticker', async () => {
		const client = createSecClient({
			userAgent: 'x y@z.co',
			minIntervalMs: 0,
			fetch: stubFetch([]),
		})
		expect(await client.findFilings('NOPE')).toEqual([])
	})

	it('throws without a user agent', async () => {
		const previous = process.env.SEC_USER_AGENT
		delete process.env.SEC_USER_AGENT
		try {
			const client = createSecClient({ minIntervalMs: 0, fetch: stubFetch([]) })
			await expect(client.findFilings('PLD')).rejects.toThrow(/SEC_USER_AGENT/)
		} finally {
			if (previous !== undefined) process.env.SEC_USER_AGENT = previous
		}
	})

	it('fetches a document as text', async () => {
		const client = createSecClient({
			userAgent: 'x y@z.co',
			minIntervalMs: 0,
			fetch: stubFetch([]),
		})
		const html = await client.fetchDocument(
			'https://www.sec.gov/Archives/edgar/data/1045609/000104560924000010/pld-20240331.htm',
		)
		expect(html).toContain('doc')
	})

	it('lists filing documents via the EDGAR index', async () => {
		const client = createSecClient({
			userAgent: 'x y@z.co',
			minIntervalMs: 0,
			fetch: stubFetch([]),
		})
		const [filing] = await client.findFilings('PLD', { forms: ['10-Q'], limit: 1 })
		const documents = await client.listDocuments(filing)

		expect(documents.map((d) => d.name)).toEqual(['pld-20240331.htm', 'ex99-1.htm'])
		expect(documents[1].size).toBe(500000)
		expect(documents[0].url).toBe(
			'https://www.sec.gov/Archives/edgar/data/1045609/000104560924000010/pld-20240331.htm',
		)
	})

	it('uses a supplied CIK without resolving the ticker', async () => {
		const log: LogEntry[] = []
		const client = createSecClient({
			userAgent: 'x y@z.co',
			minIntervalMs: 0,
			fetch: stubFetch(log),
		})
		const filings = await client.findFilings('AVB', { forms: ['10-Q'], cik: 915912 })

		expect(filings[0].cik).toBe(915912)
		expect(log.some((l) => l.url.includes('company_tickers'))).toBe(false)
		expect(log[0].url).toContain('/submissions/CIK0000915912.json')
	})

	it('falls back to a resolveCik hook when the ticker is unknown', async () => {
		const client = createSecClient({
			userAgent: 'x y@z.co',
			minIntervalMs: 0,
			fetch: stubFetch([]),
			resolveCik: (symbol) => (symbol === 'AVB' ? 915912 : undefined),
		})
		const filings = await client.findFilings('AVB', { forms: ['10-Q'] })
		expect(filings[0].cik).toBe(915912)
	})
})
