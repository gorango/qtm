import { z } from 'zod'
import type { WorkflowTool, ToolRegistry } from '../types'
import { createWorkflowTool } from '../tool'

export interface StrategyDefinition {
	id: string
	name: string
	category: string
	defaultTimeframes: string[]
	description: string | null
}

export interface StrategyRegistryData {
	strategies: Record<string, StrategyDefinition>
}

const strategyInputSchema = z.object({
	closes: z
		.array(z.number())
		.describe('Array of closing prices (oldest first)'),
	highs: z
		.array(z.number())
		.optional()
		.describe('Array of high prices (oldest first)'),
	lows: z
		.array(z.number())
		.optional()
		.describe('Array of low prices (oldest first)'),
	volumes: z
		.array(z.number())
		.optional()
		.describe('Array of volume data (oldest first)'),
	config: z
		.record(z.string(), z.unknown())
		.optional()
		.describe('Strategy-specific configuration overrides'),
})

const indicatorInputSchema = z.object({
	values: z
		.array(z.number())
		.describe('Array of input values (oldest first)'),
	config: z
		.record(z.string(), z.unknown())
		.optional()
		.describe('Indicator configuration parameters'),
})

export function createDynamicStrategyTool(
	definition: StrategyDefinition,
	strategyFn: (params: {
		closes: number[]
		highs?: number[]
		lows?: number[]
		volumes?: number[]
		config?: Record<string, unknown>
	}) => Promise<unknown>,
): WorkflowTool<typeof strategyInputSchema> {
	return createWorkflowTool({
		name: definition.id.replace(/-/g, '_'),
		description:
			definition.description ?? `${definition.name} (${definition.category}) strategy`,
		parameters: strategyInputSchema,
		execute: async (params) => {
			const start = Date.now()
			try {
				const result = await strategyFn({
					closes: params.closes,
					highs: params.highs,
					lows: params.lows,
					volumes: params.volumes,
					config: params.config,
				})
				return {
					status: 'completed',
					data: result,
					metadata: { duration: Date.now() - start, toolName: definition.id },
				}
			} catch (error) {
				return {
					status: 'failed',
					error: {
						message: error instanceof Error ? error.message : String(error),
					},
					metadata: { duration: Date.now() - start, toolName: definition.id },
				}
			}
		},
	})
}

export function createToolRegistry(
	config: {
		getStrategyRegistry?: () => StrategyRegistryData
		resolveStrategy?: (
			id: string,
			params: { closes: number[]; config?: Record<string, unknown> },
		) => Promise<unknown>
		customTools?: WorkflowTool[]
	} = {},
): ToolRegistry {
	const tools: WorkflowTool[] = [...(config.customTools ?? [])]

	if (config.getStrategyRegistry) {
		try {
			const registry = config.getStrategyRegistry()
			for (const [id, def] of Object.entries(registry.strategies)) {
				const strategyId = id
				tools.push(
					createDynamicStrategyTool(
						def,
						async (params) => {
							if (!config.resolveStrategy) {
								throw new Error('No strategy resolver configured')
							}
							return config.resolveStrategy(strategyId, {
								closes: params.closes,
								config: params.config,
							})
						},
					),
				)
			}
		} catch {
			// Registry not available; skip dynamic tools
		}
	}

	return {
		getAll(): WorkflowTool[] {
			return [...tools]
		},
		getByCategory(category: string): WorkflowTool[] {
			return tools.filter((t) => t.name.startsWith(category))
		},
		getByName(name: string): WorkflowTool | undefined {
			return tools.find((t) => t.name === name)
		},
		getManifest(): string {
			return tools
				.map(
					(t) =>
						`- **${t.name}**: ${t.description}`,
				)
				.join('\n')
		},
	}
}

export function buildSystemPrompt(tools: WorkflowTool[]): string {
	const sections: string[] = [
		'## Available Quantamental Tools',
		'',
		'You have access to the following analysis tools:',
		'',
	]

	const categorized = new Map<string, WorkflowTool[]>()
	for (const tool of tools) {
		const prefix = tool.name.split('_')[0] ?? 'other'
		if (!categorized.has(prefix)) categorized.set(prefix, [])
		categorized.get(prefix)!.push(tool)
	}

	for (const [category, categoryTools] of categorized) {
		sections.push(`### ${category.charAt(0).toUpperCase() + category.slice(1)}`)
		for (const tool of categoryTools) {
			sections.push(`- \`${tool.name}\`: ${tool.description}`)
		}
		sections.push('')
	}

	return sections.join('\n')
}
