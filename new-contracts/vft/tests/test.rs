use extended_vft_client::{
    traits::{ExtendedVftFactory, Vft},
    ExtendedVftFactory as Factory, Vft as VftClient,
};
use sails_rs::calls::*;
use sails_rs::U256;
use sails_rs::gtest::{calls::*, System};
use sails_rs::ActorId;
pub const ADMIN_ID: u64 = 10;
pub const USER_ID: [u64; 2] = [11, 12];
pub const NUM_USERS: usize = 150;
pub type SharesParticipant = U256;
#[tokio::test]
async fn test_basic_function() {
    let system = System::new();
    system.init_logger();
    println!("System initialized.");

    let mut user_ids: Vec<u64> = Vec::new();
    for i in 0..NUM_USERS {
        user_ids.push(11 + i as u64); 
    }
    let shares_list: Vec<(ActorId, SharesParticipant)> = user_ids
        .iter()
        .map(|user_id| ((*user_id).into(), sails_rs::U256::from(10))) 
        .collect();
    println!("Shares List User ids: ");

    system.mint_to(ADMIN_ID, 100_000_000_000_000);
    system.mint_to(USER_ID[0], 100_000_000_000_000);
    system.mint_to(USER_ID[1], 100_000_000_000_000);
    println!("Minted VARA balances for ADMIN_ID and USER_IDs." );

    let program_space = GTestRemoting::new(system, ADMIN_ID.into());
    println!("Program space created.");

    let code_id = program_space
        .system()
        .submit_code_file("./target/wasm32-unknown-unknown/release/vft.opt.wasm");
    println!("Contract submitted with Code ID: {:?}", code_id);

    let extended_vft_factory = Factory::new(program_space.clone());
    println!("Extended VFT Factory initialized.");
    
    let extended_vft_id = extended_vft_factory
        .new("name".to_string(), "symbol".to_string(), 10)
        .send_recv(code_id, "123")
        .await
        .unwrap();
    println!("Extended VFT created with ID: {:?}", extended_vft_id);

    let mut client = VftClient::new(program_space);
    println!("VFT Client initialized.");
    
    client.distribute_shares(shares_list)
        .send_recv(extended_vft_id)
        .await
        .unwrap();
    println!("Distribute Shares");
    // mint
    client
        .mint(ADMIN_ID.into(), 1_000.into())
        .send_recv(extended_vft_id)
        .await
        .unwrap();
    println!("Minted 1,000 tokens to ADMIN_ID.");

    // check balance
    let balance = client
        .balance_of(ADMIN_ID.into())
        .recv(extended_vft_id)
        .await
        .unwrap();
    println!("Balance of ADMIN_ID: {:?}", balance);
    assert_eq!(balance, 1_000.into());

    let total_tokens_supply = client
    .total_supply()
    .recv(extended_vft_id)
    .await
    .unwrap();
    println!("Total Supply : {:?}", total_tokens_supply);

}
