import { describe, expect, it } from 'vitest'
import { createSecReitEnricher } from '../src/sec/index'
import type { SecClient, SecFilingRef } from '../src/sec/client'
import { REIT_10Q_HTML } from './fixtures/reit-10q'

const filing: SecFilingRef = {
	symbol: 'PLD',
	cik: 1045609,
	form: '10-Q',
	accessionNumber: '0001045609-24-000010',
	filingDate: '2024-04-20',
	reportDate: '2024-03-31',
	url: 'https://www.sec.gov/Archives/edgar/data/1045609/000104560924000010/pld-20240331.htm',
}

function client(filings: SecFilingRef[], html = REIT_10Q_HTML): SecClient {
	return {
		findFilings: async () => filings,
		fetchDocument: async () => html,
		listDocuments: async () => [],
	}
}

/** Fake where 8-K lookups return a release exhibit and documents differ by URL. */
function multiClient(filings: SecFilingRef[], documents: Record<string, string>): SecClient {
	return {
		findFilings: async (_symbol, options) =>
			options?.forms?.includes('8-K')
				? filings.filter((f) => f.form === '8-K')
				: filings.filter((f) => f.form !== '8-K'),
		fetchDocument: async (url) => documents[url] ?? '',
		listDocuments: async (filing) =>
			filing.form === '8-K'
				? [{ name: 'ex99-1.htm', url: 'https://sec.example/ex99-1.htm', size: 500_000 }]
				: [],
	}
}

function point(date: string, affoPerShare: number) {
	return {
		symbol: 'PLD',
		date: Date.parse(date),
		filingDate: Date.parse(date),
		period: 'Q',
		data: { affoPerShare, forwardAnnualDividendRate: 6 },
	}
}

const fixedNow = () => new Date('2024-08-01T00:00:00.000Z')

describe('createSecReitEnricher', () => {
	it('overlays reported values on the period the filing covers', async () => {
		const enricher = createSecReitEnricher({ client: client([filing]), now: fixedNow })
		const { data, provenance } = await enricher.enrich([
			point('2023-12-31', 2.0),
			point('2024-03-31', 2.3),
		])

		expect(data[1].data.affoPerShare).toBeCloseTo(1.02)
		expect(data[1].data.ffoPerShare).toBeCloseTo(1.18)
		expect(data[1].data.payoutRatioFfo).toBeCloseTo(6 / 1.02)
		// The period the filing does not cover keeps its proxy.
		expect(data[0].data.affoPerShare).toBeCloseTo(2.0)

		expect(provenance).toMatchObject({ provider: 'sec', estimated: false })
		expect(provenance.sources?.[0]).toContain('Archives')
	})

	it('matches by report date rather than position', async () => {
		const enricher = createSecReitEnricher({ client: client([filing]), now: fixedNow })
		// newest first — the matching point is not last
		const { data } = await enricher.enrich([point('2024-03-31', 2.3), point('2023-12-31', 2.0)])
		expect(data[0].data.affoPerShare).toBeCloseTo(1.02)
		expect(data[1].data.affoPerShare).toBeCloseTo(2.0)
	})

	it('does not mutate the input panel', async () => {
		const enricher = createSecReitEnricher({ client: client([filing]), now: fixedNow })
		const input = [point('2024-03-31', 2.3)]
		await enricher.enrich(input)
		expect(input[0].data.affoPerShare).toBeCloseTo(2.3)
	})

	it('leaves the proxy in place and flags estimated when no filing is found', async () => {
		const enricher = createSecReitEnricher({ client: client([]), now: fixedNow })
		const { data, provenance } = await enricher.enrich([point('2024-03-31', 2.3)])
		expect(data[0].data.affoPerShare).toBeCloseTo(2.3)
		expect(provenance.estimated).toBe(true)
		expect(provenance.warnings.join(' ')).toMatch(/no 10-K\/10-Q/)
	})

	it('warns when the panel has no symbol', async () => {
		const enricher = createSecReitEnricher({ client: client([filing]), now: fixedNow })
		const { provenance } = await enricher.enrich([])
		expect(provenance.estimated).toBe(true)
		expect(provenance.warnings.join(' ')).toMatch(/no symbol/)
	})

	it('derives per share from a reported total and the panel share count', async () => {
		const html =
			'<tr><td>NAREIT defined FFO attributable to common stockholders</td><td>$</td><td>3,368</td></tr>'
		const enricher = createSecReitEnricher({ client: client([filing], html), now: fixedNow })
		const withShares = point('2024-03-31', 2.3)
		withShares.data.sharesOutstanding = 957_884_000

		const { data, provenance } = await enricher.enrich([withShares])

		expect(data[0].data.ffoPerShare).toBeCloseTo(3.516, 2)
		expect(provenance.estimated).toBe(true)
		expect(provenance.warnings.join(' ')).toMatch(/derived/)
	})

	it('falls back to the earnings release for a missing measure', async () => {
		const eightK: SecFilingRef = {
			...filing,
			form: '8-K',
			filingDate: '2024-07-25',
			reportDate: '',
			url: 'https://sec.example/8k.htm',
		}
		const releaseHtml =
			'<p>Reported Core FFO per share of $2.65 in 2Q24, compared to $1.87 in 2Q23</p>'
		const enricher = createSecReitEnricher({
			client: multiClient([filing, eightK], {
				[filing.url]:
					'<tr><td>NAREIT defined FFO attributable to common stockholders</td><td>$</td><td>3,368</td></tr>',
				'https://sec.example/ex99-1.htm': releaseHtml,
			}),
			now: fixedNow,
		})
		const withShares = point('2024-03-31', 2.3)
		withShares.data.sharesOutstanding = 957_884_000

		const { data, provenance } = await enricher.enrich([withShares])

		expect(data[0].data.ffoPerShare).toBeCloseTo(3.516, 2)
		expect(data[0].data.affoPerShare).toBeCloseTo(2.65)
		expect(provenance.sources).toContain('https://sec.example/ex99-1.htm')
	})

	it('forwards an explicit CIK to the client', async () => {
		let seen: number | undefined
		const spy: SecClient = {
			findFilings: async (_symbol, options) => {
				seen ??= options?.cik
				return []
			},
			fetchDocument: async () => '',
			listDocuments: async () => [],
		}
		const enricher = createSecReitEnricher({ client: spy, now: fixedNow })
		await enricher.enrich([point('2024-03-31', 2.3)], { cik: 915912 })
		expect(seen).toBe(915912)
	})
})
