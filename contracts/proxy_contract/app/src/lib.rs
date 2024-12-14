#![no_std]
use sails_rs::{
    prelude::*,
    cell::RefCell, 
    gstd::{
        calls::GStdRemoting, 
        msg
    } 
};

pub mod clients;
pub mod services;
pub mod states;

use states::proxy_state::ProxyState;
use services::{
    proxy_ping_caller_service::ProxyPingCallerService,
    proxy_taffic_light_caller_service::ProxyTrafficLightCallerService,
    proxy_service::ProxyService
};
use clients::{
    ping_client::Ping as PingClient,
    traffic_light_client::TrafficLight as TrafficLightClient
};

pub struct ProxyProgram {
    proxy_state: RefCell<ProxyState>,
    ping_client: RefCell<PingClient<GStdRemoting>>,
    traffic_light_client: RefCell<TrafficLightClient<GStdRemoting>>
}

impl ProxyProgram {
    pub fn new_proxy(
        ping_contract_id: Option<ActorId>,
        traffic_light_contract_id: Option<ActorId>
    ) -> Self {
        let proxy_state = RefCell::new(ProxyState::new(
            traffic_light_contract_id,
            ping_contract_id,
            msg::source()
            
        ));
        let ping_client = RefCell::new(PingClient::new(GStdRemoting));
        let traffic_light_client = RefCell::new(TrafficLightClient::new(GStdRemoting));

        Self {
            proxy_state,
            ping_client,
            traffic_light_client
        }
    }
}

#[program]
impl ProxyProgram {
    pub fn new_with_contracts_id(
        ping_contract_id: ActorId,
        traffic_light_contract_id: ActorId
    ) -> Self {
        Self::new_proxy(
            Some(ping_contract_id), 
            Some(traffic_light_contract_id)
        )
    }

    pub fn new() -> Self {
        Self::new_proxy(None, None)
    }

    #[route("Proxy")]
    pub fn proxy_svc(&self) -> ProxyService<'_> {
        ProxyService::new(self.proxy_state.borrow_mut())
    }

    #[route("PingCaller")]
    pub fn ping_caller_svc(&self) -> ProxyPingCallerService<'_, PingClient<GStdRemoting>> {
        ProxyPingCallerService::new(
            self.proxy_state.borrow_mut(), 
            self.ping_client.borrow_mut()
        )
    }

    #[route("TrafficLightCaller")]
    pub fn traffic_light_caller_svc(&self) -> ProxyTrafficLightCallerService<'_, TrafficLightClient<GStdRemoting>> {
        ProxyTrafficLightCallerService::new(
            self.proxy_state.borrow_mut(), 
            self.traffic_light_client.borrow_mut()
        )
    }
}



/*
Gas fees with traditional generics:

Uploading contract: 0.01211977053
Message to traffic contract green: 0.032339940006
Message to traffic contract red:   0.032680430748
*/