#![no_std]

use sails_rs::prelude::*;

struct DistributionContractsVaraService(());

#[sails_rs::service]
impl DistributionContractsVaraService {
    pub fn new() -> Self {
        Self(())
    }

    // Service's method (command)
    pub fn do_something(&mut self) -> String {
        "Hello from DistributionContractsVara!".to_string()
    }

    // Service's query
    pub fn get_something(&self) -> String {
        "Hello from DistributionContractsVara!".to_string()
    }    
}

pub struct DistributionContractsVaraProgram(());

#[sails_rs::program]
impl DistributionContractsVaraProgram {
    // Program's constructor
    pub fn new() -> Self {
        Self(())
    }

    // Exposed service
    pub fn distribution_contracts_vara(&self) -> DistributionContractsVaraService {
        DistributionContractsVaraService::new()
    }
}
