use sails_rs::{
    prelude::*,
    cell::RefMut,
    gstd::msg,
};
// Importar el estado del proxy
use crate::states::proxy_state::ProxyState;

/// Servicio principal para manejar el estado del proxy.
pub struct ProxyService<'a> {
    pub state: RefMut<'a, ProxyState>,
}

#[service]
impl<'a> ProxyService<'a> {
    /// Crea una nueva instancia del servicio.
    pub const fn new(state: RefMut<'a, ProxyState>) -> Self {
        Self { state }
    }

    /// Cambia el ID del contrato Pool Dob.
    pub fn change_pool_dob_contract_id(&mut self, contract_id: ActorId) -> ProxyEvent {
        let caller = msg::source();
        if !self.state.is_admin(caller) {
            return ProxyEvent::Error(ProxyErrors::OnlyAdminsCanChangeContractId);
        }

        self.state.pool_dob_contract_id = Some(contract_id);
        ProxyEvent::PoolDobContractIdSet
    }

    /// Agrega un nuevo administrador al proxy.
    pub fn add_admin(&mut self, new_admin: ActorId) -> ProxyEvent {
        let caller = msg::source();
        if !self.state.is_admin(caller) {
            return ProxyEvent::Error(ProxyErrors::OnlyAdminsCanChangeContractId);
        }

        if self.state.admins.contains(&new_admin) {
            return ProxyEvent::Error(ProxyErrors::AdminExistsInContract(new_admin));
        }

        self.state.admins.push(new_admin);
        ProxyEvent::AdminAdded(new_admin)
    }

    /// Consulta los IDs de contratos configurados en el proxy.
    pub fn contracts_id(&self) -> ProxyEvent {
        ProxyEvent::ContractsId(ContractsId {
            pool_dob_contract_id: self.state.pool_dob_contract_id,
        })
    }
}

/// Estructura para representar los IDs de contratos configurados.
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct ContractsId {
    pub pool_dob_contract_id: Option<ActorId>,
}

/// Eventos posibles del servicio Proxy.
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ProxyEvent {
    PoolDobContractIdSet,
    AdminAdded(ActorId),
    ContractsId(ContractsId),
    Error(ProxyErrors),
}

/// Errores posibles del servicio Proxy.
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ProxyErrors {
    OnlyAdminsCanChangeContractId,
    AdminExistsInContract(ActorId),
}
