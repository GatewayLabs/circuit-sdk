use compute::prelude::*;

/// Evaluates a loan application based on income, credit score, debt-to-income ratio, and other requirements.
///
/// The logic follows a tiered approach:
/// - "Full Approval": If income, credit score, and debt-to-income ratio meet the highest criteria.
/// - "Conditional Approval": If income or credit score partially meet the requirements.
/// - "Denied": If none of the criteria are met.
///
/// # Parameters
/// - `income`: The applicant's income level.
/// - `credit_score`: The applicant's credit score.
/// - `debt_ratio`: The applicant's debt-to-income ratio (in percentage).
///
/// # Returns
/// - `u32`: Returns 2 for "Full Approval," 1 for "Conditional Approval," and 0 for "Denied."
///
/// # Example
/// This example demonstrates evaluating an applicant with an income of 75,000, a credit score of 680,
/// and a debt-to-income ratio of 30%. The requirements are:
/// - Full approval requires income >= 70,000, credit score >= 720, and debt ratio <= 35%.
/// - Conditional approval requires credit score >= 650 and income >= 50,000 and debt ratio <= 40.

#[circuit(execute)]
fn evaluate_loan_application(income: u32, credit_score: u32, debt_ratio: u32) -> u32 {
    // Constants for loan approval criteria
    let HIGH_INCOME_REQ = 70000;
    let MIN_INCOME_REQ = 50000;
    let MIN_CREDIT_SCORE = 650;
    let MAX_DEBT_RATIO = 35;
    let MAX_CONDITIONAL_DEBT_RATIO = 40_u32;

    // Loan approval status codes
    let FULLY_APPROVED = 2;
    let CONDITIONAL_APPROVED = 1;
    let DENIED = 0;
    // Check for Full Approval
    if income >= HIGH_INCOME_REQ && credit_score >= MIN_CREDIT_SCORE && debt_ratio <= MAX_DEBT_RATIO
    {
        FULLY_APPROVED
    } else if income >= MIN_INCOME_REQ
        && credit_score >= MIN_CREDIT_SCORE
        && debt_ratio >= MAX_CONDITIONAL_DEBT_RATIO
    {
        // Check for Conditional Approval
        CONDITIONAL_APPROVED
    } else {
        // Denied if neither criteria met
        DENIED
    }
}

fn main() {
    // Example applicant data
    let income = 75000_u32;
    let credit_score = 680_u32;
    let debt_ratio = 90_u32;

    let result = evaluate_loan_application(income, credit_score, debt_ratio);

    // Output the decision based on result
    match result {
        2 => println!("Loan Status: Full Approval"),
        1 => println!("Loan Status: Conditional Approval"),
        _ => println!("Loan Status: Denied"),
    }
}
