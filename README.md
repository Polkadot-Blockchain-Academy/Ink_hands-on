[![Built with ink!](https://raw.githubusercontent.com/paritytech/ink/master/.images/badge_flat.svg)](https://github.com/paritytech/ink)

# Ink! hands-on exercises

<sup>_Companion to the PBA ink! Smart Contrcts smodule_</sup>

## Prerequisites

1. Have docker and docker-compose (`1.29.2`)
2. Have `nightly-2024-02-16` Rust toolchain installed:

```bash
rustup toolchain install nightly-2024-02-16
rustup component add rust-src --toolchain nightly-2024-02-16-x86_64-unknown-linux-gnu
```
3. Have cargo-contract (`3.2.0`) installed:

```bash
cargo +nightly-2024-02-16 install cargo-contract --vers 3.2.0 --force
```

Install [polkadot{.js}](https://polkadot.js.org/extension/) extension.

Add these two accounts:

1. Alice (bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice)
2. Bob (bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob)

## Running a node locally

```bash
# You might be asked to login into docker with `docker login`.
make devnode
```

## Contracts

To compile all contracts:

```bash
make compile
```

To deploy e.g. `escrow` contract:

```bash
make deploy-escrow
```

Deployed contracts write their addresses to the `addresses.json` file in the root directory.
You can copy the address and use it in the [contracts-ui explorer](https://contracts-ui.substrate.io/address-lookup).
Contract metadata can be found in a corresponding `target` directory, e.g. [escrow/target/ink/escrow.contract](escrow/target/ink/escrow.contract).

### Testnet

To deploy a given command run on the Azero Testnet preceed it with the name of the environment:

```bash
AZERO_ENV=testnet make <command>
```

### Testnet faucet

[https://faucet.test.azero.dev/](https://faucet.test.azero.dev/)
