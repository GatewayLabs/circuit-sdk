# Secure Arithmetic Evaluation with s2n-quic and Gateway Evaluator

This repository provides a secure arithmetic evaluation server and client using **garbled circuits** over **QUIC protocol**. The server and client securely communicate to perform arithmetic operations within a garbled circuit framework. It demonstrates a sample function, `multi_arithmetic`, using the `s2n-quic` library, tracing for logging, and gateway evaluator and garbler components.

## Features

- **Garbled Circuit Evaluation**: Uses a garbled circuit framework to perform arithmetic operations.
- **QUIC Protocol Communication**: Implements secure, low-latency communication between client and server with s2n-quic.
- **Secure Multiparty Computation (MPC)**: Protects sensitive data during evaluation.
- **Logging and Debugging**: Logs key steps in the connection process and computation.

## Prerequisites

## Installation

1. **Clone the repository**:
    ```sh
    git clone https://github.com/Gateway-DAO/priv.git
    cd secure-eval-quic
    ```

2. **Generate Certificates**:
   Place the server's TLS certificate and private key files in `server/certs/` with the names `cert.pem` and `key.pem`. This enables secure communication over QUIC.

3. **Build the Project**:
    ```sh
    cargo build --release
    ```

## Usage

The project contains two main components:

1. **Server**: Hosts the garbled circuit and accepts connections from clients to perform arithmetic operations.
2. **Client**: Initiates the garbling process and interacts with the server to evaluate the circuit.

### Run the Server

```sh
cargo run --bin server
```

The server listens on `127.0.0.1:4433` and logs its status.

### Run the Client

In a separate terminal, start the client:

```sh
cargo run --bin client
```

The client connects to the server, initiates a garbling interaction, and exchanges data with the server. The final result is printed once the garbler completes the evaluation.

### Example Circuit Evaluation

The example circuit, `multi_arithmetic`, performs a sample arithmetic calculation:

```rust
#[circuit(compile)]
fn multi_arithmetic(a: u8, b: u8, c: u8, d: u8) -> u8 {
    let res = a * b;
    let res = res + c;
    res - d
}
```

This sample multiplies `a` and `b`, adds `c`, then subtracts `d`.

## Code Overview

- **Server**:
  - Sets up the QUIC server with TLS, starts accepting connections, and creates an evaluator for the `multi_arithmetic` circuit.
  - The evaluator processes incoming data from the client, computes the circuit’s result, and returns it to the client.

- **Client**:
  - Connects to the server, sets up the garbler for the `multi_arithmetic` circuit, and sends initial data.
  - Receives responses from the server, updates the garbler state, and exchanges messages until evaluation is complete.

### Key Functions

- `multi_arithmetic`: Example circuit function performing a basic arithmetic operation.
- `handle_evaluator_connection`: Manages evaluator interactions on the server side.
- `handle_garbler_connection`: Manages garbler interactions on the client side.

## Logging and Debugging

Logging is set up with `tracing_subscriber`. Key steps, data sizes, and message contents are logged for debugging and understanding the data flow. To adjust the logging level, modify `RUST_LOG` before running:

```sh
RUST_LOG=info cargo run --bin server
```

## Future Improvements

- Implement additional circuit examples and complexity.
- Enhance modular arithmetic for complete garbled circuit operations.
- Add support for dynamic circuit generation.

## License

This repository is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

This README provides a comprehensive overview of the secure evaluation setup over QUIC using garbled circuits. For any questions or issues, please reach out via GitHub Issues.