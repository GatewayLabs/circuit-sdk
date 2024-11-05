use compute::prelude::*;

/// Checks if the current temperature is within the specified minimum and maximum range.
///
/// # Parameters
/// - `current_temp`: The current temperature of the room.
/// - `min_temp`: The minimum acceptable temperature.
/// - `max_temp`: The maximum acceptable temperature.
///
/// # Returns
/// - `bool`: Returns `true` if the current temperature is between the minimum and maximum values,
///   indicating it is within an acceptable range, otherwise `false`.
///
/// # Example
/// This example demonstrates verifying if a room with a temperature of 70°F is within the range of 65°F to 75°F.
#[encrypted(execute)]
fn within_temperature_range(current_temp: u8) -> bool {
    let MIN_TEMP = 65;
    let MAX_TEMP = 75;
    let above_min = current_temp >= MIN_TEMP;
    let below_max = current_temp <= MAX_TEMP;

    above_min && below_max
}

fn main() {
    let current_temp = 70_u8;

    let result = within_temperature_range(current_temp);
    println!("Is the temperature within range? {}", result); // Expected: true
}
