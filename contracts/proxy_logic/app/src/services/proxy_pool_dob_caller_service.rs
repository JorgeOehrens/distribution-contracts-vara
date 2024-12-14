use sails_rs::calls::{Call, Query};
use sails_rs::{
    prelude::*,
    cell::RefMut,
};
use crate::states::proxy_state::ProxyState;
use crate::clients::pool_dob_client::traits::Pool;
use crate::clients::pool_dob_client::pool::events::PoolEvents;
use crate::clients::pool_dob_client::State;

/// Servicio para interactuar con el contrato Pool desde el Proxy.
pub struct ProxyPoolDobCallerService<'a, PoolDobClient> {
    proxy_state: RefMut<'a, ProxyState>,
    pool_dob_client: RefMut<'a, PoolDobClient>,
}

#[service]
impl<'a, PoolDobClient> ProxyPoolDobCallerService<'a, PoolDobClient>
where
    PoolDobClient: Pool, // Se especifica que el cliente debe implementar el trait Pool.
{
    /// Crea una nueva instancia del servicio.
    pub const fn new(
        proxy_state: RefMut<'a, ProxyState>,
        pool_dob_client: RefMut<'a, PoolDobClient>,
    ) -> Self {
        Self {
            proxy_state,
            pool_dob_client,
        }
    }

    /// Llama al método `distribution_pool_balance` del contrato Pool.
    pub async fn call_distribution_pool_balance(
        &mut self,
        data: Vec<u8>,
        description: Option<String>,
    ) -> ProxyPoolDobCallerEvent {
        let contract_id = match self.pool_dob_contract_id() {
            Err(error_event) => return error_event,
            Ok(id) => id,
        };

        let temp = self.pool_dob_client
            .distribution_pool_balance(data, description)
            .send_recv(contract_id)
            .await;

        let contract_response = match temp {
            Ok(_) => ProxyPoolDobCallerEvent::PoolDobContractResponse(
                PoolEvents::Execution { transaction_id: U256::zero() }, // Simulado para ejemplo.
            ),
            Err(error) => ProxyPoolDobCallerEvent::Error(
                ProxyPoolDobCallerError::PoolDobContractError(error.to_string()),
            ),
        };

        contract_response
    }

    /// Consulta el estado del contrato Pool usando `get_state`.
    pub async fn call_get_state(&self) -> ProxyPoolDobCallerEvent {
        let contract_id = match self.pool_dob_contract_id() {
            Err(error_event) => return error_event,
            Ok(id) => id,
        };

        let temp = self.pool_dob_client.get_state().recv(contract_id).await;

        match temp {
            Ok(response) => ProxyPoolDobCallerEvent::PoolDobContractState(response),
            Err(error) => ProxyPoolDobCallerEvent::Error(
                ProxyPoolDobCallerError::PoolDobContractError(error.to_string()),
            ),
        }
    }

    /// Obtiene el ID del contrato Pool desde el estado del proxy.
    fn pool_dob_contract_id(&self) -> Result<ActorId, ProxyPoolDobCallerEvent> {
        self.proxy_state.pool_dob_contract_id.ok_or_else(|| {
            ProxyPoolDobCallerEvent::Error(ProxyPoolDobCallerError::PoolDobContractNotSet)
        })
    }
}

/// Enum para representar eventos del servicio Proxy Pool Dob.
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ProxyPoolDobCallerEvent {
    PoolDobContractResponse(PoolEvents),
    PoolDobContractState(State),
    Error(ProxyPoolDobCallerError),
}

/// Enum para representar errores del servicio Proxy Pool Dob.
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ProxyPoolDobCallerError {
    PoolDobContractNotSet,
    PoolDobContractError(String),
}
