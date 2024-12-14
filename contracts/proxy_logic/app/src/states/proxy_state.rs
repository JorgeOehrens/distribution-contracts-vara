// Necesary crates
use sails_rs::prelude::*;

// Set the Proxy state to store the traffic light contract and
// ping contract id
#[derive(Default)]
pub struct ProxyState {
    pub admins: Vec<ActorId>,
    pub pool_dob_contract_id: Option<ActorId>,

}

// Impl to set related functions to the state struct
impl ProxyState {
    // Related function to create a new instance of ProxyState
    pub fn new(
        pool_dob_id: Option<ActorId>, 
        admin: ActorId
    ) -> Self {
        Self {
            admins: vec![admin],
            pool_dob_contract_id: pool_dob_id,
        }
    }

    pub fn is_admin(&self, address: ActorId) -> bool {
        self.admins.contains(&address)
    }
}