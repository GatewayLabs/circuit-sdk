use crate::operations::circuits::traits::CircuitExecutor;
use crate::operations::circuits::types::GateIndexVec;
use crate::uint::GarbledUint;
use crate::{executor::get_executor, uint::GarbledBoolean};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use tandem::{Circuit, Gate};

pub type GateIndex = u32;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct WRK17CircuitBuilder {
    inputs: Vec<bool>,
    gates: Vec<Gate>,
    constant_cache: HashMap<String, GateIndexVec>,
}

impl Debug for WRK17CircuitBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("WRK17CircuitBuilder");
        debug_struct.field("inputs", &self.inputs);

        // Collect gates into a formatted string with newlines
        let gates_with_newlines: Vec<String> = self
            .gates
            .iter()
            .map(|gate| format!("{:?}", gate))
            .collect();

        debug_struct.field("gates", &gates_with_newlines);
        debug_struct.finish()
    }
}

impl WRK17CircuitBuilder {
    pub fn input<const R: usize>(&mut self, input: &GarbledUint<R>) -> GateIndexVec {
        // get the cumulative size of all inputs in input_labels
        //let input_offset = self.input_labels.iter().map(|x| x.len()).sum::<usize>();

        let mut input_label = GateIndexVec::default();
        for bool_value in input.bits.iter() {
            let new_gate_index = self.gates.len() as GateIndex;

            self.gates.push(Gate::InContrib);
            self.inputs.push(*bool_value);

            input_label.push(new_gate_index);
        }
        input_label
    }

    pub fn constant<const R: usize>(&mut self, value: &GarbledUint<R>) -> GateIndexVec {
        let key = format!("{:x}", value);
        if let Some(cached) = self.constant_cache.get(&key) {
            return cached.clone();
        }

        // Convert the value to a GarbledUint using From<Uint> implementation
        let wire = self.input(value);
        self.constant_cache.insert(key, wire.clone());
        wire
    }

    pub fn len(&self) -> GateIndex {
        self.gates.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.gates.is_empty()
    }

    pub fn inputs(&self) -> &Vec<bool> {
        &self.inputs
    }

    // Add a XOR gate between two inputs and return the index
    pub fn push_xor(&mut self, a: &GateIndex, b: &GateIndex) -> GateIndex {
        let xor_index = self.gates.len() as u32;
        self.gates.push(Gate::Xor(*a, *b));
        xor_index
    }

    // Add an Aa.len()D gate between two inputs and return the index
    pub fn push_and(&mut self, a: &GateIndex, b: &GateIndex) -> GateIndex {
        let and_index = self.gates.len() as u32;
        self.gates.push(Gate::And(*a, *b));
        and_index
    }

    // Add a NOT gate for a single input and return the index
    pub fn push_not(&mut self, a: &GateIndex) -> GateIndex {
        let not_index = self.gates.len() as u32;
        self.gates.push(Gate::Not(*a));
        not_index
    }

    // Add a gate for OR operation: OR(a, b) = (a ⊕ b) ⊕ (a & b)
    pub fn push_or(&mut self, a: &GateIndex, b: &GateIndex) -> GateIndex {
        let xor_gate = self.push_xor(a, b);
        let and_gate = self.push_and(a, b);
        self.push_xor(&xor_gate, &and_gate)
    }

    // Add a a.len()Aa.len()D gate: a.len()Aa.len()D(a, b) = a.len()OT(a & b)
    pub fn push_nand(&mut self, a: &GateIndex, b: &GateIndex) -> GateIndex {
        let and_gate = self.push_and(a, b);
        self.push_not(&and_gate)
    }

    pub fn push_nor(&mut self, a: &GateIndex, b: &GateIndex) -> GateIndex {
        let or_gate = self.push_or(a, b);
        self.push_not(&or_gate)
    }

    // Add an Xa.len()OR gate: Xa.len()OR(a, b) = a.len()OT(a ⊕ b)
    pub fn push_xnor(&mut self, a: &GateIndex, b: &GateIndex) -> GateIndex {
        let xor_gate = self.push_xor(a, b);
        self.push_not(&xor_gate)
    }

    pub fn mux_lookahead(&mut self, a: &GateIndexVec) -> GateIndexVec {
        // repeat with output_indices
        let mut output = GateIndexVec::default();
        let mut counter = self.len() + 5 + 1;

        for _ in 0..a.len() {
            output.push(counter);
            counter += 6 + 1;
        }
        output
    }

