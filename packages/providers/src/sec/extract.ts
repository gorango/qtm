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
	/**
	 * Ignore values at or above this magnitude. Per-share figures are small;
	 * filings also print dollar totals, which this filters out. Defaults to
	 * 10000 (enough to exclude totals reported in thousands).
	 */
	maxValue?: number
	/** Segments longer than this are skipped (table rows are short). */
	maxSegmentLength?: number
	/**
	 * Largest value that still looks per-share when a row omits the words
	 * "per share" (issuers print totals in thousands in the same table).
	 * Defaults to 100.
	 */
	perShareMax?: number
}

const FFO_PER_SHARE = /\bffo\b/i
const AFFO_PER_SHARE = /\baffo\b|adjusted funds from operations|core ffo|adjusted ffo/i
const FOOTNOTE = /^\(\d+\)/
const AGGREGATE = /\b(million|billion|thousand)\b/i
/** A row whose label leads with the metric, e.g. "Diluted FFO", "Core FFO". */
const LABEL_LEAD =
	/^\s*(?:(?:diluted|basic|nareit(?:\s+defined)?|core|real\s+estate|adjusted|modified|as)\s+)*(?:funds\s+from\s+operations|ffo|affo)\b/i

/**
 * Extract reported FFO/AFFO per share from a REIT 10-K/10-Q (or earnings
 * release). Deliberately heuristic: SEC filings have no standard machine
 * format for the FFO reconciliation, so this reads the per-share lines and
 * reports what it matched. Missing values are surfaced as warnings rather than
 * guessed.
 */
export function extractReitMetrics(html: string, options: ExtractOptions = {}): ReportedReit {
	const context = {
		column: options.column ?? 0,
		maxValue: options.maxValue ?? 10000,
		maxSegmentLength: options.maxSegmentLength ?? 600,
		perShareMax: options.perShareMax ?? 100,
	}

	// Table rows first — the reconciliation is usually a table. Prose (some
	// REITs state "AFFO per share of $X" in a sentence) is a fallback.
	const rows = segments(html, /<\/tr>/gi)
	const prose = segments(html, /<\/p>|<\/div>|<\/li>|<\/h[1-6]>/gi)

	const pick = (metric: RegExp): Match => {
		const fromRows = matchPerShare(rows, metric, context)
		if (fromRows.value !== undefined) return fromRows
		// Some issuers label the per-share row just "FFO"/"AFFO" in a table
		// whose header says "per share" — accept those when every value looks
		// per-share sized (which rejects the "dollars in thousands" totals rows).
		const loose = matchPerShareRow(rows, metric, context)
		return loose.value !== undefined ? loose : matchPerShare(prose, metric, context)
	}

	const ffo = pick(FFO_PER_SHARE)
	const affo = pick(AFFO_PER_SHARE)
	const warnings: string[] = []
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

interface Context {
	column: number
	maxValue: number
	maxSegmentLength: number
	perShareMax: number
}

function matchPerShare(segments: string[], metric: RegExp, context: Context): Match {
	let fallback: Match = {}
	for (const segment of segments) {
		if (segment.length > context.maxSegmentLength) continue
		if (!/per\s*share/i.test(segment) || !metric.test(segment)) continue
		// Footnotes and aggregate discussions mention "FFO per share" but quote
		// totals, not the per-share figure.
		if (FOOTNOTE.test(segment.trim()) || AGGREGATE.test(segment)) continue

		// Per-share values carry cents or a currency sign; requiring one skips
		// footnote markers like "(1)" and plain integers.
		const tokens = tokenize(segment).filter(
			(t) => (t.hasDecimal || t.hasCurrency) && Math.abs(t.value) < context.maxValue,
		)
		if (tokens.length <= context.column) continue

		const candidate: Match = { value: tokens[context.column].value, line: segment.trim().slice(0, 200) }
		if (/diluted/i.test(segment)) return candidate
		if (fallback.value === undefined) fallback = candidate
	}
	return fallback
}

/**
 * Fallback for rows that label the per-share figure without the words
 * "per share" (the table header carries them). Only accepts rows whose every
 * value is per-share sized, so totals and "% change" rows are rejected.
 */
function matchPerShareRow(segments: string[], metric: RegExp, context: Context): Match {
	let fallback: Match = {}
	for (const segment of segments) {
		const label = segment.trim()
		if (label.length > context.maxSegmentLength) continue
		if (FOOTNOTE.test(label) || AGGREGATE.test(segment)) continue
		if (!LABEL_LEAD.test(label) || !metric.test(segment)) continue

		const tokens = tokenize(segment).filter((t) => t.hasDecimal || t.hasCurrency)
		if (tokens.length <= context.column) continue
		if (!tokens.every((t) => Math.abs(t.value) < context.perShareMax)) continue

		const candidate: Match = { value: tokens[context.column].value, line: label.slice(0, 200) }
		if (/diluted/i.test(segment)) return candidate
		if (fallback.value === undefined) fallback = candidate
	}
	return fallback
}

function tokenize(segment: string): Token[] {
	const tokens: Token[] = []
	const re = /\(?\$?\s*\d[\d,]*(?:\.\d+)?\s*\)?/g
	for (const raw of segment.match(re) ?? []) {
		const negative = raw.includes('(')
		const cleaned = raw.replace(/[$,()\s]/g, '')
		const value = Number(cleaned)
		if (!Number.isFinite(value)) continue
		tokens.push({
			value: negative ? -value : value,
			hasDecimal: cleaned.includes('.'),
			hasCurrency: raw.includes('$'),
		})
	}
	return tokens
}

/**
 * Flatten filing HTML into trimmed segments split on `splitter`. Every EDGAR
 * table cell wraps its text in `<p>`, so splitting on `</p>` would separate a
 * row's label from its values — rows are split on `</tr>` (and `</table>`)
 * instead. The delimiter is swapped for a sentinel so that whitespace (which
 * may appear between cells in the source) can be collapsed first.
 */
function segments(html: string, splitter: RegExp): string[] {
	const SENTINEL = '\u0000'
	return html
		.replace(/<(script|style)[\s\S]*?<\/\1>/gi, ' ')
		.replace(splitter, SENTINEL)
		.replace(/<br\s*\/?>/gi, SENTINEL)
		.replace(/<[^>]+>/g, ' ')
		.replace(/&nbsp;/gi, ' ')
		.replace(/&amp;/gi, '&')
		.replace(/&#39;|&rsquo;|&apos;/gi, "'")
		.replace(/[\s\u200b\u200c\u200d\ufeff]+/g, ' ')
		.split(SENTINEL)
		.map((segment) => segment.trim())
		.filter(Boolean)
}
