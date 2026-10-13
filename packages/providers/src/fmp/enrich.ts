import type { FmpDividend } from './types'
import { num } from '../num'

const DAY_MS = 24 * 60 * 60 * 1000

export interface ReitInputs {
	netIncome?: number
	depreciationAndAmortization?: number
	preferredDividendsPaid?: number
	/** Recurring capex if known; otherwise the owner-earnings model value. */
	maintenanceCapex?: number
	sharesOutstanding?: number
	/** Annualized forward dividend per share. */
	annualDividend?: number
}

export interface ReitMetrics {
	ffoPerShare?: number
	affoPerShare?: number
	payoutRatioFfo?: number
	warnings: string[]
}

/**
 * Derive REIT metrics that FMP does not report directly.
 *
 * FFO/AFFO are approximated because the structured feed omits impairment and
 * gain-on-sale lines (FFO) and the maintenance-vs-growth capex split plus
 * straight-line rent (AFFO). `affoPerShare` is **annual** — matching
 * `price_to_affo`, `reit_dividend_safety`, and `high_yield_reit_strategy`
 * (P/AFFO threshold 15, payout < 0.95).
 */
export function deriveReitMetrics(inputs: ReitInputs): ReitMetrics {
	const warnings: string[] = []
	const { netIncome, depreciationAndAmortization, preferredDividendsPaid } = inputs
	const shares = inputs.sharesOutstanding

	if (netIncome === undefined) {
		return { warnings }
	}

	const ffo = netIncome + (depreciationAndAmortization ?? 0) - (preferredDividendsPaid ?? 0)
	if (depreciationAndAmortization === undefined) {
		warnings.push('FFO omits depreciation: D&A unavailable')
	}

	let affo = ffo
	if (inputs.maintenanceCapex !== undefined) {
		affo = ffo - inputs.maintenanceCapex
	} else {
		warnings.push('AFFO uses FFO: no maintenance capex available (overstates AFFO)')
	}
	// Impairments and gains on sale are not in the structured feed, so FFO here
	// is not reconciled to the NAREIT definition.

	if (shares === undefined || shares <= 0) {
		warnings.push('REIT per-share metrics skipped: shares outstanding unavailable')
		return { warnings }
	}

	const ffoPerShare = ffo / shares
	const affoPerShare = affo / shares
	const metrics: ReitMetrics = { ffoPerShare, affoPerShare, warnings }

	if (inputs.annualDividend !== undefined && affoPerShare > 0) {
		metrics.payoutRatioFfo = inputs.annualDividend / affoPerShare
	}

	return metrics
}

/** Multiplier to annualize a dividend for a given FMP payment frequency. */
export function frequencyMultiplier(frequency: string | null | undefined): number {
	switch ((frequency ?? '').trim().toLowerCase()) {
		case 'quarterly':
		case 'quarter':
			return 4
		case 'semi-annual':
		case 'semiannual':
		case 'semi annual':
			return 2
		case 'monthly':
			return 12
		default:
			return 1
	}
}

/** Sum dividend payments in the 365 days ending at `asOf` (epoch ms). */
export function trailingAnnualDividend(dividends: FmpDividend[], asOf: number): number | undefined {
	if (!Number.isFinite(asOf) || asOf <= 0) return undefined
	const from = asOf - 365 * DAY_MS
	let total = 0
	let seen = false
	for (const d of dividends) {
		const t = Date.parse(d.date)
		if (!Number.isFinite(t) || t > asOf) continue
		const amount = num(d.dividend) ?? num(d.adjDividend)
		if (amount === undefined || t < from) continue
		total += amount
		seen = true
	}
	return seen ? total : undefined
}

/**
 * Forward annual dividend per share: the most recent payment on or before
 * `asOf`, annualized by its declared frequency. This is a backwards-looking
 * proxy — FMP has no forward-looking dividend field.
 */
export function forwardAnnualDividendRate(
	dividends: FmpDividend[],
	asOf: number,
): number | undefined {
	let latest: FmpDividend | undefined
	let latestTs = -Infinity
	for (const d of dividends) {
		const t = Date.parse(d.date)
		if (!Number.isFinite(t) || (asOf > 0 && t > asOf)) continue
		if (t > latestTs) {
			latestTs = t
			latest = d
		}
	}
	if (!latest) return undefined
	const amount = num(latest.dividend) ?? num(latest.adjDividend)
	if (amount === undefined) return undefined
	return amount * frequencyMultiplier(latest.frequency)
}
