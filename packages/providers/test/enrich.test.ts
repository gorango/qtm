import { describe, expect, it } from 'vitest'
import {
	deriveReitMetrics,
	forwardAnnualDividendRate,
	frequencyMultiplier,
	trailingAnnualDividend,
} from '../src/fmp/enrich'

describe('deriveReitMetrics', () => {
	it('derives FFO/AFFO per share and the AFFO payout ratio', () => {
		const m = deriveReitMetrics({
			netIncome: 100,
			depreciationAndAmortization: 150,
			preferredDividendsPaid: 0,
			maintenanceCapex: 20,
			sharesOutstanding: 100,
			annualDividend: 6,
		})
		expect(m.ffoPerShare).toBeCloseTo(2.5)
		expect(m.affoPerShare).toBeCloseTo(2.3)
		expect(m.payoutRatioFfo).toBeCloseTo(6 / 2.3)
		expect(m.warnings).toHaveLength(0)
	})

	it('subtracts preferred dividends before per-share division', () => {
		const m = deriveReitMetrics({
			netIncome: 100,
			depreciationAndAmortization: 150,
			preferredDividendsPaid: 50,
			maintenanceCapex: 20,
			sharesOutstanding: 100,
		})
		// (100 + 150 - 50 - 20) / 100
		expect(m.affoPerShare).toBeCloseTo(1.8)
	})

	it('falls back to FFO when maintenance capex is unavailable, and warns', () => {
		const m = deriveReitMetrics({
			netIncome: 100,
			depreciationAndAmortization: 150,
			sharesOutstanding: 100,
		})
		expect(m.affoPerShare).toBeCloseTo(2.5)
		expect(m.warnings.join(' ')).toMatch(/maintenance capex/)
	})

	it('skips per-share metrics without shares', () => {
		const m = deriveReitMetrics({ netIncome: 100, depreciationAndAmortization: 150 })
		expect(m.ffoPerShare).toBeUndefined()
		expect(m.affoPerShare).toBeUndefined()
	})

	it('returns nothing when net income is missing', () => {
		expect(deriveReitMetrics({ sharesOutstanding: 100 }).affoPerShare).toBeUndefined()
	})

	it('does not divide by a non-positive AFFO', () => {
		const m = deriveReitMetrics({
			netIncome: 10,
			maintenanceCapex: 100,
			sharesOutstanding: 100,
			annualDividend: 5,
		})
		expect(m.affoPerShare).toBeLessThan(0)
		expect(m.payoutRatioFfo).toBeUndefined()
	})
})

describe('frequencyMultiplier', () => {
	it('annualizes known frequencies', () => {
		expect(frequencyMultiplier('Quarterly')).toBe(4)
		expect(frequencyMultiplier('Semi-Annual')).toBe(2)
		expect(frequencyMultiplier('Monthly')).toBe(12)
		expect(frequencyMultiplier('Annual')).toBe(1)
		expect(frequencyMultiplier(undefined)).toBe(1)
	})
})

describe('dividend helpers', () => {
	const dividends = [
		{ symbol: 'PLD', date: '2024-03-15', dividend: 1.5, frequency: 'Quarterly' },
		{ symbol: 'PLD', date: '2023-12-15', dividend: 1.4, frequency: 'Quarterly' },
		{ symbol: 'PLD', date: '2022-06-15', dividend: 1.0, frequency: 'Quarterly' },
	]
	const asOf = Date.parse('2024-03-31')

	it('sums payments in the trailing year only', () => {
		expect(trailingAnnualDividend(dividends, asOf)).toBeCloseTo(2.9)
	})

	it('annualizes the latest payment at or before the date', () => {
		expect(forwardAnnualDividendRate(dividends, asOf)).toBeCloseTo(6)
	})

	it('returns undefined when no payment precedes the date', () => {
		expect(forwardAnnualDividendRate(dividends, Date.parse('2022-01-01'))).toBeUndefined()
		expect(trailingAnnualDividend(dividends, 0)).toBeUndefined()
	})
})
