import { describe, it, expect } from 'vitest'
import { z } from 'zod'
import { createWorkflowTool } from '../src/tool'
import {
	toVercelTool,
	toOpenAISchema,
	toAnthropicTool,
	toLangChainTool,
} from '../src/adapters'
import {
	createErrorResult,
	createSuccessResult,
	ErrorCodes,
} from '../src/utils'

const testSchema = z.object({
	values: z.array(z.number()),
	period: z.number().int().positive().default(14),
})

const mockTool = createWorkflowTool({
	name: 'test_indicator',
	description: 'A test indicator tool',
	parameters: testSchema,
	execute: async (params) => {
		const result = params.values.map((v) => v * 2)
		return { status: 'completed', data: result }
	},
})

describe('createWorkflowTool', () => {
	it('creates a tool with correct name and description', () => {
		expect(mockTool.name).toBe('test_indicator')
		expect(mockTool.description).toBe('A test indicator tool')
	})

	it('creates a tool that executes successfully', async () => {
		const result = await mockTool.execute({ values: [1, 2, 3], period: 14 })
		expect(result.status).toBe('completed')
		expect(result.data).toEqual([2, 4, 6])
	})
})

describe('adapters', () => {
	it('converts to Vercel format', () => {
		const vercel = toVercelTool(mockTool)
		expect(vercel.name).toBe('test_indicator')
		expect(vercel.description).toBe('A test indicator tool')
		expect(vercel.parameters).toBeDefined()
	})

	it('converts to OpenAI format', () => {
		const openai = toOpenAISchema(mockTool)
		expect(openai.type).toBe('function')
		expect(openai.function.name).toBe('test_indicator')
		expect(openai.function.parameters).toBeDefined()
	})

	it('converts to Anthropic format', () => {
		const anthropic = toAnthropicTool(mockTool)
		expect(anthropic.name).toBe('test_indicator')
		expect(anthropic.input_schema).toBeDefined()
	})

	it('converts to LangChain format', async () => {
		const lc = toLangChainTool(mockTool)
		expect(lc.name).toBe('test_indicator')
		expect(lc.schema).toBeDefined()
		const result = await lc.func(JSON.stringify({ values: [1, 2, 3], period: 14 }))
		const parsed = JSON.parse(result)
		expect(parsed.status).toBe('completed')
	})
})

describe('utils', () => {
	it('creates error result', () => {
		const result = createErrorResult('Something went wrong', ErrorCodes.INVALID_PARAMS, {
			duration: 100,
			toolName: 'test',
		})
		expect(result.status).toBe('failed')
		expect(result.error?.message).toBe('Something went wrong')
		expect(result.error?.code).toBe('INVALID_PARAMS')
		expect(result.metadata?.duration).toBe(100)
	})

	it('creates success result', () => {
		const result = createSuccessResult([1, 2, 3], { duration: 50, toolName: 'test' })
		expect(result.status).toBe('completed')
		expect(result.data).toEqual([1, 2, 3])
		expect(result.metadata?.duration).toBe(50)
	})
})
