# **Dob-SC**

Dob-SC is a project designed for [brief description of the project's purpose].

---

## **Prerequisites**

Ensure you have the following tools installed before starting:

1. **Xcode**  
   - Download and install Xcode from: [Apple Developer](https://developer.apple.com/xcode/).

2. **Rust and Rustup**  
   - Install Rust and its version manager: [rustup.rs](https://rustup.rs/).

3. **WASM Target**  
   - Add the `wasm32-unknown-unknown` target:  
     Refer to the official documentation: [Rustup Toolchains](https://rust-lang.github.io/rustup/concepts/toolchains.html#toolchain-specification).

---

## **Building the WASM File**

Run the following command to build the project in its optimized release mode:

```bash
cargo build --release

```

## **Testing the WASM File**

```bash
cargo test
```
