use gstd::{
    msg, 
    prog::ProgramGenerator, 
    ActorId, 
    CodeId,
};
use sails_rs::prelude::*;

use crate::states::state::*;

#[derive(Clone)]
pub struct Service();

#[service]
impl Service {
    pub fn init(init_config_factory: InitConfigFactory) {
        unsafe {
            FACTORY = Some(StateFactory {
                number: 0,
                vft_code_id: init_config_factory.vft_code_id,
                pool_code_id: init_config_factory.pool_code_id,
                factory_admin_account: init_config_factory.factory_admin_account,
                gas_for_program: init_config_factory.gas_for_program,
                ..Default::default()
            });
        }
    }

    pub fn new() -> Self {
        Self()
    }

    /// Despliegue del contrato VFT
    pub async fn create_vft(
        &mut self,
        init_config: InitConfig,
    ) -> Result<ActorId, FactoryError> {
        let state = StateFactory::get_mut();
        let admin = msg::source(); // Agregar al creador como administrador

        let vft_payload = [
            "New".encode(),
            init_config.name.encode(),
            init_config.symbol.encode(),
            init_config.decimals.encode(),
            vec![admin].encode(), 

        ]
        .concat();

        let vft_future = ProgramGenerator::create_program_bytes_with_gas_for_reply(
            state.vft_code_id,
            vft_payload,
            state.gas_for_program,
            0,
            10_000_000_000,
        )
        .map_err(|e| FactoryError::ProgramInitializationFailedWithContext(e.to_string()))?;

        let (vft_address, _) = vft_future
            .await
            .map_err(|e| FactoryError::ProgramInitializationFailedWithContext(e.to_string()))?;

        Ok(vft_address)
    }

    /// Despliegue del contrato Pool
    pub async fn create_pool(
        &mut self,
        init_config: InitConfig,
        vft_address: ActorId,
    ) -> Result<ActorId, FactoryError> {
        let state = StateFactory::get_mut();
        let reply_deposit: u64 = 500_000; // Ajuste según tu necesidad

        let pool_payload = [
            "NewWithData".encode(),
            init_config.name.encode(),
            init_config.type_pool.encode(),
            init_config.distribution_mode.encode(),
            init_config.access_type.encode(),
            init_config.participants.encode(),
            Some(vft_address).encode(),
            init_config.min_tokens_to_add.encode(),
            init_config.max_tokens_to_burn.encode(),
            init_config.tokens_per_vara.encode(),
        ]
        .concat();

        let pool_future = ProgramGenerator::create_program_bytes_with_gas_for_reply(
            state.pool_code_id,
            pool_payload,
            state.gas_for_program,
            0,
            10_000_000_000,
        )
        .map_err(|e| FactoryError::ProgramInitializationFailedWithContext(e.to_string()))?;

        let (pool_address, _) = pool_future
            .await
            .map_err(|e| FactoryError::ProgramInitializationFailedWithContext(e.to_string()))?;

        Ok(pool_address)
    }

    /// Despliegue combinado de VFT y Pool
    pub async fn create_vft_and_pool(
        &mut self,
        init_config: InitConfig,
    ) -> Result<FactoryEvent, FactoryError> {
        let vft_address = self.create_vft(init_config.clone()).await?;
        let pool_address = self.create_pool(init_config.clone(), vft_address).await?;

        let state = StateFactory::get_mut();
        state.number = state.number.saturating_add(1);
        state.id_to_address.entry(state.number).or_insert(vft_address);
        state.number = state.number.saturating_add(1);
        state.id_to_address.entry(state.number).or_insert(pool_address);

        Ok(FactoryEvent::ProgramCreated {
            id: state.number,
            vft_address,
            pool_address,
            init_config,
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
            Err(FactoryError::Unauthorized)
        }
    }

    pub fn update_code_id(
        &mut self,
        new_vft_code_id: Option<CodeId>,
        new_pool_code_id: Option<CodeId>,
    ) -> Result<FactoryEvent, FactoryError> {
        let state = StateFactory::get_mut();

        if state.factory_admin_account.contains(&msg::source()) {
            if let Some(vft_code_id) = new_vft_code_id {
                state.vft_code_id = vft_code_id;
            }
            if let Some(pool_code_id) = new_pool_code_id {
                state.pool_code_id = pool_code_id;
            }

            Ok(FactoryEvent::CodeIdUpdatedSuccessfully {
                updated_by: msg::source(),
                new_code_id: new_vft_code_id.or(new_pool_code_id).unwrap(),
            })
        } else {
            Err(FactoryError::Unauthorized)
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
            Err(FactoryError::Unauthorized)
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
    /// Leer el `CodeId` actual del VFT
    pub fn vft_code_id(&self) -> CodeId {
        StateFactory::get().vft_code_id
    }
    /// Leer el `CodeId` actual del Pool
    pub fn pool_code_id(&self) -> CodeId {
        StateFactory::get().pool_code_id
    }
    /// Leer el gas asignado para los programas
    pub fn gas_for_program(&self) -> u64 {
        StateFactory::get().gas_for_program
    }

    pub fn admins(&self) -> Vec<ActorId> {
        StateFactory::get().factory_admin_account.clone()
    }
 
    pub fn id_to_address(&self) -> Vec<(Id, ActorId)> {
        StateFactory::get()
            .id_to_address
            .iter()
            .map(|(&id, &actor_id)| (id, actor_id))
            .collect()
    }
    /// Leer el registro completo
    pub fn registry(&self) -> Vec<(ActorId, Vec<(Id, Record)>)> {
        StateFactory::get()
            .registry
            .iter()
            .map(|(&actor_id, records)| (actor_id, records.clone()))
            .collect()
    }



}
