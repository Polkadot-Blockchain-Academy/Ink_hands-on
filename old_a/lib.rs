#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod a {
    use ink::{
        env::{
            call::{build_call, ExecutionInput},
            set_code_hash, DefaultEnvironment, Error as InkEnvError,
        },
        prelude::{format, string::String},
        storage::{traits::ManualKey, Lazy},
    };
    use scale::{Decode, Encode};

    pub type Selector = [u8; 4];
    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InkEnvError(String),
    }

    impl From<InkEnvError> for Error {
        fn from(why: InkEnvError) -> Self {
            Self::InkEnvError(format!("{:?}", why))
        }
    }

    #[derive(Default, Debug)]
    #[ink::storage_item]
    pub struct State {
        pub x: u32,
        pub y: bool,
    }

    #[ink(storage)]
    pub struct A {
        state: Lazy<State, ManualKey<123>>,
    }

    impl A {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut state = Lazy::new();
            state.set(&State { x: 7, y: false });
            Self { state }
        }

        #[ink(message)]
        pub fn get_values(&self) -> (u32, bool) {
            let state = self.state.get_or_default();
            (state.x, state.y)
        }

        #[ink(message)]
        pub fn set_code(&mut self, code_hash: [u8; 32], callback: Option<Selector>) -> Result<()> {
            set_code_hash(&code_hash)?;

            // Optionally call a callback function in the new contract that performs the storage data migration.
            // By convention this function should be called `migrate`, it should take no arguments
            // and be call-able only by `this` contract's instance address.
            // To ensure the latter the `migrate` in the updated contract can e.g. check if it has an Admin role on self.
            //
            // `delegatecall` ensures that the target contract is called within the caller contracts context.
            if let Some(selector) = callback {
                build_call::<DefaultEnvironment>()
                    .delegate(Hash::from(code_hash))
                    .exec_input(ExecutionInput::new(ink::env::call::Selector::new(selector)))
                    .returns::<Result<()>>()
                    .invoke()?;
            }

            Ok(())
        }
    }
}
