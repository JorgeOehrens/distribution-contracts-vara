use sails_rs::calls::{Call, Query};
// Necesary crates
use sails_rs::{
    prelude::*,
    cell::RefMut
};
// Import the state
use crate::states::proxy_state::ProxyState;
// Import the clients of contract to send messages
use crate::clients::traffic_light_client::{
    traits::TrafficLight,
    TrafficLightEvent,
    IoTrafficLightState
};

// Proxy traffic light caller service struct to build the service, it contains two 
// RefMut to set a mutable reference from a RefCell, with this, you can handle 
// the contracts state as a part of the program.
// It receives the client from the state (traffic_light_client), this helps to save tokens  
// on gas fees in contracts communications
// - It uses a lifetime to handle both references.
// - The traffic_light_client value is a generic one which will be restricted in the 
//   impl where the service will be specified.
pub struct ProxyTrafficLightCallerService<'a, TrafficLightClient>{
    proxy_state: RefMut<'a, ProxyState>,
    traffic_light_client: RefMut<'a, TrafficLightClient>
}


// Impl block that use a lifetime 'a and the generic type TrafficLightClient, which have a
// restriction:
// - The TrafficLightClient should implement the trait TrafficLight
#[service]
impl<'a, TrafficLightClient> ProxyTrafficLightCallerService<'a, TrafficLightClient>
where 
    TrafficLightClient: TrafficLight, // It is specified that the generic type must implement the TrafficLight feature
{
    // Related function to create a new instance of the service
    pub const fn new(
        proxy_state: RefMut<'a, ProxyState>,
        traffic_light_client: RefMut<'a, TrafficLightClient>
    ) -> Self {
        Self {
            proxy_state,
            traffic_light_client
        }
    }

    // Method (command) that will call the green method in the traffic light contract
    // It doesn't change the state from the proxy contract, but we need gas 
    // fees to be able to change the state of the traffic light contract
    pub async fn call_green(&mut self) -> ProxyTrafficLightCallerEvent {
        // Get the ping contract id
        let contract_id = match self.traffic_light_contract_id() {
            Err(error_event) => return error_event,
            Ok(id) => id
        };
        
        // Call the contract and store the response in a variable
        let temp = self.traffic_light_client
            .green() // Method name to call
            .send_recv(contract_id)  // Send the message and get a response 
            .await;

        // Check if the contract response was successfull
        let contract_response = match temp {
            Ok(response) => response,
            Err(error) => return ProxyTrafficLightCallerEvent::Error(
                ProxyTrafficLightCallerError::TrafficLightContractError(error.to_string())
            )
        };

        // Return the proxy contract event
        ProxyTrafficLightCallerEvent::TrafficLightContractResponse(contract_response)
    }

    // Method (command) that will call the yellow method in the traffic light contract
    // It doesn't change the state from the proxy contract, but we need gas 
    // fees to be able to change the state of the ping contract
    pub async fn call_yellow(&mut self) -> ProxyTrafficLightCallerEvent {
        let contract_id = match self.traffic_light_contract_id() {
            Err(error_event) => return error_event,
            Ok(id) => id
        };
        
        let temp = self.traffic_light_client
            .yellow() // Method name to call
            .send_recv(contract_id)  // Send the message and get a response
            .await;

        let contract_response = match temp {
            Ok(response) => response,
            Err(error) => return ProxyTrafficLightCallerEvent::Error(
                ProxyTrafficLightCallerError::TrafficLightContractError(error.to_string())
            )
        };

        ProxyTrafficLightCallerEvent::TrafficLightContractResponse(contract_response)
    }

    // Method (command) that will call the red method in the traffic light contract
    // It doesn't change the state from the proxy contract, but we need gas 
    // fees to be able to change the state of the ping contract
    pub async fn call_red(&mut self) -> ProxyTrafficLightCallerEvent {
        let contract_id = match self.traffic_light_contract_id() {
            Err(error_event) => return error_event,
            Ok(id) => id
        };
        
        let temp = self.traffic_light_client
            .red() // Method name to call
            .send_recv(contract_id)  // Send the message and get a response
            .await;

        let contract_response = match temp {
            Ok(response) => response,
            Err(error) => return ProxyTrafficLightCallerEvent::Error(
                ProxyTrafficLightCallerError::TrafficLightContractError(error.to_string())
            )
        };

        ProxyTrafficLightCallerEvent::TrafficLightContractResponse(contract_response)
    }

    // Method (query) that will call the traffic_light method in the traffic light contract
    // This method calls the query method from traffic light contract, and it will
    // send the response to the user. It does not need gas fees
    pub async fn call_traffic_light_state(&self) -> ProxyTrafficLightCallerEvent {
        let contract_id = match self.traffic_light_contract_id() {
            Err(error_event) => return error_event,
            Ok(id) => id
        };
        
        let temp = self.traffic_light_client
            .traffic_light() // Method name to call
            .recv(contract_id) // get the response from the contract
            .await;

        let contract_state = match temp {
            Ok(response) => response,
            Err(error) => return ProxyTrafficLightCallerEvent::Error(
                ProxyTrafficLightCallerError::TrafficLightContractError(error.to_string())
            )
        };

        ProxyTrafficLightCallerEvent::TrafficLightContractState(contract_state)
    }

    // Helper query method to get the ping contract id
    fn traffic_light_contract_id(&self) -> Result<ActorId, ProxyTrafficLightCallerEvent> {
        if self.proxy_state.traffic_light_contract_id.is_none() {
            return Err(ProxyTrafficLightCallerEvent::Error(
                ProxyTrafficLightCallerError::TrafficLightContractIdNotSet
            ));
        }

        Ok(self.proxy_state.traffic_light_contract_id.unwrap())
    }
}

// Enum to set the events from the proxy traffic light contract caller
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ProxyTrafficLightCallerEvent {
    Test,
    TrafficLightContractResponse(TrafficLightEvent),
    TrafficLightContractState(IoTrafficLightState),
    Error(ProxyTrafficLightCallerError)
}

// Enum to set the errors from the proxy traffic light contract caller
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ProxyTrafficLightCallerError {
    TrafficLightContractIdNotSet,
    TrafficLightContractError(String)
}