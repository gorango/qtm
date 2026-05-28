import { describe, it, expect } from 'vitest'
import { createToolRegistry } from '../src/registry'

const mockRegistry = {
	strategies: {
		'rsi-mean-reversion': {
			id: 'rsi-mean-reversion',
			name: 'RSI Mean Reversion',
			category: 'momentum',
			defaultTimeframes: ['1d'],
			description: 'Mean reversion strategy using RSI',
		},
		'macd-cross': {
			id: 'macd-cross',
			name: 'MACD Cross',
			category: 'momentum',
			defaultTimeframes: ['1d'],
			description: 'MACD crossover strategy',
		},
	},
}

describe('createToolRegistry', () => {
	it('creates an empty registry with no config', () => {
		const registry = createToolRegistry()
		expect(registry.getAll()).toHaveLength(0)
	})

	it('creates tools from strategy registry', () => {
		const registry = createToolRegistry({
			getStrategyRegistry: () => mockRegistry,
			resolveStrategy: async (id) => ({ signals: [1, 0, -1], strategy: id }),
		})
		const tools = registry.getAll()
		expect(tools.length).toBeGreaterThan(0)
		expect(tools.some((t) => t.name === 'rsi_mean_reversion')).toBe(true)
	})

	it('includes custom tools', () => {
		const customTool = {
			name: 'custom_tool',
			description: 'Custom tool',
			parameters: {} as never,
			execute: async () => ({ status: 'completed' as const, data: null }),
		}
		const registry = createToolRegistry({ customTools: [customTool] })
		expect(registry.getAll()).toHaveLength(1)
		expect(registry.getByName('custom_tool')).toBeDefined()
	})

	it('getManifest returns formatted string', () => {
		const registry = createToolRegistry({ customTools: [
			{
				name: 'test_tool',
				description: 'Test tool description',
				parameters: {} as never,
				execute: async () => ({ status: 'completed' as const, data: null }),
			},
		] })
		const manifest = registry.getManifest()
		expect(manifest).toContain('test_tool')
		expect(manifest).toContain('Test tool description')
	})
})
