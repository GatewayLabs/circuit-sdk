use compute::prelude::*;

/// Validates if the provided password length meets the minimum required length.
///
/// # Parameters
/// - `password_length`: The length of the password to be checked.
/// - `min_length`: The minimum acceptable length for the password.
///
/// # Returns
/// - `bool`: Returns `true` if the password length is greater than or equal to the minimum length,
///   indicating the password meets the strength requirement, otherwise `false`.
///
/// # Example
/// This example demonstrates verifying if a password with 12 characters meets a minimum length of 8 characters.
#[circuit(execute)]
fn password_strength(password_length: u8, min_length: u8) -> bool {
    password_length >= min_length
}

fn main() {
    let password_length = 12_u8;
    let min_length = 8_u8;

    let result = password_strength(password_length, min_length);
    println!(
        "Does the password meet the strength requirement? {}",
        result
    ); // Expected: true
}
