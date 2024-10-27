use compute::prelude::*;

/// Determines if a candidate has received enough votes to pass the specified threshold.
///
/// # Parameters
/// - `votes`: The number of votes the candidate has received.
/// - `threshold`: The minimum number of votes required for the candidate to pass.
///
/// # Returns
/// - `bool`: Returns `true` if the votes are greater than or equal to the threshold,
///   indicating the candidate has met the required vote count, otherwise `false`.
///
/// # Example
/// This example demonstrates checking if a candidate with 150 votes meets a threshold of 100 votes.
#[circuit(execute)]
fn has_enough_votes(votes: u8, threshold: u8) -> bool {
    votes >= threshold
}

fn main() {
    let votes = 150_u8;
    let threshold = 100_u8;

    let result = has_enough_votes(votes, threshold);
    println!("Does the candidate have enough votes? {}", result); // Expected: true
}
