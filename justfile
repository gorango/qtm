set shell := ["bash", "-cu"]

default:
	@just --list

# Create/sync the bindings/py dev env (uv: wheel build + dev deps).
setup:
	cd bindings/py && uv sync

# Build everything: Rust core + js (napi) + py (maturin) bindings.
build:
	cargo build --workspace --release
	cd bindings/js && bun run build
	cd bindings/py && uv run maturin build --features pyo3/abi3-py39

# Run all tests: Rust workspace, Python, Node.
test:
	cargo test --workspace
	cd bindings/py && uv run python -m pytest -q
	cd bindings/js && node --test ./index.test.js

# Lint the Rust code.
lint:
	cargo clippy --workspace

# Format the Rust code.
fmt:
	cargo fmt
	cd research/ta && uv run ruff format .

# Fast compile check (no tests).
check:
	cargo check --workspace

# Release pre-flight: everything that must be true before stamping a tag.
# Runs standalone too: clean tree, rust tests under -D warnings (CI
# parity), python binding tests, codegen snapshot freshness, js build
# with a duplicate-export scan of the generated index.js (the node smoke
# tests require the .node binary directly, so they never see index.js
# problems), then the js smoke suite.
preflight:
	#!/bin/bash
	set -euo pipefail
	git diff-index --quiet HEAD -- || {
		echo "working tree is dirty -- commit or stash before releasing" >&2
		exit 1
	}
	echo "pre-flight: tree clean"
	RUSTFLAGS="-D warnings" cargo test --workspace --all-features --quiet
	echo "pre-flight: rust tests green (-D warnings)"
	(cd bindings/py && uv sync && uv run python -m pytest -q)
	echo "pre-flight: python bindings green"
	cargo run -p codegen -- --check
	echo "pre-flight: codegen snapshot fresh"
	(cd bindings/js && bun install --frozen-lockfile && bun run build)
	node -e "
		const fs = require('fs')
		const js = fs.readFileSync('bindings/js/index.js', 'utf8')
		const m = js.match(/const \{([\s\S]*?)\} = nativeBinding/)
		const names = m[1].split(',').map((s) => s.trim().split(':')[0].trim()).filter(Boolean)
		const dups = [...new Set(names.filter((n, i) => names.indexOf(n) !== i))]
		if (dups.length) {
			console.error('duplicate exports:', dups.join(', '))
			process.exit(1)
		}
		console.log('exports:', names.length, '(unique)')
	"
	(cd bindings/js && node --test ./index.test.js)
	echo "pre-flight: js smoke green"

# Stamp <version> across every manifest, refresh the uv lockfiles, and
# verify agreement. Then: commit, `git tag v<version>`, push with tags --
# the three publish workflows fire on the tag.
#   just release 0.2.0
release version: preflight
	#!/bin/bash
	set -euo pipefail
	if ! [[ "{{version}}" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
		echo "version must be X.Y.Z, got '{{version}}'" >&2
		exit 1
	fi
	old=$(grep -m1 '^version' bindings/py/pyproject.toml | cut -d'"' -f2)
	echo "bumping $old -> {{version}}"
	cargo_files=$(git ls-files '*.toml' | grep -E '(^|/)Cargo\.toml$')
	for f in $cargo_files; do
		grep -q '^version' "$f" || continue  # workspace-only manifests
		sed -i "0,/^version = \"[^\"]*\"/s//version = \"{{version}}\"/" "$f"
	done
	sed -i "0,/^version = \"[^\"]*\"/s//version = \"{{version}}\"/" bindings/py/pyproject.toml
	sed -i "s/\"quantamental==$old\"/\"quantamental=={{version}}\"/" .github/workflows/publish-python.yml
	for f in bindings/js/package.json packages/tools/package.json; do
		sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"{{version}}\"/" "$f"
	done
	# sync member versions into the workspace lockfile (external pins untouched)
	cargo update --workspace --quiet
	(cd bindings/py && uv lock)
	(cd research/ta && uv lock)
	fail=0
	check_version() {
		f="$1"
		got="$2"
		if [ "$got" != "{{version}}" ]; then
			echo "MISMATCH $f: $got" >&2
			fail=1
		fi
	}
	for f in $cargo_files bindings/py/pyproject.toml; do
		grep -q '^version' "$f" || continue
		v=$(grep -m1 '^version' "$f" | cut -d'"' -f2)
		check_version "$f" "$v"
	done
	for f in bindings/js/package.json packages/tools/package.json; do
		v=$(grep -m1 '"version"' "$f" | cut -d'"' -f4)
		check_version "$f" "$v"
	done
	if [ "$fail" -ne 0 ]; then
		echo "aborted: manifest mismatch" >&2
		exit 1
	fi
	grep -q "\"quantamental=={{version}}\"" .github/workflows/publish-python.yml
	echo "all manifests at {{version}} -- commit, tag v{{version}}, push --tags"
