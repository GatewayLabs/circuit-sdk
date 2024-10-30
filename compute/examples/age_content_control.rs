use compute::prelude::*;

/// Determines content access level based on the age of the user.
///
/// The function categorizes users into different access levels:
/// - Restricted Access: For users aged 1-17 (typically underage).
/// - Full Access: For adult users aged 18-65.
/// - Limited Access: For senior users aged 66-120.
/// - Invalid Age: For users whose age falls outside the expected range.
///
/// # Parameters
/// - `age`: The user's age as an unsigned 8-bit integer.
///
/// # Returns
/// - `u8`: Returns:
///     - 1 for "Restricted Access"
///     - 2 for "Full Access"
///     - 3 for "Limited Access"
///     - 0 for "Invalid Age"
///
/// # Example
/// Here is an example showing how the function categorizes access based on the age:
/// ```
/// let age = 25_u8;
/// let access_level = access_content(age);
/// assert_eq!(access_level, 2); // Full access
///
/// let age = 15_u8;
/// let access_level = access_content(age);
/// assert_eq!(access_level, 1); // Restricted access
///
/// let age = 70_u8;
/// let access_level = access_content(age);
/// assert_eq!(access_level, 3); // Limited access
///
/// let age = 125_u8;
/// let access_level = access_content(age);
/// assert_eq!(access_level, 0); // Invalid age
/// ```
#[circuit(execute)]
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
