# keypair
A simple key pair generator for secp256k1 ECC built using Rust.

## Prerequisites
- Rust and Cargo installed. You can install them from [rustup.rs](https://rustup.rs/).

## Running the Project

1. **Clone the repository:**
   ```sh
   git clone https://github.com/avinashega/keypair-secp256k1.git
   cd keypair-secp256k1```

2. **Build the project:**
  `cargo build`

3. **Run the project:**
  `cargo run`

4. Run the benchmarks:
  `cargo bench`

# Example Output
When you run the project, you should see output similar to the following:
  ```
  Private Key: 0x...
  Uncompressed Public Key: 0x...
  Address: 0x...
  ```