    #[allow(dead_code)]
    // Add a MUX gate: MUX(a, b, s) = (a & !s) | (b & s)
    pub fn push_mux(&mut self, s: &GateIndex, a: &GateIndex, b: &GateIndex) -> GateIndex {
        let not_s = self.push_not(s);
        let and_a_not_s = self.push_and(a, &not_s);
        let and_b_s = self.push_and(b, s);
        self.push_or(&and_a_not_s, &and_b_s)
    }

    /// Implements the division circuit using a bit-serial long division algorithm.
    fn div_inner(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> (GateIndexVec, GateIndexVec) {
        let n = a.len();

        // zero out quotient, remainder
        let mut quotient = GateIndexVec::new(vec![self.constant::<1>(&0u128.into())[0]; n]);
        let mut remainder = GateIndexVec::new(vec![self.constant::<1>(&0u128.into())[0]; n]);

        let one_bit_vec = self.constant::<1>(&1u128.into());
        let zero_bit_vec = self.constant::<1>(&0u128.into());

        // For each bit from MSB down to LSB:
        for i in (0..n).rev() {
            remainder = self.shift_left(&remainder);
            remainder = self.set_lsb(&remainder, a[i]);

            let ge_bit = self.ge(&remainder, b);

            let remainder_sub = self.sub(&remainder, b);
            remainder = self.mux(&ge_bit, &remainder_sub, &remainder);

            let q_bit = self.mux(&ge_bit, &one_bit_vec, &zero_bit_vec)[0];
            quotient = self.shift_left(&quotient);
            quotient = self.set_lsb(&quotient, q_bit);
        }

        (quotient, remainder)
    }

    fn shift_left(&mut self, vec: &GateIndexVec) -> GateIndexVec {
        let n = vec.len();
        let zero = self.constant::<1>(&0u128.into())[0];
        let mut new_vec = GateIndexVec::default();
        new_vec.push(zero);
        for i in 0..(n - 1) {
            new_vec.push(vec[i]);
        }
        new_vec
    }

    fn set_lsb(&mut self, vec: &GateIndexVec, bit: GateIndex) -> GateIndexVec {
        let mut new_vec = vec.clone();
        if !new_vec.is_empty() {
            new_vec.set(0, bit);
        }
        new_vec
    }

    pub fn compile(&self, output_indices: &GateIndexVec) -> Circuit {
        Circuit::new(self.gates.clone(), output_indices.clone().into())
    }

    pub fn execute<const N: usize>(&self, circuit: &Circuit) -> anyhow::Result<GarbledUint<N>> {
        let result = get_executor().execute(circuit, &self.inputs, &[])?;
        Ok(GarbledUint::new(result))
    }

    // Simulate the circuit using the provided input values
    pub fn compile_and_execute<const N: usize>(
        &self,
        output_indices: &GateIndexVec,
    ) -> anyhow::Result<GarbledUint<N>> {
        let circuit = self.compile(output_indices);
        let result = get_executor().execute(&circuit, &self.inputs, &[])?;
        Ok(GarbledUint::new(result))
    }
}

impl CircuitExecutor for WRK17CircuitBuilder {
    type Type = GateIndex;
    type TypeVec = GateIndexVec;

    fn xor(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        // repeat with output_indices
        let mut output = GateIndexVec::default();
        for i in 0..a.len() {
            let xor = self.push_xor(&a[i], &b[i]);
            output.push(xor);
        }
        output
    }

    fn and(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        // repeat with output_indices
        let mut output = GateIndexVec::default();
        for i in 0..a.len() {
            let and = self.push_and(&a[i], &b[i]);
            output.push(and);
        }
        output
    }

    fn land(&mut self, a: &GateIndex, b: &GateIndex) -> GateIndex {
        // repeat with output_indices
        let mut output = GateIndexVec::default();
        let and = self.push_and(a, b);
        output.push(and);
        output.into()
    }

    fn not(&mut self, a: &GateIndexVec) -> GateIndexVec {
        let mut output = GateIndexVec::default();
        for i in 0..a.len() {
            let not_gate = self.push_not(&a[i]);
            output.push(not_gate);
        }
        output
    }

    fn or(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        // repeat with output_indices
        let mut output = GateIndexVec::default();
        for i in 0..a.len() {
            let or_gate = self.push_or(&a[i], &b[i]);
            output.push(or_gate);
        }
        output
    }

