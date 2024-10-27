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

    let a: GarbledUint128 = clear_a.into();
    let b: GarbledUint128 = clear_b.into();

    let result = &a + &b;
    let result: u128 = result.into();
    assert_eq!(result, clear_a + clear_b);
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

    let a: GarbledUint128 = clear_a.into();
    let b: GarbledUint128 = clear_b.into();

    let result = &a & &b;
    let result: u128 = result.into();
    assert_eq!(result, clear_a & clear_b);
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

    let a: GarbledUint128 = clear_a.into();
    let b: GarbledUint128 = clear_b.into();

    let result = &a ^ &b;
    let result: u128 = result.into();
    assert_eq!(result, clear_a ^ clear_b);
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

    let a: GarbledUint128 = clear_a.into();
    let b: GarbledUint128 = clear_b.into();

    let result = &a | &b;
    let result: u128 = result.into();
    assert_eq!(result, clear_a | clear_b);
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

    let a: GarbledUint128 = clear_a.into();

    let result = !&a;
    let result: u128 = result.into();
    assert_eq!(result, !clear_a);
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

    let a: GarbledUint128 = clear_a.into();
    let b: GarbledUint128 = clear_b.into();

    let result = &a - &b;
    let result: u128 = result.into();
    assert_eq!(result, clear_a - clear_b);
    Ok(())
}

fn tfhe_encrypted_multiplication() -> Result<(), Box<dyn ::std::error::Error>> {
    use tfhe::prelude::*;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint128};
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);

    let clear_a = 12345678910u128;
    let clear_b = 1234;

    // Encrypting the input data using the (private) client_key
    let encrypted_a = FheUint128::try_encrypt(clear_a, &client_key).unwrap();
    let encrypted_b = FheUint128::try_encrypt(clear_b, &client_key).unwrap();

    // On the server side:
    set_server_key(server_keys);

    // Clear equivalent computations: 12345678910 * 1234
    let encrypted_res_mul = &encrypted_a * &encrypted_b;

    let clear_res: u128 = encrypted_res_mul.decrypt(&client_key);
    assert_eq!(clear_res, clear_a * clear_b);

    Ok(())
}

fn gateway_encrypted_multiplication() -> Result<(), Box<dyn ::std::error::Error>> {
    use compute::uint::GarbledUint128;

    let clear_a = 12345678910u128;
    let clear_b = 1234;

    let a: GarbledUint128 = clear_a.into();
    let b: GarbledUint128 = clear_b.into();

    let result: u128 = (&a * &b).into();
    assert_eq!(result, clear_a * clear_b);
    Ok(())
}

fn tfhe_encrypted_nand() -> Result<(), Box<dyn ::std::error::Error>> {
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
    let encrypted_res_mul = !(&encrypted_a & &encrypted_b);

    let clear_res: u128 = encrypted_res_mul.decrypt(&client_key);
    assert_eq!(clear_res, !(clear_a & clear_b));

    Ok(())
}

fn gateway_encrypted_nand() -> Result<(), Box<dyn ::std::error::Error>> {
    use compute::uint::GarbledUint128;

    let clear_a = 12297829382473034410u128;
    let clear_b = 424242424242u128;

    let a: GarbledUint128 = clear_a.into();
    let b: GarbledUint128 = clear_b.into();

    let result = a.nand(b);
    let result: u128 = result.into();
    assert_eq!(result, !(clear_a & clear_b));
    Ok(())
}

fn tfhe_encrypted_nor() -> Result<(), Box<dyn ::std::error::Error>> {
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
    let encrypted_res_mul = !(&encrypted_a | &encrypted_b);

    let clear_res: u128 = encrypted_res_mul.decrypt(&client_key);
    assert_eq!(clear_res, !(clear_a | clear_b));

    Ok(())
}

fn gateway_encrypted_nor() -> Result<(), Box<dyn ::std::error::Error>> {
    use compute::uint::GarbledUint128;

    let clear_a = 12297829382473034410u128;
    let clear_b = 424242424242u128;

    let a: GarbledUint128 = clear_a.into();
    let b: GarbledUint128 = clear_b.into();

    let result = a.nor(b);
    let result: u128 = result.into();
    assert_eq!(result, !(clear_a | clear_b));
    Ok(())
}

