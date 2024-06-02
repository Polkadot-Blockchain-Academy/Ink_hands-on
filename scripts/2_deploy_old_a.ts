import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import OldAConstructors from "../types/constructors/old_a";
import {
  uploadCode,
  estimateContractInit,
  import_env,
  storeAddresses,
  Addresses
} from "./utils";
import "dotenv/config";
import "@polkadot/api-augment";
import type BN from "bn.js";

const envFile = process.env.AZERO_ENV || "dev";

async function main(): Promise<void> {
  const config = await import_env(envFile);

  const {
    ws_node,
    deployer_seed,
    min_gas_price,
    max_gas_price,
    default_gas_price,
  } = config;

  const wsProvider = new WsProvider(ws_node);
  const keyring = new Keyring({ type: "sr25519" });

  const api = await ApiPromise.create({ provider: wsProvider });
  const deployer = keyring.addFromUri(deployer_seed);
  console.log("Using", deployer.address, "as the deployer");

  const oldAConstructors = new OldAConstructors(api, deployer);

  let estimatedGasOldA = await estimateContractInit(
    api,
    deployer,
    "old_a.contract",
    [],
  );

  const { address: old_a_address } = await oldAConstructors.new(
    { gasLimit: estimatedGasOldA },
  );

  const address: Addresses = {
    old_a: old_a_address,
  };

  console.log("addresses:", address);

  storeAddresses(address);

  await api.disconnect();
  console.log("Done");
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
