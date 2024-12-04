use gstd::{msg, prelude::*, ActorId};
use hashbrown::HashMap;

#[derive(Default)]
pub struct DistributionPool {
    pool_data: String,
    addresses: [Option<ActorId>; 4],
    vars: [u128; 8],
    state_variable_uint256: HashMap<u128, u128>,
    state_variable_address: HashMap<u128, ActorId>,
    state_variable_token_uint256: HashMap<(u128, ActorId), u128>,
    state_variable_token_user_uint256: HashMap<(u128, ActorId, ActorId), u128>,
    regression_params: Option<(u128, u128)>,
    commission: Option<(u128, u128)>,
    key_strings: HashMap<String, String>,
    goal_amount: HashMap<ActorId, u128>,
    distribution_dates: HashMap<ActorId, (u128, u128, u128, u128)>,
    external_tokens: HashMap<ActorId, (u128, u128, u128, u128)>,
}

impl DistributionPool {
    // Initializer
    pub fn initialize(
        &mut self,
        pool_data: String,
        addresses: [Option<ActorId>; 4],
        vars: [u128; 8],
    ) {
        self.pool_data = pool_data;
        self.addresses = addresses;
        self.vars = vars;
    }

    // Distribution
    pub fn can_distribute(&self, token: ActorId) -> bool {
        self.goal_amount.contains_key(&token)
    }

    pub fn distribute(&mut self, user_list: Vec<ActorId>, token: ActorId) {
        for user in user_list {
            if let Some(amount) = self.state_variable_token_user_uint256.get(&(0, token, user)) {
                // Logic for transferring or recording distributions
                msg::send(user, amount, 0).expect("Failed to distribute");
            }
        }
    }

    // Deposits
    pub fn deposit(&mut self) {
        // Example deposit logic
        let value = msg::load_bytes().expect("Failed to load value");
        self.vars[0] += value.len() as u128;
    }

    pub fn deposit_prepay(&mut self) {
        // Logic for prepay deposit
        let value = msg::load_bytes().expect("Failed to load prepay value");
        self.vars[1] += value.len() as u128;
    }

    // Withdraw
    pub fn withdraw_token(&mut self, token: ActorId) {
        if let Some(amount) = self.state_variable_token_uint256.get(&(0, token)) {
            msg::send(msg::source(), *amount, 0).expect("Failed to withdraw token");
        }
    }

    pub fn withdraw_token_commissions(&mut self, token: ActorId) {
        if let Some(amount) = self.state_variable_token_uint256.get(&(1, token)) {
            msg::send(msg::source(), *amount, 0).expect("Failed to withdraw commissions");
        }
    }

    pub fn withdraw_prepay(&mut self) {
        // Logic for withdrawing prepays
        let amount = self.vars[1];
        msg::send(msg::source(), amount, 0).expect("Failed to withdraw prepay");
        self.vars[1] = 0;
    }

    // Getters
    pub fn get_state_variable_uint256(&self, key_type: u128) -> Option<u128> {
        self.state_variable_uint256.get(&key_type).cloned()
    }

    pub fn get_state_variable_address(&self, key_type: u128) -> Option<ActorId> {
        self.state_variable_address.get(&key_type).cloned()
    }

    pub fn get_user_amounts(&self, user_address: ActorId, token: ActorId) -> Option<u128> {
        self.state_variable_token_user_uint256
            .get(&(0, token, user_address))
            .cloned()
    }

    pub fn get_pool_version() -> &'static str {
        "1.0.0"
    }

    // Setters
    pub fn set_operational_address(&mut self, new_operational: ActorId) {
        self.addresses[0] = Some(new_operational);
    }

    pub fn set_regression_params(&mut self, coef: u128, intercept: u128) {
        self.regression_params = Some((coef, intercept));
    }

    pub fn set_key_string(&mut self, key: String, value: String) {
        self.key_strings.insert(key, value);
    }

    pub fn set_goal_amount(&mut self, new_goal: u128, token: ActorId) {
        self.goal_amount.insert(token, new_goal);
    }

    // Configure Tokens
    pub fn add_external_token(&mut self, new_token: ActorId) {
        self.external_tokens
            .insert(new_token, (0, 0, 0, 0)); // Default config
    }

    pub fn add_external_token_with_config(
        &mut self,
        new_token: ActorId,
        first_distribution_date: u128,
        n_distributions: u128,
        distribution_interval: u128,
        goal_amount: u128,
    ) {
        self.external_tokens.insert(
            new_token,
            (
                first_distribution_date,
                n_distributions,
                distribution_interval,
                goal_amount,
            ),
        );
    }
}
