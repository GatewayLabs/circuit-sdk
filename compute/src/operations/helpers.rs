use crate::simulator::simulate;
use crate::uint::GarbledUint;
use std::cmp::Ordering;
use tandem::Circuit;
use tandem::Gate;

// Helper function to build and simulate a circuit for addition or subtraction
#[allow(clippy::type_complexity)]
pub(super) fn build_and_simulate_arithmetic<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
    gate_fn: fn(u32, u32, Option<u32>, &mut Vec<Gate>, &mut Option<u32>) -> u32,
) -> GarbledUint<N> {
    let mut gates = Vec::new();
    let mut carry_or_borrow_index = None;

    // Push input gates for both Uint<N> objects
    for _ in 0..N {
        gates.push(Gate::InContrib); // From first Uint<N> (lhs)
    }
    for _ in 0..N {
        gates.push(Gate::InContrib); // From second Uint<N> (rhs)
    }

    let mut result_bit_indices = Vec::with_capacity(N);

    // Generate gates for each bit of the addition/subtraction
    for i in 0..N {
        let a = i as u32;
        let b = (N + i) as u32;

        // Use the provided gate function to define the behavior of each bit
        let result_index = gate_fn(
            a,
            b,
            carry_or_borrow_index,
            &mut gates,
            &mut carry_or_borrow_index,
        );
        result_bit_indices.push(result_index);
    }

    // Define output indices (result bits from the arithmetic operation)
    let output_indices: Vec<u32> = result_bit_indices.to_vec();

    // Create the circuit
    let program = Circuit::new(gates, output_indices);

    // combine the bits of lhs and rhs
    let mut input = lhs.bits.clone();
    input.extend_from_slice(&rhs.bits);

    // Simulate the circuit
    let result = simulate(&program, &input, &[]).unwrap();

    // Return the resulting Uint<N>
    GarbledUint::new(result)
}

