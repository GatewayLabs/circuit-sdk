[![Crates.io](https://img.shields.io/crates/v/cryptomata.svg)](https://crates.io/crates/cryptomata)
[![Docs.rs](https://docs.rs/cryptomata/badge.svg)](https://docs.rs/cryptomata)
[![CI](https://github.com/Gateway-DAO/cryptomata/workflows/CI/badge.svg)](https://github.com/Gateway-DAO/cryptomata/actions)
# Gateway Circuit SDK

The "Gateway Circuit SDK", is an encrypted runtime environment that uses **authenticated garbled circuits**. It enables users to process on encrypted data while ensuring both privacy and verifiability without the need for additional zero-knowledge proofs (such as SNARKs) for inputs and outputs.

![gateway-circuit-sdk](https://github.com/user-attachments/assets/b5051841-97d3-43d8-8b54-681fdaf1cf34)

## Building and running an encrypted function

Building an encrypted function is as easy as creating a regular rust function with primitive types, decorating it with the `encrypted` macro and running it from main, or another function. This example automatically starts up an embedded evaluator, compiles the `access_content` function into an encrypted circuit and submits the circuits with inputs via regular calls to the function.

Run this example with: 

```
cargo run --release example age_content_control
```

```rust
use compute::prelude::*;

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

### Benefits:
- **Fully Encrypted Computation**: The entire computation is hidden from external observers, including inputs, outputs, and intermediate states.
- **Input Privacy**: data is fully encrypted going into the function.


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

## Acknowlegments

The incredible [tandem project](https://github.com/sine-fdn/tandem) and the team over at [Sine Foundation](https://sine.foundation/) for providing the underlying 2PC garbling scheme
