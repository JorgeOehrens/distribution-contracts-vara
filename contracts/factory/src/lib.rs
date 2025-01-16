#![no_std]

use sails_rs::{
    gstd::{program, route},
    prelude::*,
};

pub mod services;
pub mod states;

use crate::states::state::*;
use services::service::*;

pub struct Program;

#[program]
impl Program {
    pub fn new(init: InitConfigFactory) -> Self {
        Service::init(init);
        Self
    }

    #[route("Factory")]
    pub fn template_svc(&self) -> Service {
        Service::new()
    }
}