// Helper function to add two GarbledUint<N>
fn add_garbled_uints(gates: &mut Vec<Gate>, a: &[usize], b: &[usize]) -> Vec<usize> {
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
fn full_adder(
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

// Helper function to build and simulate a circuit for multiplication
// Implements the shift-and-add method for multiplication
// To be replaced with Karatsuba's algorithm for better performance
#[allow(clippy::type_complexity)]
pub(super) fn build_and_simulate_multiplication<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut gates = Vec::new();

    // Push input gates for both GarbledUint<N> objects
    let lhs_start = gates.len();
    for _ in 0..N {
        gates.push(Gate::InContrib);
    }
    let rhs_start = gates.len();
    for _ in 0..N {
        gates.push(Gate::InContrib);
    }

    let mut partial_products = Vec::with_capacity(N);

    // Generate partial products
    for i in 0..N {
        let shifted_product = generate_partial_product(&mut gates, lhs_start, rhs_start, i, N);
        partial_products.push(shifted_product);
    }

    // Sum up all partial products
    let mut result = partial_products[0].clone();
    for i in partial_products.iter().take(N).skip(1) {
        result = add_garbled_uints(&mut gates, &result, i);
    }

    // Define output indices (result bits from the multiplication)
    let output_indices: Vec<u32> = result.iter().map(|&x| x as u32).collect();

    // Create the circuit
    let program = Circuit::new(gates, output_indices);

    // combine the bits of lhs and rhs
    let mut input = lhs.bits.clone();
    input.extend_from_slice(&rhs.bits);

    // Simulate the circuit
    let result = simulate(&program, &input, &[]).unwrap();

    // Return the resulting GarbledUint<N>
    GarbledUint::new(result)
}

// Helper function to generate a partial product
fn generate_partial_product(
    gates: &mut Vec<Gate>,
    lhs_start: usize,
    rhs_start: usize,
    shift: usize,
    n: usize,
) -> Vec<usize> {
    let mut partial_product = Vec::with_capacity(n);

    for i in 0..n {
        if i < shift {
            // For lower bits, we use a constant 0
            let zero_bit = gates.len();
            gates.push(Gate::Not(rhs_start as u32)); // NOT of any input bit is fine
            gates.push(Gate::And(rhs_start as u32, zero_bit as u32)); // AND with its NOT is always 0
            partial_product.push(gates.len() - 1);
        } else {
            let lhs_bit = lhs_start + i - shift;
            let and_gate = gates.len();
            gates.push(Gate::And(lhs_bit as u32, (rhs_start + shift) as u32));
            partial_product.push(and_gate);
        }
    }

    partial_product
}

// Helper function to generate gates for the addition of two bits
pub(super) fn add_gate_fn(
    a: u32,
    b: u32,
    carry: Option<u32>,
    gates: &mut Vec<Gate>,
    carry_out: &mut Option<u32>,
) -> u32 {
    // XOR gate for sum bit (a ⊕ b)
    let sum_xor_index = gates.len();
    gates.push(Gate::Xor(a, b));

    // If carry exists, XOR the result of the previous XOR with the carry
    let final_sum_index = if let Some(carry) = carry {
        let sum_with_carry_index = gates.len();
        gates.push(Gate::Xor(sum_xor_index as u32, carry));
        sum_with_carry_index as u32
    } else {
        sum_xor_index as u32
    };

    // Compute the new carry: (a & b) | (a & carry) | (b & carry)
    let and_ab = gates.len();
    gates.push(Gate::And(a, b));

    if let Some(carry) = carry {
        let and_a_carry = gates.len();
        gates.push(Gate::And(a, carry));

        let and_b_carry = gates.len();
        gates.push(Gate::And(b, carry));

        // Combine carry parts using XOR and AND to simulate OR
        let xor_ab_carry = gates.len();
        gates.push(Gate::Xor(and_ab as u32, and_a_carry as u32));
        gates.push(Gate::Xor(xor_ab_carry as u32, and_b_carry as u32));
        *carry_out = Some((gates.len() - 1) as u32);
    } else {
        *carry_out = Some(and_ab as u32);
    }

    final_sum_index
}

// Helper function to generate gates for the subtraction of two bits
pub(super) fn sub_gate_fn(
    a: u32,
    b: u32,
    borrow: Option<u32>,
    gates: &mut Vec<Gate>,
    borrow_out: &mut Option<u32>,
) -> u32 {
    // XOR gate for difference bit (a ⊕ b)
    let diff_xor_index = gates.len();
    gates.push(Gate::Xor(a, b));

    // If borrow exists, XOR the result of the previous XOR with the borrow
    let final_diff_index = if let Some(borrow) = borrow {
        let diff_with_borrow_index = gates.len();
        gates.push(Gate::Xor(diff_xor_index as u32, borrow));
        diff_with_borrow_index as u32
    } else {
        diff_xor_index as u32
    };

    // Compute the new borrow: (!a & b) | (a & borrow) | (!b & borrow)
    let not_a = gates.len();
    gates.push(Gate::Not(a));

    let and_not_a_b = gates.len();
    gates.push(Gate::And(not_a as u32, b));

    if let Some(borrow) = borrow {
        let and_a_borrow = gates.len();
        gates.push(Gate::And(a, borrow));

        let not_b = gates.len();
        gates.push(Gate::Not(b));

        let and_not_b_borrow = gates.len();
        gates.push(Gate::And(not_b as u32, borrow));

        // Combine borrow parts using XOR and AND to simulate OR
        let xor_borrow_parts = gates.len();
        gates.push(Gate::Xor(and_not_a_b as u32, and_a_borrow as u32));
        gates.push(Gate::Xor(xor_borrow_parts as u32, and_not_b_borrow as u32));
        *borrow_out = Some((gates.len() - 1) as u32);
    } else {
        *borrow_out = Some(and_not_a_b as u32);
    }

    final_diff_index
}

// Helper function to build and simulate a circuit for OR operation
pub(super) fn build_and_simulate_or<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut gates = Vec::new();

    // Push input gates for both Uint<N> objects (lhs and rhs)
    for _ in 0..N {
        gates.push(Gate::InContrib); // From first Uint<N> (lhs)
    }

    for _ in 0..N {
        gates.push(Gate::InContrib); // From second Uint<N> (rhs)
    }

    // Define gates for each bit in lhs and rhs
    let mut output_indices = Vec::with_capacity(N);

    for i in 0..N {
        // OR(a, b) = (a ⊕ b) ⊕ (a & b)

        // Step 1: XOR gate for (a ⊕ b)
        let xor_gate = Gate::Xor(i as u32, (N + i) as u32);
        let xor_gate_idx = gates.len() as u32;
        gates.push(xor_gate);

        // Step 2: AND gate for (a & b)
        let and_gate = Gate::And(i as u32, (N + i) as u32);
        let and_gate_idx = gates.len() as u32;
        gates.push(and_gate);

        // Step 3: XOR gate for final OR result (a ⊕ b) ⊕ (a & b)
        let final_or_gate = Gate::Xor(xor_gate_idx, and_gate_idx);
        gates.push(final_or_gate);

        // Step 4: Store the output index of this bit's OR result
        output_indices.push(gates.len() as u32 - 1);
    }

    // Create the circuit
    let program = Circuit::new(gates, output_indices);

    // combine the bits of lhs and rhs
    let mut input = lhs.bits.clone();
    input.extend_from_slice(&rhs.bits);

    // Simulate the circuit
    let result = simulate(&program, &input, &[]).unwrap();

    // Return the resulting Uint<N>
    GarbledUint::new(result)
}

