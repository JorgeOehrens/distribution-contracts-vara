use sails_rs::{
    prelude::*,
    gstd::msg
};

use crate::states::ping_pong_state::*;

// Struct that will be used for the Ping service
#[derive(Default)]
pub struct PingService;

impl PingService {
    // This related functions initiate the state of the service
    // this functions can be called only once
    pub fn seed(caller: ActorId) {
        PingState::init_state(caller);
    }
}

// Contract's Ping Service
#[service]
impl PingService {
    // Service's contructor
    pub fn new() -> Self {
        Self
    }

    // Method of the service, will return a PingEnum variant
    // Is a command because it changes the state (it use '&mut self')
    pub fn ping(&mut self) -> PingEnum {
        let caller: ActorId = msg::source();

        PingState::state_mut()
            .last_who_call = (caller.clone(), PingEnum::Ping);

        PingState::state_mut()
            .all_calls
            .push((caller, PingEnum::Ping));

        PingEnum::Pong
    }

    // Method of the service, will return a PingEnum variant
    // Is a command because it changes the state (it use '&mut self')
    pub fn pong(&mut self) -> PingEnum {
        let caller: ActorId = msg::source();

        PingState::state_mut()
            .last_who_call = (caller.clone(), PingEnum::Pong);

        PingState::state_mut()
            .all_calls
            .push((caller, PingEnum::Pong));

        PingEnum::Ping
    }

    // Method that will return data from the last caller
    // Its a query because it only reads the state (it not change the state)
    pub fn last_who_call(&self) -> UserData {
        let (last_caller, action) = &PingState::state_ref().last_who_call;

        (*last_caller, action.clone())
    }

    // Method that will return all the calls of the contract
    // Its a query because it only reads the state (it not change the state)
    pub fn all_calls(&self) -> Vec<UserData> {
        PingState::state_ref()
            .all_calls
            .to_owned()
    }
}
