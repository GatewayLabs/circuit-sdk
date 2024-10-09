use crate::int::GarbledInt;
use crate::simulator::simulate;
use crate::uint::GarbledUint;
use std::ops::{Add, Mul, Sub};
use tandem::Circuit;
use tandem::Gate;

// Helper function to add two GarbledUint<N>
pub(super) fn add_garbled_uints(gates: &mut Vec<Gate>, a: &[usize], b: &[usize]) -> Vec<usize> {
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
pub(super) fn full_adder(
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
        gates.push(Gate::InEval); // From second Uint<N> (rhs)
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

    // Simulate the circuit
    let result = simulate(&program, &lhs.bits, &rhs.bits).unwrap();

    // Return the resulting Uint<N>
    GarbledUint::new(result)
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
        gates.push(Gate::InEval);
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

    // Simulate the circuit
    let result = simulate(&program, &lhs.bits, &rhs.bits).unwrap();

    // Return the resulting GarbledUint<N>
    GarbledUint::new(result)
}

// Helper function to generate a partial product
pub(super) fn generate_partial_product(
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

// Helper function to build and simulate a circuit for binary operations
pub(super) fn build_and_simulate<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: Option<&GarbledUint<N>>,
    gate_fn: fn(u32, u32) -> Gate,
) -> GarbledUint<N> {
    let mut gates = Vec::new();

    // Push input gates for both Uint<N>s
    for _ in 0..N {
        gates.push(Gate::InContrib); // From first Uint<N> (lhs)
    }

    for _ in 0..N {
        gates.push(Gate::InEval); // From second Uint<N> (rhs)
    }

    // Define gates for each bit in lhs and rhs
    for i in 0..N {
        let gate = gate_fn(i as u32, (N + i) as u32);
        gates.push(gate);
    }

    // Define the output indices (for N-bit operation)
    let output_indices: Vec<u32> = (2 * N as u32..2 * N as u32 + N as u32).collect();

    // Create the circuit
    let program = Circuit::new(gates, output_indices);

    // Simulate the circuit
    let bits_rhs = rhs.map_or(lhs.bits.clone(), |r| r.bits.clone());
    let result = simulate(&program, &lhs.bits, &bits_rhs).unwrap();

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

    for _ in 0..N {
        gates.push(Gate::InEval); // From second Uint<N> (rhs)
    }

    // Define NOT gates for each bit in the Uint<N>
    for i in 0..N * 2 {
        gates.push(Gate::Not(i.try_into().unwrap())); // NOT gate for each bit
    }

    // Define the output indices (for N-bit NOT)
    let n = N as u32;
    let output_indices: Vec<u32> = (2 * n..2 * n + n).collect();

    // Create the circuit
    let program = Circuit::new(gates, output_indices);

    // Simulate the circuit
    let result = simulate(&program, &input.bits, &input.bits).unwrap();

    // Return the resulting Uint<N>
    GarbledUint::new(result)
}

// Implement composite bitwise operations for GarbledUint<N>
pub(super) fn build_and_simulate_nand<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: Option<&GarbledUint<N>>,
) -> GarbledUint<N> {
    let mut gates = Vec::new();

    // Push input gates for both Uint<N> objects
    for _ in 0..N {
        gates.push(Gate::InContrib); // From first Uint<N> (lhs)
    }

    for _ in 0..N {
        gates.push(Gate::InEval); // From second Uint<N> (rhs)
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

    let program = Circuit::new(gates, output_indices);
    let bits_rhs = rhs.map_or(lhs.bits.clone(), |r| r.bits.clone());
    let result = simulate(&program, &lhs.bits, &bits_rhs).unwrap();

    GarbledUint::new(result)
}

pub(super) fn build_and_simulate_nor<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: Option<&GarbledUint<N>>,
) -> GarbledUint<N> {
    let mut gates = Vec::new();

    // Push input gates for both Uint<N> objects
    for _ in 0..N {
        gates.push(Gate::InContrib); // From first Uint<N> (lhs)
    }

    for _ in 0..N {
        gates.push(Gate::InEval); // From second Uint<N> (rhs)
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

    let program = Circuit::new(gates, output_indices);
    let bits_rhs = rhs.map_or(lhs.bits.clone(), |r| r.bits.clone());
    let result = simulate(&program, &lhs.bits, &bits_rhs).unwrap();

    GarbledUint::new(result)
}

pub(super) fn build_and_simulate_xnor<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: Option<&GarbledUint<N>>,
) -> GarbledUint<N> {
    let mut gates = Vec::new();

    // Push input gates for both Uint<N> objects
    for _ in 0..N {
        gates.push(Gate::InContrib); // From first Uint<N> (lhs)
    }

    for _ in 0..N {
        gates.push(Gate::InEval); // From second Uint<N> (rhs)
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

    let program = Circuit::new(gates, output_indices);
    let bits_rhs = rhs.map_or(lhs.bits.clone(), |r| r.bits.clone());
    let result = simulate(&program, &lhs.bits, &bits_rhs).unwrap();

    GarbledUint::new(result)
}
