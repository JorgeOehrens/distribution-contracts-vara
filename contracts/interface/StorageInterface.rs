use gstd::{msg, prelude::*, ActorId};
use hashbrown::HashMap;

#[derive(Default)]
pub struct StorageContract {
    address_storage: HashMap<Vec<u8>, ActorId>,
    uint256_storage: HashMap<Vec<u8>, u128>,
    string_storage: HashMap<Vec<u8>, String>,
    bytes_storage: HashMap<Vec<u8>, Vec<u8>>,
    bool_storage: HashMap<Vec<u8>, bool>,
    int_storage: HashMap<Vec<u8>, i128>,
    bytes32_storage: HashMap<Vec<u8>, [u8; 32]>,
    guardian: Option<ActorId>,
    admin_roles: HashMap<ActorId, bool>,
    user_roles: HashMap<ActorId, bool>,
}

impl StorageContract {
    // Getters
    pub fn get_address(&self, key: Vec<u8>) -> Option<ActorId> {
        self.address_storage.get(&key).cloned()
    }

    pub fn get_uint256(&self, key: Vec<u8>) -> Option<u128> {
        self.uint256_storage.get(&key).cloned()
    }

    pub fn get_string(&self, key: Vec<u8>) -> Option<String> {
        self.string_storage.get(&key).cloned()
    }

    pub fn get_bytes(&self, key: Vec<u8>) -> Option<Vec<u8>> {
        self.bytes_storage.get(&key).cloned()
    }

    pub fn get_bool(&self, key: Vec<u8>) -> Option<bool> {
        self.bool_storage.get(&key).cloned()
    }

    pub fn get_int(&self, key: Vec<u8>) -> Option<i128> {
        self.int_storage.get(&key).cloned()
    }

    pub fn get_bytes32(&self, key: Vec<u8>) -> Option<[u8; 32]> {
        self.bytes32_storage.get(&key).cloned()
    }

    // Setters
    pub fn set_address(&mut self, key: Vec<u8>, value: ActorId) {
        self.address_storage.insert(key, value);
    }

    pub fn set_uint256(&mut self, key: Vec<u8>, value: u128) {
        self.uint256_storage.insert(key, value);
    }

    pub fn set_string(&mut self, key: Vec<u8>, value: String) {
        self.string_storage.insert(key, value);
    }

    pub fn set_bytes(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.bytes_storage.insert(key, value);
    }

    pub fn set_bool(&mut self, key: Vec<u8>, value: bool) {
        self.bool_storage.insert(key, value);
    }

    pub fn set_int(&mut self, key: Vec<u8>, value: i128) {
        self.int_storage.insert(key, value);
    }

    pub fn set_bytes32(&mut self, key: Vec<u8>, value: [u8; 32]) {
        self.bytes32_storage.insert(key, value);
    }

    // Deleters
    pub fn delete_address(&mut self, key: Vec<u8>) {
        self.address_storage.remove(&key);
    }

    pub fn delete_uint256(&mut self, key: Vec<u8>) {
        self.uint256_storage.remove(&key);
    }

    pub fn delete_string(&mut self, key: Vec<u8>) {
        self.string_storage.remove(&key);
    }

    pub fn delete_bytes(&mut self, key: Vec<u8>) {
        self.bytes_storage.remove(&key);
    }

    pub fn delete_bool(&mut self, key: Vec<u8>) {
        self.bool_storage.remove(&key);
    }

    pub fn delete_int(&mut self, key: Vec<u8>) {
        self.int_storage.remove(&key);
    }

    pub fn delete_bytes32(&mut self, key: Vec<u8>) {
        self.bytes32_storage.remove(&key);
    }

    // Guardian
    pub fn get_guardian(&self) -> Option<ActorId> {
        self.guardian
    }

    pub fn set_guardian(&mut self, new_guardian: ActorId) {
        self.guardian = Some(new_guardian);
    }

    // Role Management
    pub fn grant_admin_role(&mut self, account: ActorId) {
        self.admin_roles.insert(account, true);
    }

    pub fn grant_user_role(&mut self, account: ActorId) {
        self.user_roles.insert(account, true);
    }

    pub fn is_admin(&self, account: ActorId) -> bool {
        *self.admin_roles.get(&account).unwrap_or(&false)
    }

    pub fn is_user(&self, account: ActorId) -> bool {
        *self.user_roles.get(&account).unwrap_or(&false)
    }

    pub fn revoke_admin_role(&mut self, account: ActorId) {
        self.admin_roles.remove(&account);
    }

    pub fn revoke_user_role(&mut self, account: ActorId) {
        self.user_roles.remove(&account);
    }
}
