use sails_rs::prelude::*;

// Create the type aliase for the state.
pub type UserData = (ActorId, PingEnum);

// Static mut variable that will store the state of the contract
static mut PING_STATE:Option<PingState> = None;

// Struct for the state of the contract
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct PingState {
    pub last_who_call: UserData,
    pub all_calls: Vec<UserData>
}

// Related functions for PingState struct
impl PingState {
    // Init the state of the contract.
    // This related function need to be called only once
    pub fn init_state(caller: ActorId) {
        unsafe {
            PING_STATE = Some(PingState {
                last_who_call: (caller, PingEnum::Ping),
                all_calls: Vec::new()
            })
        }
    }

    // Get the contract state Ping state as mut
    pub fn state_mut() -> &'static mut PingState {
        let state = unsafe { PING_STATE.as_mut() };
        debug_assert!(state.is_some(), "State is not initialized");
        unsafe { state.unwrap_unchecked() }
    }

    // Get the contract state Ping state as ref
    pub fn state_ref() -> &'static PingState {
        let state = unsafe { PING_STATE.as_ref() };
        debug_assert!(state.is_some(), "State is not initialized");
        unsafe { state.unwrap_unchecked() }
    }
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum PingEnum {
    Ping,
    Pong
}