// Helper function to build and simulate a circuit for binary operations
pub(super) fn build_and_simulate<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
    gate_fn: fn(u32, u32) -> Gate,
) -> GarbledUint<N> {
    let mut gates = Vec::new();

    // Push input gates for both Uint<N>s, both from the contributor
    for _ in 0..N {
        gates.push(Gate::InContrib); // From first Uint<N> (lhs)
    }

    for _ in 0..N {
        gates.push(Gate::InContrib); // From second Uint<N> (rhs)
    }

    let mut output_indices = Vec::with_capacity(N);

    // Define gates for each bit in lhs and rhs
    for i in 0..N {
        output_indices.push(gates.len() as u32);
        let gate = gate_fn(i as u32, (N + i) as u32);
        gates.push(gate);
    }

    // Create the circuit
    let program = Circuit::new(gates, output_indices);

    // combine the bits of lhs and rhs
    let mut input = lhs.bits.clone();
    input.extend_from_slice(&rhs.bits);

    // Simulate the circuit
    let result = simulate(&program, &input, &[]).unwrap();

    // Return the resulting Uint<N>
    GarbledUint::new(result)
}

// Helper function to handle NOT operation (unary)
pub(super) fn build_and_simulate_not<const N: usize>(input: &GarbledUint<N>) -> GarbledUint<N> {
    let mut gates = Vec::new();

    // Push input gates for Uint<N> object
    for _ in 0..N {
        gates.push(Gate::InContrib); // From first Uint<N> (lhs)
    }

    let mut output_indices = Vec::with_capacity(N);
    // Define NOT gates for each bit in the Uint<N>
    for i in 0..N {
        output_indices.push(gates.len() as u32);
        gates.push(Gate::Not(i.try_into().unwrap())); // NOT gate for each bit
    }

    // Create the circuit
    let program = Circuit::new(gates, output_indices);

    // Simulate the circuit
    let result = simulate(&program, &input.bits, &[]).unwrap();

    // Return the resulting Uint<N>
    GarbledUint::new(result)
}

// Implement composite bitwise operations for GarbledUint<N>
pub(super) fn build_and_simulate_nand<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut gates = Vec::new();

    // Push input gates for both Uint<N> objects
    for _ in 0..N {
        gates.push(Gate::InContrib); // From first Uint<N> (lhs)
    }

    for _ in 0..N {
        gates.push(Gate::InContrib); // From second Uint<N> (rhs)
    }

    let mut output_indices = Vec::with_capacity(N);

    for i in 0..N {
        // Step 1: AND gate for (a & b)
        let and_gate = Gate::And(i as u32, (N + i) as u32);
        let and_gate_idx = gates.len() as u32;
        gates.push(and_gate);

        // Step 2: NOT gate to negate the AND result
        let not_gate = Gate::Not(and_gate_idx);
        gates.push(not_gate);

        output_indices.push(gates.len() as u32 - 1);
    }

    // Create the circuit
    let program = Circuit::new(gates, output_indices);

    // combine the bits of lhs and rhs
    let mut input = lhs.bits.clone();
    input.extend_from_slice(&rhs.bits);

    // Simulate the circuit
    let result = simulate(&program, &input, &[]).unwrap();

    GarbledUint::new(result)
}

