# 2025 Anchor Template

A modern Anchor development boilerplate for Solana programs, incorporating best practices for structure, and documentation as of 2025.

## Requirements

- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/) `v18+`
- [Yarn](https://yarnpkg.com/) `v4+`
- [Solana CLI](https://docs.solana.com/cli/install)
- [Anchor CLI](https://book.anchor-lang.com/cli/install.html) `v0.29+`

## Installation & Setup

Install dependencies using Yarn
```bash
yarn install
```

Generate a new Solana keypair
```bash
yarn keygen
```

Start a local test validator (optional)
```bash
yarn validate
```

Build the Anchor program
```bash
yarn build
```

Deploy to local validator
```
yarn deploy:local
```

Or deploy to devnet or testnet
```bash
yarn deploy:dev
yarn deploy:test
```

