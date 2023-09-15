#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Mintable)]
#[openbrush::contract]
pub mod example_psp34 {
    use logion_contract::impls::logion::*;
    use logion_contract::impls::types as logion;
    use openbrush::traits::Storage;
    use openbrush::traits::String;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct ExamplePsp34 {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        logion: logion::Data,
    }

    impl Logion for ExamplePsp34 {}

    impl ExamplePsp34 {
        #[ink(constructor)]
        pub fn new(nonce: String, collection_loc_id: u128, cert_host: String) -> Self {
            let mut instance = Self::default();
            instance.logion.init(nonce, collection_loc_id, cert_host);
            instance
        }
    }
}