use compute::prelude::*;

#[circuit(execute)]
fn is_salary_above_threshold(salary: u128, threshold: u128) -> bool {
    // loop 10 times
    let accumulator = salary;
    for _ in 0..10 {
        let accumulator = accumulator + threshold;
    }
    salary > threshold
}

/*
#[circuit(execute)]
fn calculate_total_salaries(sal: u128, salaries: Vec<u128>) -> u128 {
    let total = salaries.iter().fold(0_u128, |acc, &salary| acc + salary);
    total / (salaries.len() as u8)
}
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = is_salary_above_threshold(1000_u128, 500_u128);

    assert_eq!(result, true); // Expected result indicating salary exceeds threshold.

    Ok(())
}
