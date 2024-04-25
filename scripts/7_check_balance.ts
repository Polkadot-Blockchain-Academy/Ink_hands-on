import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import Token from "../types/contracts/token";
import TokenConstructors from "../types/constructors/token";
import { import_env } from "./utils";
import "dotenv/config";
import "@polkadot/api-augment";
import type BN from "bn.js";
const assert = require( "assert");

const envFile = process.env.AZERO_ENV || "dev";

async function main(): Promise<void> {
  const config = await import_env(envFile);

  const {
    ws_node,
    deployer_seed,
  } = config;

  const wsProvider = new WsProvider(ws_node);
  const keyring = new Keyring({ type: "sr25519" });

  const api = await ApiPromise.create({ provider: wsProvider });
  const deployer = keyring.addFromUri(deployer_seed);
  console.log("Using", deployer.address, "as the deployer");

  const { token: tokenAddress, multisig: multisigAddress} = await import(`../addresses.json`)

  const token = new Token(tokenAddress, deployer, api);

  let result = await token.query.balanceOf(
   multisigAddress
  );

  let balance = result.value.ok.rawNumber;
  console.log("Balance", balance);

  assert(balance.isZero(), "balance is not zero")

  await api.disconnect();
  console.log("Done");
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
