use sails_rs::calls::{Call, Query};
// Necesary crates
use sails_rs::{
    prelude::*,
    cell::RefMut
};
// Import the state
use crate::states::proxy_state::ProxyState;
// Import the client and the enum from ping contract
use crate::clients::ping_client::{
    traits::Ping,
    PingEnum
};

// Proxy ping caller service struct to build the service, it contains two 
// RefMut to set a mutable reference from a RefCell, with this, you can handle 
// the contracts state as a part of the program.
// It receives the client from the state (ping_client), this helps to save tokens  
// on gas fees in contracts communications
// - It uses a lifetime to handle both references.
// - The ping_client value is a generic one which will be restricted in the 
//   impl where the service will be specified.
pub struct ProxyPingCallerService<'a, PingClient> {
    proxy_state: RefMut<'a, ProxyState>,
    ping_client: RefMut<'a, PingClient>
}

// Impl block that use a lifetime 'a and the generic type PingClient, which have a restriction:
// - The PingClient should implement the Ping trait 
#[service]
impl<'a, PingClient> ProxyPingCallerService<'a, PingClient>
where 
    PingClient: Ping // It is specified that the generic type must implement the Ping trait
{
    // Related function to create a new instance of the service
    pub const fn new(
        proxy_state: RefMut<'a, ProxyState>,
        ping_client: RefMut<'a, PingClient>
    ) -> Self {
        Self {
            proxy_state,
            ping_client
        }
    }

    // Method (command) that will call the ping method in the ping contract
    // It doesn't change the state from the proxy contract, but we need gas 
    // fees to be able to change the state of the ping contract
    pub async fn call_ping(&mut self) -> ProxyPingCallerEvent {
        // Get the ping contract id
        let contract_id = match self.ping_contract_id() {
            Err(error_event) => return error_event,
            Ok(id) => id
        };

        // Call the contract and store the response in a variable
        let temp = self.ping_client
            .ping() // Method name to call
            .send_recv(contract_id) // Send the message and get a response
            .await;

        // Check if the contract response was successfull
        let contract_response = match temp {
            Ok(response) => response,
            Err(error) => return ProxyPingCallerEvent::Error(
                ProxyPingCallerError::PingContractError(error.to_string())
            )
        };

        // Return the proxy contract event
        ProxyPingCallerEvent::PingContractResponse(contract_response)
    }

    // Method (command) that will call the pong method in the ping contract
    // It doesn't change the state from the proxy contract, but we need gas 
    // fees to be able to change the state of the ping contract
    pub async fn call_pong(&mut self) -> ProxyPingCallerEvent {
        // Get the ping contract id
        let contract_id = match self.ping_contract_id() {
            Err(error_event) => return error_event,
            Ok(id) => id
        };

        // Call the contract and store the response in a variable
        let temp = self.ping_client
            .pong()// Method name to call
            .send_recv(contract_id)  // Send the message and get a response
            .await;

        // Check if the contract response was successfull
        let contract_response = match temp {
            Ok(response) => response,
            Err(error) => return ProxyPingCallerEvent::Error(
                ProxyPingCallerError::PingContractError(error.to_string())
            )
        };

        // Return the proxy contract event
        ProxyPingCallerEvent::PingContractResponse(contract_response)
    }

    // Method (query) that will call the last_who_call method in the ping contract
    // This method calls the query method from ping contract, and it will
    // send the response to the user. It does not need gas fees
    pub async fn call_last_who_call(&self) -> ProxyPingCallerEvent {
        // Get the ping contract id
        let contract_id = match self.ping_contract_id() {
            Err(error_event) => return error_event,
            Ok(id) => id
        };

        // Call the contract query method and store the response un a variable
        let temp = self.ping_client
            .last_who_call() // Method name to call
            .recv(contract_id)// get the response from the contract
            .await;

        // Check if the contract response was successfull
        let contract_state = match temp {
            Ok(response) => response,
            Err(error) => return ProxyPingCallerEvent::Error(
                ProxyPingCallerError::PingContractError(error.to_string())
            )
        };

        // Return the proxy contract event
        ProxyPingCallerEvent::PingContractStateLastWhoCall(contract_state)
    }

    // Method (query) that will call the all_calls methos in the ping contract
    // This method calls the query method from ping contract, and it will
    // send the response to the user. It does not need gas fees
    pub async fn call_all_calls(&self) -> ProxyPingCallerEvent {
        // Get the ping contract id
        let contract_id = match self.ping_contract_id() {
            Err(error_event) => return error_event,
            Ok(id) => id
        };

        // Call the contract query method and store the response un a variable
        let temp = self.ping_client
            .all_calls() // Method name to call
            .recv(contract_id) // get the response from the contract
            .await;

        // Check if the contract response was successfull
        let contract_state = match temp {
            Ok(response) => response,
            Err(error) => return ProxyPingCallerEvent::Error(
                ProxyPingCallerError::PingContractError(error.to_string())
            )
        };

        // Return the proxy contract event
        ProxyPingCallerEvent::PingContractStateAllCalls(contract_state)
    }

    // Helper query method to get the ping contract id
    fn ping_contract_id(&self) -> Result<ActorId, ProxyPingCallerEvent> {
        if self.proxy_state.ping_contract_id.is_none() {
            return Err(ProxyPingCallerEvent::Error(
                ProxyPingCallerError::PingContractIdNotSet
            ));
        }

        Ok(self.proxy_state.ping_contract_id.unwrap())
    }
}


// Enum to set the events from the proxy ping contract caller
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ProxyPingCallerEvent {
    Test,
    PingContractStateAllCalls(Vec<(ActorId, PingEnum)>),
    PingContractStateLastWhoCall((ActorId, PingEnum)),
    PingContractResponse(PingEnum),
    Error(ProxyPingCallerError)
}

// Enum to set the errors from the proxy ping contract caller
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ProxyPingCallerError {
    PingContractIdNotSet,
    PingContractError(String)
}