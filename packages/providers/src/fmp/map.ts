import type { FundamentalPoint, FundamentalPointData } from '@quantamental/core'
import { num, ts } from '../num'
import { deriveReitMetrics, forwardAnnualDividendRate, trailingAnnualDividend } from './enrich'
import type { FmpBundle, FmpOwnerEarnings, FmpRow } from './types'

export interface MapResult {
	points: FundamentalPoint[]
	warnings: string[]
	/** True when any value was modelled rather than reported. */
	estimated: boolean
}

/** Fields kept out of the public contract but needed for REIT derivation. */
interface Raw extends Record<string, number | undefined> {
	netIncome: number | undefined
	depreciationAndAmortization: number | undefined
	preferredDividendsPaid: number | undefined
	shares: number | undefined
	annualDividend: number | undefined
}

interface Entry {
	symbol: string
	date: number
	filingDate: number
	period: string
	data: FundamentalPointData
	raw: Raw
}

/** Copy only defined values so later rows never clobber earlier ones with `undefined`. */
function assign<T extends object>(target: T, source: Partial<T>): void {
	const t = target as Record<string, unknown>
	for (const [key, value] of Object.entries(source)) {
		if (value !== undefined) t[key] = value
	}
}

/**
 * Normalize an FMP bundle into quantamental's `FundamentalPoint[]`.
 *
 * Statement, key-metric, and ratio rows are joined on `date` (they share a
 * period key). Snapshot endpoints (`quote`, `profile`, `price-target-consensus`,
 * `ratings-snapshot`, `discounted-cash-flow`) are point-in-time, so they are
 * applied to the latest period only. Dividends are computed as a trailing
 * 365-day sum as of each period, and REIT metrics are derived per period.
 */