pub(super) fn build_and_simulate_nor<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut gates = Vec::new();

    // Push input gates for both Uint<N> objects
    for _ in 0..N {
        gates.push(Gate::InContrib); // From first Uint<N> (lhs)
    }

    for _ in 0..N {
        gates.push(Gate::InContrib); // From second Uint<N> (rhs)
    }

    let mut output_indices = Vec::with_capacity(N);

    for i in 0..N {
        // Step 1: XOR gate for (a ⊕ b)
        let xor_gate = Gate::Xor(i as u32, (N + i) as u32);
        let xor_gate_idx = gates.len() as u32;
        gates.push(xor_gate);

        // Step 2: AND gate for (a & b)
        let and_gate = Gate::And(i as u32, (N + i) as u32);
        let and_gate_idx = gates.len() as u32;
        gates.push(and_gate);

        // Step 3: XOR gate to simulate OR (a ⊕ b) ⊕ (a & b)
        let or_gate = Gate::Xor(xor_gate_idx, and_gate_idx);
        gates.push(or_gate);

        // Step 4: Apply NOT to the OR result to get NOR
        let not_gate = Gate::Not(gates.len() as u32 - 1);
        gates.push(not_gate);

        output_indices.push(gates.len() as u32 - 1);
    }

    // Create the circuit
    let program = Circuit::new(gates, output_indices);

    // combine the bits of lhs and rhs
    let mut input = lhs.bits.clone();
    input.extend_from_slice(&rhs.bits);

    // Simulate the circuit
    let result = simulate(&program, &input, &[]).unwrap();

    // Return the resulting Uint<N>
    GarbledUint::new(result)
}

pub(super) fn build_and_simulate_xnor<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut gates = Vec::new();

    // Push input gates for both Uint<N> objects
    for _ in 0..N {
        gates.push(Gate::InContrib); // From first Uint<N> (lhs)
    }

    for _ in 0..N {
        gates.push(Gate::InContrib); // From second Uint<N> (rhs)
    }

    let mut output_indices = Vec::with_capacity(N);

    for i in 0..N {
        // Step 1: XOR gate for (a ⊕ b)
        let xor_gate = Gate::Xor(i as u32, (N + i) as u32);
        let xor_gate_idx = gates.len() as u32;
        gates.push(xor_gate);

        // Step 2: Apply NOT to the XOR result to get XNOR
        let not_gate = Gate::Not(xor_gate_idx);
        gates.push(not_gate);

        output_indices.push(gates.len() as u32 - 1);
    }

    // Create the circuit
    let program = Circuit::new(gates, output_indices);

    // combine the bits of lhs and rhs
    let mut input = lhs.bits.clone();
    input.extend_from_slice(&rhs.bits);

    // Simulate the circuit
    let result = simulate(&program, &input, &[]).unwrap();

    // Return the resulting Uint<N>
    GarbledUint::new(result)
}

// Helper function to build and simulate a circuit for comparison operations
pub(super) fn build_and_simulate_equality<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
    gate_fn: fn(u32, u32, &mut Vec<Gate>) -> u32,
) -> bool {
    let mut gates = Vec::new();

    // Push input gates for both Uint<N> objects
    for _ in 0..N {
        gates.push(Gate::InContrib); // From first Uint<N> (lhs)
    }
    for _ in 0..N {
        gates.push(Gate::InContrib); // From second Uint<N> (rhs)
    }

    // Build the comparison circuit
    let mut result = gate_fn(0, N as u32, &mut gates);
    for i in 1..N {
        let current_comparison = gate_fn(i as u32, (N + i) as u32, &mut gates);
        let new_result = gates.len() as u32;
        gates.push(Gate::Xor(result, current_comparison));
        result = new_result;
    }

    // The final gate is our output
    let output_indices = vec![result];

    // Create the circuit
    let program = Circuit::new(gates, output_indices);

    // combine the bits of lhs and rhs
    let mut input = lhs.bits.clone();
    input.extend_from_slice(&rhs.bits);

    // Simulate the circuit
    let result = simulate(&program, &input, &[]).unwrap();

    // Return the boolean result
    result[0]
}

