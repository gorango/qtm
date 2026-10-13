import { describe, expect, it } from 'vitest'
import { extractReitMetrics } from '../src/sec/extract'
import { REIT_10Q_HTML } from './fixtures/reit-10q'

describe('extractReitMetrics', () => {
	it('reads the diluted per-share FFO and AFFO lines', () => {
		const out = extractReitMetrics(REIT_10Q_HTML)
		expect(out.ffoPerShare).toBeCloseTo(1.18)
		expect(out.affoPerShare).toBeCloseTo(1.02)
		expect(out.warnings).toHaveLength(0)
		expect(out.matched.ffo).toContain('per share')
	})

	it('honours the column option for multi-period tables', () => {
		const out = extractReitMetrics(REIT_10Q_HTML, { column: 1 })
		expect(out.ffoPerShare).toBeCloseTo(1.05)
		expect(out.affoPerShare).toBeCloseTo(0.95)
	})

	it('does not read AFFO lines as FFO', () => {
		const out = extractReitMetrics('<table><tr><td>AFFO per share - Diluted</td><td>$0.77</td></tr></table>')
		expect(out.affoPerShare).toBeCloseTo(0.77)
		expect(out.ffoPerShare).toBeUndefined()
	})

	it('warns instead of guessing when the lines are absent', () => {
		const out = extractReitMetrics('<p>This filing has no reconciliation.</p>')
		expect(out.ffoPerShare).toBeUndefined()
		expect(out.affoPerShare).toBeUndefined()
		expect(out.warnings).toHaveLength(2)
	})

	it('prefers diluted over basic and ignores footnote markers', () => {
		const html = `
			<table>
			<tr><td>Core FFO per share - Basic</td><td>$1.10</td></tr>
			<tr><td>Core FFO per share - Diluted (1)</td><td>$1.08</td></tr>
			</table>`
		const out = extractReitMetrics(html)
		expect(out.affoPerShare).toBeCloseTo(1.08)
	})

	it('handles negative and comma-formatted values', () => {
		const html = '<tr><td>Adjusted FFO per share - Diluted</td><td>($1,234.50)</td></tr>'
		expect(extractReitMetrics(html).affoPerShare).toBeCloseTo(-1234.5)
	})
})
