#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod assignment {

    use ed25519_dalek::{Signature as ed25519Signature, Verifier, VerifyingKey as PublicKey};
    use ink::{
        env::{
            hash::{HashOutput, Keccak256},
            hash_bytes, Error as InkEnvError,
        },
        prelude::{format, string::String, vec},
        storage::Mapping,
    };
    use psp22::{PSP22Error, PSP22};
    use scale::{Decode, Encode};

    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum MultisigError {
        InkEnvError(String),
        Ed25519Error(String),
        PSP22(PSP22Error),
        MalformedKey,
        TransferTokenNotSet,
        AlreadyInitialized,
        AlreadyRequested,
    }

    impl From<InkEnvError> for MultisigError {
        fn from(why: InkEnvError) -> Self {
            Self::InkEnvError(format!("{:?}", why))
        }
    }

    impl From<ed25519_dalek::ed25519::Error> for MultisigError {
        fn from(why: ed25519_dalek::ed25519::Error) -> Self {
            Self::Ed25519Error(format!("{:?}", why))
        }
    }

    impl From<PSP22Error> for MultisigError {
        fn from(inner: PSP22Error) -> Self {
            MultisigError::PSP22(inner)
        }
    }

    #[ink(event)]
    #[derive(Debug)]
    #[cfg_attr(feature = "std", derive(Eq, PartialEq))]
    pub struct TransferRequest {
        pub receiver: AccountId,
        pub amount: Balance,
    }

    #[ink(event)]
    #[derive(Debug)]
    #[cfg_attr(feature = "std", derive(Eq, PartialEq))]
    pub struct Transfer {
        pub token: AccountId,
        pub amount: Balance,
        pub receiver: AccountId,
        pub signature0: Signature,
        pub signature1: Signature,
    }

    pub type Keccak256HashOutput = <Keccak256 as HashOutput>::Type;
    pub type Signature = [u8; 64];

    #[ink(storage)]
    pub struct Multisig {
        signer0: AccountId,
        signer1: AccountId,
        token: Option<AccountId>,
        requests: Mapping<AccountId, ()>,
    }

    impl Multisig {
        #[ink(constructor)]
        pub fn new(signer0: AccountId, signer1: AccountId) -> Result<Self, MultisigError> {
            Ok(Self {
                signer0,
                signer1,
                token: None,
                requests: Mapping::new(),
            })
        }

        #[ink(message)]
        pub fn request_transfer(&mut self) -> Result<(), MultisigError> {
            let caller = self.env().caller();
            if self.requests.contains(caller) {
                return Err(MultisigError::AlreadyRequested);
            }

            self.requests.insert(caller, &());
            self.env().emit_event(TransferRequest {
                receiver: caller,
                amount: 1000000000000,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn transfer(
            &mut self,
            receiver: AccountId,
            amount: Balance,
            signature0: Signature,
            signature1: Signature,
        ) -> Result<(), MultisigError> {
            match self.token {
                Some(token_address) => {
                    let message = hash_data(receiver, amount);

                    Self::verify(&signature0, &message, self.signer0.as_ref())?;
                    Self::verify(&signature1, &message, self.signer1.as_ref())?;

                    let mut token: ink::contract_ref!(PSP22) = token_address.into();
                    token.transfer(receiver, amount, vec![])?;

                    self.env().emit_event(Transfer {
                        token: token_address,
                        receiver,
                        amount,
                        signature0,
                        signature1,
                    });

                    Ok(())
                }
                None => Err(MultisigError::TransferTokenNotSet),
            }
        }

        #[ink(message)]
        pub fn initialize(&mut self, token: AccountId) -> Result<(), MultisigError> {
            if self.token.is_some() {
                return Err(MultisigError::AlreadyInitialized);
            }
            self.token = Some(token);
            Ok(())
        }

        #[ink(message)]
        pub fn get_token(&self) -> Option<AccountId> {
            self.token
        }

        #[ink(message)]
        pub fn get_signers(&self) -> [AccountId; 2] {
            [self.signer0, self.signer1]
        }

        pub fn verify(
            signature: &[u8; 64],
            message: &[u8],
            pubkey: &[u8; 32],
        ) -> Result<(), MultisigError> {
            match (
                ed25519Signature::from(signature),
                PublicKey::from_bytes(pubkey),
            ) {
                (s, Ok(k)) => Ok(k.verify(message, &s)?),
                (_, Err(_why)) => Err(MultisigError::MalformedKey)?,
            }
        }
    }

    pub fn keccak256(data: &[u8]) -> Keccak256HashOutput {
        let mut output = Keccak256HashOutput::default();
        hash_bytes::<Keccak256>(data, &mut output);
        output
    }

    pub fn hash_data(receiver: AccountId, amount: Balance) -> Keccak256HashOutput {
        let data = [AsRef::<[u8]>::as_ref(&receiver), &amount.to_le_bytes()].concat();
        keccak256(&data)
    }
}
