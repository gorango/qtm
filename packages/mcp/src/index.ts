#!/usr/bin/env bun
import { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js'
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'
import * as core from '@quantamental/core'
import {
	createStrategyTools,
	createIndicatorTools,
	createFactorTools,
} from '@quantamental/tools'
import type { WorkflowTool, ToolResult } from '@quantamental/tools'
import { registerConfiguredProviders } from './providers'

// The generated registry uses snake_case ids; napi exports use camelCase
// (e.g. "awesome_oscillator" → core.awesomeOscillator). Resolve both forms.
const exports = core as unknown as Record<string, (...a: unknown[]) => unknown>
const resolveFn = (id: string) =>
	exports[id] ?? exports[id.replace(/_([a-z])/g, (_, c) => c.toUpperCase())]

const timed = (id: string, fn: () => unknown): ToolResult => {
	const start = Date.now()
	try {
		return { status: 'completed', data: fn(), metadata: { duration: Date.now() - start, toolName: id } }
	} catch (error) {
		return {
			status: 'failed',
			error: { message: error instanceof Error ? error.message : String(error) },
			metadata: { duration: Date.now() - start, toolName: id },
		}
	}
}

// ── Dispatch: strategy id → runStrategy ──────────────────
const strategyExecute = (id: string, params: {
	closes: number[]
	highs?: number[]
	lows?: number[]
	volumes?: number[]
	config?: Record<string, unknown>
}): Promise<ToolResult> => {
	return Promise.resolve(timed(id, () => core.runStrategy(id, {
		closes: params.closes,
		highs: params.highs,
		lows: params.lows,
		volumes: params.volumes,
	}, (params.config ?? null) as never)))
}

// ── Dispatch: indicator id → named core export ───────────
const indicatorExecute = (id: string, params: {
	values: number[]
	config?: Record<string, unknown>
}): Promise<ToolResult> => {
	const fn = resolveFn(id)
	if (typeof fn !== 'function') {
		return Promise.resolve({
			status: 'failed',
			error: { message: `Indicator "${id}" is not exported by @quantamental/core` },
			metadata: { duration: 0, toolName: id },
		})
	}
	return Promise.resolve(timed(id, () => fn(new Float64Array(params.values), params.config ?? null)))
}

// ── Dispatch: factor id → named core export ──────────────
// Factor functions take FundamentalPoint[] (or OnChainDataPoint[] /
// PredictionMarketPoint[]) and return FactorPoint[]. Some also take prices.
const factorExecute = (id: string, params: {
	fundamentals: unknown[]
	prices?: unknown[]
}): Promise<ToolResult> => {
	const fn = resolveFn(id)
	if (typeof fn !== 'function') {
		return Promise.resolve({
			status: 'failed',
			error: { message: `Factor "${id}" is not exported by @quantamental/core` },
			metadata: { duration: 0, toolName: id },
		})
	}
	return Promise.resolve(timed(id, () =>
		params.prices ? fn(params.fundamentals, params.prices) : fn(params.fundamentals),
	))
}

// 35 indicator/strategy ids collide (e.g. "rsi" is both an indicator and a
// strategy). Prefix with category to guarantee unique MCP tool names.
const prefix = (p: string, tools: WorkflowTool[]) =>
	tools.map((t) => ({ ...t, name: `${p}_${t.name}` }))

const tools: WorkflowTool[] = [
	...prefix('strategy', createStrategyTools(strategyExecute)),
	...prefix('indicator', createIndicatorTools(indicatorExecute)),
	...prefix('factor', createFactorTools(factorExecute)),
]

function createServer() {
	const server = new McpServer(
		{ name: 'quantamental-mcp', version: '0.1.0' },
		{ capabilities: { tools: {} } },
	)

	for (const tool of tools) {
		server.registerTool(
			tool.name,
			{
				description: tool.description,
				inputSchema: tool.parameters,
			},
			async (args) => {
				try {
					const result = await tool.execute(args as never)
					return {
						content: [{ type: 'text', text: JSON.stringify(result) }],
						isError: result.status === 'failed',
					}
				} catch (error: any) {
					return {
						content: [{ type: 'text', text: error.message || 'An unexpected error occurred' }],
						isError: true,
					}
				}
			},
		)
	}

	// Optional: registered only when FMP_KEY is set (adds network + credentials).
	registerConfiguredProviders(server)

	return server
}

export async function startMCPServer() {
	const server = createServer()
	const transport = new StdioServerTransport()
	await server.connect(transport)
}

await startMCPServer()
