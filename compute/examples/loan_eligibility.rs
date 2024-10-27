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
/// - `high_income_req`: The high income requirement for full approval.
/// - `min_credit_score`: The minimum credit score requirement for conditional approval.
/// - `max_debt_ratio`: The maximum debt-to-income ratio allowed for full approval.
/// - `max_conditional_debt_ratio`: The maximum debt-to-income ratio for conditional approval.
///
/// # Returns
/// - `u8`: Returns 2 for "Full Approval," 1 for "Conditional Approval," and 0 for "Denied."
///
/// # Example
/// This example demonstrates evaluating an applicant with an income of 75,000, a credit score of 680,
/// and a debt-to-income ratio of 30%. The requirements are:
/// - Full approval requires income >= 70,000, credit score >= 720, and debt ratio <= 35%.
/// - Conditional approval requires credit score >= 650 and income >= 50,000 and debt ratio <= 40.

#[circuit(execute)]
fn evaluate_loan_application(
    income: u32,
    credit_score: u32,
    debt_ratio: u32,
    HIGH_INCOME_REQ: u32,
    MIN_INCOME_REQ: u32,
    MIN_CREDIT_SCORE: u32,
    MAX_DEBT_RATIO: u32,
    MAX_CONDITIONAL_DEBT_RATIO: u32,
    FULLY_APPROVED: u32,
    CONDITIONAL_APPROVED: u32,
    DENIED: u32,
) -> u32 {
    //let income_and_credit_score = income >= HIGH_INCOME_REQ && credit_score >= MIN_CREDIT_SCORE;

    // Check for Full Approval
    if income >= HIGH_INCOME_REQ && credit_score >= MIN_CREDIT_SCORE && debt_ratio <= MAX_DEBT_RATIO
    {
        FULLY_APPROVED
    } else {
        let income_and_credit_score = income >= MIN_INCOME_REQ && credit_score >= MIN_CREDIT_SCORE;
        // Check for Conditional Approval
        if income_and_credit_score && debt_ratio <= MAX_CONDITIONAL_DEBT_RATIO {
            CONDITIONAL_APPROVED
        } else {
            // Denied if neither criteria met
            DENIED
        }
    }
}

fn main() {
    enum LoanStatus {
        Denied,
        ConditionalApproval,
        FullApproval,
    }

    // Example applicant data
    let income = 75000_u32;
    let credit_score = 680_u32;
    let debt_ratio = 30_u32;

    // Approval requirements passed as parameters
    let high_income_req = 70000_u32;
    let min_income_req = 50000_u32;
    let min_credit_score = 650_u32;
    let max_debt_ratio = 35_u32;
    let max_conditional_debt_ratio = 40_u32;

    let result = evaluate_loan_application(
        income,
        credit_score,
        debt_ratio,
        high_income_req,
        min_income_req,
        min_credit_score,
        max_debt_ratio,
        max_conditional_debt_ratio,
        LoanStatus::FullApproval as u32,
        LoanStatus::ConditionalApproval as u32,
        LoanStatus::Denied as u32,
    );

    // Output the decision based on result
    match result {
        2 => println!("Loan Status: Full Approval"),
        1 => println!("Loan Status: Conditional Approval"),
        _ => println!("Loan Status: Denied"),
    }
}
