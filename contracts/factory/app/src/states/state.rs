// necesary cretes
use sails_rs::{collections::HashMap, prelude::*};

pub type Id = u64;

pub static mut FACTORY: Option<StateFactory> = None;

#[derive(Debug, Default)]
pub struct StateFactory {
    pub number: Id,
    pub code_id: CodeId,
    pub factory_admin_account: Vec<ActorId>,
    pub gas_for_program: u64,
    pub id_to_address: HashMap<Id, ActorId>,
    pub registry: HashMap<ActorId, Vec<(Id, Record)>>,
}

impl StateFactory {
    pub fn get_mut() -> &'static mut Self {
        unsafe { FACTORY.as_mut().expect("State Factory Error") }
    }
    pub fn get() -> &'static Self {
        unsafe { FACTORY.as_ref().expect("State Factory Error") }
    }
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Record {
    pub name: String,
}

#[derive(Debug, Decode, Encode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitConfig {
    pub name: String,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitConfigFactory {
    pub code_id: CodeId,
    pub factory_admin_account: Vec<ActorId>,
    pub gas_for_program: u64,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FactoryEvent {
    ProgramCreated {
        id: Id,
        address: ActorId,
        init_config: InitConfig,
    },
    GasUpdatedSuccessfully {
        updated_by: ActorId,
        new_gas_amount: u64,
    },
    CodeIdUpdatedSuccessfully {
        updated_by: ActorId,
        new_code_id: CodeId,
    },
    AdminAdded {
        updated_by: ActorId,
        admin_actor_id: ActorId,
    },
    RegistryRemoved {
        removed_by: ActorId,
        program_for_id: Id,
    },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FactoryError {
    ProgramInitializationFailed,
    ProgramInitializationFailedWithContext(String),
    Unauthorized,
    UnexpectedFTEvent,
    MessageSendError,
    NotFound,
    IdNotFoundInAddress,
    IdNotFound,
}
