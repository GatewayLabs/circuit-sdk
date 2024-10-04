# Cryptomata: The Encrypted Computer

[![Crates.io](https://img.shields.io/crates/v/cryptomata.svg)](https://crates.io/crates/cryptomata)
[![Docs.rs](https://docs.rs/cryptomata/badge.svg)](https://docs.rs/cryptomata)
[![CI](https://github.com/Gateway-DAO/cryptomata/workflows/CI/badge.svg)](https://github.com/Gateway-DAO/cryptomata/actions)

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install cryptomata`

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Design

Designing the "encrypted computer" as described in 0xparc's [Programmable Cryptography 1](https://0xparc.org/blog/programmable-cryptography-1) blog post, based on garbled circuits for computation in leiu of FHE, zk-SNARKs (Zero-Knowledge Succinct Non-Interactive Arguments of Knowledge) for inputs/outputs, and Oblivious RAM for intermediate state would combine several advanced cryptographic techniques. Here's how we can structure it:

### 1. **Garbled Circuits as the CPU**
   Garbled circuits allow computation to be performed on encrypted data, where only the result is revealed. In the context of a CPU, this can serve as the computational engine that operates on encrypted inputs and produces encrypted outputs. The idea is to take the logic gates (AND, OR, XOR, etc.) of a traditional CPU and convert them into garbled circuits. The key steps are:

   - **Circuit Construction**: Define the CPU as a circuit of logic gates. For example, you would garble each ALU (Arithmetic Logic Unit) operation (addition, multiplication, bitwise ops) and control flow operations (branches, conditionals, etc.).
   - **Garbled Gate Execution**: When the user inputs encrypted data, each gate (AND, OR, XOR) is garbled, meaning that the inputs and outputs are encrypted and only the correct inputs can "unlock" the correct outputs.
   - **Circuit Evaluation**: The garbled circuit is evaluated by the party without knowing the actual values of the inputs, producing an encrypted output.

### 2. **Zero-Knowledge Proofs for Input and Output Validation**
   The input/output system could be based on zk-SNARKs, allowing the user to submit inputs and receive outputs in a way that guarantees correctness without revealing any information about the inputs or outputs themselves. ZK proofs can be generated and verified in constant time, making them ideal for validating inputs and outputs in a CPU model.

   - **Input Encoding via zk-SNARKs**: 
     - The user constructs a proof that their input is valid (e.g., falls within expected ranges or adheres to certain rules).
     - The proof is passed along with the garbled input into the CPU circuit.
     - The circuit evaluates the proof and ensures that the computation is valid without revealing the raw input.

   - **Output Proof**:
     - After the garbled circuit has completed execution, the output is encrypted.
     - A zk-SNARK is generated to prove that the output corresponds to valid inputs and valid execution of the circuit without revealing the specific data. This proof can be verified by other parties, ensuring the correct computation while preserving privacy.

### 3. **Oblivious RAM for Intermediate State**
   The intermediate state of the computation needs to be protected as well, so using Oblivious RAM (ORAM) for memory operations is essential. ORAM hides the access patterns, ensuring that even if an attacker observes the memory accesses, they cannot infer the actual operations taking place.

   - **ORAM for Memory Access**: 
     - Whenever the CPU needs to read or write from memory, it does so through an ORAM protocol. This ensures that the location of the memory access is obscured, and no one can track how the data is being used or modified.
     - The ORAM structure can be integrated into the CPU's memory controller, ensuring that all memory accesses (even temporary ones during intermediate states) are hidden.
   
   - **Interaction with Garbled Circuits**: 
     - Whenever a garbled circuit computation needs to store or load intermediate data, it interacts with the ORAM system.
     - The ORAM will manage encrypted memory blocks, ensuring no one can determine which memory cells are being accessed.

### High-Level Design Components:
1. **Input/Output Processing Unit (I/O-PU)**:
   - **ZK Input Handling**: Takes inputs and transforms them into garbled values that the CPU can process while verifying their correctness using zk-SNARKs.
   - **ZK Output Handling**: After computation, it transforms the garbled outputs back into zk-SNARK verifiable outputs.

2. **Garbled Circuit CPU**:
   - **ALU in Garbled Form**: Executes basic operations (ADD, MUL, XOR) using garbled circuits.
   - **Control Flow in Garbled Form**: Handles branches, loops, and function calls using encrypted inputs and outputs.
   - **Memory Controller**: Accesses intermediate data through the Oblivious RAM.

3. **Memory Subsystem (Oblivious RAM)**:
   - **Encrypted Memory Accesses**: Manages encrypted memory and ensures all access patterns are hidden.
   - **Intermediate State Storage**: Stores the intermediate state of the CPU’s execution, ensuring that no memory location is accessed in a way that leaks information.

4. **Proof Verification Unit (ZK-PVU)**:
   - **ZK-SNARK Verifier**: Verifies proofs generated from the input/output handling unit to ensure the inputs/outputs are valid without revealing the underlying data.
   - **Intermediate Proofs for Computation Integrity**: Ensures the CPU performed the computation correctly by verifying the execution process itself (e.g., proof of correct execution).

### How it All Fits Together:
- **Initialization**: The user generates garbled inputs and zk-SNARKs proving that the inputs are valid. These inputs are fed into the garbled circuit CPU.
- **Execution**: The garbled circuit CPU operates on these inputs. All intermediate states are stored in encrypted form in ORAM, ensuring that access patterns to memory are not leaked.
- **Finalization**: The garbled circuit completes the computation, producing encrypted outputs. zk-SNARKs are generated to prove that the output corresponds to a correct execution of the garbled circuit.
- **Result**: The user or external verifier can check the zk-SNARK to confirm the computation’s integrity without revealing any of the sensitive data.

### Benefits:
- **Fully Encrypted Computation**: The entire computation is hidden from external observers, including inputs, outputs, and intermediate states.
- **Input/Output Privacy**: zk-SNARKs ensure that inputs and outputs are valid without revealing the actual data.
- **Memory Access Privacy**: Oblivious RAM ensures that intermediate states and memory access patterns are completely private.

references:

https://0xparc.org/blog/programmable-cryptography-1
