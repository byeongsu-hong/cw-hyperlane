# cw-hyperlane

[![codecov](https://codecov.io/gh/many-things/cw-hyperlane/branch/main/graph/badge.svg?token=SGYE7FBTAO)](https://codecov.io/gh/many-things/cw-hyperlane)

> This project is under active development...!

## Prerequisites

- rust (wasm32-wasm32-unknown target)
- go 1.20 or higher
- grcov

## How to build

```bash
cargo build

cargo wasm
```

## How to test

```bash
# testing
cargo test

# coverage
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="eddy-%p-%m.profraw"

cargo build

cargo test

grcov . -s . --binary-path ./target/debug/ -t lcov --branch --ignore-not-existing -o ./target/debug/coverage/
```

## Deploy Sequence

1. Deploy [Mailbox](./contracts/core/mailbox)

2. Deploy [Validator Announce](./contracts/core/va)

3. Deploy hooks to use with Mailbox (default hook, required hook)

   - [interchain gas paymaster (IGP)](./contracts/igps/core)

   - [IGP oracle](./contracts/igps/oracle)

   - [merkle](./contracts/hooks/merkle)

   - [pausable](./contracts/hooks/pausable)

   - [domain routing](./contracts/hooks/routing)

   - [domain routing custom](./contracts/hooks/routing-custom)

   - [domain routing fallback](./contracts/hooks/routing-fallback)

   - For testing: [mock hook](./contracts/mocks/mock-hook)

4. Deploy isms to use with Mailbox (defualt ism)

   - [multisig ism](./contracts/isms/multisig)

   - [routing ism](./contracts/isms/routing)

   - [aggregate ism](./contracts/isms/aggregate)

   - For testing: [mock ism](./contracts/mocks/mock-ism)

5. Set deployed hooks and isms to Mailbox

6. Deployment for core protocol is done! You can deploy some contracts on the top.
