use gstd::{msg, prelude::*, ActorId};

#[derive(Default)]
pub struct LogicProxy {
    logic: Option<ActorId>,
    owner: ActorId,
}

impl LogicProxy {
    pub fn init_logic(&mut self, logic: ActorId) {
        self.ensure_owner();
        self.logic = Some(logic);
    }

    pub fn init_logic_and_call(&mut self, logic: ActorId, data: Vec<u8>) {
        self.init_logic(logic);
        msg::send_bytes(logic, data).expect("Failed to send message");
    }

    pub fn get_logic_address(&self) -> Option<ActorId> {
        self.logic
    }

    pub fn is_owner(&self, user: ActorId) -> bool {
        self.owner == user
    }

    pub fn transfer_owner(&mut self, new_owner: ActorId) {
        self.ensure_owner();
        self.owner = new_owner;
    }

    fn ensure_owner(&self) {
        assert_eq!(self.owner, msg::source(), "Caller is not the owner");
    }
}
