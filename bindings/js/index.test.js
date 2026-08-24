import { readFileSync, readdirSync } from 'node:fs'
import { join } from 'node:path'
import { createRequire } from 'node:module'
import test from 'node:test'
import assert from 'node:assert/strict'

const require = createRequire(import.meta.url)

const nodeFiles = readdirSync(import.meta.dirname).filter((f) => f.endsWith('.node'))
assert.ok(nodeFiles.length > 0, 'no .node artifact found — run `npm run build` first')

const binding = require(join(import.meta.dirname, nodeFiles[0]))

const registryJsonPath = join(import.meta.dirname, '../../packages/tools/src/generated/registry.json')
const registryJson = JSON.parse(readFileSync(registryJsonPath, 'utf8'))

test('native module loads and exposes expected exports', () => {
	assert.equal(typeof binding.rsi, 'function')
	assert.equal(typeof binding.getStrategyRegistry, 'function')
	assert.equal(typeof binding.awesomeOscillator, 'function')
})

test('indicator returns a numeric array with finite results', () => {
	const closes = new Float64Array([10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24])
	const out = binding.rsi(closes)
	assert.ok(Array.isArray(out))
	assert.ok(out.every((v) => typeof v === 'number'))
	assert.ok(out.some((v) => Number.isFinite(v)))
})

test('strategy registry is populated', () => {
	const registry = binding.getStrategyRegistry()
	assert.ok(registry && typeof registry === 'object')
	const ids = Object.keys(registry.strategies ?? {})
	assert.ok(ids.length > 0, 'expected at least one registered strategy')
	assert.ok(ids.includes('buy_and_hold'), 'expected buyAndHold to be registered')
})

test('registry strategies are dispatchable by id via runStrategy', () => {
	assert.equal(typeof binding.runStrategy, 'function')

	const signals = binding.runStrategy('buy_and_hold', { closes: [100, 101, 102, 103, 104] }, null)
	assert.deepEqual(signals, [1, 0, 0, 0, 0])

	assert.throws(
		() => binding.runStrategy('no-such-strategy', { closes: [100] }, null),
		/Unknown strategy: no-such-strategy/,
	)

	// Registry metadata is backed by a live handler for every advertised id.
	const closes = Array.from({ length: 300 }, (_, i) => 100 + i * 0.5)
	// Configs are strict (serde deny_unknown_fields): only pair strategies accept
	// a second series. Derive the config per strategy from its registered schema.
	const registry = binding.getStrategyRegistry()
	for (const id of Object.keys(registry.strategies)) {
		const schema = JSON.parse(registryJson.strategies[id]?.params_schema || '{}')
		const config = schema.properties?.secondCloses ? { secondCloses: closes } : null
		const out = binding.runStrategy(id, { closes }, config)
		assert.ok(Array.isArray(out), `strategy ${id} should produce a signal array`)
		assert.equal(out.length, closes.length, `strategy ${id} should produce one signal per bar`)
	}
})

test('runtime registry matches generated registry.json (no drift)', () => {
	const registry = binding.getStrategyRegistry()
	const runtimeIds = new Set(Object.keys(registry.strategies))
	const fileIds = new Set(Object.keys(registryJson.strategies))
	const missingFromRuntime = [...fileIds].filter((id) => !runtimeIds.has(id))
	const extraInRuntime = [...runtimeIds].filter((id) => !fileIds.has(id))
	assert.deepEqual(extraInRuntime, [], 'runtime-only strategy ids not in registry.json')
	assert.deepEqual(missingFromRuntime, [], 'registry.json strategy ids missing at runtime')
})

test('hand-written strategy metadata is reachable from Node', () => {
	assert.equal(typeof binding.valueStrategyMetadata, 'function')
	const meta = binding.valueStrategyMetadata()
	assert.ok(meta && typeof meta === 'object')
	assert.equal(meta.id, 'value-investing')
	assert.ok(meta.description)

	assert.equal(typeof binding.qarpStrategyMetadata, 'function')
	const qarp = binding.qarpStrategyMetadata()
	assert.equal(qarp.id, 'qarp')

	assert.equal(typeof binding.valueStrategyDefaults, 'function')
	const defaults = binding.valueStrategyDefaults()
	assert.ok(defaults.params, 'expected defaults.params object')
})

test('every *StrategyDefaults export returns camelCase params keys', () => {
	// Guards the whole hand-written strategy dialect: each `*StrategyDefaults`
	// export must return a `params` object whose keys are camelCase (never snake_case),
	// so they line up with the napi config structs under `#[serde(rename_all = "camelCase")]`.
	const defaultsFns = Object.keys(binding).filter(
		(k) => k.endsWith('StrategyDefaults') && k !== 'getStrategyDefaults',
	)
	assert.ok(defaultsFns.length >= 86, `expected >= 86 *StrategyDefaults exports, got ${defaultsFns.length}`)
	for (const name of defaultsFns) {
		const defaults = binding[name]()
		assert.ok(defaults && typeof defaults === 'object', `${name}() should return an object`)
		assert.ok(defaults.params && typeof defaults.params === 'object', `${name}().params should be an object`)
		for (const key of Object.keys(defaults.params)) {
			assert.ok(!key.includes('_'), `${name}().params key "${key}" must be camelCase`)
		}
	}
})

test('invalid input surfaces as a thrown error', () => {
	assert.throws(() => binding.awesomeOscillator([], [], null))
	assert.throws(() => binding.rsi(new Float64Array([])))
	assert.throws(() => binding.stochasticOscillator([], [], []))
})

test('runStrategy applies camelCase config keys (no silent drop)', () => {
	// Regression: serde config keys are camelCase and are actually applied. If the
	// camelCase `secondCloses` key were silently ignored (dropped to default `[]`),
	// the strategy would fail its length check even with correct-length data.
	const closes = Array.from({ length: 100 }, (_, i) => 100 + i)
	const secondCloses = Array.from({ length: 100 }, (_, i) => 200 + i * 1.5)

	// Correct-length camelCase key must be honored and produce a signal array.
	const out = binding.runStrategy(
		'correlation_pair_trading',
		{ closes },
		{ secondCloses },
	)
	assert.ok(Array.isArray(out))
	assert.equal(out.length, closes.length)

	// Deliberately wrong-length `secondCloses` must be applied (not dropped) and
	// therefore surface the length-mismatch validation error.
	assert.throws(
		() => binding.runStrategy('correlation_pair_trading', { closes }, { secondCloses: [1, 2, 3] }),
		/secondCloses must have the same length as closes/,
	)

	// The snake_case key is outside the strict config contract and now hard-fails
	// instead of being silently dropped.
	assert.throws(
		() => binding.runStrategy('correlation_pair_trading', { closes }, { second_closes: secondCloses }),
		/Invalid config/,
	)
})

test('runStrategy rejects unknown config keys', () => {
	// serde deny_unknown_fields: a config key that is not a struct field must
	// throw rather than silently run with defaults.
	assert.throws(
		() => binding.runStrategy('buy_and_hold', { closes: [1, 2, 3] }, { noSuchKey: 1 }),
		/Invalid config/,
	)
})
