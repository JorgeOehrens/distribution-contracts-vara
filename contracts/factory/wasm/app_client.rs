// Code generated by sails-client-gen. DO NOT EDIT.
#[allow(unused_imports)]
use sails_rs::collections::BTreeMap;
#[allow(unused_imports)]
use sails_rs::{
    calls::{Activation, Call, Query, Remoting, RemotingAction},
    prelude::*,
    String,
};
pub struct AppFactory<R> {
    #[allow(dead_code)]
    remoting: R,
}

impl<R> AppFactory<R> {
    #[allow(unused)]
    pub fn new(remoting: R) -> Self {
        Self { remoting }
    }
}

impl<R: Remoting + Clone> traits::AppFactory for AppFactory<R> {
    type Args = R::Args;
    fn new(&self, init: InitConfigFactory) -> impl Activation<Args = R::Args> {
        RemotingAction::<_, app_factory::io::New>::new(self.remoting.clone(), init)
    }
}

pub mod app_factory {
    use super::*;
    pub mod io {
        use super::*;
        use sails_rs::calls::ActionIo;
        pub struct New(());

        impl New {
            #[allow(dead_code)]
            pub fn encode_call(init: super::InitConfigFactory) -> Vec<u8> {
                <New as ActionIo>::encode_call(&init)
            }
        }

        impl ActionIo for New {
            const ROUTE: &'static [u8] = &[12, 78, 101, 119];
            type Params = super::InitConfigFactory;
            type Reply = ();
        }
    }
}
pub struct Factory<R> {
    remoting: R,
}

impl<R> Factory<R> {
    pub fn new(remoting: R) -> Self {
        Self { remoting }
    }
}

impl<R: Remoting + Clone> traits::Factory for Factory<R> {
    type Args = R::Args;
    fn add_admin_to_factory(
        &mut self,
        admin_actor_id: ActorId,
    ) -> impl Call<Output = Result<FactoryEvent, FactoryError>, Args = R::Args> {
        RemotingAction::<_, factory::io::AddAdminToFactory>::new(
            self.remoting.clone(),
            admin_actor_id,
        )
    }
    /// Despliegue del contrato Pool
    fn create_pool(
        &mut self,
        init_config: InitConfig,
        vft_address: ActorId,
    ) -> impl Call<Output = Result<ActorId, FactoryError>, Args = R::Args> {
        RemotingAction::<_, factory::io::CreatePool>::new(
            self.remoting.clone(),
            (init_config, vft_address),
        )
    }
    /// Despliegue del contrato VFT
    fn create_vft(
        &mut self,
        init_config: InitConfig,
    ) -> impl Call<Output = Result<ActorId, FactoryError>, Args = R::Args> {
        RemotingAction::<_, factory::io::CreateVft>::new(self.remoting.clone(), init_config)
    }
    /// Despliegue combinado de VFT y Pool
    fn create_vft_and_pool(
        &mut self,
        init_config: InitConfig,
    ) -> impl Call<Output = Result<FactoryEvent, FactoryError>, Args = R::Args> {
        RemotingAction::<_, factory::io::CreateVftAndPool>::new(self.remoting.clone(), init_config)
    }
    fn remove_registry(
        &mut self,
        program_for_id: u64,
    ) -> impl Call<Output = Result<FactoryEvent, FactoryError>, Args = R::Args> {
        RemotingAction::<_, factory::io::RemoveRegistry>::new(self.remoting.clone(), program_for_id)
    }
    fn update_code_id(
        &mut self,
        new_vft_code_id: Option<CodeId>,
        new_pool_code_id: Option<CodeId>,
    ) -> impl Call<Output = Result<FactoryEvent, FactoryError>, Args = R::Args> {
        RemotingAction::<_, factory::io::UpdateCodeId>::new(
            self.remoting.clone(),
            (new_vft_code_id, new_pool_code_id),
        )
    }
    fn update_gas_for_program(
        &mut self,
        new_gas_amount: u64,
    ) -> impl Call<Output = Result<FactoryEvent, FactoryError>, Args = R::Args> {
        RemotingAction::<_, factory::io::UpdateGasForProgram>::new(
            self.remoting.clone(),
            new_gas_amount,
        )
    }
    fn admins(&self) -> impl Query<Output = Vec<ActorId>, Args = R::Args> {
        RemotingAction::<_, factory::io::Admins>::new(self.remoting.clone(), ())
    }
    /// Leer el gas asignado para los programas
    fn gas_for_program(&self) -> impl Query<Output = u64, Args = R::Args> {
        RemotingAction::<_, factory::io::GasForProgram>::new(self.remoting.clone(), ())
    }
    fn id_to_address(&self) -> impl Query<Output = Vec<(u64, ActorId)>, Args = R::Args> {
        RemotingAction::<_, factory::io::IdToAddress>::new(self.remoting.clone(), ())
    }
    fn number(&self) -> impl Query<Output = u64, Args = R::Args> {
        RemotingAction::<_, factory::io::Number>::new(self.remoting.clone(), ())
    }
    /// Leer el `CodeId` actual del Pool
    fn pool_code_id(&self) -> impl Query<Output = CodeId, Args = R::Args> {
        RemotingAction::<_, factory::io::PoolCodeId>::new(self.remoting.clone(), ())
    }
    /// Leer el registro completo
    fn registry(&self) -> impl Query<Output = Vec<(ActorId, Vec<(u64, Record)>)>, Args = R::Args> {
        RemotingAction::<_, factory::io::Registry>::new(self.remoting.clone(), ())
    }
    /// Leer el `CodeId` actual del VFT
    fn vft_code_id(&self) -> impl Query<Output = CodeId, Args = R::Args> {
        RemotingAction::<_, factory::io::VftCodeId>::new(self.remoting.clone(), ())
    }
}

