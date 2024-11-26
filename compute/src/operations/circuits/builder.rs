use crate::operations::circuits::traits::CircuitExecutor;
use crate::operations::circuits::types::GateIndexVec;
use crate::uint::GarbledUint;
use crate::{executor::get_executor, uint::GarbledBoolean};
use std::cmp::Ordering;
use std::fmt::Debug;
use tandem::{Circuit, Gate};

pub type GateIndex = u32;

#[derive(Default)]
pub struct WRK17CircuitBuilder {
    inputs: Vec<bool>,
    gates: Vec<Gate>,
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

        let input_offset = self.inputs.len();
        let mut input_label = GateIndexVec::default();
        for (i, bool_value) in input.bits.iter().enumerate() {
            self.gates.insert(0, Gate::InContrib);

            self.inputs.push(*bool_value);
            input_label.push((input_offset + i) as GateIndex);
        }
        input_label
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

    fn div_inner(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> (GateIndexVec, GateIndexVec) {
        let n = a.len();
        let mut quotient = GateIndexVec::default();
        let mut remainder = GateIndexVec::default();

        // Initialize remainder with 0
        for _ in 0..n {
            remainder.push(GateIndex::default()); // Zero initialize
        }

        // Iterate through each bit, starting from the most significant
        for i in (0..n).rev() {
            // Shift remainder left by 1 (equivalent to adding a bit)
            remainder.insert(0, a[i]);
            if remainder.len() > n {
                remainder.truncate(n); // Ensure remainder does not exceed bit width
            }

            // Check if remainder is greater than or equal to divisor
            let greater_or_equal = self.ge(&remainder, b);

            // If remainder is greater than or equal to divisor, set quotient bit to 1 and subtract divisor from remainder
            if greater_or_equal != GateIndex::default() {
                // Subtract divisor from remainder if it’s greater than or equal
                let new_remainder = self.sub(&remainder, b);
                remainder = self.mux(&greater_or_equal, &new_remainder, &remainder);

                // Set quotient bit to 1
                quotient.insert(0, greater_or_equal);
            } else {
                // Set quotient bit to 0
                quotient.insert(0, GateIndex::default());
            }

            if quotient.len() > n {
                quotient.truncate(n); // Ensure quotient does not exceed bit width
            }
        }

        (quotient, remainder)
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
    use crate::uint::GarbledBit;
    use crate::uint::GarbledUint32;
    use crate::uint::GarbledUint64;
    use crate::uint::GarbledUint8;

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
}
