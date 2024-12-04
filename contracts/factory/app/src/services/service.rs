use gstd::{ msg, prog::ProgramGenerator, ActorId, CodeId};
use parity_scale_codec::{Decode, Encode};
use sails_rs::{gstd::service, prelude::*};

use crate::states::state::*;

#[derive(Clone)]
pub struct Service();

#[service]
impl Service {
    pub fn init(init_config_factory: InitConfigFactory) {
        unsafe {
            FACTORY = Some(StateFactory {
                number: 0,
                code_id: init_config_factory.code_id,
                factory_admin_account: init_config_factory.factory_admin_account,
                gas_for_program: init_config_factory.gas_for_program,
                ..Default::default()
            });
        }
    }

    pub fn new() -> Self {
        Self()
    }

    pub async fn create_program(
        &mut self,
        init_config: InitConfig,
    ) -> Result<FactoryEvent, FactoryError> {
        let state = StateFactory::get_mut();

        let payload = ["New".encode()].concat();

        let create_program_future =
            ProgramGenerator::create_program_bytes_with_gas_for_reply(
                state.code_id,
                payload,
                state.gas_for_program,
                0,
                5000000000000,
            )
            .map_err(|e| FactoryError::ProgramInitializationFailedWithContext(e.to_string()))?;

        let (address, _) = create_program_future
            .await
            .map_err(|e| FactoryError::ProgramInitializationFailedWithContext(e.to_string()))?;

        state.number = state.number.saturating_add(1);

        state.id_to_address.entry(state.number).or_insert(address);

        let record = Record {
            name: init_config.name.clone(),
        };

        let programs_for_actor = state.registry.entry(msg::source()).or_default();
        programs_for_actor.push((state.number, record.clone()));

        Ok(FactoryEvent::ProgramCreated {
            id: state.number,
            address: address,
            init_config: init_config,
        })
    }

    pub fn update_gas_for_program(
        &mut self,
        new_gas_amount: u64,
    ) -> Result<FactoryEvent, FactoryError> {
        let state = StateFactory::get_mut();

        if state.factory_admin_account.contains(&msg::source()) {
            state.gas_for_program = new_gas_amount;
            Ok(FactoryEvent::GasUpdatedSuccessfully {
                updated_by: msg::source(),
                new_gas_amount,
            })
        } else {
            return Err(FactoryError::Unauthorized);
        }
    }

    pub fn update_code_id(&mut self, new_code_id: CodeId) -> Result<FactoryEvent, FactoryError> {
        let state = StateFactory::get_mut();

        if state.factory_admin_account.contains(&msg::source()) {
            state.code_id = new_code_id;
            Ok(FactoryEvent::CodeIdUpdatedSuccessfully {
                updated_by: msg::source(),
                new_code_id,
            })
        } else {
            return Err(FactoryError::Unauthorized);
        }
    }

    pub fn add_admin_to_factory(
        &mut self,
        admin_actor_id: ActorId,
    ) -> Result<FactoryEvent, FactoryError> {
        let state = StateFactory::get_mut();

        if state.factory_admin_account.contains(&msg::source()) {
            state.factory_admin_account.push(admin_actor_id);

            Ok(FactoryEvent::AdminAdded {
                updated_by: msg::source(),
                admin_actor_id,
            })
        } else {
            return Err(FactoryError::Unauthorized);
        }
    }

    pub fn remove_registry(&mut self, program_for_id: Id) -> Result<FactoryEvent, FactoryError> {
        let state = StateFactory::get_mut();

        let source = msg::source();
        if state.factory_admin_account.contains(&source) {
            if state.id_to_address.remove(&program_for_id).is_none() {
                return Err(FactoryError::IdNotFoundInAddress);
            }

            let mut is_removed = false;

            for (_actor_id, info) in state.registry.iter_mut() {
                if let Some(pos) = info.iter().position(|(id, _)| *id == program_for_id) {
                    info.remove(pos);
                    is_removed = true;
                    break;
                }
            }

            if !is_removed {
                return Err(FactoryError::IdNotFound);
            }

            Ok(FactoryEvent::RegistryRemoved {
                removed_by: source,
                program_for_id,
            })
        } else {
            return Err(FactoryError::Unauthorized);
        }
    }

    pub fn number(&self) -> u64 {
        StateFactory::get().number
    }
    pub fn code_id(&self) -> CodeId {
        StateFactory::get().code_id
    }
    pub fn admins(&self) -> Vec<ActorId> {
        StateFactory::get().factory_admin_account.clone()
    }
    pub fn gas_for_program(&self) -> u64 {
        StateFactory::get().gas_for_program
    }

    pub fn id_to_address(&self) -> Vec<(Id, ActorId)> {
        StateFactory::get()
            .id_to_address
            .iter()
            .map(|(&id, &actor_id)| (id, actor_id))
            .collect()
    }
    pub fn registry(&self) -> Vec<(ActorId, Vec<(Id, Record)>)> {
        StateFactory::get()
            .registry
            .iter()
            .map(|(&actor_id, records)| (actor_id, records.clone()))
            .collect()
    }
}
