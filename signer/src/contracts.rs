use std::{collections::HashMap, str::FromStr};

use aleph_client::{
    contract::ContractInstance,
    contract_transcode::{ContractMessageTranscoder, Value},
    pallets::contract::ContractsUserApi,
    sp_weights::weight_v2::Weight,
    AccountId, TxInfo, TxStatus,
};
use log::info;
use thiserror::Error;

use crate::{connection::SignedWsConnection, Signature};

#[derive(Debug, Error)]
#[error(transparent)]
#[non_exhaustive]
pub enum ContractError {
    #[error("aleph-client error")]
    AlephClient(#[from] anyhow::Error),

    #[error("not account id")]
    NotAccountId(String),

    #[error("cannot decode data field")]
    MissingOrInvalidField(String),

    #[error("Transfer tx has failed")]
    TransferTxFailure,
}

#[derive(Debug)]
pub struct TransferRequestEvent {
    pub receiver: AccountId,
    pub amount: u128,
}

pub struct MultisigInstance {
    pub contract: ContractInstance,
    pub transcoder: ContractMessageTranscoder,
    pub address: AccountId,
}

impl MultisigInstance {
    pub fn new(address: &str, metadata_path: &str) -> Result<Self, ContractError> {
        let address = AccountId::from_str(address)
            .map_err(|why| ContractError::NotAccountId(why.to_string()))?;
        Ok(Self {
            address: address.clone(),
            transcoder: ContractMessageTranscoder::load(metadata_path)?,
            contract: ContractInstance::new(address, metadata_path)?,
        })
    }

    pub async fn transfer(
        &self,
        signed_connection: &SignedWsConnection,
        receiver: AccountId,
        amount: u128,
        signature0: Signature,
        signature1: Signature,
    ) -> Result<TxInfo, ContractError> {
        let args = [
            receiver.to_string(),
            amount.to_string(),
            bytes64_to_string(&signature0),
            bytes64_to_string(&signature1),
        ];

        info!("Sending transfer tx: {args:?}");

        let call_data = self.transcoder.encode("transfer", args)?;

        let gas_limit = Weight {
            ref_time: 100000000000,
            proof_size: 10000000,
        };

        signed_connection
            .call(
                self.address.clone(),
                0,
                gas_limit,
                None,
                call_data,
                TxStatus::Finalized,
            )
            .await
            .map_err(|_garbage| ContractError::TransferTxFailure)
    }
}

fn bytes64_to_string(data: &[u8; 64]) -> String {
    "0x".to_owned() + &hex::encode(data)
}

fn decode_uint_field(data: &HashMap<String, Value>, field: &str) -> Result<u128, ContractError> {
    if let Some(Value::UInt(x)) = data.get(field) {
        Ok(*x)
    } else {
        Err(ContractError::MissingOrInvalidField(field.into()))
    }
}

fn decode_account_id_field(
    data: &HashMap<String, Value>,
    field: &str,
) -> Result<AccountId, ContractError> {
    if let Some(Value::Literal(s)) = data.get(field) {
        Ok(AccountId::from_str(s)
            .map_err(|why| ContractError::MissingOrInvalidField(why.to_owned()))?)
    } else {
        Err(ContractError::MissingOrInvalidField(field.into()))
    }
}

// fn decode_hex_field(data: &HashMap<String, Value>, field: &str) -> Result<[u8; 32], ContractError> {
//     match data.get(field) {
//         Some(Value::Hex(hex)) => {
//             let mut result = [0u8; 32];
//             result.copy_from_slice(hex.bytes());
//             Ok(result)
//         }
//         _ => Err(ContractError::MissingOrInvalidField(field.into())),
//     }
// }

pub fn decode_request_transfer_event(
    data: &HashMap<String, Value>,
) -> Result<TransferRequestEvent, ContractError> {
    Ok(TransferRequestEvent {
        receiver: decode_account_id_field(data, "receiver")?,
        amount: decode_uint_field(data, "amount")?,
    })
}
