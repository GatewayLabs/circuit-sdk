use tandem::Gate;

// Helper function to add two GarbledUint<N>
pub fn add_garbled_uints(gates: &mut Vec<Gate>, a: &[usize], b: &[usize]) -> Vec<usize> {
    let mut result = Vec::with_capacity(a.len());
    let mut carry = None;

    for i in 0..a.len() {
        let sum = full_adder(gates, a[i], b[i], carry);
        result.push(sum.0);
        carry = sum.1;
    }

    result
}

// Helper function to add two bits with optional carry
pub fn full_adder(
    gates: &mut Vec<Gate>,
    a: usize,
    b: usize,
    carry: Option<usize>,
) -> (usize, Option<usize>) {
    let xor_ab = gates.len();
    gates.push(Gate::Xor(a as u32, b as u32));

    let sum = if let Some(c) = carry {
        let sum_with_carry = gates.len();
        gates.push(Gate::Xor(xor_ab as u32, c as u32));
        sum_with_carry
    } else {
        xor_ab
    };

    let and_ab = gates.len();
    gates.push(Gate::And(a as u32, b as u32));

    let new_carry = if let Some(c) = carry {
        let and_axorb_c = gates.len();
        gates.push(Gate::And(xor_ab as u32, c as u32));

        let or_gate = gates.len();
        gates.push(Gate::Xor(and_ab as u32, and_axorb_c as u32));
        Some(or_gate)
    } else {
        Some(and_ab)
    };

    (sum, new_carry)
}
