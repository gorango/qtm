import { describe, expect, it } from 'vitest'
import { toFundamentalPoints } from '../src/fmp/map'
import type { FmpBundle } from '../src/fmp/types'

function bundle(): FmpBundle {
	return {
		income: [
			{
				date: '2024-03-31',
				symbol: 'PLD',
				filingDate: '2024-04-20',
				period: 'Q1',
				revenue: 1000,
				netIncome: 100,
				eps: 1,
				ebitda: 300,
				operatingIncome: 250,
				depreciationAndAmortization: 150,
				weightedAverageShsOut: 100,
			},
			{
				date: '2024-06-30',
				symbol: 'PLD',
				filingDate: '2024-07-20',
				period: 'Q2',
				revenue: 1100,
				netIncome: 120,
				eps: 1.2,
				ebitda: 320,
				operatingIncome: 260,
				depreciationAndAmortization: 160,
				weightedAverageShsOut: 100,
			},
		],
		balance: [
			{
				date: '2024-03-31',
				totalAssets: 5000,
				totalCurrentAssets: 500,
				totalCurrentLiabilities: 250,
				totalStockholdersEquity: 2000,
			},
		],
		cashFlow: [
			{
				date: '2024-03-31',
				operatingCashFlow: 200,
				capitalExpenditure: 50,
				preferredDividendsPaid: 0,
			},
		],
		keyMetrics: [{ date: '2024-03-31', marketCap: 50000, enterpriseValue: 55000 }],
		ratios: [{ date: '2024-03-31', assetTurnover: 0.2, dividendYield: 0.03 }],
		quote: [{ symbol: 'PLD', sharesOutstanding: 100, marketCap: 50000 }],
		priceTarget: [{ symbol: 'PLD', targetConsensus: 120 }],
		rating: [{ symbol: 'PLD', overallScore: 4 }],
		dcf: [{ symbol: 'PLD', dcf: 130 }],
		ownerEarnings: [{ date: '2024-03-31', maintenanceCapex: 20 }],
		dividends: [{ symbol: 'PLD', date: '2024-03-15', dividend: 1.5, frequency: 'Quarterly' }],
	}
}

describe('toFundamentalPoints', () => {
	const result = toFundamentalPoints(bundle())

	it('emits one point per period, oldest first', () => {
		expect(result.points.map((p) => p.period)).toEqual(['Q1', 'Q2'])
		expect(result.points[0].date).toBe(Date.parse('2024-03-31'))
		expect(result.points[0].filingDate).toBe(Date.parse('2024-04-20'))
	})

	it('renames FMP fields onto the contract', () => {
		const d = result.points[0].data
		expect(d.revenue).toBe(1000)
		expect(d.totalAssets).toBe(5000)
		expect(d.currentAssets).toBe(500)
		expect(d.shareholdersEquity).toBe(2000)
		expect(d.operatingCashFlow).toBe(200)
		expect(d.marketCap).toBe(50000)
		expect(d.assetTurnover).toBe(0.2)
		expect(d.sharesOutstanding).toBe(100)
	})

	it('derives dividends and REIT metrics per period', () => {
		const d = result.points[0].data
		expect(d.dividendsPerShare).toBeCloseTo(1.5)
		expect(d.forwardAnnualDividendRate).toBeCloseTo(6)
		expect(d.ffoPerShare).toBeCloseTo(2.5)
		expect(d.affoPerShare).toBeCloseTo(2.3)
		expect(d.payoutRatioFfo).toBeCloseTo(6 / 2.3)
		expect(result.estimated).toBe(true)
	})

	it('applies point-in-time snapshots to the latest period only', () => {
		const [q1, q2] = result.points
		expect(q1.data.analystTargetPrice).toBeUndefined()
		expect(q1.data.rating).toBeUndefined()
		expect(q1.data.dcf).toBeUndefined()
		expect(q2.data.analystTargetPrice).toBe(120)
		expect(q2.data.rating).toBe(4)
		expect(q2.data.dcf).toBe(130)
	})

	it('backfills market cap from the quote when a period lacks key metrics', () => {
		expect(result.points[1].data.marketCap).toBe(50000)
	})

	it('returns a warning and no points for an empty bundle', () => {
		const empty = toFundamentalPoints({ income: [] })
		expect(empty.points).toEqual([])
		expect(empty.warnings).toHaveLength(1)
		expect(empty.estimated).toBe(false)
	})

	it('warns when shares are unavailable', () => {
		const noShares = toFundamentalPoints({
			income: [{ date: '2024-03-31', symbol: 'X', netIncome: 10 }],
		})
		expect(noShares.warnings.join(' ')).toMatch(/shares outstanding/)
		expect(noShares.points[0].data.ffoPerShare).toBeUndefined()
	})
})
