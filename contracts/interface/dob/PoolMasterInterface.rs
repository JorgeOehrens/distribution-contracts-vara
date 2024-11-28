use gstd::{msg, prelude::*, ActorId};
use hashbrown::HashMap;

#[derive(Default)]
pub struct PoolMaster {
    treasury_pool: Option<ActorId>,
    pool_master_config: Option<ActorId>,
    pools: Vec<Pool>,
}

#[derive(Debug, Clone)]
pub struct Pool {
    users: Vec<ActorId>,
    shares: Vec<u128>,
    time_config: Option<Vec<u128>>, // Para Payroll y Treasury Pools
    goal_amount: Option<u128>,      // Reward y Payroll Pools
    pool_data: String,
    pool_type: String, // "Reward", "Payroll", "Treasury", "MasterTreasury"
}

impl PoolMaster {
    // Getters
    pub fn get_treasury_pool(&self) -> Option<ActorId> {
        self.treasury_pool
    }

    pub fn get_pool_master_config(&self) -> Option<ActorId> {
        self.pool_master_config
    }

    // Setters
    pub fn set_pool_master_config(&mut self, config: ActorId) {
        self.pool_master_config = Some(config);
    }

    // Crear Pools
    pub fn create_reward_pool(
        &mut self,
        users: Vec<ActorId>,
        shares: Vec<u128>,
        goal_amount: u128,
        pool_data: String,
    ) {
        self.create_pool(users, shares, None, Some(goal_amount), pool_data, "Reward".to_string());
    }

    pub fn create_payroll_pool(
        &mut self,
        users: Vec<ActorId>,
        shares: Vec<u128>,
        time_config: Vec<u128>,
        goal_amount: u128,
        pool_data: String,
    ) {
        self.create_pool(
            users,
            shares,
            Some(time_config),
            Some(goal_amount),
            pool_data,
            "Payroll".to_string(),
        );
    }

    pub fn create_treasury_pool(
        &mut self,
        users: Vec<ActorId>,
        shares: Vec<u128>,
        time_config: Vec<u128>,
        pool_data: String,
    ) {
        self.create_pool(
            users,
            shares,
            Some(time_config),
            None,
            pool_data,
            "Treasury".to_string(),
        );
    }

    pub fn create_pool_master_treasury_pool(
        &mut self,
        users: Vec<ActorId>,
        shares: Vec<u128>,
        pool_data: String,
    ) {
        self.create_pool(users, shares, None, None, pool_data, "MasterTreasury".to_string());
    }

    // Inicialización
    pub fn initialize(&mut self, config: ActorId) {
        self.pool_master_config = Some(config);
    }

    // Lógica Interna de Creación de Pools
    fn create_pool(
        &mut self,
        users: Vec<ActorId>,
        shares: Vec<u128>,
        time_config: Option<Vec<u128>>,
        goal_amount: Option<u128>,
        pool_data: String,
        pool_type: String,
    ) {
        assert!(
            users.len() == shares.len(),
            "El número de usuarios y participaciones no coincide"
        );

        let pool = Pool {
            users,
            shares,
            time_config,
            goal_amount,
            pool_data,
            pool_type,
        };

        self.pools.push(pool);
    }
}
