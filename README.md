# GVM: The Encrypted Computer

[![Crates.io](https://img.shields.io/crates/v/cryptomata.svg)](https://crates.io/crates/cryptomata)
[![Docs.rs](https://docs.rs/cryptomata/badge.svg)](https://docs.rs/cryptomata)
[![CI](https://github.com/Gateway-DAO/cryptomata/workflows/CI/badge.svg)](https://github.com/Gateway-DAO/cryptomata/actions)

## Building and running an encrypted function

Building an encrypted function is as easy as creating a regular rust function with primitive types, decorating it with the `encrypted` macro and running it from main, or another function. This example automatically starts up an embedded evaluator, compiles the `access_content` function into an encrypted circuit and submits the circuits with inputs via regular calls to the function.

Run this example with 

```
cargo run --release example age_content_control
```

```rust
#[encrypted(execute)]
fn access_content(age: u8) -> u8 {
    // Access level codes
    let RESTRICTED = 1; // Restricted access for underage users
    let FULL = 2; // Full access for adult users
    let LIMITED = 3; // Limited access for senior users
    let INVALID = 0; // Invalid age code

    // Determine access level based on age ranges
    match age {
        1..=17 => RESTRICTED, // Users aged 1-17
        18..=65 => FULL,      // Users aged 18-65
        66..=120 => LIMITED,  // Users aged 66-120
        _ => INVALID,         // Age outside expected bounds
    }
}

fn main() {
    // Test cases to check access level functionality

    let age = 25_u8;
    let access_level = access_content(age);
    println!("Access Level: {}", access_level); // Expected output: 2 (Full)

    let age = 15_u8;
    let access_level = access_content(age);
    println!("Access Level: {}", access_level); // Expected output: 1 (Restricted)

    let age = 70_u8;
    let access_level = access_content(age);
    println!("Access Level: {}", access_level); // Expected output: 3 (Limited)

    let age = 125_u8;
    let access_level = access_content(age);
    println!("Access Level: {}", access_level); // Expected output: 0 (Invalid)
}
```

(For an example of running two parties p2p, see the [server](https://github.com/Gateway-DAO/gvm/blob/main/server/) crate.)

## Design

![gvm](https://github.com/user-attachments/assets/752003e7-1a50-4446-9870-cdbb2da98498)

The "Gateway Virtual Machine" (GVM), is an encrypted computer based on **authenticated garbled circuits**. It enables users to process encrypted data while ensuring both privacy and verifiability without the need for additional zero-knowledge proofs (such as SNARKs) for inputs and outputs. Here’s a breakdown of each component and how the authenticated garbled circuit scheme functions:

1. **User Encrypted Inputs**:
   - Users provide **encrypted inputs** to the GVM. These inputs are encrypted at the source, maintaining privacy throughout the computation process. Since the GVM is based on authenticated garbled circuits, it can verify these encrypted inputs directly, ensuring their integrity without requiring SNARKs for input validation.

2. **Decryption Circuit**:
   - The **Decryption Circuit** takes the encrypted inputs and decrypts them securely within the GVM. This circuit operates using **public keys** or other public parameters. The decrypted values are strictly isolated within the GVM environment, ensuring controlled access to plaintext data for the next stages of computation.

3. **Contract Circuit (Authenticated Garbled Circuit)**:
   - The **Contract Circuit** represents the main logic processor within the GVM. This circuit executes the program or contract code using **authenticated garbled circuits**—a cryptographic scheme that provides both privacy and verifiability. Since inputs are already verified through this scheme, there is no need for additional proof systems (like SNARKs) to ensure the authenticity of the inputs.
   - The authenticated garbled circuit model allows the Contract Circuit to compute on encrypted data, maintaining confidentiality while guaranteeing that all operations and outputs are consistent and verifiable. The **Gas Meter** connected to the **Contract Circuit** tracks the computational resources used, similar to gas fees in blockchain, to manage resource consumption.

4. **Encryption/Re-encryption Circuit**:
   - After the Contract Circuit finishes its computation, the **Encryption/Re-encryption Circuit** re-encrypts the results. This circuit operates with **private keys**, ensuring that the outputs are securely re-encrypted for storage or transmission. Since the scheme uses authenticated garbled circuits, the outputs are also verified as part of the garbling process, eliminating the need for SNARKs to verify output correctness.

5. **Encrypted State**:
   - The **Encrypted State** stores persistent encrypted data within the GVM, preserving it across sessions or computations. This state can be securely updated based on the results from the **Contract Circuit** and remains encrypted to maintain privacy. The authenticated garbled circuits allow the GVM to manage and verify state changes without additional proof requirements.

6. **Outputs (Plaintext and Encrypted)**:
   - The GVM produces two types of outputs:
     - **Plaintext Outputs**: Decrypted results that may be returned directly to the user or shared with authorized parties. These outputs are verified by the authenticated garbled circuit model, ensuring they are valid without requiring SNARKs.
     - **Encrypted Outputs**: Results that are re-encrypted for secure storage or further transmission. These may be used to update the **Encrypted State** or returned to the user in an encrypted format.

### Key Features of the Authenticated Garbled Circuit Scheme in the GVM
- **Verifiable Computation**: Authenticated garbled circuits provide intrinsic verification of both inputs and outputs, removing the need for SNARKs or other external proofs for correctness.
- **Confidentiality and Integrity**: The garbled circuit model ensures data remains confidential throughout, while also guaranteeing that only legitimate, authorized computations are performed on the data.
- **Efficient Processing**: By integrating verification directly into the garbled circuit operations, the GVM achieves both secure and efficient encrypted computation, with minimized overhead from external proof systems.

### Benefits:
- **Fully Encrypted Computation**: The entire computation is hidden from external observers, including inputs, outputs, and intermediate states.
- **Input/Output Privacy**: data is fully encrypted going into and coming out of the machine.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
