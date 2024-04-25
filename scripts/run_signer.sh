#!/bin/bash

# set -x
set -eo pipefail

# --- GLOBAL CONSTANTS

ADDRESSES_FILE=$(pwd)/addresses.json

# --- FUNCTIONS

function get_address {
  local addresses_file=$1
  local contract_name=$2
  cat $addresses_file | jq --raw-output ".$contract_name"
}

# --- ARGS

NODE_URL=${NODE_URL:-"ws://127.0.0.1:9944"}
SIGNER0_SEED=${SIGNER0_SEED:-"bottom drive obey lake curtain smoke basket hold race lonely fit walk"}
SIGNER1_SEED=${SIGNER1_SEED:-"shadow goat lucky kind swing drama artefact lend junk tell fortune another"}

ARGS=(
  run -- --contract-address=$(get_address $ADDRESSES_FILE multisig) \
  --node-url=${NODE_URL} \
  --signer0="${SIGNER0_SEED}" \
  --signer1="${SIGNER1_SEED}" \
)

# --- RUN

export RUST_LOG=info,aleph-client=warn

cd signer

cargo "${ARGS[@]}"
