use compute::prelude::*;

/// Determines if a user has the required access level to enter a restricted area.
///
/// # Parameters
/// - `user_level`: The access level of the current user.
/// - `required_level`: The minimum access level required for the restricted area.
///
/// # Returns
/// - `bool`: Returns `true` if the user's level is greater than or equal to the required level,
///   indicating they have the necessary access, otherwise `false`.
///
/// # Example
/// This example demonstrates verifying if a user with level 5 can access an area that requires level 4.
#[circuit(execute)]
fn has_access(user_level: u8, required_level: u8) -> bool {
    user_level >= required_level
}

fn main() {
    let user_level = 5_u8;
    let required_level = 4_u8;

    let result = has_access(user_level, required_level);
    println!("Does the user have access? {}", result); // Expected: true
}
