import type { Numeric } from '../num'

/**
 * Narrow views of the FMP rows this package reads. Fields are intentionally a
 * subset — everything else FMP returns is ignored. Types mirror
 * `@financialmodelingprep/sdk`'s `types.ts` but are declared locally so the
 * package has no build-time dependency on the SDK.
 */

export interface FmpRow {
	date: string
	symbol?: string
	filingDate?: string | null
	period?: string | null
	fiscalYear?: string | null
}

export interface FmpIncomeStatement extends FmpRow {
	revenue?: Numeric
	costOfRevenue?: Numeric
	grossProfit?: Numeric
	researchAndDevelopmentExpenses?: Numeric
	costAndExpenses?: Numeric
	interestExpense?: Numeric
	depreciationAndAmortization?: Numeric
	ebitda?: Numeric
	operatingIncome?: Numeric
	netIncome?: Numeric
	eps?: Numeric
	weightedAverageShsOut?: Numeric
}

export interface FmpBalanceSheet extends FmpRow {
	cashAndCashEquivalents?: Numeric
	totalCurrentAssets?: Numeric
	propertyPlantEquipmentNet?: Numeric
	totalAssets?: Numeric
	totalCurrentLiabilities?: Numeric
	totalLiabilities?: Numeric
	totalDebt?: Numeric
	commonStock?: Numeric
	retainedEarnings?: Numeric
	totalStockholdersEquity?: Numeric
}

export interface FmpCashFlowStatement extends FmpRow {
	netIncome?: Numeric
	preferredDividendsPaid?: Numeric
	netDividendsPaid?: Numeric
	commonStockRepurchased?: Numeric
	operatingCashFlow?: Numeric
	capitalExpenditure?: Numeric
	freeCashFlow?: Numeric
}

export interface FmpKeyMetrics extends FmpRow {
	marketCap?: Numeric
	enterpriseValue?: Numeric
}

export interface FmpFinancialRatios extends FmpRow {
	assetTurnover?: Numeric
	dividendYield?: Numeric
}

export interface FmpQuote {
	symbol: string
	price?: Numeric
	marketCap?: Numeric
	sharesOutstanding?: Numeric
}

export interface FmpCompanyProfile {
	symbol: string
	price?: Numeric
	marketCap?: Numeric
	lastDividend?: Numeric
}

export interface FmpPriceTargetConsensus {
	symbol: string
	targetHigh?: Numeric
	targetLow?: Numeric
	targetConsensus?: Numeric
	targetMedian?: Numeric
}

export interface FmpRatingSnapshot {
	symbol: string
	rating?: string | null
	overallScore?: Numeric
}

export interface FmpDcfValuation {
	symbol: string
	date?: string | null
	dcf?: Numeric
	'Stock Price'?: Numeric
}

export interface FmpOwnerEarnings extends FmpRow {
	averagePPE?: Numeric
	maintenanceCapex?: Numeric
	ownersEarnings?: Numeric
	growthCapex?: Numeric
	ownersEarningsPerShare?: Numeric
}

export interface FmpDividend {
	symbol: string
	date: string
	dividend?: Numeric
	adjDividend?: Numeric
	frequency?: string | null
	yield?: Numeric
}

/**
 * Everything the FMP provider needs to build a `FundamentalPoint[]` panel.
 * Follows the statement/metrics endpoints (per-period, joined on `date`) plus
 * the point-in-time snapshot endpoints.
 */
export interface FmpBundle {
	income: FmpIncomeStatement[]
	balance?: FmpBalanceSheet[]
	cashFlow?: FmpCashFlowStatement[]
	keyMetrics?: FmpKeyMetrics[]
	ratios?: FmpFinancialRatios[]
	/** Point-in-time snapshots — applied to the latest period only. */
	quote?: FmpQuote[]
	profile?: FmpCompanyProfile[]
	priceTarget?: FmpPriceTargetConsensus[]
	rating?: FmpRatingSnapshot[]
	dcf?: FmpDcfValuation[]
	/** Per-period by date when available. */
	ownerEarnings?: FmpOwnerEarnings[]
	dividends?: FmpDividend[]
}

/** The subset of the FMP API this provider depends on. */
export interface FmpClient {
	incomeStatement(symbol: string, options?: FmpStatementOptions): Promise<FmpIncomeStatement[]>
	balanceSheetStatement(symbol: string, options?: FmpStatementOptions): Promise<FmpBalanceSheet[]>
	cashFlowStatement(symbol: string, options?: FmpStatementOptions): Promise<FmpCashFlowStatement[]>
	keyMetrics(symbol: string, options?: FmpStatementOptions): Promise<FmpKeyMetrics[]>
	financialRatios(symbol: string, options?: FmpStatementOptions): Promise<FmpFinancialRatios[]>
	companyProfile(symbol: string): Promise<FmpCompanyProfile[]>
	stockQuote(symbol: string): Promise<FmpQuote[]>
	priceTargetConsensus(symbol: string): Promise<FmpPriceTargetConsensus[]>
	ratingsSnapshot(symbol: string): Promise<FmpRatingSnapshot[]>
	dcfValuation(symbol: string): Promise<FmpDcfValuation[]>
	ownerEarnings(symbol: string, options?: FmpStatementOptions): Promise<FmpOwnerEarnings[]>
	companyDividends(symbol: string): Promise<FmpDividend[]>
}

export interface FmpStatementOptions {
	period?: 'annual' | 'quarter'
	limit?: number
	signal?: AbortSignal
}
