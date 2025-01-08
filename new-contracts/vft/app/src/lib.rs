#![no_std]
#![allow(clippy::new_without_default)]

use sails_rs::prelude::*;
mod services;
use services::extended_vft::ExtendedService;

pub type SharesParticipant = U256;

pub struct ExtendedVftProgram(());

#[program]
impl ExtendedVftProgram {
    pub fn new(name: String, symbol: String, decimals: u8) -> Self {
        let mut _service = ExtendedService::seed(name, symbol, decimals);

        Self(())
    }

    pub fn distribute_shares(&self, shares_list: Vec<(ActorId, SharesParticipant)>) -> bool {
        let mut service = ExtendedService::new();
        if !service.distribute_shares(shares_list) {
            panic!("Failed to distribute shares");
        }
        true
    }

    pub fn vft(&self) -> ExtendedService {
        ExtendedService::new()
    }
}