pub mod factory {
    use super::*;

    pub mod io {
        use super::*;
        use sails_rs::calls::ActionIo;
        pub struct AddAdminToFactory(());

        impl AddAdminToFactory {
            #[allow(dead_code)]
            pub fn encode_call(admin_actor_id: ActorId) -> Vec<u8> {
                <AddAdminToFactory as ActionIo>::encode_call(&admin_actor_id)
            }
        }

        impl ActionIo for AddAdminToFactory {
            const ROUTE: &'static [u8] = &[
                28, 70, 97, 99, 116, 111, 114, 121, 68, 65, 100, 100, 65, 100, 109, 105, 110, 84,
                111, 70, 97, 99, 116, 111, 114, 121,
            ];
            type Params = ActorId;
            type Reply = Result<super::FactoryEvent, super::FactoryError>;
        }
        pub struct CreatePool(());

        impl CreatePool {
            #[allow(dead_code)]
            pub fn encode_call(init_config: super::InitConfig, vft_address: ActorId) -> Vec<u8> {
                <CreatePool as ActionIo>::encode_call(&(init_config, vft_address))
            }
        }

        impl ActionIo for CreatePool {
            const ROUTE: &'static [u8] = &[
                28, 70, 97, 99, 116, 111, 114, 121, 40, 67, 114, 101, 97, 116, 101, 80, 111, 111,
                108,
            ];
            type Params = (super::InitConfig, ActorId);
            type Reply = Result<ActorId, super::FactoryError>;
        }
        pub struct CreateVft(());

        impl CreateVft {
            #[allow(dead_code)]
            pub fn encode_call(init_config: super::InitConfig) -> Vec<u8> {
                <CreateVft as ActionIo>::encode_call(&init_config)
            }
        }

        impl ActionIo for CreateVft {
            const ROUTE: &'static [u8] = &[
                28, 70, 97, 99, 116, 111, 114, 121, 36, 67, 114, 101, 97, 116, 101, 86, 102, 116,
            ];
            type Params = super::InitConfig;
            type Reply = Result<ActorId, super::FactoryError>;
        }
        pub struct CreateVftAndPool(());

        impl CreateVftAndPool {
            #[allow(dead_code)]
            pub fn encode_call(init_config: super::InitConfig) -> Vec<u8> {
                <CreateVftAndPool as ActionIo>::encode_call(&init_config)
            }
        }

        impl ActionIo for CreateVftAndPool {
            const ROUTE: &'static [u8] = &[
                28, 70, 97, 99, 116, 111, 114, 121, 64, 67, 114, 101, 97, 116, 101, 86, 102, 116,
                65, 110, 100, 80, 111, 111, 108,
            ];
            type Params = super::InitConfig;
            type Reply = Result<super::FactoryEvent, super::FactoryError>;
        }
        pub struct RemoveRegistry(());

        impl RemoveRegistry {
            #[allow(dead_code)]
            pub fn encode_call(program_for_id: u64) -> Vec<u8> {
                <RemoveRegistry as ActionIo>::encode_call(&program_for_id)
            }
        }

        impl ActionIo for RemoveRegistry {
            const ROUTE: &'static [u8] = &[
                28, 70, 97, 99, 116, 111, 114, 121, 56, 82, 101, 109, 111, 118, 101, 82, 101, 103,
                105, 115, 116, 114, 121,
            ];
            type Params = u64;
            type Reply = Result<super::FactoryEvent, super::FactoryError>;
        }
        pub struct UpdateCodeId(());

        impl UpdateCodeId {
            #[allow(dead_code)]
            pub fn encode_call(
                new_vft_code_id: Option<CodeId>,
                new_pool_code_id: Option<CodeId>,
            ) -> Vec<u8> {
                <UpdateCodeId as ActionIo>::encode_call(&(new_vft_code_id, new_pool_code_id))
            }
        }

        impl ActionIo for UpdateCodeId {
            const ROUTE: &'static [u8] = &[
                28, 70, 97, 99, 116, 111, 114, 121, 48, 85, 112, 100, 97, 116, 101, 67, 111, 100,
                101, 73, 100,
            ];
            type Params = (Option<CodeId>, Option<CodeId>);
            type Reply = Result<super::FactoryEvent, super::FactoryError>;
        }
        pub struct UpdateGasForProgram(());

        impl UpdateGasForProgram {
            #[allow(dead_code)]
            pub fn encode_call(new_gas_amount: u64) -> Vec<u8> {
                <UpdateGasForProgram as ActionIo>::encode_call(&new_gas_amount)
            }
        }

        impl ActionIo for UpdateGasForProgram {
            const ROUTE: &'static [u8] = &[
                28, 70, 97, 99, 116, 111, 114, 121, 76, 85, 112, 100, 97, 116, 101, 71, 97, 115,
                70, 111, 114, 80, 114, 111, 103, 114, 97, 109,
            ];
            type Params = u64;
            type Reply = Result<super::FactoryEvent, super::FactoryError>;
        }
        pub struct Admins(());

        impl Admins {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <Admins as ActionIo>::encode_call(&())
            }
        }

        impl ActionIo for Admins {
            const ROUTE: &'static [u8] = &[
                28, 70, 97, 99, 116, 111, 114, 121, 24, 65, 100, 109, 105, 110, 115,
            ];
            type Params = ();
            type Reply = Vec<ActorId>;
        }
        pub struct GasForProgram(());

        impl GasForProgram {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <GasForProgram as ActionIo>::encode_call(&())
            }
        }

        impl ActionIo for GasForProgram {
            const ROUTE: &'static [u8] = &[
                28, 70, 97, 99, 116, 111, 114, 121, 52, 71, 97, 115, 70, 111, 114, 80, 114, 111,
                103, 114, 97, 109,
            ];
            type Params = ();
            type Reply = u64;
        }
        pub struct IdToAddress(());

        impl IdToAddress {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <IdToAddress as ActionIo>::encode_call(&())
            }
        }

        impl ActionIo for IdToAddress {
            const ROUTE: &'static [u8] = &[
                28, 70, 97, 99, 116, 111, 114, 121, 44, 73, 100, 84, 111, 65, 100, 100, 114, 101,
                115, 115,
            ];
            type Params = ();
            type Reply = Vec<(u64, ActorId)>;
        }
        pub struct Number(());

        impl Number {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <Number as ActionIo>::encode_call(&())
            }
        }

        impl ActionIo for Number {
            const ROUTE: &'static [u8] = &[
                28, 70, 97, 99, 116, 111, 114, 121, 24, 78, 117, 109, 98, 101, 114,
            ];
            type Params = ();
            type Reply = u64;
        }
        pub struct PoolCodeId(());

        impl PoolCodeId {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <PoolCodeId as ActionIo>::encode_call(&())
            }
        }

        impl ActionIo for PoolCodeId {
            const ROUTE: &'static [u8] = &[
                28, 70, 97, 99, 116, 111, 114, 121, 40, 80, 111, 111, 108, 67, 111, 100, 101, 73,
                100,
            ];
            type Params = ();
            type Reply = CodeId;
        }
        pub struct Registry(());

        impl Registry {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <Registry as ActionIo>::encode_call(&())
            }
        }

        impl ActionIo for Registry {
            const ROUTE: &'static [u8] = &[
                28, 70, 97, 99, 116, 111, 114, 121, 32, 82, 101, 103, 105, 115, 116, 114, 121,
            ];
            type Params = ();
            type Reply = Vec<(ActorId, Vec<(u64, super::Record)>)>;
        }
        pub struct VftCodeId(());

        impl VftCodeId {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <VftCodeId as ActionIo>::encode_call(&())
            }
        }

        impl ActionIo for VftCodeId {
            const ROUTE: &'static [u8] = &[
                28, 70, 97, 99, 116, 111, 114, 121, 36, 86, 102, 116, 67, 111, 100, 101, 73, 100,
            ];
            type Params = ();
            type Reply = CodeId;
        }
    }
}
#[derive(PartialEq, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct InitConfigFactory {
    pub vft_code_id: CodeId,
    pub pool_code_id: CodeId,
    pub factory_admin_account: Vec<ActorId>,
    pub gas_for_program: u64,
}
#[derive(PartialEq, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum FactoryEvent {
    ProgramCreated {
        id: u64,
        vft_address: ActorId,
        pool_address: ActorId,
        init_config: InitConfig,
    },
    GasUpdatedSuccessfully {
        updated_by: ActorId,
        new_gas_amount: u64,
    },
    CodeIdUpdatedSuccessfully {
        updated_by: ActorId,
        new_code_id: CodeId,
    },
    AdminAdded {
        updated_by: ActorId,
        admin_actor_id: ActorId,
    },
    RegistryRemoved {
        removed_by: ActorId,
        program_for_id: u64,
    },
}
#[derive(PartialEq, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct InitConfig {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub type_pool: String,
    pub distribution_mode: String,
    pub access_type: String,
    pub participants: Vec<ActorId>,
}
#[derive(PartialEq, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum FactoryError {
    ProgramInitializationFailed,
    ProgramInitializationFailedWithContext(String),
    Unauthorized,
    UnexpectedFTEvent,
    MessageSendError,
    NotFound,
    IdNotFoundInAddress,
    IdNotFound,
}
#[derive(PartialEq, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Record {
    pub name: String,
}