fn tfhe_encrypted_xnor() -> Result<(), Box<dyn ::std::error::Error>> {
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
    let encrypted_res_mul = !(&encrypted_a ^ &encrypted_b);

    let clear_res: u128 = encrypted_res_mul.decrypt(&client_key);
    assert_eq!(clear_res, !(clear_a ^ clear_b));

    Ok(())
}

fn gateway_encrypted_xnor() -> Result<(), Box<dyn ::std::error::Error>> {
    use compute::uint::GarbledUint128;

    let clear_a = 12297829382473034410u128;
    let clear_b = 424242424242u128;

    let a: GarbledUint128 = clear_a.into();
    let b: GarbledUint128 = clear_b.into();

    let result = a.xnor(b);
    let result: u128 = result.into();
    assert_eq!(result, !(clear_a ^ clear_b));
    Ok(())
}

fn tfhe_encrypted_eq() -> Result<(), Box<dyn ::std::error::Error>> {
    use tfhe::prelude::*;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint128};
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);

    let clear_a = 12297829382473034410u128;
    let clear_b = 12297829382473034410u128;

    // Encrypting the input data using the (private) client_key
    let encrypted_a = FheUint128::try_encrypt(clear_a, &client_key).unwrap();
    let encrypted_b = FheUint128::try_encrypt(clear_b, &client_key).unwrap();

    // On the server side:
    set_server_key(server_keys);

    // Clear equivalent computations: 12297829382473034410 + 1
    let encrypted_res_mul = encrypted_a.eq(&encrypted_b);

    let clear_res: bool = encrypted_res_mul.decrypt(&client_key);
    assert_eq!(clear_res, clear_a == clear_b);

    Ok(())
}

fn gateway_encrypted_eq() -> Result<(), Box<dyn ::std::error::Error>> {
    use compute::uint::GarbledUint128;

    let clear_a = 12297829382473034410u128;
    let clear_b = 12297829382473034410u128;

    let a: GarbledUint128 = clear_a.into();
    let b: GarbledUint128 = clear_b.into();

    let result = a == b;
    assert_eq!(result, clear_a == clear_b);
    Ok(())
}

fn tfhe_encrypted_neq() -> Result<(), Box<dyn ::std::error::Error>> {
    use tfhe::prelude::*;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint128};
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);

    let clear_a = 12297829382473034410u128;
    let clear_b = 12297829382473034400u128;

    // Encrypting the input data using the (private) client_key
    let encrypted_a = FheUint128::try_encrypt(clear_a, &client_key).unwrap();
    let encrypted_b = FheUint128::try_encrypt(clear_b, &client_key).unwrap();

    // On the server side:
    set_server_key(server_keys);

    // Clear equivalent computations: 12297829382473034410 + 1
    let encrypted_res_mul = encrypted_a.ne(&encrypted_b);

    let clear_res: bool = encrypted_res_mul.decrypt(&client_key);
    assert_eq!(clear_res, clear_a != clear_b);

    Ok(())
}

fn gateway_encrypted_neq() -> Result<(), Box<dyn ::std::error::Error>> {
    use compute::uint::GarbledUint128;

    let clear_a = 12297829382473034410u128;
    let clear_b = 12297829382473034400u128;

    let a: GarbledUint128 = clear_a.into();
    let b: GarbledUint128 = clear_b.into();

    let result = a != b;
    assert_eq!(result, clear_a != clear_b);
    Ok(())
}

fn tfhe_encrypted_gt() -> Result<(), Box<dyn ::std::error::Error>> {
    use tfhe::prelude::*;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint128};
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);

    let clear_a = 12297829382473034410u128;
    let clear_b = 12297829382473034409u128;

    // Encrypting the input data using the (private) client_key
    let encrypted_a = FheUint128::try_encrypt(clear_a, &client_key).unwrap();
    let encrypted_b = FheUint128::try_encrypt(clear_b, &client_key).unwrap();

    // On the server side:
    set_server_key(server_keys);

    // Clear equivalent computations: 12297829382473034410 + 1
    let encrypted_res_mul = encrypted_a.gt(&encrypted_b);

    let clear_res: bool = encrypted_res_mul.decrypt(&client_key);
    assert_eq!(clear_res, clear_a > clear_b);

    Ok(())
}

