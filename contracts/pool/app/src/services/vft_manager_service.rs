#![allow(static_mut_refs)] // Only to avoid the static variable mut reference warning

// Necesary imports
use sails_rs::calls::{ Query};
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
const _ONE_TVARA: u128 = 1e12 as u128; // Value of one TVara and Vara (1_000_000_000_000)

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
        mut admins: Vec<ActorId>

        

    ) {
        let caller = msg::source(); // Obtener el remitente del mensaje

        // Asegurarse de que el remitente siempre sea administrador inicial
        if !admins.contains(&caller) {
            admins.push(caller);
        }
    
        unsafe {
            VFT_MANAGER_STATE = Some(
                VFTManagerState {
                    admins: admins.clone(), // Asigna directamente la lista de administradores
                    name,
                    type_pool,
                    distribution_mode,
                    access_type,
                    participants,
                    vft_contract_id,
                    transactions: HashMap::new(), // Inicializa como un nuevo mapa vacío
                    transaction_count: U256::from(0) // Inicializa el contador en 0
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
                let _transaction_id = state.add_transaction(participant, share.as_u128());
    
            }
        }
    }
    
    pub fn add_participant(&mut self, participant: ActorId) -> VFTManagerEvents {
        let state = self.state_mut();
        state.participants.push(participant);
        VFTManagerEvents::NewParticipant(participant)
    }

    pub fn add_vara(&mut self) -> VFTManagerEvents {
        let state = self.state_mut();

        VFTManagerEvents::AddVara()

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
            transaction_count: state.transaction_count,
            transactions,
        }
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
    AddVara(),

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