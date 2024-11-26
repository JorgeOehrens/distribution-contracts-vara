#![no_std]

use sails_rs::prelude::*;

struct VftService(());

#[sails_rs::service]
impl VftService {
    pub fn new() -> Self {
        Self(())
    }

    // Service's method (command)
    pub fn do_something(&mut self) -> String {
        "Hello from Vft!".to_string()
    }

    // Service's query
    pub fn get_something(&self) -> String {
        "Hello from Vft!".to_string()
    }    
}

pub struct VftProgram(());

#[sails_rs::program]
impl VftProgram {
    // Program's constructor
    pub fn new() -> Self {
        Self(())
    }

    // Exposed service
    pub fn vft(&self) -> VftService {
        VftService::new()
    }
}
