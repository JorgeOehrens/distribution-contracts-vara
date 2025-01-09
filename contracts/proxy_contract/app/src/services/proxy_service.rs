// Necesary crates
use sails_rs::{
    prelude::*,
    cell::RefMut,
    gstd::msg
};
// Import the state
use crate::states::proxy_state::ProxyState;

pub struct ProxyService<'a> {
    pub state: RefMut<'a, ProxyState>
}

#[service]
impl<'a> ProxyService<'a> {
    pub const fn new(
        state: RefMut<'a, ProxyState>
    ) -> Self {
        Self {
            state
        }
    }

    pub fn change_ping_contract_id(&mut self, contract_id: ActorId) -> ProxyEvent {
        let caller = msg::source();
        if !self.state.is_admin(caller) {
            return ProxyEvent::Error(
                ProxyErrors::OnlyAdminsCanChangeContactId
            );
        }

        self.state.ping_contract_id = Some(contract_id);

        ProxyEvent::PingContractIdSet
    }

    pub fn change_traffic_light_contract_id(&mut self, contract_id: ActorId) -> ProxyEvent {
        let caller = msg::source();
        if !self.state.is_admin(caller) {
            return ProxyEvent::Error(
                ProxyErrors::OnlyAdminsCanChangeContactId
            );
        }

        self.state.traffic_light_contract_id = Some(contract_id);

        ProxyEvent::PingContractIdSet
    }

    pub fn add_admin(&mut self, new_admin: ActorId) -> ProxyEvent {
        let caller = msg::source();
        if !self.state.is_admin(caller) {
            return ProxyEvent::Error(
                ProxyErrors::OnlyAdminsCanChangeContactId
            );
        }

        if self.state.admins.contains(&new_admin) {
            return ProxyEvent::Error(
                ProxyErrors::AdminExistsInContract(new_admin)
            );
        }

        self.state.admins.push(new_admin);

        ProxyEvent::AdminAdded(new_admin)
    }

    pub fn contracts_id(&self) -> ProxyEvent {
        ProxyEvent::ContractsId(ContractsId {
            ping_contract_id: self.state.ping_contract_id,
            traffic_light_contract_id: self.state.traffic_light_contract_id
        })
    }
}

#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct ContractsId {
    ping_contract_id: Option<ActorId>,
    traffic_light_contract_id: Option<ActorId>
}

#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ProxyEvent {
    PingContractIdSet,
    TrafficLightContractIdSet,
    AdminAdded(ActorId),
    PingContractId(Option<ActorId>),
    TrafficLightContractId(Option<ActorId>),
    ContractsId(ContractsId),
    Error(ProxyErrors)
}

#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ProxyErrors {
    OnlyAdminsCanChangeContactId,
    AdminExistsInContract(ActorId)
}