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
    use compute::uint::GarbledUint128;

    let clear_a = 12297829382473034410u128;
    let clear_b = 424242424242u128;

    let a = GarbledUint128::from_u128(clear_a);
    let b = GarbledUint128::from_u128(clear_b);

    let result = &a + &b;
    assert_eq!(result.to_u128(), clear_a + clear_b);
    Ok(())
}

fn tfhe_encrypted_bitwise_and() -> Result<(), Box<dyn std::error::Error>> {
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
    let encrypted_res_mul = &encrypted_a & &encrypted_b;

    let clear_res: u128 = encrypted_res_mul.decrypt(&client_key);
    assert_eq!(clear_res, clear_a & clear_b);

    Ok(())
}

fn gateway_encrypted_bitwise_and() -> Result<(), Box<dyn std::error::Error>> {
    use compute::uint::GarbledUint128;

    let clear_a = 12297829382473034410u128;
    let clear_b = 424242424242u128;

    let a = GarbledUint128::from_u128(clear_a);
    let b = GarbledUint128::from_u128(clear_b);

    let result = &a & &b;
    assert_eq!(result.to_u128(), clear_a & clear_b);
    Ok(())
}

fn tfhe_encrypted_bitwise_xor() -> Result<(), Box<dyn std::error::Error>> {
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
    let encrypted_res_mul = &encrypted_a ^ &encrypted_b;

    let clear_res: u128 = encrypted_res_mul.decrypt(&client_key);
    assert_eq!(clear_res, clear_a ^ clear_b);

    Ok(())
}

fn gateway_encrypted_bitwise_xor() -> Result<(), Box<dyn std::error::Error>> {
    use compute::uint::GarbledUint128;

    let clear_a = 12297829382473034410u128;
    let clear_b = 424242424242u128;

    let a = GarbledUint128::from_u128(clear_a);
    let b = GarbledUint128::from_u128(clear_b);

    let result = &a ^ &b;
    assert_eq!(result.to_u128(), clear_a ^ clear_b);
    Ok(())
}

fn tfhe_encrypted_bitwise_or() -> Result<(), Box<dyn std::error::Error>> {
    use tfhe::prelude::*;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint128};
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);

    let clear_a = 12297829382473034410u128;
    let clear_b = 42424242424242424242u128;

    // Encrypting the input data using the (private) client_key
    let encrypted_a = FheUint128::try_encrypt(clear_a, &client_key).unwrap();
    let encrypted_b = FheUint128::try_encrypt(clear_b, &client_key).unwrap();

    // On the server side:
    set_server_key(server_keys);

    let encrypted_res_mul = &encrypted_a | &encrypted_b;

    let clear_res: u128 = encrypted_res_mul.decrypt(&client_key);
    assert_eq!(clear_res, clear_a | clear_b);

    Ok(())
}

fn gateway_encrypted_bitwise_or() -> Result<(), Box<dyn ::std::error::Error>> {
    use compute::uint::GarbledUint128;

    let clear_a = 12297829382473034410u128;
    let clear_b = 42424242424242424242u128;

    let a = GarbledUint128::from_u128(clear_a);
    let b = GarbledUint128::from_u128(clear_b);

    let result = &a | &b;
    assert_eq!(result.to_u128(), clear_a | clear_b);
    Ok(())
}

fn tfhe_encrypted_bitwise_not() -> Result<(), Box<dyn std::error::Error>> {
    use tfhe::prelude::*;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint128};
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);

    let clear_a = 12297829382473034410u128;

    // Encrypting the input data using the (private) client_key
    let encrypted_a = FheUint128::try_encrypt(clear_a, &client_key).unwrap();

    // On the server side:
    set_server_key(server_keys);

    // Clear equivalent computations: 12297829382473034410 + 1
    let encrypted_res_mul = !&encrypted_a;

    let clear_res: u128 = encrypted_res_mul.decrypt(&client_key);
    assert_eq!(clear_res, !clear_a);

    Ok(())
}

fn gateway_encrypted_bitwise_not() -> Result<(), Box<dyn ::std::error::Error>> {
    use compute::uint::GarbledUint128;

    let clear_a = 12297829382473034410u128;

    let a = GarbledUint128::from_u128(clear_a);

    let result = !&a;
    assert_eq!(result.to_u128(), !clear_a);
    Ok(())
}

