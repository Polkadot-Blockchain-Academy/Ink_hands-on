import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import Escrow from "../types/contracts/escrow";
import EscrowConstructors from "../types/constructors/escrow";
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

// const Alice = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
const price = 1000000000000; // # 1e12 pAZERo, 1 azero

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

  const escrowCodeHash = await uploadCode(
    api,
    deployer,
    "escrow.contract",
  );
  console.log("escrow code hash:", escrowCodeHash);

  const escrowConstructors = new EscrowConstructors(api, deployer);

  let estimatedGasEscrow = await estimateContractInit(
    api,
    deployer,
    "escrow.contract",
    [1, price],
  );

  const { address: escrowAddress } = await escrowConstructors.new(
    1,
    price,
    { gasLimit: estimatedGasEscrow },
  );

  const address: Addresses = {
    escrow: escrowAddress,
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
