#![no_std]
#![allow(static_mut_refs)]

use core::cmp::min;
use sails_rs::{
    gstd::{exec, msg},
    prelude::*,
};mod utils;
use utils::*;


pub struct ProgramMetadata;

static mut STORAGE: Option<Pool> = None;

struct PoolService(());

#[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    Confirmation {
        sender: ActorId,
        transaction_id: U256,
    },
    Revocation {
        sender: ActorId,
        transaction_id: U256,
    },
    Submission {
        transaction_id: U256,
    },
    Execution {
        transaction_id: U256,
    },
    OwnerAddition {
        owner: ActorId,
    },
    PaticipantAddition {
        participant: ActorId,
    },
    OwnerRemoval {
        owner: ActorId,
    },
    OwnerReplace {
        old_owner: ActorId,
        new_owner: ActorId,
    },
    RequirementChange(U256),
}

impl PoolService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn get_mut(&mut self) -> &'static mut Pool {
        unsafe { STORAGE.as_mut().expect("Storage is not initialized") }
    }
    pub fn get(&self) -> &'static Pool {
        unsafe { STORAGE.as_ref().expect("Storage is not initialized") }
    }
}

#[sails_rs::service(events = Event)]
impl PoolService {
    fn init(
        name: String,
        type_pool: String,
        distribution_mode: String,
        access_type: String,
        owners: Vec<ActorId>,
        participants_pool: Vec<ActorId>,
        required: u32,
    ) -> Self {
        let owners_count = owners.len();
    
        validate_requirement(owners_count, required);
    
        let mut wallet = Pool::default();
    
        for owner in &owners {
            if wallet.owners.contains(owner) {
                panic!("The same owner contained twice")
            } else {
                wallet.owners.insert(*owner);
            }
        }
    
        wallet.required = required;
        wallet.name = name;
        wallet.type_pool = type_pool;
        wallet.distribution_mode = distribution_mode;
        wallet.access_type = access_type;
        wallet.participants_pool = participants_pool.into_iter().collect();
    
        unsafe { STORAGE = Some(wallet) };
        Self(())
    }
    

    pub fn add_owner(&mut self, owner: ActorId) {
        let wallet = self.get_mut();
        wallet.validate_only_wallet();
        wallet.validate_owner_doesnt_exist(&owner);
        validate_requirement(wallet.owners.len() + 1, wallet.required);
        wallet.owners.insert(owner);
        self.notify_on(Event::OwnerAddition { owner })
            .expect("Notification Error");
    }
    pub fn add_participant(&mut self, participant: ActorId) {
        let wallet = self.get_mut();
        wallet.validate_only_wallet();
        wallet.validate_participant_doesnt_exist(&participant);
        wallet.participants_pool.insert(participant);
        self.notify_on(Event::PaticipantAddition { participant })
            .expect("Notification Error");
    }

 

    pub fn remove_owner(&mut self, owner: ActorId) {
        let wallet = self.get_mut();
        wallet.validate_only_wallet();
        wallet.validate_owner_exists(&owner);
        let next_owners_count = wallet.owners.len() - 1;
        validate_requirement(
            next_owners_count,
            min(next_owners_count as u32, wallet.required),
        );

        wallet.owners.remove(&owner);

        if (next_owners_count as u32) < wallet.required {
            wallet.change_requirement(next_owners_count as _);
            self.notify_on(Event::RequirementChange(next_owners_count.into()))
                .expect("Notification Error");
        }
        self.notify_on(Event::OwnerRemoval { owner })
            .expect("Notification Error");
    }

    pub fn replace_owner(&mut self, old_owner: ActorId, new_owner: ActorId) {
        let wallet = self.get_mut();
        wallet.validate_only_wallet();
        wallet.validate_owner_exists(&old_owner);
        wallet.validate_owner_doesnt_exist(&new_owner);

        wallet.owners.insert(new_owner);
        wallet.owners.remove(&old_owner);

        self.notify_on(Event::OwnerReplace {
            old_owner,
            new_owner,
        })
        .expect("Notification Error");
    }
    pub fn change_required_confirmations_count(&mut self, count: u32) {
        let wallet = self.get_mut();
        wallet.change_requirement(count);
        self.notify_on(Event::RequirementChange(count.into()))
            .expect("Notification Error");
    }

