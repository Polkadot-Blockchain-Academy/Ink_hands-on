use std::str::FromStr;

use aleph_client::{AccountId, Pair};
use subxt::ext::sp_core::ed25519;

#[derive(Debug, clap::Parser)]
pub struct Config {
    #[arg(long)]
    pub contract_address: String,

    #[arg(long, default_value = "../artifacts/multisig.json")]
    pub contract_metadata: String,

    #[arg(long, default_value = "ws://127.0.0.1:9944")]
    pub node_url: String,

    /// relay tx signer seed
    #[arg(long, default_value = "//Alice")]
    pub tx_relayer_seed: String,

    /// owner1 signing key seeds    
    #[arg(long)]
    pub signer0: Signer,

    /// owner2 signing key seeds    
    #[arg(long)]
    pub signer1: Signer,
}

#[derive(Debug, Clone)]
pub struct Signer {
    pub seed: String,
    pub account_id: AccountId,
}

impl FromStr for Signer {
    type Err = String;

    fn from_str(seed: &str) -> Result<Self, Self::Err> {
        let keypair = ed25519::Pair::from_phrase(seed, None).map_err(|_| "not a seed phrase")?;
        let public_key = keypair.0.public().0;

        Ok(Self {
            seed: seed.to_owned(),
            account_id: AccountId::from(public_key),
        })
    }
}
