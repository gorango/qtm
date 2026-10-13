import type { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js'
import type { FundamentalProvider } from '@quantamental/providers'
import { createFmpClient, createFmpProvider } from '@quantamental/providers/fmp'
import type { ToolResult } from '@quantamental/tools'
import { z } from 'zod'

const TOOL_NAME = 'provider_fmp_fundamentals'

const parameters = z.object({
	symbol: z.string().describe('Stock symbol, e.g. "PLD"'),
	period: z
		.enum(['quarter', 'annual'])
		.optional()
		.describe('Reporting period (default "quarter")'),
	limit: z.number().int().positive().optional().describe('Maximum number of periods'),
})

export type FmpFundamentalsArgs = z.infer<typeof parameters>

export interface ProviderToolResult extends ToolResult {
	/** Present on success: where the data came from and how much to trust it. */
	provenance?: unknown
}

/**
 * Tool definition that fetches FMP data and normalizes it into quantamental's
 * `FundamentalPoint[]` contract, ready to pass to any `factor_*` / strategy.
 * Kept separate from registration so it can be unit-tested with a fake provider.
 */
export function createFmpFundamentalsTool(provider: FundamentalProvider) {
	return {
		name: TOOL_NAME,
		description:
			'Fetch Financial Modeling Prep fundamentals normalized to quantamental FundamentalPoint[] — pass the returned data to any factor_* tool.',
		parameters,
		async execute(args: FmpFundamentalsArgs): Promise<ProviderToolResult> {
			const start = Date.now()
			try {
				const result = await provider.fetchFundamentals(args.symbol, {
					period: args.period,
					limit: args.limit,
				})
				return {
					status: 'completed',
					data: result.data,
					provenance: result.provenance,
					metadata: { duration: Date.now() - start, toolName: TOOL_NAME },
				}
			} catch (error) {
				return {
					status: 'failed',
					error: { message: error instanceof Error ? error.message : String(error) },
					metadata: { duration: Date.now() - start, toolName: TOOL_NAME },
				}
			}
		},
	}
}

export function registerProviderTools(server: McpServer, provider: FundamentalProvider): void {
	const tool = createFmpFundamentalsTool(provider)
	server.registerTool(
		tool.name,
		{ description: tool.description, inputSchema: tool.parameters },
		async (args) => {
			const result = await tool.execute(args as FmpFundamentalsArgs)
			return {
				content: [{ type: 'text', text: JSON.stringify(result) }],
				isError: result.status === 'failed',
			}
		},
	)
}

/**
 * Providers need credentials and network access, so they are registered only
 * when available. Without `FMP_KEY` the server behaves exactly as before:
 * local, offline, and credential-free.
 */
export function registerConfiguredProviders(server: McpServer): void {
	if (!process.env.FMP_KEY) return
	registerProviderTools(server, createFmpProvider({ client: createFmpClient() }))
}
