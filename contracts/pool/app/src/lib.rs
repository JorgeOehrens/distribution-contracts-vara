#![no_std]

use sails_rs::{
    prelude::*,
    gstd::{
        calls::GStdRemoting,
        msg
    }
};

pub mod services;
pub mod states;
pub mod clients;

// use services::mini_dexs_service::MiniDexsService;
use services::vft_manager_service::VFTManagerService;
use clients::extended_vft_client::Vft as VftClient;

// use states::receiver_state::ReceiverState;

#[derive(Default)]
pub struct VFTManagerProgram;

#[program]
impl VFTManagerProgram {
    pub fn new() -> Self {
        VFTManagerService::<VftClient<GStdRemoting>>::seed(
            msg::source(), 
            "".to_string(),                  // Convierte `&str` a `String`
            "".to_string(),
            "".to_string(),
            "".to_string(),
            Vec::new(),
            None,
            Vec::new(),
            0,
            true,
            0,
            0

        );

        Self
    }

    pub fn new_with_data(
        name: String,
        type_pool : String,
        distribution_mode : String,
        access_type : String,
        participants: Vec<ActorId>,
        vft_contract_id: Option<ActorId>,
        admins: Vec<ActorId>,
        last_distribution_time: u64, // Última distribución realizada
        is_manual: bool,
        period: u64,
        interval: u64 //Interval for x perdio (minutes, hours , anual)
    ) -> Self {
        VFTManagerService::<VftClient<GStdRemoting>>::seed(
            msg::source(), 
            name,
            type_pool,
            distribution_mode,
            access_type,
            participants,
            vft_contract_id,
            admins,
            last_distribution_time,
            is_manual,
            period,
            interval
        );

        Self
    }
   

    #[route("VFTManager")]
    pub fn vft_manager_svc(&self) -> VFTManagerService<VftClient<GStdRemoting>> {
        let vft_client = VftClient::new(GStdRemoting);
        VFTManagerService::new(vft_client)
    }
}