fn gateway_encrypted_gt() -> Result<(), Box<dyn ::std::error::Error>> {
    use compute::uint::GarbledUint128;

    let clear_a = 12297829382473034410u128;
    let clear_b = 12297829382473034409u128;

    let a: GarbledUint128 = clear_a.into();
    let b: GarbledUint128 = clear_b.into();

    let result = a > b;
    assert_eq!(result, clear_a > clear_b);
    Ok(())
}

fn tfhe_encrypted_ge() -> Result<(), Box<dyn ::std::error::Error>> {
    use tfhe::prelude::*;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint128};
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);

    let clear_a = 12297829382473034410u128;
    let clear_b = 12297829382473034410u128;

    // Encrypting the input data using the (private) client_key
    let encrypted_a = FheUint128::try_encrypt(clear_a, &client_key).unwrap();
    let encrypted_b = FheUint128::try_encrypt(clear_b, &client_key).unwrap();

    // On the server side:
    set_server_key(server_keys);

    // Clear equivalent computations: 12297829382473034410 + 1
    let encrypted_res_mul = encrypted_a.ge(&encrypted_b);

    let clear_res: bool = encrypted_res_mul.decrypt(&client_key);
    assert_eq!(clear_res, clear_a >= clear_b);

    Ok(())
}

fn gateway_encrypted_ge() -> Result<(), Box<dyn ::std::error::Error>> {
    use compute::uint::GarbledUint128;

    let clear_a = 12297829382473034410u128;
    let clear_b = 12297829382473034410u128;

    let a: GarbledUint128 = clear_a.into();
    let b: GarbledUint128 = clear_b.into();

    let result = a >= b;
    assert_eq!(result, clear_a >= clear_b);
    Ok(())
}

fn tfhe_encrypted_lt() -> Result<(), Box<dyn ::std::error::Error>> {
    use tfhe::prelude::*;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint128};
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);

    let clear_a = 12297829382473034409u128;
    let clear_b = 12297829382473034410u128;

    // Encrypting the input data using the (private) client_key
    let encrypted_a = FheUint128::try_encrypt(clear_a, &client_key).unwrap();
    let encrypted_b = FheUint128::try_encrypt(clear_b, &client_key).unwrap();

    // On the server side:
    set_server_key(server_keys);

    // Clear equivalent computations: 12297829382473034410 + 1
    let encrypted_res_mul = encrypted_a.lt(&encrypted_b);

    let clear_res: bool = encrypted_res_mul.decrypt(&client_key);
    assert_eq!(clear_res, clear_a < clear_b);

    Ok(())
}

fn gateway_encrypted_lt() -> Result<(), Box<dyn ::std::error::Error>> {
    use compute::uint::GarbledUint128;

    let clear_a = 12297829382473034409u128;
    let clear_b = 12297829382473034410u128;

    let a: GarbledUint128 = clear_a.into();
    let b: GarbledUint128 = clear_b.into();

    let result = a < b;
    assert_eq!(result, clear_a < clear_b);
    Ok(())
}

fn tfhe_encrypted_le() -> Result<(), Box<dyn ::std::error::Error>> {
    use tfhe::prelude::*;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint128};
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);

    let clear_a = 12297829382473034410u128;
    let clear_b = 12297829382473034410u128;

    // Encrypting the input data using the (private) client_key
    let encrypted_a = FheUint128::try_encrypt(clear_a, &client_key).unwrap();
    let encrypted_b = FheUint128::try_encrypt(clear_b, &client_key).unwrap();

    // On the server side:
    set_server_key(server_keys);

    // Clear equivalent computations: 12297829382473034410 + 1
    let encrypted_res_mul = encrypted_a.le(&encrypted_b);

    let clear_res: bool = encrypted_res_mul.decrypt(&client_key);
    assert_eq!(clear_res, clear_a <= clear_b);

    Ok(())
}

