// necesary cretes
use sails_rs::{collections::HashMap, prelude::*};
use sails_rs::collections::BTreeMap; // Ajusta según la librería que uses

pub type Id = u64;

pub static mut FACTORY: Option<StateFactory> = None;

#[derive(Debug, Default)]
pub struct StateFactory {
    pub number: u64,
    pub vft_code_id: CodeId,
    pub pool_code_id: CodeId,
    pub factory_admin_account: Vec<ActorId>,
    pub gas_for_program: u64,
    pub id_to_address: BTreeMap<u64, ActorId>,
    pub registry: BTreeMap<ActorId, Vec<(u64, Record)>>,
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
    // Configuración común
    pub name: String,
    
    // Configuración VFT
    pub symbol: String,
    pub decimals: u8,
    
    // Configuración Pool
    pub type_pool: String,
    pub distribution_mode: String,
    pub access_type: String,
    pub participants: Vec<ActorId>,
    pub min_tokens_to_add: u128,
    pub max_tokens_to_burn: u128,
    pub tokens_per_vara: u128,
}


#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitConfigFactory {
    pub vft_code_id: CodeId,  // Código del contrato VFT
    pub pool_code_id: CodeId, // Código del contrato Pool
    pub factory_admin_account: Vec<ActorId>,
    pub gas_for_program: u64,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FactoryEvent {
    ProgramCreated {
        id: u64,
        vft_address: ActorId,
        pool_address: ActorId,
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