export function toFundamentalPoints(bundle: FmpBundle): MapResult {
	const warnings: string[] = []
	const entries = new Map<string, Entry>()

	function entryFor(row: FmpRow): Entry | undefined {
		if (!row?.date) return undefined
		let entry = entries.get(row.date)
		if (!entry) {
			entry = {
				symbol: row.symbol ?? '',
				date: ts(row.date),
				filingDate: ts(row.filingDate ?? row.date),
				period: row.period ?? '',
				data: {},
				raw: {
					netIncome: undefined,
					depreciationAndAmortization: undefined,
					preferredDividendsPaid: undefined,
					shares: undefined,
					annualDividend: undefined,
				},
			}
			entries.set(row.date, entry)
		}
		if (row.symbol) entry.symbol = row.symbol
		if (row.period) entry.period = row.period
		if (row.filingDate) entry.filingDate = ts(row.filingDate)
		return entry
	}

	for (const row of bundle.income ?? []) {
		const entry = entryFor(row)
		if (!entry) continue
		assign(entry.data, {
			revenue: num(row.revenue),
			costOfRevenue: num(row.costOfRevenue),
			grossProfit: num(row.grossProfit),
			operatingIncome: num(row.operatingIncome),
			netIncome: num(row.netIncome),
			eps: num(row.eps),
			ebitda: num(row.ebitda),
			costAndExpenses: num(row.costAndExpenses),
			researchAndDevelopmentExpenses: num(row.researchAndDevelopmentExpenses),
			interestExpense: num(row.interestExpense),
		})
		entry.raw.netIncome = num(row.netIncome)
		entry.raw.depreciationAndAmortization = num(row.depreciationAndAmortization)
		entry.raw.shares = num(row.weightedAverageShsOut)
	}

	for (const row of bundle.balance ?? []) {
		const entry = entryFor(row)
		if (!entry) continue
		assign(entry.data, {
			totalAssets: num(row.totalAssets),
			currentAssets: num(row.totalCurrentAssets),
			currentLiabilities: num(row.totalCurrentLiabilities),
			totalLiabilities: num(row.totalLiabilities),
			totalDebt: num(row.totalDebt),
			shareholdersEquity: num(row.totalStockholdersEquity),
			retainedEarnings: num(row.retainedEarnings),
			commonStock: num(row.commonStock),
			cashAndEquivalents: num(row.cashAndCashEquivalents),
			propertyPlantEquipment: num(row.propertyPlantEquipmentNet),
		})
	}

	for (const row of bundle.cashFlow ?? []) {
		const entry = entryFor(row)
		if (!entry) continue
		assign(entry.data, {
			operatingCashFlow: num(row.operatingCashFlow),
			capitalExpenditure: num(row.capitalExpenditure),
			dividendsPaid: num(row.netDividendsPaid),
			shareRepurchases: num(row.commonStockRepurchased),
		})
		entry.raw.preferredDividendsPaid = num(row.preferredDividendsPaid)
	}

	for (const row of bundle.keyMetrics ?? []) {
		const entry = entryFor(row)
		if (!entry) continue
		assign(entry.data, {
			marketCap: num(row.marketCap),
			enterpriseValue: num(row.enterpriseValue),
		})
	}

	for (const row of bundle.ratios ?? []) {
		const entry = entryFor(row)
		if (!entry) continue
		assign(entry.data, {
			assetTurnover: num(row.assetTurnover),
			dividendYield: num(row.dividendYield),
		})
	}

	const ordered = [...entries.values()].sort((a, b) => a.date - b.date)
	if (ordered.length === 0) {
		return { points: [], warnings: ['no fundamental periods returned'], estimated: false }
	}

	// Shares outstanding: prefer the period's weighted-average count; fall back
	// to the current quote so per-share factors still run.
	const quoteShares = num(bundle.quote?.[0]?.sharesOutstanding)
	for (const entry of ordered) {
		const shares = entry.raw.shares ?? quoteShares
		if (shares !== undefined) entry.data.sharesOutstanding = shares
	}
	if (quoteShares === undefined && !ordered.some((e) => e.raw.shares !== undefined)) {
		warnings.push('shares outstanding unavailable; per-share factors will be skipped')
	}

	// Point-in-time snapshots apply to the latest period.
	const latest = ordered[ordered.length - 1]
	if (latest) {
		const marketCap = num(bundle.quote?.[0]?.marketCap) ?? num(bundle.profile?.[0]?.marketCap)
		if (latest.data.marketCap === undefined && marketCap !== undefined) {
			latest.data.marketCap = marketCap
		}
		const target = num(bundle.priceTarget?.[0]?.targetConsensus)
		if (target !== undefined) latest.data.analystTargetPrice = target
		const rating = num(bundle.rating?.[0]?.overallScore)
		if (rating !== undefined) latest.data.rating = rating
		const dcf = num(bundle.dcf?.[0]?.dcf)
		if (dcf !== undefined) latest.data.dcf = dcf
	}

	// Dividends: trailing sum per period, plus an annualized forward rate.
	const dividends = bundle.dividends ?? []
	for (const entry of ordered) {
		const trailing = trailingAnnualDividend(dividends, entry.date)
		if (trailing !== undefined) entry.data.dividendsPerShare = trailing
		entry.raw.annualDividend = forwardAnnualDividendRate(dividends, entry.date) ?? trailing
		if (entry.raw.annualDividend !== undefined) {
			entry.data.forwardAnnualDividendRate = entry.raw.annualDividend
		}
	}

	// REIT metrics (proxy — see `deriveReitMetrics`).
	const ownerEarnings = [...(bundle.ownerEarnings ?? [])].sort((a, b) =>
		a.date.localeCompare(b.date),
	)
	for (const entry of ordered) {
		const metrics = deriveReitMetrics({
			netIncome: entry.raw.netIncome,
			depreciationAndAmortization: entry.raw.depreciationAndAmortization,
			preferredDividendsPaid: entry.raw.preferredDividendsPaid,
			maintenanceCapex: maintenanceCapexFor(ownerEarnings, entry.date),
			sharesOutstanding: entry.data.sharesOutstanding,
			annualDividend: entry.raw.annualDividend,
		})
		if (metrics.ffoPerShare !== undefined) entry.data.ffoPerShare = metrics.ffoPerShare
		if (metrics.affoPerShare !== undefined) entry.data.affoPerShare = metrics.affoPerShare
		if (metrics.payoutRatioFfo !== undefined) entry.data.payoutRatioFfo = metrics.payoutRatioFfo
		warnings.push(...metrics.warnings)
	}

	const points = ordered.map<FundamentalPoint>((entry) => ({
		symbol: entry.symbol,
		date: entry.date,
		filingDate: entry.filingDate,
		period: entry.period,
		data: entry.data,
	}))

	return {
		points,
		warnings: [...new Set(warnings)],
		estimated: points.some((p) => p.data.affoPerShare !== undefined),
	}
}

/** Owner-earnings maintenance-capex row at or before `date`; else the latest. */
function maintenanceCapexFor(rows: FmpOwnerEarnings[], date: number): number | undefined {
	if (rows.length === 0) return undefined
	let match: FmpOwnerEarnings | undefined
	for (const row of rows) {
		if (ts(row.date) <= date) match = row
		else break
	}
	return num((match ?? rows[rows.length - 1]).maintenanceCapex)
}
