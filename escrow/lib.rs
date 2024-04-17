#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod escrow {

    use scale::{Decode, Encode};

    type TokenIdentifier = u32;

    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum EscrowError {
        /// not the exact price was transferred
        WrongAmountTransfered,
    }

    #[derive(Debug, Encode, Decode, Clone, Copy, PartialEq, Eq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct PrettyKittyNft {
        id: TokenIdentifier,
        owner: AccountId,
    }

    #[ink(storage)]
    pub struct Escrow {
        seller: AccountId,
        nft: PrettyKittyNft,
        price: Balance,
    }

    impl Escrow {
        #[ink(constructor)]
        pub fn new(nft: TokenIdentifier, price: Balance) -> Self {
            let seller = Self::env().caller();
            Self {
                seller,
                price,
                // assumes the seller holds the rights to the Nft with this id
                nft: PrettyKittyNft {
                    id: nft,
                    owner: seller,
                },
            }
        }

        #[ink(message, payable)]
        pub fn deposit_funds(&mut self) -> Result<(), EscrowError> {
            if self.price != self.env().transferred_value() {
                return Err(EscrowError::WrongAmountTransfered);
            }
            self.nft.owner = self.env().caller();
            Ok(())
        }

        #[ink(message)]
        pub fn get_nft(&self) -> PrettyKittyNft {
            self.nft
        }
    }
}
