#![no_std]

use sails_rs::prelude::*;

struct HelloWorldService(());

#[sails_rs::service]
impl HelloWorldService {
    pub fn new() -> Self {
        Self(())
    }

    // Service's method (command)
    pub fn do_something(&mut self) -> String {
        "Hello from HelloWorld!".to_string()
    }

    // Service's query
    pub fn get_something(&self) -> String {
        "Hello from HelloWorld!".to_string()
    }    
}

pub struct HelloWorldProgram(());

#[sails_rs::program]
impl HelloWorldProgram {
    // Program's constructor
    pub fn new() -> Self {
        Self(())
    }

    // Exposed service
    pub fn hello_world(&self) -> HelloWorldService {
        HelloWorldService::new()
    }
}
