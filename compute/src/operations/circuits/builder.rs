use crate::executor::get_executor;
use crate::operations::circuits::types::GateIndexVec;
use crate::uint::GarbledUint;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::thread;
use tandem::GateIndex;
use tandem::{Circuit, Gate};

// Global instance of CircuitBuilder
thread_local! {
    pub(super) static CIRCUIT_BUILDER: RefCell<CircuitBuilder> = RefCell::new(CircuitBuilder::default());
}

static CIRCUIT_BUILDER2: OnceLock<Mutex<CircuitBuilder>> = OnceLock::new();

#[derive(Default)]
pub struct CircuitBuilder {
    inputs: Vec<bool>,
    gates: Vec<Gate>,
}

impl CircuitBuilder {
    // Provides a global accessor to a mutable `CircuitBuilder`
    pub fn global() -> &'static Mutex<CircuitBuilder> {
        CIRCUIT_BUILDER2.get_or_init(|| Mutex::new(CircuitBuilder::default()))
    }
    pub fn input<const R: usize>(&mut self, input: &GarbledUint<R>) -> GateIndexVec {
        // get the cumulative size of all inputs in input_labels
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

    pub fn xor(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        // repeat with output_indices
        let mut output = GateIndexVec::default();
        for i in 0..a.len() {
            let xor = self.push_xor(&a[i], &b[i]);
            output.push(xor);
        }
        output
    }

    // Add an Aa.len()D gate between two inputs and return the index
    pub fn push_and(&mut self, a: &GateIndex, b: &GateIndex) -> GateIndex {
        let and_index = self.gates.len() as u32;
        self.gates.push(Gate::And(*a, *b));
        and_index
    }

    pub fn and(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        // repeat with output_indices
        let mut output = GateIndexVec::default();
        for i in 0..a.len() {
            let and = self.push_and(&a[i], &b[i]);
            output.push(and);
        }
        output
    }

    // Add a a.len()OT gate for a single input and return the index
    pub fn push_not(&mut self, a: &GateIndex) -> GateIndex {
        let not_index = self.gates.len() as u32;
        self.gates.push(Gate::Not(*a));
        not_index
    }

    pub fn not(&mut self, a: &GateIndexVec) -> GateIndexVec {
        let mut output = GateIndexVec::default();
        for i in 0..a.len() {
            let not = self.push_not(&a[i]);
            output.push(not);
        }
        output
    }

    // Add a gate for OR operation: OR(a, b) = (a ⊕ b) ⊕ (a & b)
    pub fn push_or(&mut self, a: &GateIndex, b: &GateIndex) -> GateIndex {
        let xor_gate = self.push_xor(a, b);
        let and_gate = self.push_and(a, b);
        self.push_xor(&xor_gate, &and_gate)
    }

    pub fn or(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        // repeat with output_indices
        let mut output = GateIndexVec::default();
        for i in 0..a.len() {
            let or_gate = self.push_or(&a[i], &b[i]);
            output.push(or_gate);
        }
        output
    }

    // Add a a.len()Aa.len()D gate: a.len()Aa.len()D(a, b) = a.len()OT(a & b)
    pub fn push_nand(&mut self, a: &GateIndex, b: &GateIndex) -> GateIndex {
        let and_gate = self.push_and(a, b);
        self.push_not(&and_gate)
    }

    pub fn nand(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        // repeat with output_indices
        let mut output = GateIndexVec::default();
        for i in 0..a.len() {
            let nand = self.push_nand(&a[i], &b[i]);
            output.push(nand);
        }
        output
    }

    pub fn push_nor(&mut self, a: &GateIndex, b: &GateIndex) -> GateIndex {
        let or_gate = self.push_or(a, b);
        self.push_not(&or_gate)
    }

    pub fn nor(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        // repeat with output_indices
        let mut output = GateIndexVec::default();
        for i in 0..a.len() {
            let nor = self.push_nor(&a[i], &b[i]);
            output.push(nor);
        }
        output
    }

    // Add an Xa.len()OR gate: Xa.len()OR(a, b) = a.len()OT(a ⊕ b)
    pub fn push_xnor(&mut self, a: &GateIndex, b: &GateIndex) -> GateIndex {
        let xor_gate = self.push_xor(a, b);
        self.push_not(&xor_gate)
    }

    pub fn xnor(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        // repeat with output_indices
        let mut output = GateIndexVec::default();
        for i in 0..a.len() {
            let xnor = self.push_xnor(&a[i], &b[i]);
            output.push(xnor);
        }
        output
    }

    pub fn mux(&mut self, s: &GateIndexVec, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        // repeat with output_indices
        let mut output = GateIndexVec::default();
        for i in 0..a.len() {
            let mux = self.push_mux(&s[0], &b[i], &a[i]);
            output.push(mux);
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

    pub fn add(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        let mut carry = None;
        let mut output_indices = GateIndexVec::default();
        for i in 0..a.len() {
            let (sum, new_carry) = full_adder(self, a[i], b[i], carry);
            output_indices.push(sum);
            carry = new_carry;
        }
        output_indices
    }

    pub fn sub(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
        let mut borrow = None;
        let mut output_indices = GateIndexVec::default();
        for i in 0..a.len() {
            let (diff, new_borrow) = full_subtractor(self, &a[i], &b[i], &borrow);
            output_indices.push(diff);
            borrow = new_borrow;
        }
        output_indices
    }

    pub fn mul(&mut self, a: &GateIndexVec, b: &GateIndexVec) -> GateIndexVec {
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

    pub fn div(&mut self, _a: &GateIndexVec, _b: &GateIndexVec) -> GateIndexVec {
        unimplemented!()
    }

    pub fn rem(&mut self, _a: &GateIndexVec, _b: &GateIndexVec) -> GateIndexVec {
        unimplemented!()
    }

    pub fn compare<const N: usize>(&mut self) -> (u32, u32) {
        let mut eq_list = vec![0; N];
        let mut lt_list = vec![0; N];

        let n = N as u32;
        let i = n - 1;
        let eq_i = self.push_xnor(&i, &(n + i));
        eq_list[i as usize] = eq_i;

        let nt = self.push_not(&i);
        let lt_i = self.push_and(&nt, &(n + i));
        lt_list[i as usize] = lt_i;

        for idx in (0..i).rev() {
            let xn = self.push_xnor(&idx, &(n + idx));
            let eq_i = self.push_and(&eq_list[(idx + 1) as usize], &xn);
            eq_list[idx as usize] = eq_i;

            let nt = self.push_not(&idx);
            let aa = self.push_and(&nt, &(n + idx));
            let temp_lt = self.push_and(&eq_list[(idx + 1) as usize], &aa);
            lt_list[idx as usize] = self.push_or(&lt_list[(idx + 1) as usize], &temp_lt);
        }

        (lt_list[0], eq_list[0])
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

macro_rules! build_and_execute {
    ($fn_name:ident, $op:ident) => {
        pub(crate) fn $fn_name<const N: usize>(
            lhs: &GarbledUint<N>,
            rhs: &GarbledUint<N>,
        ) -> GarbledUint<N> {
            //let mut builder = CircuitBuilder::default();
            // Access the global CircuitBuilder instance
            // Clone the inputs to ensure they have a 'static lifetime
            let lhs_clone = lhs.clone();
            let rhs_clone = rhs.clone();
            thread::spawn(move || {
                let mut builder = CircuitBuilder::default();
                let a = builder.input(&lhs_clone);
                let b = builder.input(&rhs_clone);

                let output = builder.$op(&a, &b);
                let circuit = builder.compile(&output);

                // Execute the circuit
                builder
                    .execute(&circuit)
                    .expect("Failed to execute circuit")
            })
            .join()
            .unwrap()

            /*
            thread::spawn(move || {
                CIRCUIT_BUILDER.with(|builder| {
                    let a = builder.borrow_mut().input(&lhs_clone);
                    let b = builder.borrow_mut().input(&rhs_clone);

                    let output = builder.borrow_mut().$op(&a, &b);
                    let circuit = builder.borrow_mut().compile(&output);

                    // Execute the circuit
                    builder
                        .borrow_mut()
                        .execute(&circuit)
                        .expect("Failed to execute circuit")
                })
            })
            .join()
            .unwrap()
            */
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

fn full_adder(
    builder: &mut CircuitBuilder,
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
    builder: &mut CircuitBuilder,
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
    builder: &mut CircuitBuilder,
    lhs: &GateIndexVec,
    rhs: &GateIndexVec,
    shift: usize,
) -> GateIndexVec {
    let mut shifted = GateIndexVec::default();

    for i in 0..lhs.len() {
        if i < shift {
            // For the lower bits, we push a constant 0.
            let zero_bit = builder.len();
            builder.push_not(&rhs[0]);
            let _zero = builder.push_and(&rhs[0], &zero_bit); // Constant 0
            shifted.push(builder.len() - 1);
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
    let mut builder = CircuitBuilder::default();
    builder.input(lhs);
    builder.input(rhs);

    let n = lhs.len() as u32;
    let mut result = builder.push_xnor(&0, &n);

    for i in 1..n {
        let current_comparison = builder.push_xnor(&i, &(n + i));
        result = builder.push_and(&result, &current_comparison);
    }
    let result = builder
        .compile_and_execute::<N>(&vec![result].into())
        .unwrap();
    result.bits[0]
}

pub(crate) fn build_and_execute_comparator<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> Ordering {
    let mut builder = CircuitBuilder::default();
    builder.input(lhs);
    builder.input(rhs);

    let (lt_output, eq_output) = builder.compare::<N>();

    let program = builder.compile(&vec![lt_output, eq_output].into());
    let input = [lhs.bits.clone(), rhs.bits.clone()].concat();
    let result = get_executor().execute(&program, &input, &[]).unwrap();

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

pub(crate) fn build_and_execute_not<const N: usize>(input: &GarbledUint<N>) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
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

#[allow(dead_code)]
pub(crate) fn build_and_execute_mux<const N: usize, const S: usize>(
    condition: &GarbledUint<S>,
    if_true: &GarbledUint<N>,
    if_false: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
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

    let output = builder.mux(&s, &a, &b);

    // Simulate the circuit
    builder
        .compile_and_execute(&output)
        .expect("Failed to execute MUX circuit")
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::uint::GarbledBit;
    use crate::uint::GarbledUint32;
    use crate::uint::GarbledUint64;
    use crate::uint::GarbledUint8;

    #[test]
    fn test1() {
        fn build_and_execute_mixed<const N: usize>(
            lhs: &GarbledUint<N>,
            rhs: &GarbledUint<N>,
        ) -> GarbledUint<N> {
            //let mut builder = CircuitBuilder::default();
            let lhs_clone = lhs.clone();
            let rhs_clone = rhs.clone();
            thread::spawn(move || {
                CIRCUIT_BUILDER.with(|builder| {
                    let a = builder.borrow_mut().input(&lhs_clone);
                    let b = builder.borrow_mut().input(&rhs_clone);

                    let output = &a + &b;
                    let output = &output + &a;

                    // Create a full adder for each bit
                    //let add_output = builder.add(&a, &b).0;
                    //let sub_output = builder.sub(&add_output, &b).0;
                    //let output = builder.or(&sub_output, &a);

                    //let output = builder.mul(&a, &b);
                    //let output = builder.mul(&output, &a);

                    //let output = builder.add(&a, &b);
                    let circuit = builder.borrow_mut().compile(&output);

                    // Execute the circuit
                    builder
                        .borrow_mut()
                        .execute(&circuit)
                        .expect("Failed to execute addition circuit")
                })
            })
            .join()
            .unwrap()
        }

        let a: GarbledUint8 = 2_u8.into();
        let b: GarbledUint8 = 5_u8.into();

        let result = build_and_execute_mixed(&a, &b);
        let result_value: u8 = result.into();
        assert_eq!(result_value, 2 + 5 + 2);
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
        let s: GarbledUint32 = 1_u32.into();
        let a: GarbledUint32 = 28347823_u32.into();
        let b: GarbledUint32 = 8932849_u32.into();

        let result = build_and_execute_mux(&s, &a, &b);
        assert_eq!(result, a);

        let s: GarbledUint32 = 0_u32.into();
        let result = build_and_execute_mux(&s, &a, &b);
        assert_eq!(result, b);
    }

    #[test]
    fn test_build_and_execute_mux64() {
        let s: GarbledUint64 = 1_u64.into();
        let a: GarbledUint64 = 23948323290804923_u64.into();
        let b: GarbledUint64 = 834289823983634323_u64.into();

        let result = build_and_execute_mux(&s, &a, &b);
        assert_eq!(result, a);

        let s: GarbledUint64 = 0_u64.into();
        let result = build_and_execute_mux(&s, &a, &b);
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
    fn test_build_and_execute_mixed() {
        fn build_and_execute_mixed<const N: usize>(
            lhs: &GarbledUint<N>,
            rhs: &GarbledUint<N>,
        ) -> GarbledUint<N> {
            let mut builder = CircuitBuilder::default();
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
        let mut builder = CircuitBuilder::default();
        let a: GarbledUint8 = 2_u8.into();
        let a = builder.input(&a);

        let b: GarbledUint8 = 5_u8.into();
        let b = builder.input(&b);

        let c: GarbledUint8 = 3_u8.into();
        let c = builder.input(&c);

        let output = builder.add(&a, &b);
        let output = builder.add(&output, &c);

        println!("output: {:?}", output);
        // debug gates
        builder.gates.iter().for_each(|gate| {
            println!("{:?}", gate);
        });

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
        let mut builder = CircuitBuilder::default();
        let a: GarbledUint8 = 2_u8.into();
        let a = builder.input(&a);

        let b: GarbledUint8 = 5_u8.into();
        let b = builder.input(&b);

        let s: GarbledUint8 = 0_u8.into();
        let s: GateIndexVec = builder.input(&s);

        // fails with 'cannot borrow `builder` as mutable more than once at a time'
        // let output = builder.mux(s, builder.mul(a.clone(), b.clone()), builder.add(a.clone(), b.clone()));

        let if_true = builder.mul(&a, &b);
        let if_false = builder.add(&a, &b);
        let output = builder.mux(&s, &if_true, &if_false);

        println!("output: {:?}", output);

        let circuit = builder.compile(&output);

        // Execute the circuit
        let result = builder
            .execute::<8>(&circuit)
            .expect("Failed to execute addition circuit");

        let result_value: u8 = result.into();
        assert_eq!(result_value, 2 + 5);
    }

    use circuit_macro::circuit;

    #[test]
    fn test_macro_arithmetic() {
        let a = 2_u8;
        let b = 5_u8;
        let c = 3_u8;
        let d = 4_u8;

        let result_u8 = my_circuit(&2u8, &3u8, &1u8, &4u8);
        println!("Result for u8: {}", result_u8);

        let result: u8 = my_circuit(&a, &b, &c, &d);
        assert_eq!(result, a * b + c - d);

        let result = my_circuit_from_macro(&a, &b, &c, &d);
        assert_eq!(result, a * b + c - d);

        let result = my_circuit_from_macro2(&a, &b, &c, &d);
        assert_eq!(result, a * b + c - d);
    }

    #[circuit(execute)]
    fn my_circuit_from_macro(a: &T, b: &T, c: &T, d: &T) -> T {
        let res = a * b;
        let res = res + c;
        res - d
    }

    fn my_circuit_from_macro2<T>(a: &T, b: &T, c: &T, d: &T) -> T
    where
        T: Into<GarbledUint<8>>
            + From<GarbledUint<8>>
            + Into<GarbledUint<16>>
            + From<GarbledUint<16>>
            + Into<GarbledUint<32>>
            + From<GarbledUint<32>>
            + Into<GarbledUint<64>>
            + From<GarbledUint<64>>
            + Into<GarbledUint<128>>
            + From<GarbledUint<128>>
            + Clone,
    {
        fn generate<const N: usize, T>(a: &T, b: &T, c: &T, d: &T) -> T
        where
            T: Into<GarbledUint<N>> + From<GarbledUint<N>> + Clone,
        {
            let mut context = CircuitBuilder::default();
            let a = &context.input(&a.clone().into());
            let b = &context.input(&b.clone().into());
            let c = &context.input(&c.clone().into());
            let d = &context.input(&d.clone().into());
            let output = {
                {
                    let res = &context.mul(a, b);
                    let res = &context.add(res, c);
                    &context.sub(res, d)
                }
            };
            let compiled_circuit = context.compile(output);
            let result = context
                .execute::<N>(&compiled_circuit)
                .expect("Failed to execute the circuit");
            result.into()
        }
        match std::any::type_name::<T>() {
            "u8" => generate::<8, T>(a, b, c, d),
            "u16" => generate::<16, T>(a, b, c, d),
            "u32" => generate::<32, T>(a, b, c, d),
            "u64" => generate::<64, T>(a, b, c, d),
            "u128" => generate::<128, T>(a, b, c, d),
            _ => panic!("Unsupported type"),
        }
    }

    fn my_circuit<T>(a: &T, b: &T, c: &T, d: &T) -> T
    where
        T: Into<GarbledUint<8>>
            + From<GarbledUint<8>>
            + Into<GarbledUint<16>>
            + From<GarbledUint<16>>
            + Into<GarbledUint<32>>
            + From<GarbledUint<32>>
            + Into<GarbledUint<64>>
            + From<GarbledUint<64>>
            + Into<GarbledUint<128>>
            + From<GarbledUint<128>>
            + Clone,
    {
        fn generate<const N: usize, T>(a: &T, b: &T, c: &T, d: &T) -> T
        where
            T: Into<GarbledUint<N>> + From<GarbledUint<N>> + Clone,
        {
            let mut context = CircuitBuilder::default();
            //let a = &2_u8;
            let a = &context.input(&a.clone().into());
            let b = &context.input(&b.clone().into());
            let c = &context.input(&c.clone().into());
            let d = &context.input(&d.clone().into());

            let output = {
                let res = a * b + c - d;
                let res = &res + &b;
                res
            };

            let compiled_circuit = context.compile(&output);
            let result = context
                .execute::<N>(&compiled_circuit)
                .expect("Failed to execute the circuit");
            result.into()
        }

        match std::any::type_name::<T>() {
            "u8" => generate::<8, T>(a, b, c, d),
            "u16" => generate::<16, T>(a, b, c, d),
            "u32" => generate::<32, T>(a, b, c, d),
            "u64" => generate::<64, T>(a, b, c, d),
            "u128" => generate::<128, T>(a, b, c, d),
            _ => panic!("Unsupported type"),
        }
    }
}
