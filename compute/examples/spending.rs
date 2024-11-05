use compute::prelude::*;

/// Determines if the remaining budget after expenses is within the allowable limit.
///
/// # Parameters
/// - `budget`: The initial budget available.
/// - `spent`: The amount already spent from the budget.
///
/// # Returns
/// - `bool`: Returns `true` if the remaining budget is less than or equal to the maximum allowable,
///   indicating spending is within the limits, otherwise `false`.
///
/// # Example
/// This example demonstrates checking if a remaining budget of 3000 (after spending 2000 from a budget of 5000)
/// stays within the maximum allowable limit of 3000.
#[encrypted(execute)]
fn can_spend(budget: u16, spent: u16) -> bool {
    let MAX_ALLOWED = 5000;
    let remaining_budget = budget - spent;
    remaining_budget <= MAX_ALLOWED
}

fn main() {
    let budget = 5000_u16;
    let spent = 2000_u16;

    let result = can_spend(budget, spent);
    println!("Is spending within the allowable limit? {}", result); // Expected: true
}
