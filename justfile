set shell := ["bash", "-cu"]

default: test

# Create the Python dev environment (first-time setup).
setup:
    python3 -m venv bindings/py/.venv
    bindings/py/.venv/bin/pip install maturin pytest numpy

# Build everything: Rust core + js (napi) + py (maturin) bindings.
build:
    cargo build --workspace --release
    cd bindings/js && bun run build
    cd bindings/py && ./.venv/bin/maturin build --features pyo3/abi3-py39

# Run all tests: Rust workspace, Python, Node.
test:
    cargo test --workspace
    cd bindings/py && ./.venv/bin/python -m pytest -q
    cd bindings/js && node --test ./index.test.js

# Lint the Rust code.
lint:
    cargo clippy --workspace

# Fast compile check (no tests).
check:
    cargo check --workspace
