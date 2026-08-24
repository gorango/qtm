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

# Fast compile check (no tests).
check:
    cargo check --workspace
