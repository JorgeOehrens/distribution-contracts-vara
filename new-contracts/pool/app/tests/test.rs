use aplication_builder::{traits::VftManager, VftManager as VftManagerImpl};
use sails_rs::ActorId;

#[tokio::test]
async fn test_basic_function() {
    // Mensaje de prueba para verificar la ejecución
    println!("This is a simple test.");

    // Una afirmación simple para comprobar que el test pasa
    assert_eq!(1 + 1, 2);
}
