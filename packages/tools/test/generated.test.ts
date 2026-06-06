import { describe, it, expect } from 'vitest'
import {
	createStrategyTools,
	createIndicatorTools,
	createFactorTools,
	factorCount,
	indicatorCount,
	strategyCount,
	registryData,
} from '../src/generated/tools'
import type { ToolResult } from '../src/types'

function mockSignalExecute(_id: string, _params: unknown): Promise<ToolResult> {
	return Promise.resolve({ status: 'completed', data: [1, 0, -1, 1, 0] })
}

function mockIndicatorExecute(_id: string, _params: unknown): Promise<ToolResult> {
	return Promise.resolve({ status: 'completed', data: [15.5, 16.2, 14.8, 15.9] })
}

function mockFactorExecute(_id: string, _params: unknown): Promise<ToolResult> {
	return Promise.resolve({ status: 'completed', data: [0.85, 0.92, 0.78] })
}

describe('generated pipeline integration', () => {
	describe('counts match registry', () => {
		it('factorCount matches registry', () => {
			expect(factorCount).toBe(Object.keys(registryData.factors).length)
		})
		it('indicatorCount matches registry', () => {
			expect(indicatorCount).toBe(Object.keys(registryData.indicators).length)
		})
		it('strategyCount matches registry', () => {
			expect(strategyCount).toBe(Object.keys(registryData.strategies).length)
		})
	})

	describe('createStrategyTools', () => {
		const tools = createStrategyTools(mockSignalExecute)

		it('creates the expected number of tools', () => {
			expect(tools).toHaveLength(strategyCount)
		})

		it('every tool has required fields', () => {
			for (const tool of tools) {
				expect(tool.name).toBeTruthy()
				expect(tool.description).toContain('(returns: signal)')
				expect(tool.parameters).toBeDefined()
				expect(typeof tool.execute).toBe('function')
			}
		})

		it('every tool has valid parameter schema', () => {
			for (const tool of tools) {
				const shape = tool.parameters.shape
				expect(shape).toBeDefined()
				expect(shape.closes).toBeDefined()
			}
		})

		it('execute returns correct status', async () => {
			const result = await tools[0].execute({
				closes: [100, 101, 102],
			})
			expect(result.status).toBe('completed')
			expect(Array.isArray(result.data)).toBe(true)
		})

		it('executes all tools without error', async () => {
			const results = await Promise.all(
				tools.map(t => t.execute({ closes: [100, 101, 102] }).catch(e => ({ status: 'failed', error: { message: e.message } } as ToolResult)))
			)
			for (const r of results) {
				expect(r.status).toBe('completed')
				expect(Array.isArray(r.data)).toBe(true)
			}
		})
	})

	describe('createIndicatorTools', () => {
		const tools = createIndicatorTools(mockIndicatorExecute)

		it('creates the expected number of tools', () => {
			expect(tools).toHaveLength(indicatorCount)
		})

		it('every tool has required fields and return type hint', () => {
			for (const tool of tools) {
				expect(tool.name).toBeTruthy()
				expect(tool.description).toContain('(returns: indicator)')
				expect(tool.parameters).toBeDefined()
				expect(typeof tool.execute).toBe('function')
			}
		})

		it('every tool has values parameter', () => {
			for (const tool of tools) {
				const shape = tool.parameters.shape
				expect(shape).toBeDefined()
				expect(shape.values).toBeDefined()
			}
		})

		it('executes all tools without error', async () => {
			const results = await Promise.all(
				tools.map(t => t.execute({ values: [100, 101, 102] }).catch(e => ({ status: 'failed', error: { message: e.message } } as ToolResult)))
			)
			for (const r of results) {
				expect(r.status).toBe('completed')
				expect(Array.isArray(r.data)).toBe(true)
			}
		})
	})

	describe('createFactorTools', () => {
		const tools = createFactorTools(mockFactorExecute)

		it('creates the expected number of tools', () => {
			expect(tools).toHaveLength(factorCount)
		})

		it('every tool has required fields and return type hint', () => {
			for (const tool of tools) {
				expect(tool.name).toBeTruthy()
				expect(tool.description).toContain('(returns: factor)')
				expect(tool.parameters).toBeDefined()
				expect(typeof tool.execute).toBe('function')
			}
		})

		it('every tool has fundamentals parameter', () => {
			for (const tool of tools) {
				const shape = tool.parameters.shape
				expect(shape).toBeDefined()
				expect(shape.fundamentals).toBeDefined()
			}
		})

		it('executes all tools without error', async () => {
			const results = await Promise.all(
				tools.map(t => t.execute({ fundamentals: [] }).catch(e => ({ status: 'failed', error: { message: e.message } } as ToolResult)))
			)
			for (const r of results) {
				expect(r.status).toBe('completed')
				expect(Array.isArray(r.data)).toBe(true)
			}
		})
	})
})
