import { describe, it, expect } from 'vitest'
import { spawn, type ChildProcess } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'
import { once } from 'node:events'

const packageRoot = join(dirname(fileURLToPath(import.meta.url)), '..')

async function withServer<T>(fn: (server: ChildProcess) => Promise<T>): Promise<T> {
	const server = spawn('bun', ['run', 'src/index.ts'], {
		cwd: packageRoot,
		env: { ...process.env },
		stdio: ['pipe', 'pipe', 'inherit'],
	})

	let timeout: ReturnType<typeof setTimeout>

	try {
		const result = await Promise.race([
			fn(server),
			new Promise<never>((_, reject) => {
				timeout = setTimeout(() => reject(new Error('Server test timed out')), 15_000)
			}),
		])
		return result
	} finally {
		clearTimeout(timeout!)
		server.stdin?.end()
		server.kill('SIGTERM')
		if (server.exitCode === null) {
			await once(server, 'exit')
		}
	}
}

function request(server: ChildProcess, method: string, params: unknown = {}): Promise<any> {
	return new Promise<any>((resolve, reject) => {
		const msg = JSON.stringify({ jsonrpc: '2.0', id: 1, method, params }) + '\n'
		const chunks: Buffer[] = []

		const onData = (data: Buffer) => {
			chunks.push(data)
			const combined = Buffer.concat(chunks).toString()
			try {
				resolve(JSON.parse(combined))
				server.stdout?.removeListener('data', onData)
			} catch {}
		}

		const onError = (err: Error) => {
			server.stdout?.removeListener('data', onData)
			reject(err)
		}

		const onClose = () => {
			server.stdout?.removeListener('data', onData)
			reject(new Error('Server closed before response'))
		}

		server.stdout?.on('data', onData)
		server.stdout?.on('error', onError)
		server.stdout?.on('end', onClose)
		server.stdin?.write(msg)
	})
}

describe('quantamental-mcp server', () => {
	it('lists all tools', async () => {
		await withServer(async (server) => {
			const res = await request(server, 'tools/list')
			expect(res.result).toBeDefined()
			expect(Array.isArray(res.result.tools)).toBe(true)
			// 163 indicators + 72 factors + 106 strategies = 341
			expect(res.result.tools.length).toBeGreaterThan(300)

			const names = res.result.tools.map((t: any) => t.name)
			// strategy
			expect(names).toContain('strategy_buy_and_hold')
			// indicator
			expect(names).toContain('indicator_rsi')
			// factor
			expect(names).toContain('factor_price_to_earnings')
		})
	})

	it('calls a strategy tool and returns a signal array', async () => {
		await withServer(async (server) => {
			const res = await request(server, 'tools/call', {
				name: 'strategy_macd',
				arguments: { closes: Array.from({ length: 50 }, (_, i) => 100 + i * 0.5) },
			})

			expect(res.result).toBeDefined()
			expect(res.result.content).toBeDefined()
			expect(res.result.content[0].type).toBe('text')
			const result = JSON.parse(res.result.content[0].text)
			expect(result.status).toBe('completed')
			expect(Array.isArray(result.data)).toBe(true)
			expect(result.data).toHaveLength(50)
		})
	})

	it('calls an indicator tool and returns numeric output', async () => {
		await withServer(async (server) => {
			const closes = Array.from({ length: 20 }, (_, i) => 100 + i)
			const res = await request(server, 'tools/call', {
				name: 'indicator_rsi',
				arguments: { values: closes },
			})

			expect(res.result).toBeDefined()
			const result = JSON.parse(res.result.content[0].text)
			expect(result.status).toBe('completed')
			expect(Array.isArray(result.data)).toBe(true)
			expect(result.data.length).toBe(closes.length)
		})
	})

	it('calls a factor tool and returns factor points', async () => {
		await withServer(async (server) => {
			const res = await request(server, 'tools/call', {
				name: 'factor_price_to_book',
				arguments: {
					fundamentals: [
						{
							symbol: 'AAPL',
							date: 1000,
							filingDate: 1000,
							period: 'Q1',
							data: {
								marketCap: 300000,
								shareholdersEquity: 150000,
							},
						},
					],
				},
			})

			expect(res.result).toBeDefined()
			expect(res.result.isError).not.toBe(true)
			const result = JSON.parse(res.result.content[0].text)
			expect(result.status).toBe('completed')
			expect(Array.isArray(result.data)).toBe(true)
			expect(result.data[0].value).toBe(2)
		})
	})
})
