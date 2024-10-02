fn main() {
    benchmark_tfhe().unwrap();
}

fn benchmark_tfhe() -> Result<(), Box<dyn std::error::Error>> {
    use tfhe::prelude::*;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint128};
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);

    let clear_a = 12297829382473034410u128;
    let clear_b = 1u128;

    // Encrypting the input data using the (private) client_key
    // FheUint32: Encrypted equivalent to u32
    let encrypted_a = FheUint128::try_encrypt(clear_a, &client_key)?;
    let encrypted_b = FheUint128::try_encrypt(clear_b, &client_key)?;

    // On the server side:
    set_server_key(server_keys);

    // Clear equivalent computations: 12297829382473034410 + 1
    let mut encrypted_res_mul = &encrypted_a + &encrypted_b;

    let iterations = 100;
    // loop 100 times
    for _ in 0..iterations {
        encrypted_res_mul = &encrypted_res_mul + &encrypted_b;
    }

    let clear_res: u128 = encrypted_res_mul.decrypt(&client_key);
    println!("Decrypted result: {}", clear_res);

    assert_eq!(clear_res, clear_a + iterations + 1);

    Ok(())
}
