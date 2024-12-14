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
    proxy_pool_dob_caller_service::ProxyPoolDobCallerService,
    proxy_service::ProxyService
};
use clients::{
    pool_dob_client::Pool as PoolDobClient,
};

pub struct ProxyProgram {
    proxy_state: RefCell<ProxyState>,
    pool_dob_client: RefCell<PoolDobClient<GStdRemoting>>,
}

impl ProxyProgram {
    pub fn new_proxy(
        pool_dob_contract_id: Option<ActorId>,
    ) -> Self {
        let proxy_state = RefCell::new(ProxyState::new(
            pool_dob_contract_id,
            msg::source()
            
        ));
        let pool_dob_client = RefCell::new(PoolDobClient::new(GStdRemoting));

        Self {
            proxy_state,
            pool_dob_client,
        }
    }
}

#[program]
impl ProxyProgram {
    pub fn new_with_contracts_id(
        pool_dob_contract_id: ActorId,
    ) -> Self {
        Self::new_proxy(
            Some(pool_dob_contract_id), 
        )
    }

    pub fn new() -> Self {
        Self::new_proxy(None)
    }

    #[route("Proxy")]
    pub fn proxy_svc(&self) -> ProxyService<'_> {
        ProxyService::new(self.proxy_state.borrow_mut())
    }

    #[route("PoolDobCaller")]
    pub fn pool_dob_caller_svc(&self) -> ProxyPoolDobCallerService<'_, PoolDobClient<GStdRemoting>> {
        ProxyPoolDobCallerService::new(
            self.proxy_state.borrow_mut(), 
            self.pool_dob_client.borrow_mut()
        )
    }
}



/*
Gas fees with traditional generics:

Uploading contract: 0.01211977053
Message to traffic contract green: 0.032339940006
Message to traffic contract red:   0.032680430748
*/