// Helper method for ordering comparison
pub(super) fn build_and_simulate_comparator<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> Ordering {
    let mut gates = Vec::new();

    // Prepare input indices for both operands
    let mut a_indices = Vec::with_capacity(N);
    let mut b_indices = Vec::with_capacity(N);
    for _ in 0..N {
        a_indices.push(gates.len() as u32);
        gates.push(Gate::InContrib); // Inputs from 'self'
    }
    for _ in 0..N {
        b_indices.push(gates.len() as u32);
        gates.push(Gate::InContrib); // Inputs from 'other'
    }

    // Build the comparator circuit
    let (lt_output, eq_output) = comparator_circuit::<N>(&a_indices, &b_indices, &mut gates);

    // Define outputs
    let output_indices = vec![lt_output, eq_output];

    // Create the circuit
    let program = Circuit::new(gates, output_indices);

    // Simulate the circuit
    // let result = simulate(&program, &self.bits, &other.bits).unwrap();

    // combine the bits of lhs and rhs
    let mut input = lhs.bits.clone();
    input.extend_from_slice(&rhs.bits);

    // Simulate the circuit
    let result = simulate(&program, &input, &[]).unwrap();

    // Interpret the result
    let lt = result[0];
    let eq = result[1];

    if lt {
        Ordering::Less
    } else if eq {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}

fn comparator_circuit<const N: usize>(
    a_indices: &[u32],
    b_indices: &[u32],
    gates: &mut Vec<Gate>,
) -> (u32, u32) {
    let mut eq_list = vec![0; N];
    let mut lt_list = vec![0; N];

    let n = N;

    // Start from the most significant bit (MSB)
    let i = n - 1;

    // Compute initial eq and lt for MSB
    // eq[i] = ¬(A[i] ⊻ B[i])
    let a_xor_b = gates.len() as u32;
    gates.push(Gate::Xor(a_indices[i], b_indices[i]));

    let eq_i = gates.len() as u32;
    gates.push(Gate::Not(a_xor_b));

    eq_list[i] = eq_i;

    // lt[i] = ¬A[i] ∧ B[i]
    let not_a = gates.len() as u32;
    gates.push(Gate::Not(a_indices[i]));

    let lt_i = gates.len() as u32;
    gates.push(Gate::And(not_a, b_indices[i]));

    lt_list[i] = lt_i;

    // Iterate from MSB-1 down to LSB
    for idx in (0..i).rev() {
        // Compute eq[i] = eq[i+1] ∧ ¬(A[i] ⊻ B[i])
        let a_xor_b = gates.len() as u32;
        gates.push(Gate::Xor(a_indices[idx], b_indices[idx]));

        let not_a_xor_b = gates.len() as u32;
        gates.push(Gate::Not(a_xor_b));

        let eq_i = gates.len() as u32;
        gates.push(Gate::And(eq_list[idx + 1], not_a_xor_b));

        eq_list[idx] = eq_i;

        // Compute lt[i]
        // temp_lt = ¬A[i] ∧ B[i]
        let not_a = gates.len() as u32;
        gates.push(Gate::Not(a_indices[idx]));

        let not_a_and_b = gates.len() as u32;
        gates.push(Gate::And(not_a, b_indices[idx]));

        // temp_lt = eq[i+1] ∧ not_a_and_b
        let temp_lt = gates.len() as u32;
        gates.push(Gate::And(eq_list[idx + 1], not_a_and_b));

        // lt[i] = lt[i+1] ∨ temp_lt
        // Since we don't have an OR gate, use lt_i = (lt_prev ⊻ temp_lt) ⊻ (lt_prev ∧ temp_lt)
        let lt_prev = lt_list[idx + 1];

        let lt_xor_temp = gates.len() as u32;
        gates.push(Gate::Xor(lt_prev, temp_lt));

        let lt_and_temp = gates.len() as u32;
        gates.push(Gate::And(lt_prev, temp_lt));

        let lt_i = gates.len() as u32;
        gates.push(Gate::Xor(lt_xor_temp, lt_and_temp));

        lt_list[idx] = lt_i;
    }

    // Return the final lt and eq outputs
    (lt_list[0], eq_list[0])
}
