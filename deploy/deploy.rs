use std::process::Command;

fn main() {
    println!("Building contract 1...");
    let output_1 = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir("../contracts/hello-world") 
        .output()
        .expect("Failed to build contract 1");

    if !output_1.status.success() {
        eprintln!("Error building contract 1: {}", String::from_utf8_lossy(&output_1.stderr));
        return;
    }


    println!("Contract hello-world built successfully!");
}
