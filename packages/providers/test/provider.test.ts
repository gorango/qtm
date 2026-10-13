import { describe, expect, it } from 'vitest'
import { createFmpProvider } from '../src/fmp/index'
import type { FmpClient } from '../src/fmp/types'
import { ProviderError } from '../src/provider'

function fakeClient(overrides: Partial<FmpClient> = {}): FmpClient {
	const base: FmpClient = {
		incomeStatement: async () => [
			{
				date: '2024-03-31',
				symbol: 'PLD',
				period: 'Q1',
				netIncome: 100,
				ebitda: 300,
				operatingIncome: 250,
				depreciationAndAmortization: 150,
				weightedAverageShsOut: 100,
			},
		],
		balanceSheetStatement: async () => [{ date: '2024-03-31', totalAssets: 5000 }],
		cashFlowStatement: async () => [
			{ date: '2024-03-31', operatingCashFlow: 200, capitalExpenditure: 50 },
		],
		keyMetrics: async () => [{ date: '2024-03-31', marketCap: 50000 }],
		financialRatios: async () => [{ date: '2024-03-31', assetTurnover: 0.2 }],
		companyProfile: async () => [{ symbol: 'PLD', marketCap: 50000 }],
		stockQuote: async () => [{ symbol: 'PLD', sharesOutstanding: 100 }],
		priceTargetConsensus: async () => [{ symbol: 'PLD', targetConsensus: 120 }],
		ratingsSnapshot: async () => [{ symbol: 'PLD', overallScore: 4 }],
		dcfValuation: async () => [{ symbol: 'PLD', dcf: 130 }],
		ownerEarnings: async () => [{ date: '2024-03-31', maintenanceCapex: 20 }],
		companyDividends: async () => [
			{ symbol: 'PLD', date: '2024-03-15', dividend: 1.5, frequency: 'Quarterly' },
		],
	}
	return { ...base, ...overrides }
}

const fixedNow = () => new Date('2024-08-01T00:00:00.000Z')

describe('createFmpProvider', () => {
	it('returns normalized points with provenance', async () => {
		const provider = createFmpProvider({ client: fakeClient(), now: fixedNow })
		const result = await provider.fetchFundamentals('PLD', { period: 'quarter' })

		expect(provider.id).toBe('fmp')
		expect(result.data).toHaveLength(1)
		expect(result.data[0].data.affoPerShare).toBeCloseTo(2.3)
		expect(result.provenance).toMatchObject({
			provider: 'fmp',
			retrievedAt: '2024-08-01T00:00:00.000Z',
			estimated: true,
		})
	})

	it('degrades to a warning when an optional endpoint fails', async () => {
		const provider = createFmpProvider({
			client: fakeClient({
				financialRatios: async () => {
					throw new Error('502')
				},
			}),
			now: fixedNow,
		})
		const result = await provider.fetchFundamentals('PLD')
		expect(result.data).toHaveLength(1)
		expect(result.provenance.warnings).toContain('ratios unavailable')
	})

	it('throws a ProviderError when the income statement fails', async () => {
		const provider = createFmpProvider({
			client: fakeClient({
				incomeStatement: async () => {
					throw new Error('401')
				},
			}),
			now: fixedNow,
		})
		await expect(provider.fetchFundamentals('PLD')).rejects.toBeInstanceOf(ProviderError)
	})

	it('rejects an empty symbol', async () => {
		const provider = createFmpProvider({ client: fakeClient(), now: fixedNow })
		await expect(provider.fetchFundamentals('')).rejects.toBeInstanceOf(ProviderError)
	})
})
