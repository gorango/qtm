import type { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js'
import type { FundamentalEnricher, FundamentalProvider } from '@quantamental/providers'
import { createFmpClient, createFmpProvider } from '@quantamental/providers/fmp'
import { createSecClient, createSecReitEnricher } from '@quantamental/providers/sec'
import type { ToolResult } from '@quantamental/tools'
import { z } from 'zod'

const FMP_TOOL = 'provider_fmp_fundamentals'
const SEC_TOOL = 'provider_sec_reit'

const fmpParameters = z.object({
	symbol: z.string().describe('Stock symbol, e.g. "PLD"'),
	period: z.enum(['quarter', 'annual']).optional().describe('Reporting period (default "quarter")'),
	limit: z.number().int().positive().optional().describe('Maximum number of periods'),
})

const secParameters = z.object({
	fundamentals: z
		.array(z.unknown())
		.describe('A fundamentals panel from provider_fmp_fundamentals'),
	symbol: z.string().optional().describe('Overrides the symbol inferred from the panel'),
	cik: z
		.number()
		.int()
		.positive()
		.optional()
		.describe('CIK, for issuers EDGAR cannot resolve by ticker'),
})

export type FmpFundamentalsArgs = z.infer<typeof fmpParameters>
export type SecReitArgs = z.infer<typeof secParameters>

export interface ProviderToolResult extends ToolResult {
	/** Present on success: where the data came from and how much to trust it. */
	provenance?: unknown
}

interface ProviderTool {
	name: string
	description: string
	parameters: z.ZodType
	execute(args: any): Promise<ProviderToolResult>
}

/**
 * Fetches FMP data and normalizes it into quantamental's `FundamentalPoint[]`
 * contract, ready to pass to any `factor_*` / strategy. Kept separate from
 * registration so it can be unit-tested with a fake provider.
 */
export function createFmpFundamentalsTool(provider: FundamentalProvider) {
	return {
		name: FMP_TOOL,
		description:
			'Fetch Financial Modeling Prep fundamentals normalized to quantamental FundamentalPoint[] — pass the returned data to any factor_* tool.',
		parameters: fmpParameters,
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
					metadata: { duration: Date.now() - start, toolName: FMP_TOOL },
				}
			} catch (error) {
				return {
					status: 'failed',
					error: { message: error instanceof Error ? error.message : String(error) },
					metadata: { duration: Date.now() - start, toolName: FMP_TOOL },
				}
			}
		},
	}
}

/**
 * Overlays reported FFO/AFFO onto a panel, replacing the modelled proxy. Pass
 * the returned data on to `factor_price_to_affo` / `factor_reit_dividend_safety`.
 */
export function createSecReitTool(enricher: FundamentalEnricher) {
	return {
		name: SEC_TOOL,
		description:
			'Overlay reported (SEC 10-K/10-Q) FFO/AFFO per share onto a fundamentals panel, replacing modelled values. Pass the returned data to REIT factors.',
		parameters: secParameters,
		async execute(args: SecReitArgs): Promise<ProviderToolResult> {
			const start = Date.now()
			try {
				const result = await enricher.enrich(args.fundamentals as never, {
					symbol: args.symbol,
					cik: args.cik,
				})
				return {
					status: 'completed',
					data: result.data,
					provenance: result.provenance,
					metadata: { duration: Date.now() - start, toolName: SEC_TOOL },
				}
			} catch (error) {
				return {
					status: 'failed',
					error: { message: error instanceof Error ? error.message : String(error) },
					metadata: { duration: Date.now() - start, toolName: SEC_TOOL },
				}
			}
		},
	}
}

function register(server: McpServer, tool: ProviderTool) {
	server.registerTool(
		tool.name,
		{ description: tool.description, inputSchema: tool.parameters },
		async (args) => {
			const result = await tool.execute(args)
			return {
				content: [{ type: 'text', text: JSON.stringify(result) }],
				isError: result.status === 'failed',
			}
		},
	)
}

/**
 * Providers need credentials and network access, so they are registered only
 * when available. Without `FMP_KEY` / `SEC_USER_AGENT` the server behaves
 * exactly as before: local, offline, and credential-free.
 */
export function registerConfiguredProviders(server: McpServer): void {
	if (process.env.FMP_KEY) {
		register(server, createFmpFundamentalsTool(createFmpProvider({ client: createFmpClient() })))
	}
	if (process.env.SEC_USER_AGENT) {
		register(server, createSecReitTool(createSecReitEnricher({ client: createSecClient() })))
	}
}
