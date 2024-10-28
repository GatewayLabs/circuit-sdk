use compute::prelude::*;

/// Validates if the provided password length meets the minimum required length.
///
/// # Parameters
/// - `password_length`: The length of the password to be checked.
///
/// # Returns
/// - `bool`: Returns `true` if the password length is greater than or equal to the minimum length,
///   indicating the password meets the strength requirement, otherwise `false`.
///
/// # Example
/// This example demonstrates verifying if a password with 12 characters meets a minimum length of 8 characters.
#[circuit(execute)]
fn password_strength(password_length: u8) -> bool {
    password_length >= 8
}

fn main() {
    let password_length = 12_u8;

    let result = password_strength(password_length);
    println!(
        "Does the password meet the strength requirement? {}",
        result
    ); // Expected: true
}
