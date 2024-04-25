use std::sync::Arc;

use aleph_client::{
    contract::event::{translate_events, BlockDetails, ContractEvent},
    sp_core::{crypto::SecretStringError, ByteArray},
    AlephConfig, Pair,
};
use clap::Parser;
use config::Config;
use connection::SignedWsConnection;
use contracts::ContractError;
use futures::StreamExt;
use log::{debug, error, info, trace};
use subxt::{events::Events, ext::sp_core::ed25519, utils::H256};
use thiserror::Error;
use tiny_keccak::{Hasher, Keccak};

use crate::{
    connection::signed_connection,
    contracts::{decode_request_transfer_event, MultisigInstance},
};

mod config;
mod connection;
mod contracts;
#[cfg(test)]
mod tests;

pub type Signature = [u8; 64];

#[derive(Debug, Error)]
#[error(transparent)]
#[non_exhaustive]
pub enum SignerError {
    #[error("Contract error")]
    Contract(#[from] ContractError),

    #[error("Subxt error")]
    Subxt(#[from] subxt::Error),

    #[error("Signing key error")]
    SecretStringError(#[from] SecretStringError),

    #[error("no such field in event data")]
    NoEventField(String),
}

#[tokio::main]
async fn main() -> Result<(), SignerError> {
    let config = Config::parse();

    env_logger::init();

    info!("{:#?}", &config);

    let connection = &connection::init(&config.node_url).await;

    let signed_connection = Arc::new(signed_connection(
        connection,
        &aleph_client::keypair_from_string(&config.tx_relayer_seed),
    ));

    let mut subscription = connection
        .as_client()
        .blocks()
        .subscribe_finalized()
        .await?;

    info!("Subscribing to new events");

    let multisig = MultisigInstance::new(&config.contract_address, &config.contract_metadata)?;
    let config = Arc::new(config);

    let multisig = Arc::new(multisig);
    while let Some(Ok(block)) = subscription.next().await {
        let block_number = block.number();

        let events = block.events().await?;

        handle_events(
            config.clone(),
            multisig.clone(),
            events,
            block_number,
            block.hash(),
            signed_connection.clone(),
        )
        .await?;
    }

    Ok(())
}

async fn handle_events(
    config: Arc<Config>,
    multisig: Arc<MultisigInstance>,
    events: Events<AlephConfig>,
    block_number: u32,
    block_hash: H256,
    signed_connection: Arc<SignedWsConnection>,
) -> Result<(), SignerError> {
    for event in translate_events(
        events.iter(),
        &[&multisig.contract],
        Some(BlockDetails {
            block_number,
            block_hash,
        }),
    ) {
        match event {
            Ok(event) => {
                handle_event(&config, event, multisig.clone(), signed_connection.clone()).await?
            }
            Err(why) => {
                debug!("Failed to translate event: {why:?}")
            }
        };
    }

    Ok(())
}

async fn handle_event(
    config: &Config,
    event: ContractEvent,
    multisig: Arc<MultisigInstance>,
    signed_connection: Arc<SignedWsConnection>,
) -> Result<(), SignerError> {
    let Config {
        signer0, signer1, ..
    } = config;

    if let Some(name) = &event.name {
        if name.eq("TransferRequest") {
            info!("Handling contract event: {event:?}");

            let data = event.data;
            let decoded_event = decode_request_transfer_event(&data)?;

            info!("Event decoding: {decoded_event:?}",);

            let bytes = concat_u8_arrays(vec![
                decoded_event.receiver.as_slice(),
                &decoded_event.amount.to_le_bytes(),
            ]);

            trace!("Concatenated event bytes: {bytes:?}");

            let hashed_bytes = keccak256(bytes.clone());
            debug!("Hashed event data: {hashed_bytes:?}");

            info!(
                "hex encoding of hashed event data: 0x{}",
                hex::encode(hashed_bytes)
            );

            let keypair0 = ed25519::Pair::from_phrase(&signer0.seed, None)?;
            let signature0 = keypair0.0.sign(&hashed_bytes).0;

            let keypair1 = ed25519::Pair::from_phrase(&signer1.seed, None)?;
            let signature1 = keypair1.0.sign(&hashed_bytes).0;

            multisig
                .transfer(
                    &signed_connection,
                    decoded_event.receiver,
                    decoded_event.amount,
                    signature0,
                    signature1,
                )
                .await?;

            info!("Success");
        }
    }

    Ok(())
}

pub fn concat_u8_arrays(arrays: Vec<&[u8]>) -> Vec<u8> {
    let mut result = Vec::new();
    for array in arrays {
        result.extend_from_slice(array);
    }
    result
}

pub fn keccak256<T: AsRef<[u8]>>(bytes: T) -> [u8; 32] {
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(bytes.as_ref());
    hasher.finalize(&mut output);
    output
}
