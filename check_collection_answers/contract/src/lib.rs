use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::env::random_seed;
use near_sdk::serde_json::json;
use near_sdk::{env, log, near_bindgen, AccountId, Gas, Promise, PromiseError};

// import trait from external.rs
pub mod external;
pub use crate::external::*;

// Set constants for cross contract call parameter
pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const NO_ARGS: Vec<u8> = vec![];

// Steps to check
// Their contract has a map I can store a value in
// Their contract lets me pull that stored value using the key I created
// Their contract lets me push a random value into a vector
// Their contract lets me pop and retrieve that value
// Their contract lets me print a vector

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Default)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    pub fn evaluate_check_collection_test_lookup(&mut self, contract_name: AccountId) -> Promise {
        // Lookup Map
        let random_map_key_array: Vec<u8> = random_seed();
        let random_map_value_array: Vec<u8> = random_seed();

        let random_map_key = String::from_utf8_lossy(&random_map_key_array).to_string();
        let random_map_value = String::from_utf8_lossy(&random_map_value_array).to_string();

        // Test Variables
        // let fix_key = String::from("key");
        // let fix_val = String::from("value");

        // Turn random values into arguments
        let insert_lookup_args = json!({ "key": random_map_key,"value":random_map_value })
            .to_string()
            .into_bytes();

        let get_lookup_args = json!({ "key": random_map_key }).to_string().into_bytes();

        Promise::new(contract_name.clone())
            .function_call(
                "add_to_map".to_string(),
                insert_lookup_args,
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .function_call(
                "get_from_map".to_string(),
                get_lookup_args,
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * TGAS))
                    .evaluate_lookup_callback(contract_name.clone()),
            )
    }

    #[private]
    pub fn evaluate_lookup_callback(
        #[callback_result] last_result: Result<String, PromiseError>,
        contract_name: AccountId,
    ) -> bool {
        //  The callback only has access to the last action's result
        if let Ok(result) = last_result {
            log!(format!("The last result is {:?}", result));
            true
        } else {
            log!("The batch call failed and all calls got reverted");
            false
        }
    }

    pub fn evaluate_check_collection_test_vector(&mut self, contract_name: AccountId) -> Promise {
        // Vector
        let random_vec_value_array: Vec<u8> = random_seed();

        // Serialize into Json Arguments
        let vec_arg = json!({"value":random_vec_value_array[0]})
            .to_string()
            .into_bytes();

        log!("vec val is {:?}", vec_arg);
        let no_arg = json!("").to_string().into_bytes();

        Promise::new(contract_name.clone())
            .function_call(
                "add_to_vector".to_string(),
                vec_arg,
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .function_call(
                "vector_pop_test".to_string(),
                no_arg,
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * TGAS))
                    .evaluate_vec_callback(contract_name.clone()),
            )
    }

    #[private]
    pub fn evaluate_vec_callback(
        #[callback_result] last_result: Result<u8, PromiseError>,
        contract_name: AccountId,
    ) -> bool {
        //  The callback only has access to the last action's result
        if let Ok(result) = last_result {
            log!(format!("The last result is {:?}", result));
            true
        } else {
            log!("The batch call failed and all calls got reverted");
            false
        }
    }

    pub fn evaluate_check_collection_test_multi_vector(
        &mut self,
        contract_name: AccountId,
    ) -> Promise {
        // Vector
        let random_vec_value_array: Vec<u8> = random_seed();

        // Serialize into Json Arguments
        let vec_multi_arg = json!({ "value": random_vec_value_array })
            .to_string()
            .into_bytes();

        log!("vec val is {:?}", vec_arg);
        let no_arg = json!("").to_string().into_bytes();

        Promise::new(contract_name.clone())
            .function_call(
                "vector_multi_add".to_string(),
                vec_multi_arg,
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .function_call(
                "get_full_array_test".to_string(),
                no_arg,
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * TGAS))
                    .evaluate_vec_multi_callback(contract_name.clone()),
            )
    }

    #[private]
    pub fn evaluate_vec_multi_callback(
        #[callback_result] last_result: Result<Vec<u8>, PromiseError>,
        contract_name: AccountId,
    ) {
    }
}
// #[near_bindgen]
// #[derive(BorshDeserialize, BorshSerialize)]
// pub struct Contract {
//     records: LookupMap<AccountId, bool>,
// }

// impl Default for Contract {
//     fn default() -> Self {
//         env::panic(b"The contract is not initialized.")
//     }
// }

// #[near_bindgen]
// impl Contract {
//     #[init]
//     pub fn new() -> Self {
//         // Useful snippet to copy/paste, making sure state isn't already initialized
//         assert!(env::state_read::<Self>().is_none(), "Already initialized");
//         // Note this is an implicit "return" here
//         Self {
//             records: LookupMap::new(b"r".to_vec()),
//         }
//     }

//     // Public - query external greeting
//     pub fn evaluate_check_collection_test(&mut self, contract_name: AccountId) -> Promise {
//         // // First let's get a random string from random seed
//         // starting off with gettin a byte vector based off the block height
//         let get_array_key: Vec<u8> = random_seed();
//         let get_array_value: Vec<u8> = random_seed();

//         // now lets convert that byte vector into a string
//         let random_key_string: String = String::from_utf8_lossy(&get_array_key).to_string();
//         let random_value_string: String = String::from_utf8_lossy(&get_array_value).to_string();
//         // let's print the string to check our input
//         log!("The random string key  is {:?}", random_key_string);
//         log!("The random sting value is {:?}", random_value_string);

//         //Arguments for change functions
//         let lookup_map_set_args = json!({ "key": random_key_string,"value":random_value_string })
//             .to_string()
//             .into_bytes();

//         let vector_args = json!({"value":get_array_value.first().unwrap_or(&0)})
//             .to_string()
//             .into_bytes();

//         //Arguments for view functions
//         let lookup_map_key = json!({ "key": random_key_string }).to_string().into_bytes();

//         log!("The random new value is {:?}", vector_args);

//         Promise::new(contract_name.clone())
//             .function_call(
//                 "add_to_map".to_string(),
//                 lookup_map_set_args,
//                 NO_DEPOSIT,
//                 Gas(5 * TGAS),
//             )
//             .function_call(
//                 "add_to_vector".to_string(),
//                 vector_args,
//                 NO_DEPOSIT,
//                 Gas(5 * TGAS),
//             )
//             .function_call(
//                 "get_from_map".to_string(),
//                 lookup_map_key,
//                 NO_DEPOSIT,
//                 Gas(5 * TGAS),
//             )
//             .then(
//                 Self::ext(env::current_account_id())
//                     .with_static_gas(Gas(5 * TGAS))
//                     .evaluate_check_collection_callback(contract_name.clone()),
//             )
//     }

//     #[private]
//     pub fn evaluate_check_collection_callback(
//         &mut self,
//         #[callback_result] last_result: Result<String, PromiseError>,
//         contract_name: AccountId,
//     ) {
//         // The callback only has access to the last action's result
//         if let Ok(result) = last_result {
//             log!(format!("The last result is {:?}", result));
//         } else {
//             log!("The batch call failed and all calls got reverted");
//         }
//     }

//     pub fn account_participation(&self, account_name: AccountId) -> bool {
//         self.records.get(&account_name).unwrap_or(false)
//     }
// }