pub mod traits {
    use super::*;
    #[allow(dead_code)]
    pub trait AppFactory {
        type Args;
        #[allow(clippy::new_ret_no_self)]
        #[allow(clippy::wrong_self_convention)]
        fn new(&self, init: InitConfigFactory) -> impl Activation<Args = Self::Args>;
    }

    #[allow(clippy::type_complexity)]
    pub trait Factory {
        type Args;
        fn add_admin_to_factory(
            &mut self,
            admin_actor_id: ActorId,
        ) -> impl Call<Output = Result<FactoryEvent, FactoryError>, Args = Self::Args>;
        fn create_pool(
            &mut self,
            init_config: InitConfig,
            vft_address: ActorId,
        ) -> impl Call<Output = Result<ActorId, FactoryError>, Args = Self::Args>;
        fn create_vft(
            &mut self,
            init_config: InitConfig,
        ) -> impl Call<Output = Result<ActorId, FactoryError>, Args = Self::Args>;
        fn create_vft_and_pool(
            &mut self,
            init_config: InitConfig,
        ) -> impl Call<Output = Result<FactoryEvent, FactoryError>, Args = Self::Args>;
        fn remove_registry(
            &mut self,
            program_for_id: u64,
        ) -> impl Call<Output = Result<FactoryEvent, FactoryError>, Args = Self::Args>;
        fn update_code_id(
            &mut self,
            new_vft_code_id: Option<CodeId>,
            new_pool_code_id: Option<CodeId>,
        ) -> impl Call<Output = Result<FactoryEvent, FactoryError>, Args = Self::Args>;
        fn update_gas_for_program(
            &mut self,
            new_gas_amount: u64,
        ) -> impl Call<Output = Result<FactoryEvent, FactoryError>, Args = Self::Args>;
        fn admins(&self) -> impl Query<Output = Vec<ActorId>, Args = Self::Args>;
        fn gas_for_program(&self) -> impl Query<Output = u64, Args = Self::Args>;
        fn id_to_address(&self) -> impl Query<Output = Vec<(u64, ActorId)>, Args = Self::Args>;
        fn number(&self) -> impl Query<Output = u64, Args = Self::Args>;
        fn pool_code_id(&self) -> impl Query<Output = CodeId, Args = Self::Args>;
        fn registry(
            &self,
        ) -> impl Query<Output = Vec<(ActorId, Vec<(u64, Record)>)>, Args = Self::Args>;
        fn vft_code_id(&self) -> impl Query<Output = CodeId, Args = Self::Args>;
    }
}
