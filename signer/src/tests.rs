use std::str::FromStr;

use aleph_client::{sp_core::ByteArray, AccountId, Pair};
use ed25519_dalek::{Signature as ed25519Signature, Verifier, VerifyingKey};
use subxt::ext::sp_core::ed25519;

use crate::{concat_u8_arrays, contracts::TransferRequestEvent, keccak256};

#[test]
fn it_works() {
    let signer_seed = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    // let signer_seed = "shadow goat lucky kind swing drama artefact lend junk tell fortune another";

    let event = TransferRequestEvent {
        receiver: AccountId::from_str("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
            .expect("str is account id"),
        amount: 1000000000000,
    };

    let bytes = concat_u8_arrays(vec![event.receiver.as_slice(), &event.amount.to_le_bytes()]);
    let hashed_bytes = keccak256(bytes.clone());

    let keypair = ed25519::Pair::from_phrase(signer_seed, None).expect("keypair");
    let signature = keypair.0.sign(&hashed_bytes).0;
    let public_key = keypair.0.public().0;

    // subkey inspect "bottom drive obey lake curtain smoke basket hold race lonely fit walk" --scheme=ed25519
    println!("pubkey ss58 {}", AccountId::from(public_key));

    verify(&signature, &hashed_bytes, &public_key);
    // assert!(false);
}

pub fn verify(signature: &[u8; 64], message: &[u8], pubkey: &[u8; 32]) {
    let sig = ed25519Signature::from(signature);
    let vkey = VerifyingKey::from_bytes(pubkey).expect("not a pubkey");

    let status = vkey.verify(message, &sig);
    println!("{status:?}");

    assert!(status.is_ok())
}
