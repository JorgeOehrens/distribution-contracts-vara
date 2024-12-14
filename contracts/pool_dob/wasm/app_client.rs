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
    fn new(
        &self,
        name: String,
        type_pool: String,
        distribution_mode: String,
        access_type: String,
        owners: Vec<ActorId>,
        participants_pool: Vec<ActorId>,
        required: u32,
    ) -> impl Activation<Args = R::Args> {
        RemotingAction::<_, app_factory::io::New>::new(
            self.remoting.clone(),
            (
                name,
                type_pool,
                distribution_mode,
                access_type,
                owners,
                participants_pool,
                required,
            ),
        )
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
            pub fn encode_call(
                name: String,
                type_pool: String,
                distribution_mode: String,
                access_type: String,
                owners: Vec<ActorId>,
                participants_pool: Vec<ActorId>,
                required: u32,
            ) -> Vec<u8> {
                <New as ActionIo>::encode_call(&(
                    name,
                    type_pool,
                    distribution_mode,
                    access_type,
                    owners,
                    participants_pool,
                    required,
                ))
            }
        }

        impl ActionIo for New {
            const ROUTE: &'static [u8] = &[12, 78, 101, 119];
            type Params = (
                String,
                String,
                String,
                String,
                Vec<ActorId>,
                Vec<ActorId>,
                u32,
            );
            type Reply = ();
        }
    }
}
pub struct Pool<R> {
    remoting: R,
}

impl<R> Pool<R> {
    pub fn new(remoting: R) -> Self {
        Self { remoting }
    }
}

impl<R: Remoting + Clone> traits::Pool for Pool<R> {
    type Args = R::Args;
    fn add_owner(&mut self, owner: ActorId) -> impl Call<Output = (), Args = R::Args> {
        RemotingAction::<_, pool::io::AddOwner>::new(self.remoting.clone(), owner)
    }
    fn add_participant(&mut self, participant: ActorId) -> impl Call<Output = (), Args = R::Args> {
        RemotingAction::<_, pool::io::AddParticipant>::new(self.remoting.clone(), participant)
    }
    fn change_required_confirmations_count(
        &mut self,
        count: u32,
    ) -> impl Call<Output = (), Args = R::Args> {
        RemotingAction::<_, pool::io::ChangeRequiredConfirmationsCount>::new(
            self.remoting.clone(),
            count,
        )
    }
    fn confirm_transaction(
        &mut self,
        transaction_id: U256,
    ) -> impl Call<Output = (), Args = R::Args> {
        RemotingAction::<_, pool::io::ConfirmTransaction>::new(
            self.remoting.clone(),
            transaction_id,
        )
    }
    fn distribution_pool(
        &mut self,
        data: Vec<u8>,
        total_value: u128,
        description: Option<String>,
    ) -> impl Call<Output = (), Args = R::Args> {
        RemotingAction::<_, pool::io::DistributionPool>::new(
            self.remoting.clone(),
            (data, total_value, description),
        )
    }
    fn distribution_pool_2(
        &mut self,
        participants: Vec<ActorId>,
        data: Vec<u8>,
        total_value: u128,
        description: Option<String>,
    ) -> impl Call<Output = (), Args = R::Args> {
        RemotingAction::<_, pool::io::DistributionPool2>::new(
            self.remoting.clone(),
            (participants, data, total_value, description),
        )
    }
    fn distribution_pool_balance(
        &mut self,
        data: Vec<u8>,
        description: Option<String>,
    ) -> impl Call<Output = (), Args = R::Args> {
        RemotingAction::<_, pool::io::DistributionPoolBalance>::new(
            self.remoting.clone(),
            (data, description),
        )
    }
    fn execute_transaction(
        &mut self,
        transaction_id: U256,
    ) -> impl Call<Output = (), Args = R::Args> {
        RemotingAction::<_, pool::io::ExecuteTransaction>::new(
            self.remoting.clone(),
            transaction_id,
        )
    }
    fn remove_owner(&mut self, owner: ActorId) -> impl Call<Output = (), Args = R::Args> {
        RemotingAction::<_, pool::io::RemoveOwner>::new(self.remoting.clone(), owner)
    }
    fn replace_owner(
        &mut self,
        old_owner: ActorId,
        new_owner: ActorId,
    ) -> impl Call<Output = (), Args = R::Args> {
        RemotingAction::<_, pool::io::ReplaceOwner>::new(
            self.remoting.clone(),
            (old_owner, new_owner),
        )
    }
    fn revoke_confirmation(
        &mut self,
        transaction_id: U256,
    ) -> impl Call<Output = (), Args = R::Args> {
        RemotingAction::<_, pool::io::RevokeConfirmation>::new(
            self.remoting.clone(),
            transaction_id,
        )
    }
    fn submit_transaction(
        &mut self,
        destination: ActorId,
        data: Vec<u8>,
        value: u128,
        description: Option<String>,
    ) -> impl Call<Output = (), Args = R::Args> {
        RemotingAction::<_, pool::io::SubmitTransaction>::new(
            self.remoting.clone(),
            (destination, data, value, description),
        )
    }
    fn get_state(&self) -> impl Query<Output = State, Args = R::Args> {
        RemotingAction::<_, pool::io::GetState>::new(self.remoting.clone(), ())
    }
}

