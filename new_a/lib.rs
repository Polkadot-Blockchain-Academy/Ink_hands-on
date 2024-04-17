#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod a {

    use ink::{
        env::{get_contract_storage, Error as InkEnvError},
        prelude::{format, string::String},
        storage::{traits::ManualKey, Lazy},
    };
    use scale::{Decode, Encode};

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InkEnvError(String),
        ContractHasNoConstructor,
        MigrationFailed,
    }

    impl From<InkEnvError> for Error {
        fn from(why: InkEnvError) -> Self {
            Self::InkEnvError(format!("{:?}", why))
        }
    }

    #[derive(Default, Debug)]
    #[ink::storage_item]
    pub struct OldState {
        pub x: u32,
        pub y: bool,
    }

    #[derive(Default, Debug)]
    #[ink::storage_item]
    pub struct NewState {
        pub x: bool,
        pub y: u32,
    }

    #[ink(storage)]
    pub struct A {
        state: Lazy<NewState, ManualKey<123>>,
    }

    impl A {
        #[ink(constructor)]
        pub fn new() -> Result<Self> {
            Err(Error::ContractHasNoConstructor)
        }

        #[ink(message)]
        pub fn get_values(&self) -> (u32, bool) {
            let state = self.state.get_or_default();
            (state.y, state.x)
        }

        /// Performs a contract storage migration.
        ///
        /// NOTE: in a production code this tx should be guarded with access control
        /// You should also make sure the migration can be called only once        
        #[ink(message, selector = 0x4D475254)]
        pub fn migrate(&mut self) -> Result<()> {
            if let Some(OldState { x, y }) = get_contract_storage(&123)? {
                // swap fields
                self.state.set(&NewState { x: !y, y: x + 1 });
                return Ok(());
            }

            Err(Error::MigrationFailed)
        }
    }
}