    pub fn submit_transaction(
        &mut self,
        destination: ActorId,
        data: Vec<u8>,
        value: u128,
        description: Option<String>,
    ) {
        let wallet = self.get_mut();
        let msg_source = msg::source();
        let transaction_id = wallet.add_transaction(&destination, data, value, description);
        wallet.confirm_transaction(&transaction_id, msg_source);

        self.notify_on(Event::Submission { transaction_id })
            .expect("Notification Error");
    }
    pub fn distribution_pool(
        &mut self,
        data: Vec<u8>,
        total_value: u128,
        description: Option<String>,
    ) {
        let wallet = self.get_mut();
        let msg_source = msg::source();
        
        let participants: Vec<ActorId> = wallet.participants_pool.iter().cloned().collect();
    
        let num_participants = participants.len() as u128;
        if num_participants == 0 {
            msg::reply("No participants available in the pool", 0).expect("Failed to reply");
            return;
        }
    
        let value_per_participant = total_value / num_participants;
    
        if value_per_participant == 0 {
            msg::reply("Total value too small to distribute", 0).expect("Failed to reply");
            return;
        }
    
        for participant in participants {
            let transaction_id = wallet.add_transaction(&participant, data.clone(), value_per_participant, description.clone());
            wallet.confirm_transaction(&transaction_id, msg_source);
    
            self.notify_on(Event::Submission { transaction_id })
                .expect("Notification Error");
        }
    }
    pub fn distribution_pool_balance(&mut self, data: Vec<u8>, description: Option<String>) {
        let wallet = self.get_mut();
        let msg_source = msg::source();
    
        let participants: Vec<ActorId> = wallet.participants_pool.iter().cloned().collect();
    
        if participants.is_empty() {
            return;
        }
    
        let total_value = exec::value_available();
    
        if total_value <= 1 {
            return;
        }
    
        let distributable_value = total_value - 1;
    
        let value_per_participant = distributable_value / participants.len() as u128;
    
        // Verificar si el valor por participante es válido
        if value_per_participant == 0 {
            return;
        }
    
        // Distribuir los tokens
        for participant in participants {
            let transaction_id = wallet.add_transaction(
                &participant,
                data.clone(),
                value_per_participant,
                description.clone(),
            );
            wallet.confirm_transaction(&transaction_id, msg_source);
    
            self.notify_on(Event::Submission { transaction_id })
                .expect("Notification Error");
        }
    
    }
    
    
    
    

    pub fn distribution_pool2(
        &mut self,
        participants: Vec<ActorId>,
        data: Vec<u8>,
        total_value: u128,
        description: Option<String>,
    ) {
        let wallet = self.get_mut();
        let msg_source = msg::source();
    
        let num_participants = participants.len() as u128;
        if num_participants == 0 {
            msg::reply("No participants provided", 0).expect("Failed to reply");
            return;
        }
        let value_per_participant = total_value / num_participants;
    
        if value_per_participant == 0 {
            msg::reply("Total value too small to distribute", 0).expect("Failed to reply");
            return;
        }
    
        for participant in participants {
            let transaction_id = wallet.add_transaction(&participant, data.clone(), value_per_participant, description.clone());
            wallet.confirm_transaction(&transaction_id, msg_source);
    
            self.notify_on(Event::Submission { transaction_id })
                .expect("Notification Error");
        }
    }
    
    pub fn confirm_transaction(&mut self, transaction_id: U256) {
        let wallet = self.get_mut();
        let msg_source = msg::source();
        wallet.confirm_transaction(&transaction_id, msg_source);
        self.notify_on(Event::Confirmation {
            sender: msg_source,
            transaction_id,
        })
        .expect("Notification Error");
    }
    pub fn revoke_confirmation(&mut self, transaction_id: U256) {
        let wallet = self.get_mut();
        let msg_source = msg::source();

        wallet.validate_owner_exists(&msg_source);
        wallet.validate_confirmed(&transaction_id, &msg_source);
        wallet.validate_not_executed(&transaction_id);

        wallet
            .confirmations
            .entry(transaction_id)
            .or_default()
            .remove(&msg_source);

        self.notify_on(Event::Revocation {
            sender: msg_source,
            transaction_id,
        })
        .expect("Notification Error");
    }
    pub fn execute_transaction(&mut self, transaction_id: U256) {
        let wallet = self.get_mut();
        let completion = || {
            self.notify_on(Event::Execution { transaction_id })
                .expect("Notification Error");
        };

        wallet.execute_transaction(&transaction_id, Some(completion));
    }

    // Service's query
    pub fn get_state(&self) -> State {
        self.get().clone().into()
    }
}

pub struct PoolProgram(());

#[allow(clippy::new_without_default)]
#[sails_rs::program]
impl PoolProgram {
    // Program's constructor
    pub fn new(name: String, type_pool:String, distribution_mode:String, access_type: String,owners: Vec<ActorId>, participants_pool: Vec<ActorId>, required: u32) -> Self {
        PoolService::init(name, type_pool, distribution_mode, access_type, owners, participants_pool, required);
        Self(())
    }

    // Exposed service
    pub fn pool(&self) -> PoolService {
        PoolService::new()
    }
}
