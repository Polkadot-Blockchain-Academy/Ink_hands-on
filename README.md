[![Built with ink!](https://raw.githubusercontent.com/paritytech/ink/master/.images/badge_flat.svg)](https://github.com/paritytech/ink)

# Ink! hands-on exercises

<sup>_Companion to the PBA ink! Smart Contrcts smodule_</sup>

## Prerequisites

1. Have docker and docker-compose (`1.29.2`)
2. Have cargo-contract (`3.2.0`) installed:

```bash
cargo install cargo-contract --vers 3.2.0
```

Install [polkadot{.js}](https://polkadot.js.org/extension/) extension.

Add these two accounts:

1. Alice (bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice)
2. Bob (bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob)

## Running a node locally

```bash
make devnode
```

## Escrow Smart Contract

To compile:

```bash
make compile
```

To deploy e.g. `escrow` contract:

```bash
deploy escrow
```

Deployed contracts write their addresses to the `addresses.json` file in the root directory. You can copy the address and use it in the [https://contracts-ui.substrate.io/contract](https://contracts-ui.substrate.io/contract)

### Testnet

To deploy a given command run on the Azero Testnet preceed it with the name of the environment:

```bash
AZERO_ENV=testnet make <command>
```

## Upgradeable contracts

1. Deploy the `old_a` contract:
```bash
AZERO_ENV=testnet make deploy-old-a
```
2. Import and query it using the [https://ui.use.ink/add-contract](https://ui.use.ink/add-contract) UI dashboard.
3. Upgrade the contract:
```bash
AZERO_ENV=testnet make upgrade-a
```
4. Query the state again
5. Take a look at the `set_code` call in the old contract and try to understand it.
 - what is the raw cross-contract call doing?
 - what happens if it fails?
 - why is it important to use the `ManualKey` in contracts storage here?
 - Could you achieve a similar upgrade without using `Lazy`?


### Testnet faucet

[https://faucet.test.azero.dev/](https://faucet.test.azero.dev/)

### Cargo contract calls

```bash
cargo contract call --url wss://ws.test.azero.dev --contract <address> --message get_values --suri "//Alice" --skip-confirm
```
