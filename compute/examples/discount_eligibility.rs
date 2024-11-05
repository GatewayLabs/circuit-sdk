use compute::prelude::*;

/// Checks if a customer's purchase amount qualifies for a discount.
///
/// # Parameters
/// - `purchase_amount`: The total amount of the customer's purchase.
/// - `discount_threshold`: The minimum amount required to be eligible for a discount.
///
/// # Returns
/// - `bool`: Returns `true` if the purchase amount is greater than or equal to the discount threshold,
///   otherwise `false`.
///
/// # Example
/// This example demonstrates checking if a purchase of 100 qualifies for a discount with a threshold of 80.
#[encrypted(execute)]
fn qualifies_for_discount(purchase_amount: u16) -> bool {
    let DISCOUNT_THRESHOLD = 80;
    purchase_amount >= DISCOUNT_THRESHOLD
}

fn main() {
    let purchase_amount = 100_u16;

    let result = qualifies_for_discount(purchase_amount);
    println!("Does the purchase qualify for a discount? {}", result); // Expected: true
}
