#!/usr/bin/env python3
"""Fetch top coins from CoinGecko, cross-check against Binance Futures USDT
perpetuals, and write config/symbols.yaml — the initial hydration universe
for the standalone Binance OHLCV pipeline (binance_loader.py)."""

import json
import os
import urllib.request

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SYMBOLS_PATH = os.path.join(REPO, "config", "symbols.yaml")
COINGECKO_URL = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=200&page=1"
BINANCE_URL = "https://fapi.binance.com/fapi/v1/exchangeInfo"


def fetch_json(url):
    resp = urllib.request.urlopen(url)
    return json.loads(resp.read())


def fmt_cap(n):
    if n >= 1e12:
        return f"${n / 1e12:.2f}T"
    elif n >= 1e9:
        return f"${n / 1e9:.2f}B"
    elif n >= 1e6:
        return f"${n / 1e6:.1f}M"
    return f"${n:.0f}"


def main():
    print("Fetching CoinGecko top 200...")
    coins = fetch_json(COINGECKO_URL)

    print("Fetching Binance Futures exchangeInfo...")
    exchange_info = fetch_json(BINANCE_URL)

    binance_symbols = {
        s["symbol"]: s
        for s in exchange_info["symbols"]
        if s["status"] == "TRADING"
        and s["contractType"] == "PERPETUAL"
        and s["quoteAsset"] == "USDT"
    }
    print(f"  {len(binance_symbols)} USDT perpetuals on Binance Futures")

    exclude = {
        "usdt",
        "usdc",
        "usds",
        "usde",
        "dai",
        "usdg",
        "pyusd",
        "usd1",
        "usdd",
        "usdf",
        "usd0",
        "usx",
        "crvusd",
        "usdai",
        "usdy",
        "susdc",
        "susde",
        "reusd",
        "satusd",
        "ausd",
        "usat",
        "fdusd",
        "tusd",
        "busd",
        "frax",
        "lusd",
        "gusd",
        "husd",
        "musd",
        "ustb",
        "eutbl",
        "ousg",
        "usdglo",
        "apxusd",
        "usdgo",
        "usyc",
        "gho",
        "eurc",
        "payb",
        "stable",
        "bfusd",
        "figr_heloc",
    }

    matched = []
    for c in coins:
        sym = c["symbol"].lower()
        rank = c.get("market_cap_rank") or "?"
        cap = c.get("market_cap") or 0

        if sym in exclude:
            continue
        if not sym.isascii():
            continue

        candidates = [sym.upper() + "USDT"]
        if len(sym) <= 6:
            candidates.append("1000" + sym.upper() + "USDT")

        for bsym in candidates:
            if bsym in binance_symbols:
                canon = bsym.replace("USDT", "/USDT:PERP")
                matched.append((rank, canon, c["name"], cap))
                break

    matched.sort(key=lambda x: int(x[0]) if str(x[0]).isdigit() else 999)

    lines = [
        "# Symbols that trade on Binance Futures USDT perpetuals",
        "# (canonical {BASE}/USDT:PERP spelling; the OHLCV loader hydrates these)",
        f"# Generated {__import__('datetime').date.today()} from CoinGecko top-200 × Binance exchangeInfo",
    ]
    for rank, canon, name, cap in matched:
        lines.append(f'- "{canon}"  # {rank}. {name} ({fmt_cap(cap)})')

    out = "\n".join(lines) + "\n"
    with open(SYMBOLS_PATH, "w") as f:
        f.write(out)

    print(f"\nWrote {len(matched)} symbols to {SYMBOLS_PATH}")


if __name__ == "__main__":
    main()