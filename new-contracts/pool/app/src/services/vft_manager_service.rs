#![allow(static_mut_refs)] // Only to avoid the static variable mut reference warning

// Necesary imports
use sails_rs::calls::{Call, Query};
use sails_rs::{
    prelude::*,
    gstd::msg,
    collections::{HashMap}

};
use gstd::exec;
use crate::states::vft_manager_state::Transaction;

// Import the struct state (VFTManagerState)
use crate::states::vft_manager_state::VFTManagerState;
// Import the Client from the extended-vft contract (is in the clients directory)
use crate::clients::extended_vft_client::traits::Vft;

// VFTManager state
static mut VFT_MANAGER_STATE: Option<VFTManagerState> = None;

// Constant ONE_TVARA, represent one VARA in Vara Network
const ONE_TVARA: u128 = 1e12 as u128; // Value of one TVara and Vara (1_000_000_000_000)

// Service struct, that specify the client to use VftClient (generic type,
// it will take its type in the impl block)
pub struct VFTManagerService<VftClient> {
    pub vft_client: VftClient
}
pub type TransactionId = U256;

#[service]
impl<VftClient> VFTManagerService<VftClient> // VFTManager service, with the extended-vft client
where VftClient: Vft // We specify the type of the generic type (The client) to be used
{
    // Related function "seed", it will initiate the Service state. IMPORTANT: only call once
    pub fn seed(
        admin: ActorId,
        name: String,
        type_pool: String,
        distribution_mode: String,
        access_type: String,
        participants: Vec<ActorId>,
        vft_contract_id: Option<ActorId>,
        min_tokens_to_add: u128,
        max_tokens_to_burn: u128,
        tokens_per_vara: u128,
        

    ) {
        unsafe {
            VFT_MANAGER_STATE = Some(
                VFTManagerState {
                    admins: vec![admin],
                    name,
                    type_pool,
                    distribution_mode,
                    access_type,
                    participants,
                    vft_contract_id,
                    min_tokens_to_add,
                    max_tokens_to_burn,
                    tokens_per_vara,
                    transactions: HashMap::new(), // Inicializa como un nuevo mapa vacío

                    transaction_count: U256::from(0), // Inicializa el contador en 0

                }
            );
        };
    }

    // Related function "new", returns a new Struct Service instance
    pub fn new(
        vft_client: VftClient
    ) -> Self {
        Self {
            vft_client
        }
    }
    pub fn add_transaction(
        &mut self,
        destination: ActorId,
        value: u128,
    ) -> TransactionId {
        let state = self.state_mut();
    
        let transaction_id = state.transaction_count;
        let transaction = Transaction { // Usa el tipo correcto
            destination,
            value,
            executed: false,
        };
    
        state.transactions.insert(transaction_id, transaction); // Inserta en el estado
        state.transaction_count += U256::from(1); // Incrementa el contador
    
        transaction_id
    }
    // ## Add new a new admin
    // Only admins can add others admins
    pub fn add_admin(&mut self, new_admin_address: ActorId) -> VFTManagerEvents {
        let state = self.state_mut();
        let caller = msg::source();

        if !state.is_admin(&caller) {
            return VFTManagerEvents::Error(
                VFTManagerErrors::OnlyAdminsCanDoThatAction
            );
        }

        state.admins.push(new_admin_address);

        VFTManagerEvents::NewAdminAdded(new_admin_address)
    }

      
    pub fn distribution_pool_balance(&mut self) {
        let state = self.state_mut();
        let _caller = msg::source();
    
        let participants: Vec<ActorId> = state.participants.iter().cloned().collect();
    
        if participants.is_empty() {
            return;
        }
        let total_value = exec::value_available();

        let distributable_value = total_value - 1;
    
        let value_per_participant = distributable_value / participants.len() as u128;
    
        // Verificar si el valor por participante es válido
        if value_per_participant == 0 {
            return;
        }
    
        // Distribuir los tokens
        for participant in participants {
            let _transaction_id =state.add_transaction(
                participant,
                value_per_participant,
            );
        
        }
    
    }
    pub async fn distribution(&mut self) {
        let state = self.state_mut();
        let caller = msg::source();
    
        if !state.is_admin(&caller) {
            panic!("Only admins can initiate distribution.");
        }
    
        let participants = state.participants.clone();
    
        if participants.is_empty() {
            panic!("No participants to distribute to.");
        }
    
        // Verificar si el vft_contract_id está configurado
        let vft_contract_id = match state.vft_contract_id {
            Some(id) => id,
            None => panic!("VFT contract ID is not set."),
        };
    
        // Obtener el total disponible en la pool nativa (VARA)
        let total_value = exec::value_available();
        if total_value == 0 {
            panic!("No VARA available for distribution.");
        }
    
        // Acumular balances de participantes
        let mut total_supply: U256 = U256::zero();
        let mut balances: HashMap<ActorId, U256> = HashMap::new();
    
        for participant in participants.iter() {
            let balance = self
                .vft_client
                .balance_of(*participant)
                .recv(vft_contract_id)
                .await
                .expect("Failed to get participant balance");
            
            total_supply += balance;
            balances.insert(*participant, balance);
        }
    
        if total_supply.is_zero() {
            panic!("Total supply is zero.");
        }
    
        // Distribuir proporcionalmente
        for (participant, balance) in balances {
            let share = U256::from(total_value) * balance / total_supply;
    
            if share > U256::zero() {
                // Registrar transacción
                let transaction_id = state.add_transaction(participant, share.as_u128());
    
            }
        }
    }
    
    pub fn add_participant(&mut self, participant: ActorId) -> VFTManagerEvents {
        let state = self.state_mut();
        state.participants.push(participant);
        VFTManagerEvents::NewParticipant(participant)
    }

    // ## Change vft contract id
    // Only the contract admins can perform this action
    pub fn set_vft_contract_id(&mut self, vft_contract_id: ActorId) -> VFTManagerEvents {
        let state = self.state_mut();
        let caller = msg::source();

        if !state.is_admin(&caller) {
            return VFTManagerEvents::Error(
                VFTManagerErrors::OnlyAdminsCanDoThatAction
            );
        }

        state.vft_contract_id = Some(vft_contract_id);

        VFTManagerEvents::VFTContractIdSet
    }

    // ## Change the max number of tokens to burn to the contract
    // Only the contract admins can perform this action
    pub fn set_max_tokens_to_burn(&mut self, max_tokens_to_burn: u128) -> VFTManagerEvents {
        let state = self.state_mut();
        let caller = msg::source();

        if !state.is_admin(&caller) {
            return VFTManagerEvents::Error(
                VFTManagerErrors::OnlyAdminsCanDoThatAction
            );
        }

        state.max_tokens_to_burn = max_tokens_to_burn;

        VFTManagerEvents::MaxTokensToBurnSet
    }

    // ## Change the minimum number of tokens to add to the contract
    // Only the contract admins can perform this action
    pub fn set_min_tokens_to_add(&mut self, min_tokens_to_add: u128) -> VFTManagerEvents {
        let state = self.state_mut();
        let caller = msg::source();

        if !state.is_admin(&caller) {
            return VFTManagerEvents::Error(
                VFTManagerErrors::OnlyAdminsCanDoThatAction
            );
        }

        state.min_tokens_to_add = min_tokens_to_add;

        VFTManagerEvents::MinTokensToAddSet
    }

    // ## Change the number of tokens to exchange for one VARA
    // Only the contract admins can perform this action
    pub fn set_tokens_per_vara(&mut self, tokens_per_vara: u128) -> VFTManagerEvents {
        let state = self.state_mut();
        let caller = msg::source();

        if !state.is_admin(&caller) {
            return VFTManagerEvents::Error(
                VFTManagerErrors::OnlyAdminsCanDoThatAction
            );
        }

        state.tokens_per_vara = tokens_per_vara;

        VFTManagerEvents::SetTokensPerVaras
    }

    // ## Add an amount of tokens to the vft contract for this contract
    // Only the contract admins can perform this action
    pub async fn add_tokens_to_contract(&mut self, tokens_to_add: u128) ->  VFTManagerEvents {
        let state = self.state_mut();
        let caller = msg::source();

        if !state.is_admin(&caller) {
            return VFTManagerEvents::Error(
                VFTManagerErrors::OnlyAdminsCanDoThatAction
            );
        }

        if state.vft_contract_id.is_none() {
            return VFTManagerEvents::Error(
                VFTManagerErrors::VftContractIdNotSet
            );
        }

        if tokens_to_add < state.min_tokens_to_add {
            return VFTManagerEvents::Error(
                VFTManagerErrors::MinTokensToAdd(state.min_tokens_to_add)
            );
        }

        let result = self
            .add_num_of_tokens_to_contract(
                tokens_to_add, 
                state.vft_contract_id.unwrap()
            )
            .await;

        if let Err(error_variant) = result {
            return VFTManagerEvents::Error(error_variant);
        }

        VFTManagerEvents::TokensAdded
    }

    // ## Burn an amount of tokens to the vft contract for this contract
    // Only the contract admins can perform this action
    pub async fn burn_tokens_from_contract(&mut self, tokens_to_burn: u128) -> VFTManagerEvents {
        let state = self.state_mut();
        let caller = msg::source();

        if !state.is_admin(&caller) {
            return VFTManagerEvents::Error(
                VFTManagerErrors::OnlyAdminsCanDoThatAction
            );
        }

        if state.vft_contract_id.is_none() {
            return VFTManagerEvents::Error(
                VFTManagerErrors::VftContractIdNotSet
            );
        }

        if tokens_to_burn < state.max_tokens_to_burn {
            return VFTManagerEvents::Error(
                VFTManagerErrors::MaxTokensToBurn(state.max_tokens_to_burn)
            );
        }

        let response = self.vft_client
            .total_supply()
            .recv(state.vft_contract_id.unwrap())
            .await;

        let Ok(total_contract_suply) = response else {
            return VFTManagerEvents::Error(
                VFTManagerErrors::VftContractIdNotSet
            );  
        };

        if total_contract_suply < U256::from(tokens_to_burn) {
            return VFTManagerEvents::Error(
                VFTManagerErrors::InsufficientTokens { 
                    total_contract_suply: total_contract_suply.as_u128(), 
                    tokens_to_burn 
                }
            )
        }

        let result = self
            .burn_num_of_tokens(tokens_to_burn, state.vft_contract_id.unwrap())
            .await;

        if let Err(error) = result {
            return VFTManagerEvents::Error(error);
        }

        VFTManagerEvents::TokensBurned
    }

    // ## Swap Varas for tokens
    // Receive a certain amount of varas and then make a swap for a certain number of tokens
    // Command reply is a helper struct that can send tokens in the response from the contract
    pub async fn swap_tokens_by_num_of_varas(&mut self) ->  CommandReply<VFTManagerEvents> {
        let value = msg::value();
        let caller = msg::source();

        // Check if value is zero
        if value == 0 {
            // VFTManagerEvents::Error(
            //     VFTManagerErrors::CantSwapTokensWithAmount {
            //         min_amount: 1,
            //         actual_amount: 0
            //     }
            // )
            return CommandReply::new(
                VFTManagerEvents::Error(
                    VFTManagerErrors::CantSwapTokensWithAmount { 
                        min_amount: 1, 
                        actual_amount: 0 
                    }
                )
            );
        }

        let state = self.state_ref();

        // If vft contract id is None, notify to the user
        if state.vft_contract_id.is_none() {
            return CommandReply::new(
                VFTManagerEvents::Error(
                    VFTManagerErrors::VftContractIdNotSet
                )
            );
        }

        // We converted to a more manageable format (1 = 1_000_000_000_000)
        let num_of_tvaras = value / ONE_TVARA;
        let tokens: u128 = num_of_tvaras * state.tokens_per_vara;

        let total_tokens_supply = self
            .vft_client // Set the client
            .balance_of(exec::program_id()) // Set the method to call
            .recv(state.vft_contract_id.unwrap()) // Call the method and wait for response
            .await; 

        // Get the total suply from caller
        let Ok(total_suply) = total_tokens_supply else {
            return CommandReply::new(
                VFTManagerEvents::Error(
                    VFTManagerErrors::ErrorInVFTContract
                )
            );
        };

        // If contract total suply is not enough, then return the tokens to the user
        if total_suply < U256::from(tokens) {
            // msg::send(
            //     msg::source(),
            //     VFTManagerEvents::RefundOfVaras(num_of_tvaras), 
            //     value
            // )
            // .expect("Error sending message");

            return CommandReply::new(
                VFTManagerEvents::RefundOfVaras(num_of_tvaras)
            ).with_value(value);
        }

        let response = self
            .vft_client
            .transfer(caller, U256::from(tokens))
            .send_recv(state.vft_contract_id.unwrap())
            .await;

        let Ok(transfer_status) = response else {
            return CommandReply::new(
                VFTManagerEvents::Error(
                    VFTManagerErrors::ErrorInVFTContract
                )
            );
        };

        if !transfer_status {
            return CommandReply::new(
                VFTManagerEvents::Error(
                    VFTManagerErrors::OperationWasNotPerformed
                )
            );
        }

        CommandReply::new(
            VFTManagerEvents::TokensSwapSuccessfully {
                total_tokens: tokens,
                total_varas: num_of_tvaras
            }
        )
    }

    /// ## Swap tokens for Varas
    /// CommandReply is a helper struct that can bind tokens to the response of the contract 
    pub async fn swap_tokens_to_varas(&mut self, amount_of_tokens: u128) -> CommandReply<VFTManagerEvents> {
        let state = self.state_ref();

        // If the specified tokens are not greater than the value 
        // of a vara, the user is notified
        if amount_of_tokens < state.tokens_per_vara {
            return CommandReply::new(
                VFTManagerEvents::Error(
                    VFTManagerErrors::CantSwapTokensWithAmount {
                        min_amount: state.tokens_per_vara,
                        actual_amount: amount_of_tokens
                    }
                )
            );
        } 

        // Get the total tokens to send (example, 100 / 50 = 2 VARAS)
        let varas_to_send = amount_of_tokens / state.tokens_per_vara;
        // Get the real amount of tokens from user 
        // If it sends 105, but a VARA cost 100, the real amount value is 100
        let amount_of_tokens = varas_to_send * state.tokens_per_vara;
        // Get the total tokens to swap in 256 type
        let total_tokens_to_swap: U256 = U256::from(amount_of_tokens);
        let caller = msg::source();

        // If vft contract is None, notify to the user
        if state.vft_contract_id.is_none() {
            return CommandReply::new(
                VFTManagerEvents::Error(
                    VFTManagerErrors::VftContractIdNotSet
                )
            );
        }

        let response = self
            .vft_client // Set the client
            .balance_of(caller) // Set the method to call from the client
            .recv(state.vft_contract_id.unwrap()) // Send and wait for response
            .await;

        // Check if the response was successfull
        let Ok(user_total_tokens) = response else {
            return CommandReply::new(
                VFTManagerEvents::Error(
                    VFTManagerErrors::ErrorInVFTContract
                )
            );
        };

        // Check if the user has enough tokens
        if user_total_tokens < total_tokens_to_swap {
            return CommandReply::new(
                VFTManagerEvents::Error(
                    VFTManagerErrors::CantSwapUserTokens { 
                        user_tokens: user_total_tokens, 
                        tokens_to_swap: total_tokens_to_swap 
                    }
                )
            );
        }

        let response = self
            .vft_client // Set the client to call
            .burn(caller, total_tokens_to_swap) // Set the method
            .send_recv(state.vft_contract_id.unwrap()) // Call the client and wait for response
            .await;

        // Check if the response was successfully
        let Ok(operation_result) = response else {
            return CommandReply::new(
                VFTManagerEvents::Error(
                    VFTManagerErrors::ErrorInVFTContract
                )
            );
        };

        // Check if the operation was performed
        if !operation_result {
            return CommandReply::new(
                VFTManagerEvents::Error(
                    VFTManagerErrors::OperationWasNotPerformed
                )
            );
        }

        // Remove tokens from user and return that tokens to the contract
        let result = self   
            .add_num_of_tokens_to_contract(
                amount_of_tokens, 
                state.vft_contract_id.unwrap()
            )
            .await;

        // Check if the operation was successful
        if let Err(error_variant) = result {
            return CommandReply::new(
                VFTManagerEvents::Error(error_variant)
            );
        }

        // msg::send(
        //     caller, 
        //     VFTManagerEvents::TotalSwapInVaras(varas_to_send), 
        //     varas_to_send * ONE_TVARA
        // )
        // .expect("Error sending message");

        CommandReply::new(
            VFTManagerEvents::TokensSwapSuccessfully { 
                total_tokens: amount_of_tokens, 
                total_varas: varas_to_send 
            }
        ).with_value(varas_to_send * ONE_TVARA)
    }

    /// ## Varas stored in contract
    pub fn contract_total_varas_stored(&self) -> VFTManagerQueryEvents {
        VFTManagerQueryEvents::ContractBalanceInVaras(exec::value_available() / ONE_TVARA)
    }

    /// ## Returns the total number of tokens in the contract (In U256 format)
    pub async fn total_tokens_to_swap(&self) -> VFTManagerQueryEvents {
        let state = self.state_mut();

        if state.vft_contract_id.is_none() {
            return VFTManagerQueryEvents::Error(
                VFTManagerErrors::VftContractIdNotSet
            );
        }

        let response = self
            .vft_client
            .balance_of(exec::program_id())
            .recv(state.vft_contract_id.unwrap())
            .await;

        let Ok(total_tokens_to_swap) = response else {
            return VFTManagerQueryEvents::Error(
                VFTManagerErrors::ErrorInVFTContract
            );
        };

        VFTManagerQueryEvents::TotalTokensToSwap(total_tokens_to_swap)
    }
    /// ## Returns the total number of tokens in the contract (In U256 format)
    /// Additionally, it returns all transactions with their execution status.
    pub async fn rewards(&self) -> VFTManagerQueryEvents {
        let state = self.state_ref();
        // Obtener transacciones con su estado
        let transactions: Vec<(TransactionId, Transaction, bool)> = state
            .transactions
            .iter()
            .map(|(id, tx)| (*id, tx.clone(), tx.executed))
            .collect();

        VFTManagerQueryEvents::Rewards (
            transactions
        )
    }
    pub fn pending_rewards(&self, address: ActorId) -> VFTManagerQueryEvents {
        let state = self.state_ref();
    
        // Filtrar transacciones no ejecutadas para el `address` proporcionado
        let pending_transactions: Vec<Transaction> = state
            .transactions
            .values()
            .filter(|tx| tx.destination == address && !tx.executed)
            .cloned() // Clonar cada `Transaction` para recolectarlo como un `Transaction`
            .collect();
    
        // Calcular el balance acumulado
        let total_rewards: u128 = pending_transactions.iter().map(|tx| tx.value).sum();
    
        VFTManagerQueryEvents::PendingRewards {
            address,
            total_rewards,
            transactions: pending_transactions,
        }
    }
    pub fn rewards_claimed(&mut self, address: ActorId) -> VFTManagerEvents {
        let state = self.state_mut();
    
        // Filtrar transacciones no ejecutadas para el `address` proporcionado
        let pending_transactions: Vec<(TransactionId, u128)> = state
            .transactions
            .iter_mut()
            .filter_map(|(id, tx)| {
                if tx.destination == address && !tx.executed {
                    // Devuelve el ID y el valor si la transacción es pendiente
                    Some((*id, tx.value))
                } else {
                    None
                }
            })
            .collect();
    
        // Calcular el balance acumulado
        let total_rewards: u128 = pending_transactions.iter().map(|(_, value)| *value).sum();
    
        if total_rewards == 0 {
            return VFTManagerEvents::Error(VFTManagerErrors::NoPendingRewards);
        }
    
        // Realizar el envío de las recompensas
        if let Err(_) = msg::send(address, VFTManagerEvents::RewardsClaimed { total_rewards }, total_rewards) {
            return VFTManagerEvents::Error(VFTManagerErrors::FailedToSendRewards);
        }
    
        // Marcar las transacciones como ejecutadas
        for (id, _) in pending_transactions {
            if let Some(tx) = state.transactions.get_mut(&id) {
                tx.executed = true;
            }
        }
    
        VFTManagerEvents::RewardsClaimed { total_rewards }
    }
    

    /// ## Returns the total number of tokens in the contract (In u128 format)
    pub async fn total_tokens_to_swap_as_u128(&self) -> VFTManagerQueryEvents {
        let state = self.state_mut();

        if state.vft_contract_id.is_none() {
            return VFTManagerQueryEvents::Error(
                VFTManagerErrors::VftContractIdNotSet
            );
        }

        let response = self
            .vft_client
            .balance_of(exec::program_id())
            .recv(state.vft_contract_id.unwrap())
            .await;

        let Ok(total_tokens_to_swap) = response else {
            return VFTManagerQueryEvents::Error(
                VFTManagerErrors::ErrorInVFTContract
            );
        };

        VFTManagerQueryEvents::TotalTokensToSwapAsU128(total_tokens_to_swap.as_u128())
    }

    /// ## get the amount of tokens to be able to change to one VARA
    pub fn tokens_to_swap_one_vara(&self) -> VFTManagerQueryEvents {
        VFTManagerQueryEvents::TokensToSwapOneVara(self.state_ref().tokens_per_vara)
    }
    pub fn pool_details(&self) -> VFTManagerQueryEvents {
        let state = self.state_ref();
    
        let transactions: Vec<(TransactionId, Transaction)> = state
            .transactions
            .iter()
            .map(|(id, tx)| (*id, tx.clone()))
            .collect();
    
        VFTManagerQueryEvents::PoolDetails {
            admins: state.admins.clone(),
            name: state.name.clone(),
            type_pool: state.type_pool.clone(),
            distribution_mode: state.distribution_mode.clone(),
            access_type: state.access_type.clone(),
            participants: state.participants.clone(),
            vft_contract_id: state.vft_contract_id,
            min_tokens_to_add: state.min_tokens_to_add,
            max_tokens_to_burn: state.max_tokens_to_burn,
            tokens_per_vara: state.tokens_per_vara,
            transaction_count: state.transaction_count,
            transactions,
        }
    }
    
    

    /// ## Send an amount of tokens to the vft contract
    async fn add_num_of_tokens_to_contract(&mut self, tokens_to_add: u128, vft_contract_id: ActorId) -> Result<(), VFTManagerErrors> {
        let response = self
            .vft_client
            .mint(
                exec::program_id(), 
                U256::from(tokens_to_add)
            )
            .send_recv(vft_contract_id)
            .await;

        let Ok(operation_result) = response else {
            return Err(VFTManagerErrors::ErrorInVFTContract);
        };

        if !operation_result {
            return Err(VFTManagerErrors::OperationWasNotPerformed);
        }

        Ok(())
    }

    // ## Burn an amount of tokens to the vft contract
    async fn burn_num_of_tokens(&mut self, tokens_to_burn: u128, vft_contract_id: ActorId) -> Result<(), VFTManagerErrors> {
        let response = self.vft_client
            .burn(
                exec::program_id(), 
                U256::from(tokens_to_burn)
            )
            .send_recv(vft_contract_id)
            .await;

        let Ok(result) = response else {
            return Err(VFTManagerErrors::ErrorInVFTContract);
        };

        if !result {
            return Err(VFTManagerErrors::OperationWasNotPerformed)
        }

        Ok(())
    }

    fn state_mut(&self) -> &'static mut VFTManagerState {
        let state = unsafe { VFT_MANAGER_STATE.as_mut() };
        debug_assert!(state.is_none(), "state is not started!");
        unsafe { state.unwrap_unchecked() }
    }

    fn state_ref(&self) -> &'static VFTManagerState {
        let state = unsafe { VFT_MANAGER_STATE.as_ref() };
        debug_assert!(state.is_none(), "state is not started!");
        unsafe { state.unwrap_unchecked() }
    }
  
    
    
}


