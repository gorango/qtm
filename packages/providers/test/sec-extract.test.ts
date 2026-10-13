import { describe, expect, it } from 'vitest'
import { extractReitMetrics, perShareFromTotal } from '../src/sec/extract'
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

	it('joins label and value across <p>-wrapped cells (real EDGAR markup)', () => {
		const html = `
			<table><tr>
				<td><p>Diluted FFO per share and unit <sup>(2)(3)</sup></p></td>
				<td><p><span>&#8203;</span></p></td>
				<td><p>$</p></td>
				<td><p><span> 2.73</span></p></td>
			</tr></table>`
		expect(extractReitMetrics(html).ffoPerShare).toBeCloseTo(2.73)
	})

	it('skips footnotes and aggregate discussions', () => {
		const html = `
			<tr><td>(1) FFO and Diluted FFO per share includes $40.0 million, or $0.10 per share, of accelerated stock compensation</td><td></td></tr>
			<tr><td>(2) The Teraco noncontrolling share of FFO was $19,979 for the three months ended</td></tr>`
		expect(extractReitMetrics(html).ffoPerShare).toBeUndefined()
	})

	it('reads a per-share figure stated in prose', () => {
		const html = '<p>our diluted Adjusted Funds from Operations ("AFFO") per share of $2.22.</p>'
		expect(extractReitMetrics(html).affoPerShare).toBeCloseTo(2.22)
	})

	it('reads a per-share row labelled just "FFO" but rejects totals in thousands', () => {
		const html = `
			<table>
			<tr><td>FFO</td><td>$</td><td>1.56</td><td>$</td><td>1.24</td></tr>
			<tr><td>FFO attributable to common stockholders</td><td>$</td><td>1,153,653</td><td>$</td><td>982,548</td></tr>
			</table>`
		expect(extractReitMetrics(html).ffoPerShare).toBeCloseTo(1.56)
	})

	it('exposes totals when the filing omits a per-share line', () => {
		const html =
			'<tr><td>NAREIT defined FFO attributable to common stockholders</td><td>$</td><td>3,368</td><td>$</td><td>2,598</td></tr>'
		const out = extractReitMetrics(html)
		expect(out.ffoPerShare).toBeUndefined()
		expect(out.totals.ffo).toBe(3368)
		expect(out.warnings.join(' ')).toMatch(/total found/)
	})

	it('ignores numeric HTML entities (&#160;) that would parse as numbers', () => {
		const html =
			'<tr><td>&#160; Funds from operations attributable to common stockholders</td><td>&#160; $ 457,305</td></tr>'
		const out = extractReitMetrics(html)
		expect(out.totals.ffo).toBe(457305)
	})
})

describe('perShareFromTotal', () => {
	it('infers millions for a PLD-style total', () => {
		expect(perShareFromTotal(3368, 957_884_000)).toBeCloseTo(3.516, 2)
	})

	it('infers thousands for an EXR-style total', () => {
		expect(perShareFromTotal(457_305, 215_000_000)).toBeCloseTo(2.127, 2)
	})

	it('honours an explicit scale', () => {
		expect(perShareFromTotal(3368, 957_884_000, 1e6)).toBeCloseTo(3.516, 2)
		expect(perShareFromTotal(3368, 957_884_000, 1)).toBeUndefined()
	})

	it('returns undefined rather than guessing when no scale fits', () => {
		expect(perShareFromTotal(1, 1e12)).toBeUndefined()
		expect(perShareFromTotal(0, 1000)).toBeUndefined()
		expect(perShareFromTotal(100, 0)).toBeUndefined()
	})
})
