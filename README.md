# Prefade Multi-Target Project

## Project Structure

Multi-language domain library:
- Write business logic in Rust (single source of truth)
- consume from Python & TypeScript.

```
prefade-project/
├── .vscode/
│   └── settings.json             # rust-analyzer config
│
├── crates/                       # Rust crates
│   ├── core/                     # ← Pure domain logic
│   │   ├── Cargo.toml
│   │   └── src/lib.rs            # Shared implementation
│   │
│   ├── node/                     # ← Node.js FFI bindings (napi-rs)
│   │   ├── Cargo.toml
│   │   └── src/lib.rs            # Node.js wrappers
│   │
│   └── py/                       # ← Python FFI bindings (PyO3)
│       ├── Cargo.toml
│       └── src/lib.rs            # Python wrappers
│
├── node/                         # TypeScript/Node.js package
│   ├── native/
│   │   └── index.d.ts            # Type stubs
│   ├── prefade/
│   │   ├── index.ts              # Re-exports native module
│   │   └── main.ts               # Example usage
│   ├── tsconfig.json             # TypeScript config
│   ├── package.json              # Package config
│   └── package-lock.json         # Lockfile
│
├── python/                       # Python package
│   ├── prefade/
│   │   ├── __init__.py           # Re-exports native module
│   │   ├── __init__.pyi          # Type stubs
│   │   └── py.typed              # Type marker file
│   ├── use-case1/
│   │   ├── main.py               # Example usage
│   │   └── example.ipynb         # Example notebook
│   ├── pyproject.toml            # Package config
│   └── uv.lock                   # Lockfile
│
├── target/                       # Cargo build artifacts
│   └── release/
│       ├── _prefade_native.dll   # → copied to python/prefade/
│       └── prefade_node.dll      # → copied to node/
│
├── .gitignore                    # Multi-language gitignore
├── Cargo.toml                    # Rust workspace config
└── README.md
```

# Development

## Prerequisites

- Rust toolchain (required for all builds): https://rustup.rs/
- Python 3.8+ and `uv` — only if you plan to run the Python package/notebooks: https://github.com/astral-sh/uv
- Node.js (npm) — only if you plan to run the TypeScript/Node example: https://nodejs.org/

## Environment setup (one time per clone)

- Python:
```bash
cd python
uv sync
```

- Node:
```bash
cd node
npm install
```

## Build all FFI bindings (repeat after Rust changes)

```bash
cargo run --release -p build-all
```

This builds Python/Node native binaries and copies them into their package folders. It is safe to rerun any time the Rust code changes.

## Running code (repeatable)

- Rust core:
```bash
cargo run --release -p prefade-core
```

- Python example:
```bash
cd python
uv run python use-case1/main.py
```

- Node example:
```bash
cd node
npm run build
node dist/main.js
```

# Development Environment

## VSCode

Create a `.vscode/settings.json` file with the following content:
```json
{
  // Known issue with napi macros causing false positive errors
  "rust-analyzer.diagnostics.disabled": ["macro-error"],
  // Point rust-analyzer to the right interpreter (non-standard .venv/ location)
  "rust-analyzer.cargo.extraEnv": {"PYO3_PYTHON": "${workspaceFolder}/python/.venv/Scripts/python.exe"},
  // Point jupyter to the right interpreter (non-standard .venv/ location)
  "python.defaultInterpreterPath": "${workspaceFolder}/python/.venv/Scripts/python.exe"
}
```