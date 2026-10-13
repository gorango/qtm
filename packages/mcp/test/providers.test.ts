import { describe, expect, it } from 'vitest'
import type { FundamentalProvider } from '@quantamental/providers'
import { createFmpFundamentalsTool, createSecReitTool } from '../src/providers'

const point = { symbol: 'PLD', date: 1, filingDate: 1, period: 'Q1', data: { marketCap: 1 } }

const fake: FundamentalProvider = {
	id: 'fake',
	async fetchFundamentals(symbol) {
		return {
			data: [{ ...point, symbol }],
			provenance: {
				provider: 'fake',
				retrievedAt: '2024-01-01T00:00:00.000Z',
				estimated: false,
				warnings: [],
			},
		}
	},
}

describe('createFmpFundamentalsTool', () => {
	it('returns normalized points plus provenance', async () => {
		const tool = createFmpFundamentalsTool(fake)
		const result = await tool.execute({ symbol: 'PLD', period: 'quarter' })

		expect(tool.name).toBe('provider_fmp_fundamentals')
		expect(result.status).toBe('completed')
		expect(result.data).toHaveLength(1)
		expect(result.provenance).toMatchObject({ provider: 'fake' })
	})

	it('returns a failed result when the provider throws', async () => {
		const tool = createFmpFundamentalsTool({
			id: 'x',
			fetchFundamentals: async () => {
				throw new Error('boom')
			},
		})
		const result = await tool.execute({ symbol: 'PLD' })

		expect(result.status).toBe('failed')
		expect(result.error?.message).toBe('boom')
	})
})

describe('createSecReitTool', () => {
	const enricher = {
		id: 'sec',
		async enrich(points: unknown[]) {
			return {
				data: points,
				provenance: {
					provider: 'sec',
					retrievedAt: '2024-01-01T00:00:00.000Z',
					estimated: false,
					warnings: [],
				},
			}
		},
	}

	it('passes the panel through and reports provenance', async () => {
		const tool = createSecReitTool(enricher)
		const result = await tool.execute({ fundamentals: [{ symbol: 'PLD' }] })

		expect(tool.name).toBe('provider_sec_reit')
		expect(result.status).toBe('completed')
		expect(result.provenance).toMatchObject({ provider: 'sec', estimated: false })
	})
})
