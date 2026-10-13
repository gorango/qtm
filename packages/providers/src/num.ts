/** FMP returns a mix of `number | string | null`, so every field is coerced here. */
export type Numeric = number | string | null | undefined

/** Coerce an FMP numeric field to a finite number, or `undefined`. */
export function num(value: Numeric): number | undefined {
	if (value === null || value === undefined || value === '') return undefined
	const n = typeof value === 'number' ? value : Number(value)
	return Number.isFinite(n) ? n : undefined
}

/**
 * Parse an FMP date (`YYYY-MM-DD`, sometimes `YYYY-MM-DD HH:MM:SS`) to epoch
 * milliseconds. Returns `0` for missing/unparseable input so the value stays
 * usable as a sort key.
 *
 * Quantamental's `date`, `filingDate`, and `Bar.time` are unit-agnostic — the
 * core only compares and sorts them — so the provider and any `prices` passed
 * alongside it must agree. This package standardizes on epoch milliseconds.
 */
export function ts(date: string | null | undefined): number {
	if (!date) return 0
	const t = Date.parse(date)
	return Number.isFinite(t) ? t : 0
}
