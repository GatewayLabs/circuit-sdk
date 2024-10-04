use criterion::{criterion_group, criterion_main, Criterion};

// Function to benchmark (example function)
fn tfhe_encrypted_addition() -> Result<(), Box<dyn std::error::Error>> {
    use tfhe::prelude::*;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint128};
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);

    let clear_a = 12297829382473034410u128;
    let clear_b = 424242424242u128;

    // Encrypting the input data using the (private) client_key
    let encrypted_a = FheUint128::try_encrypt(clear_a, &client_key).unwrap();
    let encrypted_b = FheUint128::try_encrypt(clear_b, &client_key).unwrap();

    // On the server side:
    set_server_key(server_keys);

    // Clear equivalent computations: 12297829382473034410 + 1
    let encrypted_res_mul = &encrypted_a + &encrypted_b;

    let clear_res: u128 = encrypted_res_mul.decrypt(&client_key);
    assert_eq!(clear_res, clear_a + clear_b);

    Ok(())
}

// Another function to benchmark
fn gateway_encrypted_addition() -> Result<(), Box<dyn std::error::Error>> {
    use compute::uint::Uint;

    let clear_a = 12297829382473034410u128;
    let clear_b = 424242424242u128;

    let a = Uint::<128>::from_u128(clear_a);
    let b = Uint::<128>::from_u128(clear_b);

    let result = &a + &b;
    assert_eq!(result.to_u128(), clear_a + clear_b);
    Ok(())
}

// Benchmark 1: Benchmarking benchmark_gateway_encrypted_addition
fn benchmark_gateway_encrypted_addition(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_addition", |b| {
        b.iter(gateway_encrypted_addition)
    });
}

// Benchmark 2: Benchmarking benchmark_tfhe_encrypted_addition
fn benchmark_tfhe_encrypted_addition(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_addition", |b| {
        b.iter(tfhe_encrypted_addition)
    });
}

// Group the benchmarks together
criterion_group!(
    benches,
    benchmark_gateway_encrypted_addition,
    benchmark_tfhe_encrypted_addition
);
criterion_main!(benches);
