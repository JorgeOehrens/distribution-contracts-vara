use extended_vft_client::{
    traits::{ExtendedVftFactory, Vft},
    ExtendedVftFactory as Factory, Vft as VftClient,
};
use sails_rs::calls::*;
use sails_rs::U256;
use sails_rs::gtest::{calls::*, System};
use sails_rs::ActorId;
use serde_json::json;
use std::fs;

pub const ADMIN_ID: u64 = 10;
pub const USER_ID: [u64; 2] = [11, 12];
pub const NUM_USERS: usize = 150;
pub type SharesParticipant = U256;

#[tokio::test]
async fn test_basic_function() {
    let system = System::new();
    system.init_logger();
    println!("System initialized.");

    // Generate user IDs
    let mut user_ids: Vec<u64> = Vec::new();
    for i in 0..NUM_USERS {
        user_ids.push(11 + i as u64); 
    }

    let shares_list: Vec<(ActorId, SharesParticipant)> = user_ids
        .iter()
        .map(|user_id| ((*user_id).into(), sails_rs::U256::from(10))) 
        .collect();
    println!("Shares List User ids: {}", shares_list.len());

    // Mint balances to admin and predefined users
    system.mint_to(ADMIN_ID, 100_000_000_000_000);
    system.mint_to(USER_ID[0], 100_000_000_000_000);
    system.mint_to(USER_ID[1], 100_000_000_000_000);
    println!("Minted VARA balances for ADMIN_ID and USER_IDs.");

    let program_space = GTestRemoting::new(system, ADMIN_ID.into());
    println!("Program space created.");

    let code_id = program_space
        .system()
        .submit_code_file("./target/wasm32-unknown-unknown/release/vft.opt.wasm");
    println!("Contract VTF submitted with Code ID: {:?}", code_id);

    let extended_vft_factory = Factory::new(program_space.clone());
    println!("Extended VFT Factory initialized.");

    let extended_vft_id = extended_vft_factory
        .new("Dob".to_string(), "DB".to_string(), 18)
        .send_recv(code_id, "123")
        .await
        .unwrap();
    println!("Extended VFT created with ID: {:?}", extended_vft_id);

    let mut client = VftClient::new(program_space);
    println!("VFT Client initialized.");

    client.distribute_shares(shares_list.clone())
        .send_recv(extended_vft_id) I 
        .await
        .unwrap();
    println!("Distribute Shares");

    // Mint to admin
    client
        .mint(ADMIN_ID.into(), 1_000.into())
        .send_recv(extended_vft_id)
        .await
        .unwrap();
    println!("Minted 1,000 tokens to ADMIN_ID.");

    // Get token name
    let token_name = client
        .name()
        .recv(extended_vft_id)
        .await
        .unwrap();
    println!("Token Name: {}", token_name);

    // Get balances of all holders (admin + predefined users + generated users)
    let mut all_holders: Vec<u64> = vec![ADMIN_ID];
    all_holders.extend(USER_ID);
    all_holders.extend(user_ids);

    let mut holder_balances = vec![];
    for holder_id in all_holders.iter() {
        let balance = client
            .balance_of((*holder_id).into())
            .recv(extended_vft_id)
            .await
            .unwrap();
        holder_balances.push(json!({
            "user_id": holder_id,
            "balance": balance.to_string()
        }));
    }

    // Save data to JSON
    let output_data = json!({
        "code_id": code_id.to_string(),
        "extended_vft_id": extended_vft_id.to_string(),
        "token_name": token_name,
        "holders": holder_balances
    });

    fs::write("vtf_data.json", serde_json::to_string_pretty(&output_data).unwrap())
        .expect("Unable to write data to vtf_data.json");

    println!("VTF data saved to vtf_data.json.");

    // Check total supply
    let total_tokens_supply = client
        .total_supply()
        .recv(extended_vft_id)
        .await
        .unwrap();
    println!("Total Supply : {:?}", total_tokens_supply);
}
