export interface ReportedReit {
	/** FFO per share as reported (diluted preferred). */
	ffoPerShare?: number
	/** AFFO / Core FFO / Adjusted FFO per share as reported. */
	affoPerShare?: number
	/** The filing lines the values were read from, for provenance/debugging. */
	matched: { ffo?: string; affo?: string }
	warnings: string[]
}

export interface ExtractOptions {
	/**
	 * Which numeric column to read when a filing shows several periods side by
	 * side (e.g. quarter / prior quarter / YTD). Defaults to the first.
	 */
	column?: number
}

const FFO_PER_SHARE = /\bffo\b/i
const AFFO_PER_SHARE = /\baffo\b|adjusted funds from operations|core ffo|adjusted ffo/i

/**
 * Extract reported FFO/AFFO per share from a REIT 10-K/10-Q (or earnings
 * release). Deliberately heuristic: SEC filings have no standard machine
 * format for the FFO reconciliation, so this reads the per-share lines and
 * reports what it matched. Missing values are surfaced as warnings rather than
 * guessed.
 */
export function extractReitMetrics(html: string, options: ExtractOptions = {}): ReportedReit {
	const column = options.column ?? 0
	const lines = htmlLines(html)
	const warnings: string[] = []

	const ffo = matchPerShare(lines, FFO_PER_SHARE, column)
	const affo = matchPerShare(lines, AFFO_PER_SHARE, column)

	if (ffo.value === undefined) warnings.push('no FFO per share line found')
	if (affo.value === undefined) warnings.push('no AFFO per share line found')

	return {
		ffoPerShare: ffo.value,
		affoPerShare: affo.value,
		matched: { ffo: ffo.line, affo: affo.line },
		warnings,
	}
}

interface Token {
	value: number
	hasDecimal: boolean
	hasCurrency: boolean
}

interface Match {
	value?: number
	line?: string
}

function matchPerShare(lines: string[], metric: RegExp, column: number): Match {
	let fallback: Match = {}
	for (const line of lines) {
		if (!/per\s*share/i.test(line) || !metric.test(line)) continue
		// Per-share values always carry cents; requiring a decimal or currency
		// sign skips footnote markers like "(1)" and plain integers.
		const tokens = tokenize(line).filter((t) => t.hasDecimal || t.hasCurrency)
		if (tokens.length <= column) continue
		const candidate: Match = { value: tokens[column].value, line: line.trim().slice(0, 200) }
		if (/diluted/i.test(line)) return candidate
		if (fallback.value === undefined) fallback = candidate
	}
	return fallback
}

function tokenize(line: string): Token[] {
	const tokens: Token[] = []
	const re = /\(?\$?\s*\d[\d,]*(?:\.\d+)?\s*\)?/g
	for (const raw of line.match(re) ?? []) {
		const hasCurrency = raw.includes('$')
		const negative = raw.includes('(')
		const cleaned = raw.replace(/[$,()\s]/g, '')
		const value = Number(cleaned)
		if (!Number.isFinite(value)) continue
		tokens.push({
			value: negative ? -value : value,
			hasDecimal: cleaned.includes('.'),
			hasCurrency,
		})
	}
	return tokens
}

/** Flatten filing HTML into trimmed lines, one per table row / block. */
function htmlLines(html: string): string[] {
	return html
		.replace(/<(script|style)[\s\S]*?<\/\1>/gi, ' ')
		.replace(/<\/(tr|p|div|li|h[1-6]|table)>/gi, '\n')
		.replace(/<br\s*\/?>/gi, '\n')
		.replace(/<[^>]+>/g, ' ')
		.replace(/&nbsp;/gi, ' ')
		.replace(/&amp;/gi, '&')
		.replace(/&#39;|&rsquo;|&apos;/gi, "'")
		.replace(/[ \t\u00a0]+/g, ' ')
		.split('\n')
		.map((line) => line.trim())
		.filter(Boolean)
}
