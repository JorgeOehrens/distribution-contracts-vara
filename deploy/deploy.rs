use std::process::Command;

fn main() {


    println!("Building contract 2...");
    let output_2 = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir("../contracts/vft") 
        .output()
        .expect("Failed to build contract 2");

    if !output_2.status.success() {
        eprintln!("Error building contract 2: {}", String::from_utf8_lossy(&output_2.stderr));
        return;
    }
    println!("Contract vft built successfully!");
}
