use compute::prelude::*;

/// Determines loan eligibility based on credit score, income, and debt-to-income ratio.
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

#[circuit(execute)]
fn check_loan_eligibility(credit_score: u16, income: u16, debt_ratio: u16) -> u8 {
    // Eligibility levels
    let PRIME = 1;
    let SUBPRIME = 2;
    let HIGH_RISK = 3;
    let INELIGIBLE = 0;

    // Use `if let` with range checks to determine eligibility
    if let 750..=850 = credit_score {
        if let 50..=200 = income {
            if let 1..=30 = debt_ratio {
                // High income, low debt ratio, prime credit score
                PRIME
            } else {
                HIGH_RISK // High debt ratio, even with prime credit score and high income
            }
        } else {
            SUBPRIME // Prime credit score but income does not meet high-income range
        }
    } else if let 650..=749 = credit_score {
        if let 35..=199 = income {
            if let 1..=40 = debt_ratio {
                // Moderate income and debt ratio, subprime credit score
                SUBPRIME
            } else {
                HIGH_RISK // High debt ratio for subprime credit
            }
        } else {
            HIGH_RISK // Income below acceptable range for subprime category
        }
    } else if let 300..=649 = credit_score {
        HIGH_RISK // Low credit score is always high risk
    } else {
        INELIGIBLE // Credit score out of bounds
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
