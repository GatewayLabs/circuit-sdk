use tfhe::boolean::client_key::ClientKey;
use tfhe::boolean::prelude::*;
use tfhe::boolean::server_key::ServerKey;

// Function to perform exponentiation with encrypted base and exponent
pub fn fhe_exponentiation(
    base_bits: &[Ciphertext],
    exponent_bits: &[Ciphertext],
    server_key: &ServerKey,
) -> Vec<Ciphertext> {
    // Initialize result to 1 (encrypted bits)
    let mut result_bits = vec![server_key.trivial_encrypt(true)];
    for _ in 1..base_bits.len() {
        result_bits.push(server_key.trivial_encrypt(false));
    }

    // Copy of base_bits
    let mut base_power_bits = base_bits.to_vec();

    for exponent_bit in exponent_bits {
        // Multiply result_bits by base_power_bits
        let multiplied_bits =
            homomorphic_multiplication(&result_bits, &base_power_bits, server_key);

        // Use MUX to select between result_bits and multiplied_bits based on exponent_bit
        result_bits =
            homomorphic_mux_vector(exponent_bit, &multiplied_bits, &result_bits, server_key);

        // Square base_power_bits
        base_power_bits =
            homomorphic_multiplication(&base_power_bits, &base_power_bits, server_key);
    }

    result_bits
}

// Function to encrypt a u32 value into an array of Ciphertext bits
pub(crate) fn encrypt_u32_to_bits(value: u32, client_key: &ClientKey) -> Vec<Ciphertext> {
    (0..32)
        .map(|i| {
            let bit = (value >> i) & 1;
            client_key.encrypt(bit != 0)
        })
        .collect()
}

pub(crate) fn homomorphic_multiplication(
    a_bits: &[Ciphertext],
    b_bits: &[Ciphertext],
    server_key: &ServerKey,
) -> Vec<Ciphertext> {
    let n = a_bits.len();
    let m = b_bits.len();
    let mut product_bits = vec![server_key.trivial_encrypt(false); n + m];

    // Iterate over each bit of b_bits (multiplier)
    for i in 0..m {
        let mut partial_product = Vec::with_capacity(n + m);

        // Shift partial product by i bits (equivalent to multiplying by 2^i)
        for _ in 0..i {
            partial_product.push(server_key.trivial_encrypt(false));
        }

        // Multiply each bit of a_bits (multiplicand) by b_bits[i]
        for j in 0..n {
            let and_bit = server_key.and(&a_bits[j], &b_bits[i]);
            partial_product.push(and_bit);
        }

        // Pad the rest with zeros to match the length of product_bits
        while partial_product.len() < n + m {
            partial_product.push(server_key.trivial_encrypt(false));
        }

        // Add partial product to the accumulated product
        product_bits = homomorphic_addition(&product_bits, &partial_product, server_key);
    }

    // Return the product bits
    product_bits
}

fn homomorphic_addition(
    a_bits: &[Ciphertext],
    b_bits: &[Ciphertext],
    server_key: &ServerKey,
) -> Vec<Ciphertext> {
    let n = a_bits.len().max(b_bits.len());
    let mut sum_bits = Vec::with_capacity(n + 1);
    let zero = server_key.trivial_encrypt(false);
    let mut carry = zero.clone();

    for i in 0..n {
        let a_bit = a_bits.get(i).unwrap_or(&zero);
        let b_bit = b_bits.get(i).unwrap_or(&zero);
        let (sum, new_carry) = full_adder(a_bit, b_bit, &carry, server_key);
        sum_bits.push(sum);
        carry = new_carry;
    }

    // Handle the final carry-out
    sum_bits.push(carry);

    sum_bits
}

// MUX function: selects between ct_then and ct_else based on ct_condition
fn mux(
    ct_condition: &Ciphertext,
    ct_then: &Ciphertext,
    ct_else: &Ciphertext,
    server_key: &ServerKey,
) -> Ciphertext {
    let not_condition = server_key.not(ct_condition);
    let and_then = server_key.and(ct_then, ct_condition);
    let and_else = server_key.and(ct_else, &not_condition);
    server_key.or(&and_then, &and_else)
}

fn full_adder(
    a: &Ciphertext,
    b: &Ciphertext,
    cin: &Ciphertext,
    server_key: &ServerKey,
) -> (Ciphertext, Ciphertext) {
    let sum1 = server_key.xor(a, b);
    let sum = server_key.xor(&sum1, cin);
    let carry1 = server_key.and(a, b);
    let carry2 = server_key.and(&sum1, cin);
    let cout = server_key.or(&carry1, &carry2);
    (sum, cout)
}

fn homomorphic_mux_vector(
    ct_condition: &Ciphertext,
    ct_then: &[Ciphertext],
    ct_else: &[Ciphertext],
    server_key: &ServerKey,
) -> Vec<Ciphertext> {
    ct_then
        .iter()
        .zip(ct_else.iter())
        .map(|(ct_t, ct_e)| mux(ct_condition, ct_t, ct_e, server_key))
        .collect()
}

// Function to decrypt an array of Ciphertext bits into a u32 value
pub(crate) fn decrypt_bits_to_u32(bits: &[Ciphertext], client_key: &ClientKey) -> u32 {
    bits.iter().enumerate().fold(0u32, |acc, (i, ct)| {
        let bit = client_key.decrypt(ct);
        acc | ((bit as u32) << i)
    })
}
