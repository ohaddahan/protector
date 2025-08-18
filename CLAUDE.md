# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Solana blockchain program built with the Anchor framework. The project structure follows the standard Anchor workspace layout:

- **Rust Program**: Located in `programs/protector/src/lib.rs` - contains the on-chain smart contract logic
- **TypeScript Tests**: Located in `tests/protector.ts` - integration tests for the blockchain program
- **Configuration**: `Anchor.toml` defines the program deployment settings and test configuration

## Common Commands

### Building and Testing
```bash
# Build the Rust program
anchor build

# Run tests (uses ts-mocha with 1000s timeout)
anchor test

# Alternative test command
yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts
```

### Code Quality
```bash
# Format code with prettier
yarn run lint:fix

# Check formatting
yarn run lint
```

### Development Setup
```bash
# Install dependencies
yarn install

# Start local Solana test validator (if needed)
solana-test-validator
```

## Architecture

### Anchor Framework Structure
- The project uses Anchor framework version 0.31.1 for Solana development
- Program ID: `4Lh7VjsEgG9XnShnq1CAsH37sWevuiWPfzqjWKo2s4DK`
- Currently contains a minimal `initialize` instruction

### Key Files
- `programs/protector/src/lib.rs` - Main program logic
- `tests/protector.ts` - Integration tests using Mocha/Chai
- `Anchor.toml` - Anchor workspace configuration
- `Cargo.toml` - Rust workspace configuration with release optimizations

### Testing Environment
- Tests use the local cluster by default
- Mocha/Chai framework for TypeScript tests
- Extended timeout (1000s) for blockchain operations
- Anchor workspace automatically generates TypeScript types

## Development Notes

- The project uses Yarn as the package manager (specified in Anchor.toml)
- Rust release profile is optimized with LTO and single codegen unit
- TypeScript configuration targets ES6 with CommonJS modules
- Prettier is configured for code formatting