pub mod pool {
    use super::*;

    pub mod io {
        use super::*;
        use sails_rs::calls::ActionIo;
        pub struct AddOwner(());

        impl AddOwner {
            #[allow(dead_code)]
            pub fn encode_call(owner: ActorId) -> Vec<u8> {
                <AddOwner as ActionIo>::encode_call(&owner)
            }
        }

        impl ActionIo for AddOwner {
            const ROUTE: &'static [u8] = &[
                16, 80, 111, 111, 108, 32, 65, 100, 100, 79, 119, 110, 101, 114,
            ];
            type Params = ActorId;
            type Reply = ();
        }
        pub struct AddParticipant(());

        impl AddParticipant {
            #[allow(dead_code)]
            pub fn encode_call(participant: ActorId) -> Vec<u8> {
                <AddParticipant as ActionIo>::encode_call(&participant)
            }
        }

        impl ActionIo for AddParticipant {
            const ROUTE: &'static [u8] = &[
                16, 80, 111, 111, 108, 56, 65, 100, 100, 80, 97, 114, 116, 105, 99, 105, 112, 97,
                110, 116,
            ];
            type Params = ActorId;
            type Reply = ();
        }
        pub struct ChangeRequiredConfirmationsCount(());

        impl ChangeRequiredConfirmationsCount {
            #[allow(dead_code)]
            pub fn encode_call(count: u32) -> Vec<u8> {
                <ChangeRequiredConfirmationsCount as ActionIo>::encode_call(&count)
            }
        }

        impl ActionIo for ChangeRequiredConfirmationsCount {
            const ROUTE: &'static [u8] = &[
                16, 80, 111, 111, 108, 128, 67, 104, 97, 110, 103, 101, 82, 101, 113, 117, 105,
                114, 101, 100, 67, 111, 110, 102, 105, 114, 109, 97, 116, 105, 111, 110, 115, 67,
                111, 117, 110, 116,
            ];
            type Params = u32;
            type Reply = ();
        }
        pub struct ConfirmTransaction(());

        impl ConfirmTransaction {
            #[allow(dead_code)]
            pub fn encode_call(transaction_id: U256) -> Vec<u8> {
                <ConfirmTransaction as ActionIo>::encode_call(&transaction_id)
            }
        }

        impl ActionIo for ConfirmTransaction {
            const ROUTE: &'static [u8] = &[
                16, 80, 111, 111, 108, 72, 67, 111, 110, 102, 105, 114, 109, 84, 114, 97, 110, 115,
                97, 99, 116, 105, 111, 110,
            ];
            type Params = U256;
            type Reply = ();
        }
        pub struct DistributionPool(());

        impl DistributionPool {
            #[allow(dead_code)]
            pub fn encode_call(
                data: Vec<u8>,
                total_value: u128,
                description: Option<String>,
            ) -> Vec<u8> {
                <DistributionPool as ActionIo>::encode_call(&(data, total_value, description))
            }
        }

        impl ActionIo for DistributionPool {
            const ROUTE: &'static [u8] = &[
                16, 80, 111, 111, 108, 64, 68, 105, 115, 116, 114, 105, 98, 117, 116, 105, 111,
                110, 80, 111, 111, 108,
            ];
            type Params = (Vec<u8>, u128, Option<String>);
            type Reply = ();
        }
        pub struct DistributionPool2(());

        impl DistributionPool2 {
            #[allow(dead_code)]
            pub fn encode_call(
                participants: Vec<ActorId>,
                data: Vec<u8>,
                total_value: u128,
                description: Option<String>,
            ) -> Vec<u8> {
                <DistributionPool2 as ActionIo>::encode_call(&(
                    participants,
                    data,
                    total_value,
                    description,
                ))
            }
        }

        impl ActionIo for DistributionPool2 {
            const ROUTE: &'static [u8] = &[
                16, 80, 111, 111, 108, 68, 68, 105, 115, 116, 114, 105, 98, 117, 116, 105, 111,
                110, 80, 111, 111, 108, 50,
            ];
            type Params = (Vec<ActorId>, Vec<u8>, u128, Option<String>);
            type Reply = ();
        }
        pub struct DistributionPoolBalance(());

        impl DistributionPoolBalance {
            #[allow(dead_code)]
            pub fn encode_call(data: Vec<u8>, description: Option<String>) -> Vec<u8> {
                <DistributionPoolBalance as ActionIo>::encode_call(&(data, description))
            }
        }

        impl ActionIo for DistributionPoolBalance {
            const ROUTE: &'static [u8] = &[
                16, 80, 111, 111, 108, 92, 68, 105, 115, 116, 114, 105, 98, 117, 116, 105, 111,
                110, 80, 111, 111, 108, 66, 97, 108, 97, 110, 99, 101,
            ];
            type Params = (Vec<u8>, Option<String>);
            type Reply = ();
        }
        pub struct ExecuteTransaction(());

        impl ExecuteTransaction {
            #[allow(dead_code)]
            pub fn encode_call(transaction_id: U256) -> Vec<u8> {
                <ExecuteTransaction as ActionIo>::encode_call(&transaction_id)
            }
        }

        impl ActionIo for ExecuteTransaction {
            const ROUTE: &'static [u8] = &[
                16, 80, 111, 111, 108, 72, 69, 120, 101, 99, 117, 116, 101, 84, 114, 97, 110, 115,
                97, 99, 116, 105, 111, 110,
            ];
            type Params = U256;
            type Reply = ();
        }
        pub struct RemoveOwner(());

        impl RemoveOwner {
            #[allow(dead_code)]
            pub fn encode_call(owner: ActorId) -> Vec<u8> {
                <RemoveOwner as ActionIo>::encode_call(&owner)
            }
        }

        impl ActionIo for RemoveOwner {
            const ROUTE: &'static [u8] = &[
                16, 80, 111, 111, 108, 44, 82, 101, 109, 111, 118, 101, 79, 119, 110, 101, 114,
            ];
            type Params = ActorId;
            type Reply = ();
        }
        pub struct ReplaceOwner(());

        impl ReplaceOwner {
            #[allow(dead_code)]
            pub fn encode_call(old_owner: ActorId, new_owner: ActorId) -> Vec<u8> {
                <ReplaceOwner as ActionIo>::encode_call(&(old_owner, new_owner))
            }
        }

        impl ActionIo for ReplaceOwner {
            const ROUTE: &'static [u8] = &[
                16, 80, 111, 111, 108, 48, 82, 101, 112, 108, 97, 99, 101, 79, 119, 110, 101, 114,
            ];
            type Params = (ActorId, ActorId);
            type Reply = ();
        }
        pub struct RevokeConfirmation(());

        impl RevokeConfirmation {
            #[allow(dead_code)]
            pub fn encode_call(transaction_id: U256) -> Vec<u8> {
                <RevokeConfirmation as ActionIo>::encode_call(&transaction_id)
            }
        }

        impl ActionIo for RevokeConfirmation {
            const ROUTE: &'static [u8] = &[
                16, 80, 111, 111, 108, 72, 82, 101, 118, 111, 107, 101, 67, 111, 110, 102, 105,
                114, 109, 97, 116, 105, 111, 110,
            ];
            type Params = U256;
            type Reply = ();
        }
        pub struct SubmitTransaction(());

        impl SubmitTransaction {
            #[allow(dead_code)]
            pub fn encode_call(
                destination: ActorId,
                data: Vec<u8>,
                value: u128,
                description: Option<String>,
            ) -> Vec<u8> {
                <SubmitTransaction as ActionIo>::encode_call(&(
                    destination,
                    data,
                    value,
                    description,
                ))
            }
        }

        impl ActionIo for SubmitTransaction {
            const ROUTE: &'static [u8] = &[
                16, 80, 111, 111, 108, 68, 83, 117, 98, 109, 105, 116, 84, 114, 97, 110, 115, 97,
                99, 116, 105, 111, 110,
            ];
            type Params = (ActorId, Vec<u8>, u128, Option<String>);
            type Reply = ();
        }
        pub struct GetState(());

        impl GetState {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <GetState as ActionIo>::encode_call(&())
            }
        }

        impl ActionIo for GetState {
            const ROUTE: &'static [u8] = &[
                16, 80, 111, 111, 108, 32, 71, 101, 116, 83, 116, 97, 116, 101,
            ];
            type Params = ();
            type Reply = super::State;
        }
    }

    #[allow(dead_code)]
    #[cfg(not(target_arch = "wasm32"))]
    pub mod events {
        use super::*;
        use sails_rs::events::*;
        #[derive(PartialEq, Debug, Encode, Decode)]
        #[codec(crate = sails_rs::scale_codec)]
        pub enum PoolEvents {
            Confirmation {
                sender: ActorId,
                transaction_id: U256,
            },
            Revocation {
                sender: ActorId,
                transaction_id: U256,
            },
            Submission {
                transaction_id: U256,
            },
            Execution {
                transaction_id: U256,
            },
            OwnerAddition {
                owner: ActorId,
            },
            PaticipantAddition {
                participant: ActorId,
            },
            OwnerRemoval {
                owner: ActorId,
            },
            OwnerReplace {
                old_owner: ActorId,
                new_owner: ActorId,
            },
            RequirementChange(U256),
        }
        impl EventIo for PoolEvents {
            const ROUTE: &'static [u8] = &[16, 80, 111, 111, 108];
            const EVENT_NAMES: &'static [&'static [u8]] = &[
                &[48, 67, 111, 110, 102, 105, 114, 109, 97, 116, 105, 111, 110],
                &[40, 82, 101, 118, 111, 99, 97, 116, 105, 111, 110],
                &[40, 83, 117, 98, 109, 105, 115, 115, 105, 111, 110],
                &[36, 69, 120, 101, 99, 117, 116, 105, 111, 110],
                &[
                    52, 79, 119, 110, 101, 114, 65, 100, 100, 105, 116, 105, 111, 110,
                ],
                &[
                    72, 80, 97, 116, 105, 99, 105, 112, 97, 110, 116, 65, 100, 100, 105, 116, 105,
                    111, 110,
                ],
                &[48, 79, 119, 110, 101, 114, 82, 101, 109, 111, 118, 97, 108],
                &[48, 79, 119, 110, 101, 114, 82, 101, 112, 108, 97, 99, 101],
                &[
                    68, 82, 101, 113, 117, 105, 114, 101, 109, 101, 110, 116, 67, 104, 97, 110,
                    103, 101,
                ],
            ];
            type Event = Self;
        }

        pub fn listener<R: Listener<Vec<u8>>>(remoting: R) -> impl Listener<PoolEvents> {
            RemotingListener::<_, PoolEvents>::new(remoting)
        }
    }
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct State {
    pub name: String,
    pub type_pool: String,
    pub distribution_mode: String,
    pub access_type: String,
    pub transactions: Vec<(U256, Transaction)>,
    pub confirmations: Vec<(U256, Vec<ActorId>)>,
    pub owners: Vec<ActorId>,
    pub participants_pool: Vec<ActorId>,
    pub required: u32,
    pub transaction_count: U256,
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Transaction {
    pub destination: ActorId,
    pub payload: Vec<u8>,
    pub value: u128,
    pub description: Option<String>,
    pub executed: bool,
}

pub mod traits {
    use super::*;
    #[allow(dead_code)]
    pub trait AppFactory {
        type Args;
        #[allow(clippy::new_ret_no_self)]
        #[allow(clippy::wrong_self_convention)]
        fn new(
            &self,
            name: String,
            type_pool: String,
            distribution_mode: String,
            access_type: String,
            owners: Vec<ActorId>,
            participants_pool: Vec<ActorId>,
            required: u32,
        ) -> impl Activation<Args = Self::Args>;
    }

    #[allow(clippy::type_complexity)]
    pub trait Pool {
        type Args;
        fn add_owner(&mut self, owner: ActorId) -> impl Call<Output = (), Args = Self::Args>;
        fn add_participant(
            &mut self,
            participant: ActorId,
        ) -> impl Call<Output = (), Args = Self::Args>;
        fn change_required_confirmations_count(
            &mut self,
            count: u32,
        ) -> impl Call<Output = (), Args = Self::Args>;
        fn confirm_transaction(
            &mut self,
            transaction_id: U256,
        ) -> impl Call<Output = (), Args = Self::Args>;
        fn distribution_pool(
            &mut self,
            data: Vec<u8>,
            total_value: u128,
            description: Option<String>,
        ) -> impl Call<Output = (), Args = Self::Args>;
        fn distribution_pool_2(
            &mut self,
            participants: Vec<ActorId>,
            data: Vec<u8>,
            total_value: u128,
            description: Option<String>,
        ) -> impl Call<Output = (), Args = Self::Args>;
        fn distribution_pool_balance(
            &mut self,
            data: Vec<u8>,
            description: Option<String>,
        ) -> impl Call<Output = (), Args = Self::Args>;
        fn execute_transaction(
            &mut self,
            transaction_id: U256,
        ) -> impl Call<Output = (), Args = Self::Args>;
        fn remove_owner(&mut self, owner: ActorId) -> impl Call<Output = (), Args = Self::Args>;
        fn replace_owner(
            &mut self,
            old_owner: ActorId,
            new_owner: ActorId,
        ) -> impl Call<Output = (), Args = Self::Args>;
        fn revoke_confirmation(
            &mut self,
            transaction_id: U256,
        ) -> impl Call<Output = (), Args = Self::Args>;
        fn submit_transaction(
            &mut self,
            destination: ActorId,
            data: Vec<u8>,
            value: u128,
            description: Option<String>,
        ) -> impl Call<Output = (), Args = Self::Args>;
        fn get_state(&self) -> impl Query<Output = State, Args = Self::Args>;
    }
}
