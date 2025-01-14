use sails_rs::{
    collections::{HashMap},
    prelude::*,
};

pub type TransactionId = U256;

// Struct to handle the state of the contract
#[derive(Default)]
pub struct VFTManagerState {
    // Vec to store admins that can do special actions
    pub admins: Vec<ActorId>,
    // Vec to store admins that can do special actions
    pub name: String,
    // Vec to store admins that can do special actions
    pub type_pool: String,

    // Vec to store admins that can do special actions
    pub distribution_mode: String,
    // Vec to store admins that can do special actions
    pub access_type: String,
    // Vec to store admins that can do special actions
    pub participants: Vec<ActorId>,
    // contract id from the extended vft contract
    pub vft_contract_id: Option<ActorId>,


    pub transactions: HashMap<TransactionId, Transaction>,
    pub transaction_count: U256,


}


#[derive(Debug, Default, Encode, Decode, TypeInfo, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Transaction {
    pub destination: ActorId,
    pub value: u128,
    pub executed: bool,
}

impl VFTManagerState {
    // Related function "new", returns a new VFTManagerState instance with a new admin address
    // Is necessary to pass an address to be the first admin to perform the actions (commands) in
    // the contract
    pub fn add_transaction(
        &mut self,
        destination: ActorId,
        value: u128,
    ) -> TransactionId {
        let transaction_id = self.transaction_count;
        let transaction = Transaction {
            destination,
            value,
            executed: false,
        };

        self.transactions.insert(transaction_id, transaction);
        self.transaction_count += U256::from(1);

        transaction_id
    }
    pub fn new(admin: ActorId) -> Self {
        let mut temp = Self::default();
        temp.admins.push(admin);
        temp
    }

    // Helper function that returns if an address is an admin
    pub fn is_admin(&self, address: &ActorId) -> bool {
        self.admins.contains(address)
    }
}