fn gateway_encrypted_le() -> Result<(), Box<dyn ::std::error::Error>> {
    use compute::uint::GarbledUint128;

    let clear_a = 12297829382473034410u128;
    let clear_b = 12297829382473034410u128;

    let a: GarbledUint128 = clear_a.into();
    let b: GarbledUint128 = clear_b.into();

    let result = a <= b;
    assert_eq!(result, clear_a <= clear_b);
    Ok(())
}

fn tfhe_encrypted_division() -> Result<(), Box<dyn ::std::error::Error>> {
    use tfhe::prelude::*;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint128};
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);

    let clear_a = 12345678910u128;
    let clear_b = 1234;

    // Encrypting the input data using the (private) client_key
    let encrypted_a = FheUint128::try_encrypt(clear_a, &client_key).unwrap();
    let encrypted_b = FheUint128::try_encrypt(clear_b, &client_key).unwrap();

    // On the server side:
    set_server_key(server_keys);

    // Clear equivalent computations: 12345678910 * 1234
    let encrypted_res_mul = &encrypted_a / &encrypted_b;

    let clear_res: u128 = encrypted_res_mul.decrypt(&client_key);
    assert_eq!(clear_res, clear_a / clear_b);

    Ok(())
}

fn gateway_encrypted_division() -> Result<(), Box<dyn ::std::error::Error>> {
    use compute::uint::GarbledUint128;

    let clear_a = 12345678910u128;
    let clear_b = 1234;

    let a: GarbledUint128 = clear_a.into();
    let b: GarbledUint128 = clear_b.into();

    let result: u128 = (&a / &b).into();
    assert_eq!(result, clear_a / clear_b);
    Ok(())
}

fn tfhe_encrypted_modulus() -> Result<(), Box<dyn std::error::Error>> {
    use tfhe::prelude::*;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint128};
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);

    let clear_a = 12345678910u128;
    let clear_b = 1234;

    // Encrypting the input data using the (private) client_key
    let encrypted_a = FheUint128::try_encrypt(clear_a, &client_key).unwrap();
    let encrypted_b = FheUint128::try_encrypt(clear_b, &client_key).unwrap();

    // On the server side:
    set_server_key(server_keys);

    // Clear equivalent computations: 12345678910 * 1234
    let encrypted_res_mul = &encrypted_a % &encrypted_b;

    let clear_res: u128 = encrypted_res_mul.decrypt(&client_key);
    assert_eq!(clear_res, clear_a % clear_b);

    Ok(())
}

fn gateway_encrypted_modulus() -> Result<(), Box<dyn ::std::error::Error>> {
    use compute::uint::GarbledUint128;

    let clear_a = 12345678910u128;
    let clear_b = 1234;

    let a: GarbledUint128 = clear_a.into();
    let b: GarbledUint128 = clear_b.into();

    let result: u128 = (&a % &b).into();
    assert_eq!(result, clear_a % clear_b);
    Ok(())
}

fn tfhe_encrypted_mux() {
    use tfhe::boolean::prelude::*;
    // We generate a set of client/server keys, using the default parameters:
    let (client_key, server_key) = gen_keys();

    let bool1 = true;
    let bool2 = false;
    let bool3 = true;

    // We use the client secret key to encrypt a message:
    let ct_1 = client_key.encrypt(true);
    let ct_2 = client_key.encrypt(false);
    let ct_3 = client_key.encrypt(false);

    // We use the server public key to execute the NOT gate:
    let ct_xor = server_key.mux(&ct_1, &ct_2, &ct_3);

    // We use the client key to decrypt the output of the circuit:
    let output = client_key.decrypt(&ct_xor);
    assert_eq!(output, if bool1 { bool2 } else { bool3 });
}

