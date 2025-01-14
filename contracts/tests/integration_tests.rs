#[cfg(test)]
mod tests {
    use pool_client::PoolFactory;
    use extended_vft_client::Vft;
    use sails_rs::prelude::*;

    #[tokio::test]
    async fn test_combined_operations() {
        let remoting = MockRemoting::new();
        let mut pool = PoolFactory::new(remoting.clone());
        let mut vft = Vft::new(remoting.clone());

        let user: ActorId = [1; 32].into();
        let pool_actor: ActorId = [2; 32].into();

        // Mint tokens
        vft.mint(user, 1000).call().await.unwrap();

        // Transfer and deposit in one step
        vft.transfer(pool_actor, 500).call().await.unwrap();
        pool.add_tokens(pool_actor, user, 500).call().await.unwrap();

        // Verify balances
        assert_eq!(vft.balance_of(user).query().await.unwrap(), 500);
        assert_eq!(vft.balance_of(pool_actor).query().await.unwrap(), 500);
    }
}
