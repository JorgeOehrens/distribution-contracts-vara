use sails_rs::gtest::{calls::*, System};
use app::Program;



pub const ADMIN_ID: u64 = 10;
pub const USER_ID: [u64; 2] = [11, 12];
#[tokio::test]
async fn test_basic_function() {
    let system = System::new();
    system.init_logger();
    println!("System initialized.");

    // Mensaje de prueba para verificar la ejecución
    println!("Starting test for FACTORY...");

    // Mint balances to admin and predefined users
    system.mint_to(ADMIN_ID, 100_000_000_000_000);
    system.mint_to(USER_ID[0], 100_000_000_000_000);
    system.mint_to(USER_ID[1], 100_000_000_000_000);
    println!("Minted VARA balances for ADMIN_ID and USER_IDs.");

    // Inicializa el programa 
    let program_space = GTestRemoting::new(system, ADMIN_ID.into());
    println!("Program space created.");

    let code_id = program_space
        .system()
        .submit_code_file("../target/wasm32-unknown-unknown/release/wasm.opt.wasm");
    println!("Contract FACTORY submitted with Code ID: {:?}", code_id);


    // Mensaje final
    println!("Test for FACTORY completed successfully.");
}