fn tfhe_encrypted_subtraction() -> Result<(), Box<dyn ::std::error::Error>> {
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
    let encrypted_res_mul = &encrypted_a - &encrypted_b;

    let clear_res: u128 = encrypted_res_mul.decrypt(&client_key);
    assert_eq!(clear_res, clear_a - clear_b);

    Ok(())
}

fn gateway_encrypted_subtraction() -> Result<(), Box<dyn ::std::error::Error>> {
    use compute::uint::GarbledUint128;

    let clear_a = 12297829382473034410u128;
    let clear_b = 424242424242u128;

    let a = GarbledUint128::from_u128(clear_a);
    let b = GarbledUint128::from_u128(clear_b);

    let result = &a - &b;
    assert_eq!(result.to_u128(), clear_a - clear_b);
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

// Benchmark 3: Benchmarking benchmark_gateway_encrypted_bitwise_and
fn benchmark_gateway_encrypted_bitwise_and(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_bitwise_and", |b| {
        b.iter(gateway_encrypted_bitwise_and)
    });
}

// Benchmark 4: Benchmarking benchmark_tfhe_encrypted_bitwise_and
fn benchmark_tfhe_encrypted_bitwise_and(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_bitwise_and", |b| {
        b.iter(tfhe_encrypted_bitwise_and)
    });
}

// Benchmark 5: Benchmarking benchmark_gateway_encrypted_bitwise_xor
fn benchmark_gateway_encrypted_bitwise_xor(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_bitwise_xor", |b| {
        b.iter(gateway_encrypted_bitwise_xor)
    });
}

// Benchmark 6: Benchmarking benchmark_tfhe_encrypted_bitwise_xor
fn benchmark_tfhe_encrypted_bitwise_xor(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_bitwise_xor", |b| {
        b.iter(tfhe_encrypted_bitwise_xor)
    });
}

// Benchmark 7: Benchmarking benchmark_gateway_encrypted_bitwise_not
fn benchmark_gateway_encrypted_bitwise_not(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_bitwise_not", |b| {
        b.iter(gateway_encrypted_bitwise_not)
    });
}

// Benchmark 8: Benchmarking benchmark_tfhe_encrypted_bitwise_not
fn benchmark_tfhe_encrypted_bitwise_not(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_bitwise_not", |b| {
        b.iter(tfhe_encrypted_bitwise_not)
    });
}

// Benchmark 9: Benchmarking benchmark_gateway_encrypted_subtraction
fn benchmark_gateway_encrypted_subtraction(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_subtraction", |b| {
        b.iter(gateway_encrypted_subtraction)
    });
}

// Benchmark 10: Benchmarking benchmark_tfhe_encrypted_subtraction
fn benchmark_tfhe_encrypted_subtraction(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_subtraction", |b| {
        b.iter(tfhe_encrypted_subtraction)
    });
}

// Benchmark 11: Benchmarking benchmark_gateway_encrypted_bitwise_or
fn benchmark_gateway_encrypted_bitwise_or(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_bitwise_or", |b| {
        b.iter(gateway_encrypted_bitwise_or)
    });
}

// Benchmark 12: Benchmarking benchmark_tfhe_encrypted_bitwise_or
fn benchmark_tfhe_encrypted_bitwise_or(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_bitwise_or", |b| {
        b.iter(tfhe_encrypted_bitwise_or)
    });
}

// Configure Criterion with a sample size of 10
fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(10)
}

// Group the benchmarks together
criterion_group!(
    name = benches;
    config = custom_criterion();
    targets =
            benchmark_gateway_encrypted_addition,
            benchmark_tfhe_encrypted_addition,
            benchmark_gateway_encrypted_subtraction,
            benchmark_tfhe_encrypted_subtraction,
            benchmark_gateway_encrypted_bitwise_and,
            benchmark_tfhe_encrypted_bitwise_and,
            benchmark_gateway_encrypted_bitwise_xor,
            benchmark_tfhe_encrypted_bitwise_xor,
            benchmark_gateway_encrypted_bitwise_not,
            benchmark_tfhe_encrypted_bitwise_not,
            benchmark_gateway_encrypted_bitwise_or,
            benchmark_tfhe_encrypted_bitwise_or,

);
criterion_main!(benches);
