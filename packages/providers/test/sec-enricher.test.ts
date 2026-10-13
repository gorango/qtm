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
})
