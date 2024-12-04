use gstd::{msg, prelude::*, ActorId};
use hashbrown::HashMap;

#[derive(Default)]
pub struct ParticipationToken {
    balances: HashMap<ActorId, u128>,
    paused: bool,
    total_supply: u128,
}

impl ParticipationToken {
    /// Mint tokens for multiple participants with initial supply and shares
    pub fn mint_participants(
        &mut self,
        initial_supply: u128,
        users_address: Vec<ActorId>,
        shares: Vec<u128>,
        pause_token: bool,
    ) {
        // Ensure inputs are valid
        assert!(
            users_address.len() == shares.len(),
            "Users and shares length mismatch"
        );

        // Mint tokens to participants based on shares
        for (user, share) in users_address.iter().zip(shares.iter()) {
            let amount = (initial_supply * share) / 100; // Assuming shares are in percentages
            self.balances
                .entry(*user)
                .and_modify(|balance| *balance += amount)
                .or_insert(amount);
        }

        // Update total supply
        self.total_supply += initial_supply;

        // Set pause state if required
        self.paused = pause_token;
    }

    /// Mint tokens for a single owner
    pub fn mint_single_owner(
        &mut self,
        initial_supply: u128,
        single_participant: ActorId,
        pause_token: bool,
    ) {
        // Mint the entire supply to the single owner
        self.balances
            .entry(single_participant)
            .and_modify(|balance| *balance += initial_supply)
            .or_insert(initial_supply);

        // Update total supply
        self.total_supply += initial_supply;

        // Set pause state if required
        self.paused = pause_token;
    }

    /// Check if the token is paused
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Get the balance of a participant
    pub fn get_balance(&self, participant: ActorId) -> u128 {
        *self.balances.get(&participant).unwrap_or(&0)
    }

    /// Get the total supply of the token
    pub fn get_total_supply(&self) -> u128 {
        self.total_supply
    }

    /// Pause or unpause the token
    pub fn set_pause_state(&mut self, pause: bool) {
        self.paused = pause;
    }
}
