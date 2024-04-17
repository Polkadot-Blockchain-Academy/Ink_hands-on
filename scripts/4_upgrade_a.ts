import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import OldA from "../types/contracts/old_a";
import { import_env, hexToBytes } from "./utils";
import "dotenv/config";
import "@polkadot/api-augment";
import { ethers } from "ethers";
import { KeyringPair } from "@polkadot/keyring/types";
import type BN from "bn.js";

const envFile = process.env.AZERO_ENV || "dev";

async function main(): Promise<void> {
  const config = await import_env(envFile);

  const { ws_node, deployer_seed, dev } = config;

  const wsProvider = new WsProvider(ws_node);
  const keyring = new Keyring({ type: "sr25519" });

  const api = await ApiPromise.create({ provider: wsProvider });
  const deployer = keyring.addFromUri(deployer_seed);

  console.log("Using ", deployer.address, "as the transaction signer");

  const { old_a: old_a_address } = await import(`../addresses.json`)
  const { new_a: new_a_code_hash } = await import(`../code_hashes.json`)

  const old_a = new OldA(old_a_address, deployer, api);
  let old_state = await old_a.query.getValues ();
  console.log ("Querying state before the upgrade:", old_state.value);

  console.log("Upgrading", old_a_address, "to", new_a_code_hash);

  await old_a.tx.setCode(hexToBytes ("0x79a6daeb605459e3ad1b54e1938a02058c2b90ecc5a84249c36b899b77017fe8"), hexToBytes ("0x4D475254"));

  let new_state = await old_a.query.getValues ();
  console.log ("Querying state after the upgrade:", new_state.value);

  await api.disconnect();
  console.log("Done");
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