    fn lor(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndex {
        let output = self.or(a, b);
        output.into()
    }

    fn nand(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        // repeat with output_indices
        let mut output = GateIndexVec::default();
        for i in 0..a.len() {
            let nand = self.push_nand(&a[i], &b[i]);
            output.push(nand);
        }
        output
    }

    fn nor(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        // repeat with output_indices
        let mut output = GateIndexVec::default();
        for i in 0..a.len() {
            let nor = self.push_nor(&a[i], &b[i]);
            output.push(nor);
        }
        output
    }

    fn xnor(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        // repeat with output_indices
        let mut output = GateIndexVec::default();
        for i in 0..a.len() {
            let xnor = self.push_xnor(&a[i], &b[i]);
            output.push(xnor);
        }
        output
    }

    fn mux(&mut self, s: &GateIndex, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        // repeat with output_indices
        let mut output = GateIndexVec::default();
        for i in 0..a.len() {
            let mux = self.push_mux(s, &b[i], &a[i]);
            output.push(mux);
        }
        output
    }

    fn add(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        let mut carry = None;
        let mut output_indices = GateIndexVec::default();
        for i in 0..a.len() {
            let (sum, new_carry) = full_adder(self, a[i], b[i], carry);
            output_indices.push(sum);
            carry = new_carry;
        }
        output_indices
    }

    fn sub(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        let mut borrow = None;
        let mut output_indices = GateIndexVec::default();
        for i in 0..a.len() {
            let (diff, new_borrow) = full_subtractor(self, &a[i], &b[i], &borrow);
            output_indices.push(diff);
            borrow = new_borrow;
        }
        output_indices
    }

    fn mul(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        let mut partial_products: Vec<GateIndexVec> = Vec::with_capacity(a.len());

        // Generate partial products
        for i in 0..a.len() {
            let shifted_product = partial_product_shift(self, a, b, i);
            partial_products.push(shifted_product);
        }

        // Sum up all partial products
        let mut result = partial_products[0].clone();
        for partial_product in partial_products.iter().take(a.len()).skip(1) {
            result = self.add(&result, partial_product);
        }

        result
    }

    fn div(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        self.div_inner(a, b).0
    }

    fn rem(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        self.div_inner(a, b).1
    }

    fn eq(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndex {
        let mut eq_list = vec![0; a.len()];

        let i = a.len() - 1;
        let eq_i = self.push_xnor(&a[i], &b[i]);
        eq_list[i] = eq_i;

        for idx in (0..i).rev() {
            let xn = self.push_xnor(&a[idx], &b[idx]);
            let eq_i = self.push_and(&eq_list[idx + 1], &xn);
            eq_list[idx] = eq_i;
        }

        eq_list[0]
    }

    fn ne(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndex {
        let eq = self.eq(a, b);
        self.push_not(&eq)
    }

    fn gt(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndex {
        let (lt, eq) = self.compare(a, b);
        let or_gate = self.push_or(&lt, &eq);
        self.push_not(&or_gate)
    }

    fn ge(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndex {
        let lt = self.lt(a, b);
        self.push_not(&lt)
    }

    fn lt(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndex {
        let (lt, _eq) = self.compare(a, b);
        lt
    }

    fn le(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndex {
        let gt = self.gt(a, b);
        self.push_not(&gt)
    }

    fn compare(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> (GateIndex, GateIndex) {
        let mut eq_list = vec![0; a.len()];
        let mut lt_list = vec![0; a.len()];

        let i = a.len() - 1;
        let eq_i = self.push_xnor(&a[i], &b[i]);
        eq_list[i] = eq_i;

        let nt = self.push_not(&a[i]);
        let lt_i = self.push_and(&nt, &b[i]);
        lt_list[i] = lt_i;

        for idx in (0..i).rev() {
            let xn = self.push_xnor(&a[idx], &b[idx]);
            let eq_i = self.push_and(&eq_list[idx + 1], &xn);
            eq_list[idx] = eq_i;

            let nt = self.push_not(&a[idx]);
            let aa = self.push_and(&nt, &b[idx]);
            let temp_lt = self.push_and(&eq_list[idx + 1], &aa);
            lt_list[idx] = self.push_or(&lt_list[idx + 1], &temp_lt);
        }

        (lt_list[0], eq_list[0])
    }
}

macro_rules! build_and_execute {
    ($fn_name:ident, $op:ident) => {
        pub(crate) fn $fn_name<const N: usize>(
            lhs: &GarbledUint<N>,
            rhs: &GarbledUint<N>,
        ) -> GarbledUint<N> {
            let mut builder = WRK17CircuitBuilder::default();
            // Access the global CircuitBuilder instance
            //let mut builder = WRK17CircuitBuilder::instance().lock().unwrap();

            let a = builder.input(lhs);
            let b = builder.input(rhs);

            let output = builder.$op(&a, &b);
            let circuit = builder.compile(&output);

            // Execute the circuit
            builder
                .execute(&circuit)
                .expect("Failed to execute circuit")
        }
    };
}

build_and_execute!(build_and_execute_xor, xor);
build_and_execute!(build_and_execute_and, and);
build_and_execute!(build_and_execute_or, or);
build_and_execute!(build_and_execute_nand, nand);
build_and_execute!(build_and_execute_nor, nor);
build_and_execute!(build_and_execute_xnor, xnor);
build_and_execute!(build_and_execute_addition, add);
build_and_execute!(build_and_execute_subtraction, sub);
build_and_execute!(build_and_execute_multiplication, mul);
build_and_execute!(build_and_execute_division, div);
build_and_execute!(build_and_execute_remainder, rem);

fn full_adder(
    builder: &mut WRK17CircuitBuilder,
    a: GateIndex,
    b: GateIndex,
    carry: Option<GateIndex>,
) -> (GateIndex, Option<GateIndex>) {
    let xor_ab = builder.len();
    builder.gates.push(Gate::Xor(a, b));

    let sum = if let Some(c) = carry {
        let sum_with_carry = builder.len();
        builder.gates.push(Gate::Xor(xor_ab, c));
        sum_with_carry
    } else {
        xor_ab
    };

    let and_ab = builder.len();
    builder.gates.push(Gate::And(a, b));

    let new_carry = if let Some(c) = carry {
        let and_axorb_c = builder.len();
        builder.gates.push(Gate::And(xor_ab, c));

        let or_gate = builder.len();
        builder.gates.push(Gate::Xor(and_ab, and_axorb_c));
        Some(or_gate)
    } else {
        Some(and_ab)
    };

    (sum, new_carry)
}

fn full_subtractor(
    builder: &mut WRK17CircuitBuilder,
    a: &u32,
    b: &u32,
    borrow: &Option<u32>,
) -> (u32, Option<u32>) {
    // XOR gate for difference bit (a ⊕ b)
    let xor_ab = builder.push_xor(a, b);

    // If borrow exists, XOR the result of the previous XOR with the borrow
    let diff = if let Some(borrow) = borrow {
        builder.push_xor(&xor_ab, borrow)
    } else {
        xor_ab
    };

    // Compute the new borrow: (!a & b) | (a & borrow) | (!b & borrow)
    let not_a = builder.push_not(a);
    let and_not_a_b = builder.push_and(&not_a, b);

    let new_borrow = if let Some(borrow) = borrow {
        let and_a_borrow = builder.push_and(a, borrow);
        let not_b = builder.push_not(b);
        let and_not_b_borrow = builder.push_and(&not_b, borrow);

        // Combine borrow parts using XOR and Aa.len()D to simulate OR
        let xor_borrow_parts = builder.push_xor(&and_not_a_b, &and_a_borrow);
        builder.push_xor(&xor_borrow_parts, &and_not_b_borrow)
    } else {
        and_not_a_b
    };

    (diff, Some(new_borrow))
}

fn partial_product_shift(
    builder: &mut WRK17CircuitBuilder,
    lhs: &GateIndexVec,
    rhs: &GateIndexVec,
    shift: usize,
) -> GateIndexVec {
    let mut shifted = GateIndexVec::default();

    for i in 0..lhs.len() {
        if i < shift {
            // For the lower bits, we push a constant 0.
            let zero_bit = builder.push_not(&rhs[0]);
            let and_gate = builder.push_and(&rhs[0], &zero_bit); // Constant 0
            shifted.push(and_gate);
        } else {
            let lhs_bit = lhs[i - shift];
            let and_gate = builder.push_and(&lhs_bit, &(rhs[shift]));
            // Shift the bit from the input array
            shifted.push(and_gate);
        }
    }

    shifted
}

pub(crate) fn build_and_execute_equality<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> bool {
    let mut builder = WRK17CircuitBuilder::default();
    let a = builder.input(lhs);
    let b = builder.input(rhs);

    let result = builder.eq(&a, &b);
    let result = builder
        .compile_and_execute::<1>(&vec![result].into())
        .expect("Failed to execute equality circuit");
    result.into()
}

pub(crate) fn build_and_execute_comparator<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> Ordering {
    let mut builder = WRK17CircuitBuilder::default();
    let a = builder.input(lhs);
    let b = builder.input(rhs);

    let (lt_output, eq_output) = builder.compare(&a, &b);

    let result = builder
        .compile_and_execute::<2>(&vec![lt_output, eq_output].into())
        .expect("Failed to execute equality circuit");

    let lt = result.bits[0];
    let eq = result.bits[1];

    if lt {
        Ordering::Less
    } else if eq {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}

pub(crate) fn build_and_execute_not<const N: usize>(input: &GarbledUint<N>) -> GarbledUint<N> {
    let mut builder = WRK17CircuitBuilder::default();
    builder.input(input);

    let mut output_indices = GateIndexVec::default();

    let n = N as u32;
    for i in 0..n {
        let not_gate = builder.push_not(&i);
        output_indices.push(not_gate);
    }

    builder
        .compile_and_execute(&output_indices)
        .expect("Failed to execute a.len()OT circuit")
}

pub(crate) fn build_and_execute_mux<const N: usize>(
    condition: &GarbledBoolean,
    if_true: &GarbledUint<N>,
    if_false: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = WRK17CircuitBuilder::default();
    let a = builder.input(if_true);
    let b = builder.input(if_false);
    let s = builder.input(condition);

    // Add MUX gates for each bit
    /*
    let mut output_indices = Vec::with_capacity(a.len());
    let n = a.len() as u32;
    for i in 0..n {
        let mux_gate = builder.push_mux(&i, &(n + i), &(2 * n));
        output_indices.push(mux_gate);
    }
    */

    let output = builder.mux(&s[0], &a, &b);

    // Simulate the circuit
    builder
        .compile_and_execute(&output)
        .expect("Failed to execute MUX circuit")
}

// tests
#[cfg(test)]
mod tests {
    use tracing::debug;

    use super::*;
    use crate::uint::{
        GarbledBit, GarbledUint128, GarbledUint16, GarbledUint32, GarbledUint64, GarbledUint8,
    };

    #[test]
    fn test_div() {
        let a: GarbledUint8 = 10_u8.into();
        let b: GarbledUint8 = 2_u8.into();

        let result = build_and_execute_division(&a, &b);
        let result_value: u8 = result.into();
        assert_eq!(result_value, 10 / 2);
    }

    #[test]
    fn test_rem() {
        let a: GarbledUint8 = 10_u8.into();
        let b: GarbledUint8 = 3_u8.into();

        let result = build_and_execute_remainder(&a, &b);
        let result_value: u8 = result.into();
        assert_eq!(result_value, 10 % 3);
    }

    // Test several u8 (8-bit) cases.
    #[test]
    fn test_div_rem_u8_complex() {
        let test_cases: &[(u8, u8, u8, u8)] = &[
            (10, 2, 5, 0),
            (250, 3, 83, 1),
            (255, 3, 85, 0),
            (200, 7, 28, 4),
            (123, 5, 24, 3),
            (17, 4, 4, 1),
        ];
        for &(dividend, divisor, expected_q, expected_r) in test_cases {
            let a: GarbledUint8 = dividend.into();
            let b: GarbledUint8 = divisor.into();
            let quotient = build_and_execute_division(&a, &b);
            let remainder = build_and_execute_remainder(&a, &b);
            let q: u8 = quotient.into();
            let r: u8 = remainder.into();
            println!(
                "dividing {} by {}: quotient {} remainder {}",
                dividend, divisor, q, r
            );
            assert_eq!(
                q, expected_q,
                "Incorrect quotient for {} / {}",
                dividend, divisor
            );
            assert_eq!(
                r, expected_r,
                "Incorrect remainder for {} / {}",
                dividend, divisor
            );
        }
    }

    // Test several u16 (16-bit) cases.
    #[test]
    fn test_div_rem_u16_complex() {
        let test_cases: &[(u16, u16, u16, u16)] = &[
            (1000, 7, 142, 6),
            (12345, 123, 100, 45),
            (65535, 255, 257, 0),
            (4321, 13, 332, 5),
            (100, 3, 33, 1),
        ];
        for &(dividend, divisor, expected_q, expected_r) in test_cases {
            let a: GarbledUint16 = dividend.into();
            let b: GarbledUint16 = divisor.into();
            let quotient = build_and_execute_division(&a, &b);
            let remainder = build_and_execute_remainder(&a, &b);
            let q: u16 = quotient.into();
            let r: u16 = remainder.into();
            println!(
                "dividing {} by {}: quotient {} remainder {}",
                dividend, divisor, q, r
            );
            assert_eq!(
                q, expected_q,
                "Incorrect quotient for {} / {}",
                dividend, divisor
            );
            assert_eq!(
                r, expected_r,
                "Incorrect remainder for {} / {}",
                dividend, divisor
            );
        }
    }

    #[test]
    fn test_div_rem_u32_complex() {
        let test_cases: &[(u32, u32, u32, u32)] = &[
            (1_000_000_000, 3, 333_333_333, 1),
            (2863311530, 7, 409044504, 2),
            (123456789, 12345, 10000, 6789),
        ];
        for &(dividend, divisor, expected_q, expected_r) in test_cases {
            let a: GarbledUint32 = dividend.into();
            let b: GarbledUint32 = divisor.into();
            let quotient = build_and_execute_division(&a, &b);
            let remainder = build_and_execute_remainder(&a, &b);
            let q: u32 = quotient.into();
            let r: u32 = remainder.into();
            println!(
                "dividing {} by {}: quotient {} remainder {}",
                dividend, divisor, q, r
            );
            assert_eq!(
                q, expected_q,
                "Incorrect quotient for {} / {}",
                dividend, divisor
            );
            assert_eq!(
                r, expected_r,
                "Incorrect remainder for {} / {}",
                dividend, divisor
            );
        }
    }

    #[test]
    fn test_div_rem_u64_complex() {
        let test_cases: &[(u64, u64, u64, u64)] = &[
            (u64::MAX, 3, 6148914691236517205, 0),
            (100_000_000_000, 3, 33_333_333_333, 1),
        ];
        for &(dividend, divisor, expected_q, expected_r) in test_cases {
            let a: GarbledUint64 = dividend.into();
            let b: GarbledUint64 = divisor.into();
            let quotient = build_and_execute_division(&a, &b);
            let remainder = build_and_execute_remainder(&a, &b);
            let q: u64 = quotient.into();
            let r: u64 = remainder.into();
            println!(
                "dividing {} by {}: quotient {} remainder {}",
                dividend, divisor, q, r
            );
            assert_eq!(
                q, expected_q,
                "Incorrect quotient for {} / {}",
                dividend, divisor
            );
            assert_eq!(
                r, expected_r,
                "Incorrect remainder for {} / {}",
                dividend, divisor
            );
        }
    }

    #[test]
    fn test_div_rem_u128_complex() {
        let test_cases: &[(u128, u128, u128, u128)] = &[
            (12297829382473034410u128, 3, 4099276460824344803, 1),
            (500u128, 7, 71, 3),
        ];
        for &(dividend, divisor, expected_q, expected_r) in test_cases {
            let a: GarbledUint128 = dividend.into();
            let b: GarbledUint128 = divisor.into();
            let quotient = build_and_execute_division(&a, &b);
            let remainder = build_and_execute_remainder(&a, &b);
            let q: u128 = quotient.into();
            let r: u128 = remainder.into();
            println!(
                "dividing {} by {}: quotient {} remainder {}",
                dividend, divisor, q, r
            );
            assert_eq!(
                q, expected_q,
                "Incorrect quotient for {} / {}",
                dividend, divisor
            );
            assert_eq!(
                r, expected_r,
                "Incorrect remainder for {} / {}",
                dividend, divisor
            );
        }
    }

    #[test]
    fn test_build_and_execute_mux1() {
        let s: GarbledBit = true.into();
        let a: GarbledBit = false.into();
        let b: GarbledBit = true.into();

        let result = build_and_execute_mux(&s, &a, &b);
        assert_eq!(result, a);

        let s: GarbledBit = false.into();
        let result = build_and_execute_mux(&s, &a, &b);
        assert_eq!(result, b);
    }

    #[ignore = "mixed bits not supported yet"]
    #[test]
    fn test_build_and_execute_mux() {
        let s: GarbledBit = true.into();
        let a: GarbledUint8 = 170_u8.into();
        let b: GarbledUint8 = 85_u8.into();

        let result = build_and_execute_mux(&s, &a, &b);
        assert_eq!(result, a);

        let s: GarbledBit = false.into();
        let result = build_and_execute_mux(&s, &a, &b);
        assert_eq!(result, b);
    }

    #[test]
    fn test_build_and_execute_mux32() {
        let s: GarbledBoolean = true.into();
        let a: GarbledUint32 = 28347823_u32.into();
        let b: GarbledUint32 = 8932849_u32.into();

        let result = build_and_execute_mux(&s, &a, &b);
        assert_eq!(result, a);

        let result = build_and_execute_mux(&false.into(), &a, &b);
        assert_eq!(result, b);
    }

    #[test]
    fn test_build_and_execute_mux64() {
        let s: GarbledBoolean = true.into();
        let a: GarbledUint64 = 23948323290804923_u64.into();
        let b: GarbledUint64 = 834289823983634323_u64.into();

        let result = build_and_execute_mux(&s, &a, &b);
        assert_eq!(result, a);

        let result = build_and_execute_mux(&false.into(), &a, &b);
        assert_eq!(result, b);
    }

    #[test]
    fn test_build_and_execute_multiplication() {
        let a: GarbledUint8 = 9_u8.into();
        let b: GarbledUint8 = 3_u8.into();

        let result = build_and_execute_multiplication(&a, &b);
        let result_value: u8 = result.into();
        assert_eq!(result_value, 9 * 3);
    }

    #[test]
    fn test_eq_true() {
        let a: GarbledUint8 = 42_u8.into();
        let b: GarbledUint8 = 42_u8.into();

        let mut builder = WRK17CircuitBuilder::default();
        let a = builder.input(&a);
        let b = builder.input(&b);

        let output = builder.eq(&a, &b);

        let circuit = builder.compile(&vec![output].into());
        let result = builder
            .execute::<1>(&circuit)
            .expect("Failed to execute equality circuit");
        let result_value: bool = result.into();
        assert!(result_value);
    }

    #[test]
    fn test_eq_false() {
        let a: GarbledUint8 = 123_u8.into();
        let b: GarbledUint8 = 124_u8.into();

        let mut builder = WRK17CircuitBuilder::default();
        let a = builder.input(&a);
        let b = builder.input(&b);

        let output = builder.eq(&a, &b);

        let circuit = builder.compile(&vec![output].into());
        let result = builder
            .execute::<1>(&circuit)
            .expect("Failed to execute equality circuit");
        let result_value: bool = result.into();
        assert!(!result_value);
    }

    #[test]
    fn test_ne_true() {
        let a: GarbledUint8 = 123_u8.into();
        let b: GarbledUint8 = 124_u8.into();

        let mut builder = WRK17CircuitBuilder::default();
        let a = builder.input(&a);
        let b = builder.input(&b);

        let output = builder.ne(&a, &b);

        let circuit = builder.compile(&vec![output].into());
        let result = builder
            .execute::<1>(&circuit)
            .expect("Failed to execute inequality circuit");
        let result_value: bool = result.into();
        assert!(result_value);
    }

    #[test]
    fn test_ne_false() {
        let a: GarbledUint8 = 42_u8.into();
        let b: GarbledUint8 = 42_u8.into();

        let mut builder = WRK17CircuitBuilder::default();
        let a = builder.input(&a);
        let b = builder.input(&b);

        let output = builder.ne(&a, &b);

        let circuit = builder.compile(&vec![output].into());
        let result = builder
            .execute::<1>(&circuit)
            .expect("Failed to execute inequality circuit");
        let result_value: bool = result.into();
        assert!(!result_value);
    }

    #[test]
    fn test_lt_true() {
        let a: GarbledUint8 = 42_u8.into();
        let b: GarbledUint8 = 43_u8.into();

        let mut builder = WRK17CircuitBuilder::default();
        let a = builder.input(&a);
        let b = builder.input(&b);

        let output = builder.lt(&a, &b);

        let circuit = builder.compile(&vec![output].into());
        let result = builder
            .execute::<1>(&circuit)
            .expect("Failed to execute less than circuit");
        let result_value: bool = result.into();
        assert!(result_value);
    }

    #[test]
    fn test_lt_false() {
        let a: GarbledUint8 = 43_u8.into();
        let b: GarbledUint8 = 42_u8.into();

        let mut builder = WRK17CircuitBuilder::default();
        let a = builder.input(&a);
        let b = builder.input(&b);

        let output = builder.lt(&a, &b);

        let circuit = builder.compile(&vec![output].into());
        let result = builder
            .execute::<1>(&circuit)
            .expect("Failed to execute less than circuit");
        let result_value: bool = result.into();
        assert!(!result_value);
    }

    #[test]
    fn test_build_and_execute_mixed() {
        fn build_and_execute_mixed<const N: usize>(
            lhs: &GarbledUint<N>,
            rhs: &GarbledUint<N>,
        ) -> GarbledUint<N> {
            let mut builder = WRK17CircuitBuilder::default();
            let a = builder.input(lhs);
            let b = builder.input(rhs);

            // Create a full adder for each bit
            //let add_output = builder.add(&a, &b).0;
            //let sub_output = builder.sub(&add_output, &b).0;
            //let output = builder.or(&sub_output, &a);

            let output = builder.mul(&a, &b);
            let output = builder.mul(&output, &a);

            println!("output: {:?}", output);
            // debug gates
            builder.gates.iter().for_each(|gate| {
                println!("{:?}", gate);
            });

            let circuit = builder.compile(&output);

            // Execute the circuit
            builder
                .execute(&circuit)
                .expect("Failed to execute addition circuit")
        }

        let a: GarbledUint8 = 2_u8.into();
        let b: GarbledUint8 = 5_u8.into();

        let result = build_and_execute_mixed(&a, &b);
        let result_value: u8 = result.into();
        assert_eq!(result_value, 2 * 5 * 2);
    }

    #[test]
    fn test_add_three() {
        let mut builder = WRK17CircuitBuilder::default();
        let a: GarbledUint8 = 2_u8.into();
        let a = builder.input(&a);

        let b: GarbledUint8 = 5_u8.into();
        let b = builder.input(&b);

        let c: GarbledUint8 = 3_u8.into();
        let c = builder.input(&c);

        let output = builder.add(&a, &b);
        let output = builder.add(&output, &c);

        debug!("{:#?}", builder);

        let circuit = builder.compile(&output);

        // Execute the circuit
        let result = builder
            .execute::<8>(&circuit)
            .expect("Failed to execute addition circuit");

        let result_value: u8 = result.into();
        assert_eq!(result_value, 2 + 5 + 3);
    }

    #[test]
    fn test_embedded_if_else() {
        let mut builder = WRK17CircuitBuilder::default();
        let a: GarbledUint8 = 2_u8.into();
        let a = builder.input(&a);

        let b: GarbledUint8 = 5_u8.into();
        let b = builder.input(&b);

        let s: GarbledBoolean = false.into();
        let s: GateIndexVec = builder.input(&s);

        // fails with 'cannot borrow `builder` as mutable more than once at a time'
        // let output = builder.mux(s, builder.mul(a.clone(), b.clone()), builder.add(a.clone(), b.clone()));

        let if_true = builder.mul(&a, &b);
        let if_false = builder.add(&a, &b);
        let output = builder.mux(&s[0], &if_true, &if_false);

        println!("output: {:?}", output);

        let circuit = builder.compile(&output);

        // Execute the circuit
        let result = builder
            .execute::<8>(&circuit)
            .expect("Failed to execute addition circuit");

        let result_value: u8 = result.into();
        assert_eq!(result_value, 2 + 5);
    }

    #[test]
    fn test_constant_caching() {
        let mut builder = WRK17CircuitBuilder::default();

        // Get constant 1 twice
        let one: GarbledUint32 = 1u32.into();
        let wire1 = builder.constant::<32>(&one);
        let wire2 = builder.constant::<32>(&one);

        // Verify we got the same wire indices
        assert_eq!(wire1, wire2);

        // Verify the cache size is 1 (not 2)
        assert_eq!(builder.constant_cache.len(), 1);
    }

    #[test]
    fn test_constant_in_circuit() {
        let mut builder = WRK17CircuitBuilder::default();

        // Create a circuit that compares a variable with constant 1
        let var: GarbledUint32 = 1u32.into();
        let var_wire = builder.input(&var);

        let one: GarbledUint32 = 1u32.into();
        let const_wire = builder.constant::<32>(&one);

        // Take first 32 bits of const_wire for comparison
        let const_wire_32 = GateIndexVec::new(const_wire.iter().take(32).copied().collect());

        // Compare them for equality
        let eq = builder.eq(&var_wire, &const_wire_32);

        let circuit = builder.compile(&vec![eq].into());
        let result = builder
            .execute::<1>(&circuit)
            .expect("Failed to execute circuit");

        // Should be equal
        assert!(bool::from(result));
    }
}
