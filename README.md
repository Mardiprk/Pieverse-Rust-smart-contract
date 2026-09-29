# Rust / Anchor (Solana) Skill Test — Starter Codebase

This is the starter project for the Solana smart contract skill test.

It's a small **Vault program**: users can initialize a personal vault (a PDA account),
deposit SOL into it, and withdraw SOL from it.

The program **compiles**, but it contains a number of real bugs. Your job (Part 1)
is to find and fix them. Part 2 asks you to extend the program with new features.
See the full task description (shared separately) for details.

## Project structure

```
/
├── Anchor.toml
├── Cargo.toml
├── package.json
├── programs/
│   └── vault/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs        # the program — this is what you'll be editing
└── tests/
    └── vault.ts               # starter test file (incomplete — extend it)
```

## Prerequisites

- Rust (stable) — https://rustup.rs
- Solana CLI — https://docs.solanalabs.com/cli/install
- Anchor CLI (0.30.x) — https://www.anchor-lang.com/docs/installation
- Node.js 18+ and Yarn or npm (for running tests)

## Getting started

```bash
# install JS deps for the test suite
npm install

# build the program
anchor build

# run a local validator + run the tests
anchor test
```

If your Anchor/Solana CLI versions differ from what's pinned in `Anchor.toml` /
`Cargo.toml`, feel free to adjust — just note the change in your `NOTES.md`.