fn gateway_encrypted_mux() {
    use compute::uint::GarbledBoolean;

    let bool1 = true;
    let bool2 = false;
    let bool3 = true;

    let a: GarbledBoolean = bool1.into();
    let b: GarbledBoolean = bool2.into();
    let c: GarbledBoolean = bool3.into();

    let result = GarbledBoolean::mux(&a, &b, &c);
    let result: bool = result.into();
    assert_eq!(result, if bool1 { bool2 } else { bool3 });
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

// Benchmark 11: Benchmarking benchmark_gateway_encrypted_multiplication
fn benchmark_gateway_encrypted_multiplication(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_multiplication", |b| {
        b.iter(gateway_encrypted_multiplication)
    });
}

// Benchmark 12: Benchmarking benchmark_tfhe_encrypted_multiplication
fn benchmark_tfhe_encrypted_multiplication(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_multiplication", |b| {
        b.iter(tfhe_encrypted_multiplication)
    });
}

// Benchmark 13: Benchmarking benchmark_gateway_encrypted_nand
fn benchmark_gateway_encrypted_nand(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_nand", |b| b.iter(gateway_encrypted_nand));
}

// Benchmark 14: Benchmarking benchmark_tfhe_encrypted_nand
fn benchmark_tfhe_encrypted_nand(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_nand", |b| b.iter(tfhe_encrypted_nand));
}

// Benchmark 15: Benchmarking benchmark_gateway_encrypted_nor
fn benchmark_gateway_encrypted_nor(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_nor", |b| b.iter(gateway_encrypted_nor));
}

// Benchmark 16: Benchmarking benchmark_tfhe_encrypted_nor
fn benchmark_tfhe_encrypted_nor(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_nor", |b| b.iter(tfhe_encrypted_nor));
}

// Benchmark 17: Benchmarking benchmark_gateway_encrypted_xnor
fn benchmark_gateway_encrypted_xnor(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_xnor", |b| b.iter(gateway_encrypted_xnor));
}

// Benchmark 18: Benchmarking benchmark_tfhe_encrypted_xnor
fn benchmark_tfhe_encrypted_xnor(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_xnor", |b| b.iter(tfhe_encrypted_xnor));
}

// Benchmark 19: Benchmarking benchmark_gateway_encrypted_bitwise_or
fn benchmark_gateway_encrypted_bitwise_or(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_bitwise_or", |b| {
        b.iter(gateway_encrypted_bitwise_or)
    });
}

// Benchmark 20: Benchmarking benchmark_tfhe_encrypted_bitwise_or
fn benchmark_tfhe_encrypted_bitwise_or(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_bitwise_or", |b| {
        b.iter(tfhe_encrypted_bitwise_or)
    });
}

// Benchmark 21: Benchmarking benchmark_gateway_encrypted_eq
fn benchmark_gateway_encrypted_eq(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_eq", |b| b.iter(gateway_encrypted_eq));
}

// Benchmark 22: Benchmarking benchmark_tfhe_encrypted_eq
fn benchmark_tfhe_encrypted_eq(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_eq", |b| b.iter(tfhe_encrypted_eq));
}

// Benchmark 23: Benchmarking benchmark_gateway_encrypted_neq
fn benchmark_gateway_encrypted_neq(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_neq", |b| b.iter(gateway_encrypted_neq));
}

// Benchmark 24: Benchmarking benchmark_tfhe_encrypted_neq
fn benchmark_tfhe_encrypted_neq(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_neq", |b| b.iter(tfhe_encrypted_neq));
}

// Benchmark 25: Benchmarking benchmark_gateway_encrypted_gt
fn benchmark_gateway_encrypted_gt(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_gt", |b| b.iter(gateway_encrypted_gt));
}

// Benchmark 26: Benchmarking benchmark_tfhe_encrypted_gt
fn benchmark_tfhe_encrypted_gt(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_gt", |b| b.iter(tfhe_encrypted_gt));
}

// Benchmark 27: Benchmarking benchmark_gateway_encrypted_lt
fn benchmark_gateway_encrypted_lt(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_lt", |b| b.iter(gateway_encrypted_lt));
}

// Benchmark 28: Benchmarking benchmark_tfhe_encrypted_lt
fn benchmark_tfhe_encrypted_lt(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_lt", |b| b.iter(tfhe_encrypted_lt));
}

