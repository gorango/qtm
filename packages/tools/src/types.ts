import type { z } from 'zod'

export type ToolStatus = 'completed' | 'failed' | 'started'

export interface ToolResult {
	status: ToolStatus
	data?: unknown
	error?: { message: string; code?: string }
	metadata?: {
		duration: number
		toolName?: string
	}
}

export interface WorkflowToolConfig<TParams extends z.ZodType = z.ZodType> {
	name: string
	description: string
	parameters: TParams
}

export interface WorkflowTool<TParams extends z.ZodType = z.ZodType> {
	name: string
	description: string
	parameters: TParams
	execute(params: z.infer<TParams>): Promise<ToolResult>
}

export interface ToolDefinition {
	id: string
	name: string
	description: string
	category: 'indicator' | 'factor' | 'strategy'
	subcategory: string
	params: Record<string, unknown>
	defaults: Record<string, unknown>
	warmup?: (params: Record<string, unknown>) => number
}

export interface ToolRegistry {
	getAll(): WorkflowTool[]
	getByCategory(category: string): WorkflowTool[]
	getByName(name: string): WorkflowTool | undefined
	getManifest(): string
}