#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum VFTManagerQueryEvents {
    ContractBalanceInVaras(u128),
    PoolDetails {
        admins: Vec<ActorId>,
        name: String,
        type_pool: String,
        distribution_mode: String,
        access_type: String,
        participants: Vec<ActorId>,
        vft_contract_id: Option<ActorId>,
        min_tokens_to_add: u128,
        max_tokens_to_burn: u128,
        tokens_per_vara: u128,
        transaction_count: U256,
        transactions: Vec<(TransactionId, Transaction)>,
    },

    PendingRewards {
        address: ActorId,
        total_rewards: u128,
        transactions: Vec<Transaction>, // Las transacciones no ejecutadas del `address`
    },
    Rewards(Vec<(TransactionId, Transaction,bool)>),

    UserTotalTokensAsU128(u128),
    UserTotalTokens(U256),
    TotalTokensToSwap(U256),
    TotalTokensToSwapAsU128(u128),
    TokensToSwapOneVara(u128),
    NumOfTokensForOneVara(u128),
    Error(VFTManagerErrors)
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum VFTManagerEvents {
    NewAdminAdded(ActorId),
    NewParticipant(ActorId),
    RefundOfVaras(u128),
    VFTContractIdSet,
    MinTokensToAddSet,
    MaxTokensToBurnSet,
    TokensAdded,
    TokensBurned,
    SetTokensPerVaras,
    TotalSwapInVaras(u128),
    TokensSwapSuccessfully {
        total_tokens: u128,
        total_varas: u128
    },
    RewardsClaimed {
        total_rewards: u128,
    },
    Error(VFTManagerErrors)
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum VFTManagerErrors {
    MinTokensToAdd(u128),
    NoPendingRewards,
    FailedToSendRewards,

    MaxTokensToBurn(u128),
    InsufficientTokens {
        total_contract_suply: u128,
        tokens_to_burn: u128
    },
    CantSwapTokens {
        tokens_in_vft_contract: U256
    }, 
    CantSwapUserTokens {
        user_tokens: U256,
        tokens_to_swap: U256
    },
    ContractCantMint,
    CantSwapTokensWithAmount {
        min_amount: u128,
        actual_amount: u128
    },
    OnlyAdminsCanDoThatAction,
    VftContractIdNotSet,
    ErrorInVFTContract,
    ErrorInGetNumOfVarasToSwap,
    OperationWasNotPerformed
}