// Benchmark 29: Benchmarking benchmark_gateway_encrypted_ge
fn benchmark_gateway_encrypted_ge(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_ge", |b| b.iter(gateway_encrypted_ge));
}

// Benchmark 30: Benchmarking benchmark_tfhe_encrypted_ge
fn benchmark_tfhe_encrypted_ge(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_ge", |b| b.iter(tfhe_encrypted_ge));
}

// Benchmark 31: Benchmarking benchmark_gateway_encrypted_le
fn benchmark_gateway_encrypted_le(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_le", |b| b.iter(gateway_encrypted_le));
}

// Benchmark 32: Benchmarking benchmark_tfhe_encrypted_le
fn benchmark_tfhe_encrypted_le(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_le", |b| b.iter(tfhe_encrypted_le));
}

// Benchmark 33: Benchmarking benchmark_gateway_encrypted_mux
fn benchmark_gateway_encrypted_mux(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_mux", |b| b.iter(gateway_encrypted_mux));
}

// Benchmark 34: Benchmarking benchmark_tfhe_encrypted_mux
fn benchmark_tfhe_encrypted_mux(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_mux", |b| b.iter(tfhe_encrypted_mux));
}

// Benchmark 35: Benchmarking benchmark_gateway_encrypted_division
fn benchmark_gateway_encrypted_division(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_division", |b| {
        b.iter(gateway_encrypted_division)
    });
}

// Benchmark 36: Benchmarking benchmark_tfhe_encrypted_division
fn benchmark_tfhe_encrypted_division(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_division", |b| {
        b.iter(tfhe_encrypted_division)
    });
}

// Benchmark 37: Benchmarking benchmark_gateway_encrypted_modulus
fn benchmark_gateway_encrypted_modulus(c: &mut Criterion) {
    c.bench_function("gateway_encrypted_modulus", |b| {
        b.iter(gateway_encrypted_modulus)
    });
}

// Benchmark 38: Benchmarking benchmark_tfhe_encrypted_modulus
fn benchmark_tfhe_encrypted_modulus(c: &mut Criterion) {
    c.bench_function("tfhe_encrypted_modulus", |b| b.iter(tfhe_encrypted_modulus));
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
            benchmark_gateway_encrypted_division,
            benchmark_tfhe_encrypted_division,
            benchmark_gateway_encrypted_modulus,
            benchmark_tfhe_encrypted_modulus,

            benchmark_gateway_encrypted_mux,
            benchmark_tfhe_encrypted_mux,
            benchmark_gateway_encrypted_addition,
            benchmark_tfhe_encrypted_addition,
            benchmark_gateway_encrypted_subtraction,
            benchmark_tfhe_encrypted_subtraction,
            benchmark_gateway_encrypted_multiplication,
            benchmark_tfhe_encrypted_multiplication,
            benchmark_gateway_encrypted_bitwise_and,
            benchmark_tfhe_encrypted_bitwise_and,
            benchmark_gateway_encrypted_bitwise_xor,
            benchmark_tfhe_encrypted_bitwise_xor,
            benchmark_gateway_encrypted_bitwise_not,
            benchmark_tfhe_encrypted_bitwise_not,
            benchmark_gateway_encrypted_nand,
            benchmark_tfhe_encrypted_nand,
            benchmark_gateway_encrypted_nor,
            benchmark_tfhe_encrypted_nor,
            benchmark_gateway_encrypted_xnor,
            benchmark_tfhe_encrypted_xnor,
            benchmark_gateway_encrypted_bitwise_or,
            benchmark_tfhe_encrypted_bitwise_or,
            benchmark_gateway_encrypted_eq,
            benchmark_tfhe_encrypted_eq,
            benchmark_gateway_encrypted_neq,
            benchmark_tfhe_encrypted_neq,
            benchmark_gateway_encrypted_gt,
            benchmark_tfhe_encrypted_gt,
            benchmark_gateway_encrypted_lt,
            benchmark_tfhe_encrypted_lt,
            benchmark_gateway_encrypted_ge,
            benchmark_tfhe_encrypted_ge,
            benchmark_gateway_encrypted_le,
            benchmark_tfhe_encrypted_le
);
criterion_main!(benches);
