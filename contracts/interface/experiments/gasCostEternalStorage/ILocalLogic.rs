use gstd::{msg, prelude::*};
use hashbrown::HashMap;

#[derive(Default)]
pub struct LocalLogic {
    uint256_storage: HashMap<String, u128>,
    string_storage: HashMap<String, String>,
}

impl LocalLogic {
    // Operaciones sobre uint256
    pub fn local_set_uint256(&mut self, key: String, value: u128) {
        self.uint256_storage.insert(key, value);
    }

    pub fn local_get_uint256(&self, key: String) -> Option<u128> {
        self.uint256_storage.get(&key).cloned()
    }

    pub fn local_delete_uint256(&mut self, key: String) {
        self.uint256_storage.remove(&key);
    }

    pub fn local_add_uint256(&mut self, key: String, add: u128) {
        let current = self.uint256_storage.get(&key).cloned().unwrap_or(0);
        self.uint256_storage.insert(key, current + add);
    }

    pub fn local_sub_uint256(&mut self, key: String, sub: u128) {
        let current = self.uint256_storage.get(&key).cloned().unwrap_or(0);
        assert!(current >= sub, "Underflow error");
        self.uint256_storage.insert(key, current - sub);
    }

    // Operaciones sobre String
    pub fn local_set_string(&mut self, key: String, value: String) {
        self.string_storage.insert(key, value);
    }

    pub fn local_get_string(&self, key: String) -> Option<String> {
        self.string_storage.get(&key).cloned()
    }

    pub fn local_delete_string(&mut self, key: String) {
        self.string_storage.remove(&key);
    }

    pub fn local_append_string(&mut self, key: String, other: String) {
        let current = self.string_storage.get(&key).cloned().unwrap_or_default();
        let updated = format!("{}{}", current, other);
        self.string_storage.insert(key, updated);
    }
}
