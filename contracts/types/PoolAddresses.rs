#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PoolAddresses {
    pub creator: ActorId,      // Dirección del creador
    pub operational: ActorId,  // Dirección operativa
    pub treasury: ActorId,     // Dirección de la tesorería
    pub token: ActorId,        // Dirección del token
    pub storage_address: ActorId, // Dirección del almacenamiento
}


impl PoolAddresses {
    pub fn new(
        creator: ActorId,
        operational: ActorId,
        treasury: ActorId,
        token: ActorId,
        storage_address: ActorId,
    ) -> Self {
        Self {
            creator,
            operational,
            treasury,
            token,
            storage_address,
        }
    }

    pub fn display(&self) {
        println!("Pool Addresses:");
        println!("Creator: {:?}", self.creator);
        println!("Operational: {:?}", self.operational);
        println!("Treasury: {:?}", self.treasury);
        println!("Token: {:?}", self.token);
        println!("Storage: {:?}", self.storage_address);
    }
}
