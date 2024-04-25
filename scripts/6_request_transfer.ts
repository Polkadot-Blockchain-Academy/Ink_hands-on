import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import Multisig from "../types/contracts/multisig";
import { import_env } from "./utils";
import "dotenv/config";
import "@polkadot/api-augment";

const envFile = process.env.AZERO_ENV || "dev";

async function main(): Promise<void> {
  const config = await import_env(envFile);

  const {
    ws_node,
    deployer_seed
  } = config;

  const wsProvider = new WsProvider(ws_node);
  const keyring = new Keyring({ type: "sr25519" });

  const api = await ApiPromise.create({ provider: wsProvider });
  const deployer = keyring.addFromUri(deployer_seed);

  console.log("Using", deployer.address, "as the deployer");

  const { multisig: multisigAddress} = await import(`../addresses.json`)

  const multisig = new Multisig(multisigAddress, deployer, api);

  console.log("Requesting transfer from", multisigAddress);

  await multisig.tx.requestTransfer();

  await api.disconnect();
  console.log("Done");
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
