// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::{log, near_bindgen, BorshStorageKey};

// The use of enums ensure consistent storage keys,
//reducing bugs from incorrect/inconsistent keys.
#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    LookupMapTest,
    VectorTest,
}

// Define the contract structure, including collections
// and their data types.

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub test_lookup_map: LookupMap<String, String>,
    pub test_vector: Vector<u8>,
}

// Define the default, initializing the contract and its collections automatically.

impl Default for Contract {
    fn default() -> Self {
        Self {
            test_lookup_map: LookupMap::new(StorageKey::LookupMapTest),
            test_vector: Vector::new(StorageKey::VectorTest),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_from_map(&self, key: String) -> String {
        self.test_lookup_map
            .get(&key)
            .unwrap_or(String::from("No Value Found"))
    }

    pub fn get_full_array_test(&self) -> Vec<u8> {
        self.test_vector.to_vec()
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn add_to_map(&mut self, key: String, value: String) {
        self.test_lookup_map.insert(&key, &value);
    }

    pub fn add_to_vector(&mut self, value: u8) {
        self.test_vector.push(&value)
    }

    pub fn vector_pop_test(&mut self) -> u8 {
        self.test_vector.pop().unwrap_or(0)
    }

    pub fn vector_multi_add(&mut self, vec_to_add: Vec<u8>) {
        self.test_vector.clear();
        self.test_vector.extend(vec_to_add);
    }
}
