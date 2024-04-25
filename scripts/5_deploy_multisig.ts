import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import Token from "../types/contracts/token";
import TokenConstructors from "../types/constructors/token";
import Multisig from "../types/contracts/multisig";
import MultisigConstructors from "../types/constructors/multisig";
import {
  uploadCode,
  estimateContractInit,
  import_env,
  storeAddresses,
  Addresses,
  parseUnits
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
    signer0,
    signer1
  } = config;

  const wsProvider = new WsProvider(ws_node);
  const keyring = new Keyring({ type: "sr25519" });

  const api = await ApiPromise.create({ provider: wsProvider });
  const deployer = keyring.addFromUri(deployer_seed);
  console.log("Using", deployer.address, "as the deployer");

  let estimatedGasMultisigConstructor = await estimateContractInit(
    api,
    deployer,
    "multisig.contract",
    [signer0, signer1]
  );

  const multisigConstructors = new MultisigConstructors(api, deployer);

  const { address: multisigAddress } = await multisigConstructors.new(
    signer0,
    signer1,
    { gasLimit: estimatedGasMultisigConstructor },
  );

  let estimatedGasTokenConstructor = await estimateContractInit(
    api,
    deployer,
    "token.contract",
    [ "PWNED", "PWN", multisigAddress],
  );

  const tokenConstructors = new TokenConstructors(api, deployer);

  const { address: tokenAddress } = await tokenConstructors.new(
    "PWNED",
    "PWN",
     multisigAddress,
    { gasLimit: estimatedGasTokenConstructor },
  );

  const multisig = new Multisig(multisigAddress, deployer, api);

  await multisig.tx.initialize(
    tokenAddress
  );

  const address: Addresses = {
    token: tokenAddress,
    multisig: multisigAddress
  };
  console.log("addresses:", address);

  storeAddresses(address);

  await api.disconnect();
  console.log("Done");
}

main().catch((error) => {
  console.error(error);;
  process.exit(1);
});
