import type { ToolResult } from '../types'

export const ErrorCodes = {
	INSUFFICIENT_DATA: 'INSUFFICIENT_DATA',
	INVALID_PARAMS: 'INVALID_PARAMS',
	EXECUTION_ERROR: 'EXECUTION_ERROR',
	NAPI_ERROR: 'NAPI_ERROR',
	TOOL_NOT_FOUND: 'TOOL_NOT_FOUND',
} as const

export function createErrorResult(
	message: string,
	code?: string,
	metadata?: { duration?: number; toolName?: string },
): ToolResult {
	return {
		status: 'failed',
		error: { message, code },
		metadata: {
			duration: metadata?.duration ?? 0,
			toolName: metadata?.toolName,
		},
	}
}

export function createSuccessResult(
	data: unknown,
	metadata?: { duration?: number; toolName?: string },
): ToolResult {
	return {
		status: 'completed',
		data,
		metadata: {
			duration: metadata?.duration ?? 0,
			toolName: metadata?.toolName,
		},
	}
}

export function createToolResult(data: unknown, start: number, toolName?: string): ToolResult {
	const duration = Date.now() - start
	try {
		return createSuccessResult(data, { duration, toolName })
	} catch (error) {
		return createErrorResult(
			error instanceof Error ? error.message : String(error),
			ErrorCodes.EXECUTION_ERROR,
			{ duration, toolName },
		)
	}
}

export function toFloat64Array(values: number[]): Float64Array {
	return new Float64Array(values)
}
