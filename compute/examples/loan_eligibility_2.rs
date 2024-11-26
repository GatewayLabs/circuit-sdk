use compute::prelude::*;

/// Determines loan eligibility based on credit score, income, and debt-to-income ratio using `match`.
///
/// This function categorizes applicants as:
/// - **Prime**: High credit score (750+), high income, and low debt ratio.
/// - **Subprime**: Moderate credit score (650-749), moderate income, and acceptable debt ratio.
/// - **High Risk**: Low credit score (<650) or high debt ratio, regardless of income.
/// - **Ineligible**: For applicants with values outside expected ranges.
///
/// # Parameters
/// - `credit_score`: The applicant's credit score (as u16).
/// - `income`: The applicant's annual income in thousands of dollars (as u16).
/// - `debt_ratio`: The applicant's debt-to-income ratio in percentage (as u16).
///
/// # Returns
/// - `u8`: Eligibility level:
///     - 1 for "Prime"
///     - 2 for "Subprime"
///     - 3 for "High Risk"
///     - 0 for "Ineligible"
///
/// # Example
/// ```
/// let credit_score = 720_u16;
/// let income = 60_u16; // $60,000 income
/// let debt_ratio = 25_u16; // 25% debt ratio
///
/// let eligibility = check_loan_eligibility(credit_score, income, debt_ratio);
/// assert_eq!(eligibility, 1); // Prime eligibility
/// ```

#[encrypted(execute)]
fn check_loan_eligibility(credit_score: u16, income: u16, debt_ratio: u16) -> u16 {
    // Eligibility levels
    let prime = 1;
    let subprime = 2;
    let high_risk = 3;
    let ineligible = 0;

    match credit_score {
        750..=850 => match income {
            50..=200 => match debt_ratio {
                1..=30 => prime, // High income, low debt ratio, prime credit score
                _ => high_risk,  // High debt ratio, even with prime credit score and high income
            },
            _ => subprime, // Prime credit score but income does not meet high-income range
        },
        650..=749 => match income {
            35..=199 => match debt_ratio {
                1..=40 => subprime, // Moderate income and debt ratio, subprime credit score
                _ => high_risk,     // High debt ratio for subprime credit
            },
            _ => high_risk, // Income below acceptable range for subprime category
        },
        300..=649 => high_risk, // Low credit score is always high risk
        _ => ineligible,        // Credit score out of bounds
    }
}

fn main() {
    // Test cases for loan eligibility with varying criteria

    // Prime case: High credit score, high income, low debt ratio
    let credit_score = 780_u16;
    let income = 75_u16; // $75,000
    let debt_ratio = 25_u16;
    let eligibility = check_loan_eligibility(credit_score, income, debt_ratio);
    println!("Eligibility Level: {}", eligibility); // Expected output: 1 (Prime)

    // Subprime case: Moderate credit score, moderate income, moderate debt ratio
    let credit_score = 700_u16;
    let income = 50_u16; // $50,000
    let debt_ratio = 35_u16;
    let eligibility = check_loan_eligibility(credit_score, income, debt_ratio);
    println!("Eligibility Level: {}", eligibility); // Expected output: 2 (Subprime)

    // High Risk case: Low credit score with moderate income and debt ratio
    let credit_score = 620_u16;
    let income = 45_u16; // $45,000
    let debt_ratio = 30_u16;
    let eligibility = check_loan_eligibility(credit_score, income, debt_ratio);
    println!("Eligibility Level: {}", eligibility); // Expected output: 3 (High Risk)

    // Ineligible case: Credit score out of expected range
    let credit_score = 900_u16;
    let income = 100_u16; // $100,000
    let debt_ratio = 15_u16;
    let eligibility = check_loan_eligibility(credit_score, income, debt_ratio);
    println!("Eligibility Level: {}", eligibility); // Expected output: 0 (Ineligible)
}
