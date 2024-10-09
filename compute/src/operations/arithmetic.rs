use crate::uint::GarbledUint;
use std::ops::{Add, Mul, Sub};
use tandem::{Circuit, Gate};

use super::helpers::add_garbled_uints;

// Helper function to build and simulate a circuit for addition or subtraction
#[allow(clippy::type_complexity)]
fn build_and_simulate_arithmetic<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
    gate_fn: fn(u32, u32, Option<u32>, &mut Vec<Gate>, &mut Option<u32>) -> u32,
) -> GarbledUint<N> {
    let mut gates = Vec::new();
    let mut carry_or_borrow_index = None; // Carry/borrow bit

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
    let result = lhs.simulate(&program, &lhs.bits, &rhs.bits).unwrap();

    // Return the resulting Uint<N>
    GarbledUint::new(result)
}

// Helper function to build and simulate a circuit for multiplication
// Implements the shift-and-add method for multiplication
// To be replaced with Karatsuba's algorithm for better performance
#[allow(clippy::type_complexity)]
fn build_and_simulate_multiplication<const N: usize>(
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
    for i in 1..N {
        result = add_garbled_uints(&mut gates, &result, &partial_products[i]);
    }

    // Define output indices (result bits from the multiplication)
    let output_indices: Vec<u32> = result.iter().map(|&x| x as u32).collect();

    // Create the circuit
    let program = Circuit::new(gates, output_indices);

    // Simulate the circuit
    let result = lhs.simulate(&program, &lhs.bits, &rhs.bits).unwrap();

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
fn add_gate_fn(
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
fn sub_gate_fn(
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

// Implement the Add operation for Uint<N> and &Uint<N>
impl<const N: usize> Add for GarbledUint<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        build_and_simulate_arithmetic(&self, &rhs, add_gate_fn)
    }
}

impl<const N: usize> Add for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn add(self, rhs: Self) -> Self::Output {
        build_and_simulate_arithmetic(self, rhs, add_gate_fn)
    }
}

// Implement the Sub operation for Uint<N> and &Uint<N>
impl<const N: usize> Sub for GarbledUint<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        build_and_simulate_arithmetic(&self, &rhs, sub_gate_fn)
    }
}

impl<const N: usize> Sub for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        build_and_simulate_arithmetic(self, rhs, sub_gate_fn)
    }
}

impl<const N: usize> Mul for GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        build_and_simulate_multiplication(&self, &rhs)
    }
}

impl<const N: usize> Mul for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        build_and_simulate_multiplication(self, rhs)
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::uint::{GarbledUint128, GarbledUint16, GarbledUint32, GarbledUint64, GarbledUint8};

    #[test]
    fn test_uint_add() {
        let a = GarbledUint::<4>::new(vec![true, true, false, false]); // Binary 1100
        let b = GarbledUint::<4>::new(vec![false, false, true, true]); // Binary 0011

        let result = a + b; // Perform addition on the 4-bit values
        assert_eq!(result.to_u8(), 0b1111); // Binary 1111 (Addition result of 1100 + 0011)
    }

    #[test]
    fn test_from_u8_add() {
        let a = GarbledUint8::from_u8(170); // Binary 10101010
        let b = GarbledUint8::from_u8(85); // Binary 01010101

        let result = a + b; // Perform addition on the 4-bit values
        assert_eq!(result.to_u8(), 170 + 85); // Expected result of addition between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_add() {
        let a = GarbledUint16::from_u16(43690); // Binary 1010101010101010
        let b = GarbledUint16::from_u16(21845); // Binary 0101010101010101

        let result = a + b;
        assert_eq!(result.to_u16(), 43690 + 21845); // Expected result of addition between 1010101010101010 and 0101010101010101
    }

    #[test]
    fn test_from_u32_add() {
        let a = GarbledUint32::from_u32(2863311530); // Binary 10101010101010101010101010101010
        let b = GarbledUint32::from_u32(1431655765); // Binary 01010101010101010101010101010101

        let result = a + b;
        assert_eq!(result.to_u32(), 2863311530 + 1431655765); // Expected result of addition between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_add() {
        let a = GarbledUint64::from_u64(12297829382473034410); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b = GarbledUint64::from_u64(6148914691236517205); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result = a + b;
        assert_eq!(result.to_u64(), 12297829382473034410 + 6148914691236517205);
        // Expected result of addition between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_add() {
        let a = GarbledUint128::from_u128(12297829382473034410); // Binary 10101010
        let b = GarbledUint128::from_u128(6148914691236517205); // Binary 01010101

        let result = a + b;
        assert_eq!(result.to_u128(), 12297829382473034410 + 6148914691236517205);

        println!("{}", result.to_u128());
        // Expected result of addition between 10101010 and 01010101
    }

    #[test]
    fn test_uint_sub() {
        let a = GarbledUint::<4>::from_u8(3);
        let b = GarbledUint::<4>::from_u8(2);

        let result = a - b; // Perform subtraction on the 4-bit values
        assert_eq!(result.to_u8(), 3 - 2);
    }

    #[test]
    fn test_from_u8_sub() {
        let a = GarbledUint8::from_u8(170); // Binary 10101010
        let b = GarbledUint8::from_u8(100); // Binary 01100100

        let result = a - b;
        assert_eq!(result.to_u8(), 170 - 100); // Expected result of subtraction between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_sub() {
        let a = GarbledUint16::from_u16(43690); // Binary 1010101010101010
        let b = GarbledUint16::from_u16(21845); // Binary 0101010101010101

        let result = a - b;
        assert_eq!(result.to_u16(), 43690 - 21845); // Expected result of subtraction between 1010101010101010 and 0101010101010101
    }

    #[test]
    fn test_from_u32_sub() {
        let a = GarbledUint32::from_u32(2863311530); // Binary 10101010101010101010101010101010
        let b = GarbledUint32::from_u32(1431655765); // Binary 01010101010101010101010101010101

        let result = a - b;
        assert_eq!(result.to_u32(), 2863311530 - 1431655765); // Expected result of subtraction between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_sub() {
        let a = GarbledUint64::from_u64(12297829382473034410); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b = GarbledUint64::from_u64(6148914691236517205); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result = a - b;
        assert_eq!(result.to_u64(), 12297829382473034410 - 6148914691236517205);
        // Expected result of subtraction between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_sub() {
        let a = GarbledUint128::from_u128(170); // Binary 10101010
        let b = GarbledUint128::from_u128(85); // Binary 01010101

        let result = a - b;
        assert_eq!(result.to_u128(), 170 - 85); // Expected result of subtraction between 10101010 and 01010101
    }

    #[test]
    fn test_uint_mul() {
        let a = GarbledUint::<4>::from_u8(3); // Binary 0011
        let b = GarbledUint::<4>::from_u8(2); // Binary 0010

        let result = a * b;
        assert_eq!(result.to_u8(), 6); // 0011 * 0010 = 0110
    }

    #[test]
    fn test_from_u8_mul() {
        let a = GarbledUint8::from_u8(7); // Binary 0000 0111
        let b = GarbledUint8::from_u8(5); // Binary 0000 0101

        let result = a * b;
        assert_eq!(result.to_u8(), 35); // Binary 0010 0011
    }

    #[test]
    fn test_from_u16_mul() {
        let a = GarbledUint16::from_u16(300); // Binary 0000 0001 0010 1100
        let b = GarbledUint16::from_u16(7); // Binary 0000 0000 0000 0111

        let result = a * b;
        assert_eq!(result.to_u16(), 2100); // Binary 0000 1000 0010 0100
    }
}
