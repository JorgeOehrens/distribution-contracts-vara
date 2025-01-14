



#[tokio::test]
mod integration_tests {
    use pool::Pool;
    use vft::Token;

    #[tokio::test]
    async fn test_pool_with_token_interaction() {
        let mut pool = Pool::new();
        let mut token = Token::new();

        token.mint("user1", 100).unwrap();
        token.transfer("user1", "pool", 100).unwrap();

        pool.deposit("user1", 100).unwrap();

        assert_eq!(pool.total_balance(), 100);
        assert_eq!(token.balance_of("user1").unwrap(), 0);
        assert_eq!(token.balance_of("pool").unwrap(), 100);
    }
}
