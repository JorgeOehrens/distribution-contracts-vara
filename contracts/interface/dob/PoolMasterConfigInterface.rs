use gstd::{msg, prelude::*, ActorId};
use hashbrown::HashMap;

#[derive(Default)]
pub struct PoolMasterConfig {
    operational_address: Option<ActorId>,
    regression_params: (u128, u128, u128), // coef, intercept, gasPrice
    commission: u128,
    logic_versions: HashMap<u128, (ActorId, String)>, // version => (logic address, logic name)
    latest_version: Option<u128>,
}

impl PoolMasterConfig {
    // Getters
    pub fn get_operational_address(&self) -> Option<ActorId> {
        self.operational_address
    }

    pub fn get_regression_params(&self) -> (u128, u128, u128) {
        self.regression_params
    }

    pub fn get_commission(&self) -> u128 {
        self.commission
    }

    // Setters
    pub fn set_operational_address(&mut self, new_operational: ActorId) {
        self.operational_address = Some(new_operational);
    }

    pub fn set_regression_params(&mut self, new_coef: u128, new_intercept: u128, new_gas_price: u128) {
        self.regression_params = (new_coef, new_intercept, new_gas_price);
    }

    pub fn set_commission(&mut self, commission: u128) {
        self.commission = commission;
    }

    // Deploys (Expected Gas Calculation)
    pub fn expected_total_gas(&self, n_users: u128, n_distributions: u128) -> u128 {
        let (coef, intercept, gas_price) = self.regression_params;
        coef * n_users * n_distributions + intercept * gas_price
    }

    // Logic Proxy Management
    pub fn add_logic_version(&mut self, logic: ActorId, version: u128, logic_name: String) {
        self.logic_versions.insert(version, (logic, logic_name));
        self.latest_version = Some(version);
    }

    pub fn add_logic(&mut self, logic: ActorId, logic_name: String) {
        let version = self.latest_version.unwrap_or(0) + 1;
        self.add_logic_version(logic, version, logic_name);
    }

    pub fn get_logic_version(&self, version: u128) -> Option<(ActorId, String)> {
        self.logic_versions.get(&version).cloned()
    }

    pub fn get_latest_version(&self) -> Option<(ActorId, String)> {
        if let Some(version) = self.latest_version {
            self.get_logic_version(version)
        } else {
            None
        }
    }

    pub fn get_latest_version_number(&self) -> Option<u128> {
        self.latest_version
    }
}
