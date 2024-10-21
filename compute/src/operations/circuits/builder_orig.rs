use crate::executor::get_executor;
use crate::uint::GarbledUint;
use crate::uint::GarbledUintBits;
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::sync::Arc;
use std::sync::RwLock;
use tandem::GateIndex;
use tandem::{Circuit, Gate};

lazy_static! {
    pub static ref CIRCUIT_BUILDER_8: Arc<RwLock<CircuitBuilder<8>>> =
        Arc::new(RwLock::new(CircuitBuilder::default()));
}

// Function to get the shared reference to the circuit builder
pub fn get_circuit_builder_8() -> Arc<RwLock<CircuitBuilder<8>>> {
    Arc::clone(&CIRCUIT_BUILDER_8) // Clone the Arc to allow shared ownership
}

enum GateOp {
    Xor,
    And,
    Not,
}

pub struct CircuitBuilder<const N: usize> {
    inputs: Vec<bool>,
    input_gates: Vec<Gate>,
    gates: Vec<Gate>,
    output_indices: Vec<GateIndex>,

    gate_ops: Vec<GateOp>,
    input_uints: Vec<GarbledUint<N>>,
}

impl<const N: usize> Default for CircuitBuilder<N> {
    fn default() -> Self {
        Self {
            gates: Vec::new(),
            input_gates: Vec::new(),
            output_indices: vec![0; N],
            inputs: Vec::new(),
            gate_ops: Vec::new(),
            input_uints: Vec::new(),
        }
    }
}

impl<const N: usize> CircuitBuilder<N> {
    pub fn new(inputs: Vec<&GarbledUint<N>>) -> Self {
        let mut input_gates = Vec::new();

        // Iterate over the inputs and push a `Gate::InContrib` for each bit
        input_gates.extend(
            inputs
                .iter()
                .flat_map(|input| input.bits().iter().map(|_| Gate::InContrib)),
        );

        // output_indices should be of size N
        Self {
            gates: Vec::new(),
            input_gates,
            output_indices: vec![0; N],
            inputs: inputs
                .iter()
                .flat_map(|input| input.bits().iter())
                .cloned()
                .collect(),
            gate_ops: Vec::new(),
            input_uints: inputs.iter().cloned().cloned().collect(),
        }
    }

    pub fn inputs(&mut self, inputs: Vec<GarbledUint<N>>) {
        self.input_uints = inputs;
        println!("Inner inputs: {:?}", self.inputs);
    }

    pub fn add_input(&mut self, input: &GarbledUint<N>) {
        self.input_uints.push(input.clone());
    }

    // add an output index to output_indices, the output_indices should shift by 1, and the first index should be removed
    pub fn add_output(&mut self, output_index: GateIndex) {
        self.output_indices.rotate_left(1);
        self.output_indices[N - 1] = output_index;
    }

    pub fn len(&self) -> GateIndex {
        self.gates.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.gates.is_empty()
    }

    pub fn current_position(&self) -> usize {
        self.gates.len()
    }

    pub fn xor_op(&mut self) {
        self.gate_ops.push(GateOp::Xor);
    }

    pub fn and_op(&mut self) {
        self.gate_ops.push(GateOp::And);
    }

    pub fn not_op(&mut self) {
        self.gate_ops.push(GateOp::Not);
    }

    // Add a XOR gate between two inputs and return the index
    pub fn push_xor(&mut self, a: GateIndex, b: GateIndex) -> GateIndex {
        let xor_index = self.gates.len() as u32;
        self.gates.push(Gate::Xor(a, b));
        xor_index
    }

    // Add an AND gate between two inputs and return the index
    pub fn push_and(&mut self, a: GateIndex, b: GateIndex) -> GateIndex {
        let and_index = self.gates.len() as u32;
        self.gates.push(Gate::And(a, b));
        and_index
    }

    // Add a NOT gate for a single input and return the index
    pub fn push_not(&mut self, a: GateIndex) -> GateIndex {
        let not_index = self.gates.len() as u32;
        self.gates.push(Gate::Not(a));
        not_index
    }

    // Add a gate for OR operation: OR(a, b) = (a ⊕ b) ⊕ (a & b)
    pub fn push_or(&mut self, a: GateIndex, b: GateIndex) -> GateIndex {
        let xor_gate = self.push_xor(a, b);
        let and_gate = self.push_and(a, b);
        self.push_xor(xor_gate, and_gate)
    }

    // Add a NAND gate: NAND(a, b) = NOT(a & b)
    pub fn push_nand(&mut self, a: GateIndex, b: GateIndex) -> GateIndex {
        let and_gate = self.push_and(a, b);
        self.push_not(and_gate)
    }

    // Add a NOR gate: NOR(a, b) = NOT(OR(a, b))
    pub fn push_nor(&mut self, a: GateIndex, b: GateIndex) -> GateIndex {
        let or_gate = self.push_or(a, b);
        self.push_not(or_gate)
    }

    // Add an XNOR gate: XNOR(a, b) = NOT(a ⊕ b)
    pub fn push_xnor(&mut self, a: GateIndex, b: GateIndex) -> GateIndex {
        let xor_gate = self.push_xor(a, b);
        self.push_not(xor_gate)
    }

    #[allow(dead_code)]
    // Add a MUX gate: MUX(a, b, s) = (a & !s) | (b & s)
    pub fn push_mux(&mut self, a: GateIndex, b: GateIndex, s: GateIndex) -> GateIndex {
        let not_s = self.push_not(s);
        let and_a_not_s = self.push_and(a, not_s);
        let and_b_s = self.push_and(b, s);
        self.push_or(and_a_not_s, and_b_s)
    }

    // Build and return a Circuit from the current gates with given output indices
    pub fn build(self, output_indices: Vec<GateIndex>) -> Circuit {
        Circuit::new(self.gates, output_indices)
    }

    fn push_garbled_uints(
        &mut self,
        a: &[GateIndex],
        b: &[GateIndex],
    ) -> (Vec<GateIndex>, Option<GateIndex>) {
        let mut result = Vec::with_capacity(a.len());
        let mut carry = None;

        for i in 0..a.len() {
            let sum = self.full_adder(a[i], b[i], carry);
            result.push(sum.0);
            carry = sum.1;
        }

        (result, carry)
    }

    fn full_adder(
        &mut self,
        a: GateIndex,
        b: GateIndex,
        carry: Option<GateIndex>,
    ) -> (GateIndex, Option<GateIndex>) {
        let xor_ab = self.len();
        self.gates.push(Gate::Xor(a, b));

        let sum = if let Some(c) = carry {
            let sum_with_carry = self.len();
            self.gates.push(Gate::Xor(xor_ab, c));
            sum_with_carry
        } else {
            xor_ab
        };

        let and_ab = self.len();
        self.gates.push(Gate::And(a, b));

        let new_carry = if let Some(c) = carry {
            let and_axorb_c = self.len();
            self.gates.push(Gate::And(xor_ab, c));

            let or_gate = self.len();
            self.gates.push(Gate::Xor(and_ab, and_axorb_c));
            Some(or_gate)
        } else {
            Some(and_ab)
        };

        (sum, new_carry)
    }

    pub fn execute(
        &mut self,
        inputs: Vec<&GarbledUint<N>>,
        output_indices: Vec<u32>,
    ) -> anyhow::Result<GarbledUint<N>> {
        // Combine all input bits into a single concatenated vector
        let combined_input: Vec<bool> =
            inputs.iter().flat_map(|input| input.bits.clone()).collect();

        // Execute the circuit with the concatenated inputs
        self.execute_with_input(&combined_input, output_indices)
    }

    pub fn execute_new_back(&mut self) -> anyhow::Result<GarbledUint<N>> {
        // Combine all input bits into a single concatenated vector
        let mut gates = Vec::new();
        let mut output_indices = Vec::new();

        let inputs_len = self.input_uints.len() as u32; // total number of inputs

        // Add input contribution gates
        for _ in 0..(inputs_len * N as u32) {
            gates.push(Gate::InContrib);
        }

        let mut gate_index = 0;

        // Process each gate operation
        for op in self.gate_ops.iter() {
            match op {
                GateOp::Xor => {
                    // Generalized XOR over all inputs
                    for i in 0..N {
                        let mut current_xor = gate_index; // start with the first input bit

                        // XOR across all inputs, step by step, for each bit position `i`
                        for input_idx in 1..inputs_len {
                            // iterate through inputs
                            let next_gate_index = gate_index + (N as u32 * input_idx) + i as u32;
                            gates.push(Gate::Xor(current_xor, next_gate_index));
                            current_xor = next_gate_index; // update to the latest XOR result
                        }

                        gate_index += 1; // Increment after processing all inputs for this bit
                    }
                }
                GateOp::And => {
                    // Generalized AND over all inputs
                    for i in 0..N {
                        let mut current_and = gate_index; // start with the first input bit
                        for input_idx in 1..inputs_len {
                            let next_gate_index = gate_index + (N as u32 * input_idx) + i as u32;
                            gates.push(Gate::And(current_and, next_gate_index));
                            current_and = next_gate_index; // update to the latest AND result
                        }
                        gate_index += 1;
                    }
                }
                GateOp::Not => {
                    for i in 0..N {
                        gates.push(Gate::Not(gate_index));
                        gate_index += 1;
                    }
                }
            }
        }

        // Output indices: Get the results of the last set of gates (either XOR or AND results)
        for i in 0..N {
            output_indices.push(gate_index + i as u32);
        }

        // Print the gates for debugging
        for gate in gates.iter() {
            match gate {
                Gate::InEval => println!("InEval (Invalid)"),
                Gate::InContrib => println!("InContrib"),
                Gate::Xor(a, b) => println!("Xor({}, {})", a, b),
                Gate::And(a, b) => println!("And({}, {})", a, b),
                Gate::Not(a) => println!("Not({})", a),
            }
        }

        // Print output indices for debugging
        println!("output_indices: {:?}", output_indices.clone());

        let combined_input: Vec<bool> = self
            .input_uints
            .iter()
            .flat_map(|input| input.bits.clone())
            .collect();

        // Execute the circuit with the concatenated inputs
        let program = Circuit::new(gates, output_indices);
        let result = get_executor().execute(&program, &combined_input, &[])?;
        Ok(GarbledUint::new(result))
    }

    pub fn execute_new(&mut self) -> anyhow::Result<GarbledUint<N>> {
        // Combine all input bits into a single concatenated vector

        // build the circuit
        // looping through the gate_ops and building the circuit, base on N sized Uints
        let mut gates = Vec::new();
        let mut output_indices = Vec::new();

        let inputs_len = self.input_uints.len() as u32;

        // inputs
        for _ in 0..(inputs_len * N as u32) {
            gates.push(Gate::InContrib);
        }

        let mut gate_index = 0;
        for op in self.gate_ops.iter() {
            match op {
                GateOp::Xor => {
                    for _ in 0..N {
                        gates.push(Gate::Xor(
                            gate_index,
                            gate_index + (N as u32 * (inputs_len - 1)),
                        ));
                        gate_index += 1;
                    }
                }
                GateOp::And => {
                    for _ in 0..N {
                        gates.push(Gate::And(
                            gate_index,
                            gate_index + (N as u32 * (inputs_len - 1)),
                        ));
                        gate_index += 1;
                    }
                }
                GateOp::Not => {
                    for _ in 0..N {
                        gates.push(Gate::Not(gate_index));
                        gate_index += 1;
                    }
                }
            }
        }

        for _ in 0..N {
            output_indices.push(gate_index + (N as u32 * (inputs_len - 1)));
            gate_index += 1;
        }

        // print gates
        for gate in gates.iter() {
            match gate {
                Gate::InEval => println!("InEval (Invalid)"),
                Gate::InContrib => println!("InContrib"),
                Gate::Xor(a, b) => println!("Xor({}, {})", a, b),
                Gate::And(a, b) => println!("And({}, {})", a, b),
                Gate::Not(a) => println!("Not({})", a),
            }
        }

        // print output indices
        println!("output_indices: {:?}", output_indices.clone());

        let combined_input: Vec<bool> = self
            .input_uints
            .iter()
            .flat_map(|input| input.bits.clone())
            .collect();

        // Execute the circuit with the concatenated inputs
        let program = Circuit::new(gates, output_indices);
        let result = get_executor().execute(&program, &combined_input, &[])?;
        Ok(GarbledUint::new(result))
    }

    pub fn execute2(&mut self) -> anyhow::Result<GarbledUint<N>> {
        // Combine all input bits into a single concatenated vector
        //let combined_input: Vec<bool> =
        //    inputs.iter().flat_map(|input| input.bits.clone()).collect();

        // Execute the circuit with the concatenated inputs
        // self.execute_with_input(&self.inputs, self.output_indices)
        //self.output_indices = vec![16, 17, 18, 19, 20, 21, 22, 23];

        let mut combined_gates: Vec<Gate> = self.input_gates.clone();
        combined_gates.extend(self.gates.clone());

        self.print_gates();
        let program = Circuit::new(combined_gates, self.output_indices.clone());
        let result = get_executor().execute(&program, &self.inputs, &[])?;
        Ok(GarbledUint::new(result))
    }

    fn print_gates(&self) {
        // print input gates
        for gate in self.input_gates.iter() {
            match gate {
                Gate::InEval => println!("InEval"),
                Gate::InContrib => println!("InContrib"),
                _ => println!("Other"),
            }
        }

        for gate in self.gates.iter() {
            match gate {
                Gate::InEval => println!("SHOULDNT BE HERE: InEval"),
                Gate::InContrib => println!("SHOULDNT BE HERE: InContrib"),
                Gate::Xor(a, b) => println!("Xor({}, {})", a, b),
                Gate::And(a, b) => println!("And({}, {})", a, b),
                Gate::Not(a) => println!("Not({})", a),
            }
        }

        // print output indices
        println!("self.output_indeces: {:?}", self.output_indices);
    }

    pub fn execute_with_input(
        &mut self,
        input: &[bool],
        output_indices: Vec<u32>,
    ) -> anyhow::Result<GarbledUint<N>> {
        let mut combined_gates: Vec<Gate> = self.input_gates.clone();
        combined_gates.extend(self.gates.clone());

        let program = Circuit::new(combined_gates, output_indices);
        let result = get_executor().execute(&program, input, &[])?;
        println!("execute_with_input: result: {:?}", result);
        Ok(GarbledUint::new(result))
    }
}

pub(crate) fn build_and_execute_xor<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::new(vec![lhs, rhs]);

    // Add XOR gates for each bit
    let mut output_indices = Vec::with_capacity(N);
    for i in 0..N {
        let xor_gate = builder.push_xor(i as u32, (N + i) as u32);
        output_indices.push(xor_gate);
    }

    // Simulate the circuit
    builder
        .execute(vec![lhs, rhs], output_indices)
        .expect("Failed to execute XOR circuit")
}

pub(crate) fn build_and_execute_and<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::new(vec![lhs, rhs]);

    // Add AND gates for each bit
    let mut output_indices = Vec::with_capacity(N);
    for i in 0..N {
        let and_gate = builder.push_and(i as u32, (N + i) as u32);
        output_indices.push(and_gate);
    }

    // Simulate the circuit
    builder
        .execute(vec![lhs, rhs], output_indices)
        .expect("Failed to execute AND circuit")
}

pub(crate) fn build_and_execute_or<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::new(vec![lhs, rhs]);

    // Add OR gates for each bit
    let mut output_indices = Vec::with_capacity(N);
    for i in 0..N {
        let or_gate = builder.push_or(i as u32, (N + i) as u32);
        output_indices.push(or_gate);
    }

    // Simulate the circuit
    builder
        .execute(vec![lhs, rhs], output_indices)
        .expect("Failed to execute OR circuit")
}

pub(crate) fn build_and_execute_addition<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::new(vec![lhs, rhs]);

    let mut carry = None;

    // Create a full adder for each bit
    let mut output_indices = Vec::with_capacity(N);
    for i in 0..N {
        let (sum, new_carry) = builder.full_adder(i as GateIndex, (N + i) as GateIndex, carry);
        output_indices.push(sum);
        carry = new_carry;
    }

    // Simulate the circuit
    builder
        .execute(vec![lhs, rhs], output_indices)
        .expect("Failed to execute addition circuit")
}

pub(crate) fn build_and_execute_subtraction<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::new(vec![lhs, rhs]);

    let mut borrow = None;

    // Create a full subtractor for each bit
    let mut output_indices = Vec::with_capacity(N);
    for i in 0..N {
        let (diff, new_borrow) = full_subtractor(&mut builder, i as u32, (N + i) as u32, borrow);
        output_indices.push(diff);
        borrow = new_borrow;
    }

    // Simulate the circuit
    builder
        .execute(vec![lhs, rhs], output_indices)
        .expect("Failed to execute subtraction circuit")
}

fn full_subtractor<const N: usize>(
    builder: &mut CircuitBuilder<N>,
    a: u32,
    b: u32,
    borrow: Option<u32>,
) -> (u32, Option<u32>) {
    // XOR gate for difference bit (a ⊕ b)
    let xor_ab = builder.push_xor(a, b);

    // If borrow exists, XOR the result of the previous XOR with the borrow
    let diff = if let Some(borrow) = borrow {
        builder.push_xor(xor_ab, borrow)
    } else {
        xor_ab
    };

    // Compute the new borrow: (!a & b) | (a & borrow) | (!b & borrow)
    let not_a = builder.push_not(a);
    let and_not_a_b = builder.push_and(not_a, b);

    let new_borrow = if let Some(borrow) = borrow {
        let and_a_borrow = builder.push_and(a, borrow);
        let not_b = builder.push_not(b);
        let and_not_b_borrow = builder.push_and(not_b, borrow);

        // Combine borrow parts using XOR and AND to simulate OR
        let xor_borrow_parts = builder.push_xor(and_not_a_b, and_a_borrow);
        builder.push_xor(xor_borrow_parts, and_not_b_borrow)
    } else {
        and_not_a_b
    };

    (diff, Some(new_borrow))
}

pub(crate) fn build_and_execute_multiplication<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::new(vec![lhs, rhs]);

    let mut partial_products = Vec::with_capacity(N);

    // Generate partial products
    for i in 0..N {
        let shifted_product = generate_partial_product(&mut builder, 0, N as GateIndex, i);
        partial_products.push(shifted_product);
    }

    // Sum up all partial products
    let mut result = partial_products[0].clone();
    for partial_product in partial_products.iter().take(N).skip(1) {
        (result, _) = builder.push_garbled_uints(&result, partial_product);
    }

    // Simulate the circuit
    builder
        .execute(vec![lhs, rhs], result.to_vec())
        .expect("Failed to execute multiplication circuit")
}

fn generate_partial_product<const N: usize>(
    builder: &mut CircuitBuilder<N>,
    lhs_start: GateIndex,
    rhs_start: GateIndex,
    shift: usize,
) -> Vec<GateIndex> {
    let mut partial_product = Vec::with_capacity(N);

    for i in 0..N {
        if i < shift {
            // For lower bits, we use a constant 0
            let zero_bit = builder.len();
            builder.push_not(rhs_start);
            builder.push_and(rhs_start, zero_bit); // Constant 0
            partial_product.push(builder.len() - 1);
        } else {
            let lhs_bit = lhs_start + (i - shift) as u32;
            let and_gate = builder.len();
            builder.push_and(lhs_bit, rhs_start + shift as u32);
            partial_product.push(and_gate);
        }
    }

    partial_product
}

pub(crate) fn build_and_execute_nand<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::new(vec![lhs, rhs]);

    let mut output_indices = Vec::with_capacity(N);

    for i in 0..N {
        let nand_gate = builder.push_nand(i as u32, (N + i) as u32);
        output_indices.push(nand_gate);
    }

    builder
        .execute(vec![lhs, rhs], output_indices)
        .expect("Failed to execute NAND circuit")
}

pub(crate) fn build_and_execute_nor<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::new(vec![lhs, rhs]);
    let mut output_indices = Vec::with_capacity(N);

    for i in 0..N {
        let nor_gate = builder.push_nor(i as u32, (N + i) as u32);
        output_indices.push(nor_gate);
    }

    builder
        .execute(vec![lhs, rhs], output_indices)
        .expect("Failed to execute NOR circuit")
}

pub(crate) fn build_and_execute_xnor<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::new(vec![lhs, rhs]);
    let mut output_indices = Vec::with_capacity(N);

    for i in 0..N {
        let xnor_gate = builder.push_xnor(i as u32, (N + i) as u32);
        output_indices.push(xnor_gate);
    }

    builder
        .execute(vec![lhs, rhs], output_indices)
        .expect("Failed to execute XNOR circuit")
}

pub(crate) fn build_and_execute_equality<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> bool {
    let mut builder: CircuitBuilder<N> = CircuitBuilder::new(vec![lhs, rhs]);
    let mut result = builder.push_xnor(0, N as u32);

    for i in 1..N {
        let current_comparison = builder.push_xnor(i as u32, (N + i) as u32);
        result = builder.push_and(result, current_comparison);
    }
    let result = builder.execute(vec![lhs, rhs], vec![result]).unwrap();
    result.bits[0]
}

pub(crate) fn build_and_execute_comparator<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> Ordering {
    let mut builder: CircuitBuilder<N> = CircuitBuilder::new(vec![lhs, rhs]);
    let (lt_output, eq_output) = comparator_circuit::<N>(&mut builder);

    let program = builder.build(vec![lt_output, eq_output]);

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

fn comparator_circuit<const N: usize>(builder: &mut CircuitBuilder<N>) -> (u32, u32) {
    let mut eq_list = vec![0; N];
    let mut lt_list = vec![0; N];

    let i = N - 1;
    let eq_i = builder.push_xnor(i as u32, (N + i) as u32);
    eq_list[i] = eq_i;

    let nt = builder.push_not(i as u32);
    let lt_i = builder.push_and(nt, (N + i) as u32);
    lt_list[i] = lt_i;

    for idx in (0..i).rev() {
        let xn = builder.push_xnor(idx as u32, (N + idx) as u32);
        let eq_i = builder.push_and(eq_list[idx + 1], xn);
        eq_list[idx] = eq_i;

        let nt = builder.push_not(idx as u32);
        let aa = builder.push_and(nt, (N + idx) as u32);
        let temp_lt = builder.push_and(eq_list[idx + 1], aa);
        lt_list[idx] = builder.push_or(lt_list[idx + 1], temp_lt);
    }

    (lt_list[0], eq_list[0])
}

pub(crate) fn build_and_execute_not<const N: usize>(input: &GarbledUint<N>) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::new(vec![input]);

    let mut output_indices = Vec::with_capacity(N);

    for i in 0..N {
        let not_gate = builder.push_not(i as u32);
        output_indices.push(not_gate);
    }

    builder
        .execute(vec![input], output_indices)
        .expect("Failed to execute NOT circuit")
}

/*
#[allow(dead_code)]
pub(crate) fn build_and_execute_mux<const N: usize, const S: usize>(
    condition: &GarbledUint<S>,
    if_true: &GarbledUint<N>,
    if_false: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::new(vec![condition, if_true, if_false]);

    // Add MUX gates for each bit
    let mut output_indices = Vec::with_capacity(N);
    for i in 0..N {
        let mux_gate = builder.push_mux(i as u32, (N + i) as u32, (2 * N) as u32);
        output_indices.push(mux_gate);
    }

    // combine the three inputs into a single value
    let input = [
        if_false.bits.clone(),
        if_true.bits.clone(),
        condition.bits.clone(),
    ]
    .concat();

    // Simulate the circuit
    builder
        .execute_with_input(&input, output_indices)
        .expect("Failed to execute MUX circuit")
}
*/

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::operations;
    use crate::uint::GarbledBoolean;
    use crate::uint::GarbledUint8;

    #[test]
    fn test_xor_8() {
        let lhs: GarbledUint8 = 5_u8.into();
        let rhs: GarbledUint8 = 8_u8.into();

        {
            let builder = get_circuit_builder_8();
            //let builder = Arc::clone(&builder);
            let mut builder = builder.write().unwrap();
            builder.inputs(vec![lhs.clone(), rhs.clone()]);
        }

        let _result = lhs ^ rhs; //(&lhs ^ &rhs) ^ (&rhs ^ &lhs);

        {
            let builder = get_circuit_builder_8();
            //let builder = Arc::clone(&builder);
            let mut builder = builder.write().unwrap();
            let result = builder.execute_new().unwrap();
            let res: u8 = result.into();
            // assert_eq!(res, (5_u8 ^ 8_u8) ^ (8_u8 ^ 5_u8));
            //assert_eq!(res, 5_u8 ^ 8_u8);
            assert_eq!(res, 5_u8 ^ 8_u8);
        }
    }

    #[test]
    fn test_and_8_flat() {
        const N: usize = 8;
        let a: GarbledUint8 = 5_u8.into();
        let b: GarbledUint8 = 8_u8.into();

        let mut builder: CircuitBuilder<N> = CircuitBuilder::new(vec![&a, &b]);

        let num_inputs = 2;

        let mut output_indices = Vec::with_capacity(N);
        let current_pos = builder.current_position() as u32;
        // Add XOR gates for each bit
        let n = N as u32;
        for i in current_pos..current_pos + n {
            let xor_gate = builder.push_xor(i, n + i);
            output_indices.push(xor_gate + n * num_inputs);
        }

        // combine the three inputs into a single value
        let input = [a.bits.clone(), b.bits.clone()].concat();

        builder.print_gates();

        println!("Output indices: {:?}", output_indices);

        // Simulate the circuit
        let result = builder
            .execute_with_input(&input, output_indices.clone())
            .expect("Failed to execute XOR circuit");

        println!("XOR result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, 5_u8 ^ 8_u8);
    }

    #[test]
    fn test_two_or() {
        let a: GarbledUint8 = 8_u8.into();
        let b: GarbledUint8 = 4_u8.into();

        let mut builder = CircuitBuilder::default();
        builder.add_input(&a);

        builder.xor_op();
        builder.add_input(&b);

        // Simulate the circuit
        let result = builder
            .execute_new()
            .expect("Failed to execute XOR circuit");

        println!("XOR result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, 8_u8 ^ 4_u8);
    }

    #[test]
    fn test_xor_three() {
        let a: GarbledUint8 = 2_u8.into();
        let b: GarbledUint8 = 4_u8.into();
        let c: GarbledUint8 = 3_u8.into();

        let mut builder = CircuitBuilder::default();
        builder.add_input(&a);

        builder.xor_op();
        builder.add_input(&b);

        builder.xor_op();
        builder.add_input(&c);

        // Simulate the circuit
        let result = builder
            .execute_new()
            .expect("Failed to execute XOR circuit");

        println!("XOR result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, 2_u8 ^ 4_u8 ^ 3_u8);
    }

    #[test]
    fn test_two_and() {
        let a: GarbledUint8 = 8_u8.into();
        let b: GarbledUint8 = 4_u8.into();

        let mut builder = CircuitBuilder::default();
        builder.add_input(&a);

        builder.and_op();
        builder.add_input(&b);

        // Simulate the circuit
        let result = builder
            .execute_new()
            .expect("Failed to execute XOR circuit");

        println!("XOR result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, 8_u8 & 4_u8);
    }

    #[test]
    fn test_three() {
        let a: GarbledUint8 = 3_u8.into();
        let b: GarbledUint8 = 2_u8.into();

        let mut builder = CircuitBuilder::default();
        builder.add_input(&a);

        builder.and_op();
        builder.add_input(&b);

        builder.xor_op();
        let c: GarbledUint8 = 5_u8.into();
        builder.add_input(&c);

        // Simulate the circuit
        let result = builder
            .execute_new()
            .expect("Failed to execute XOR circuit");

        println!("XOR result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, 3_u8 & 2_u8 ^ 5_u8);
    }

    #[test]
    fn test_three2() {
        let a: GarbledUint8 = 4_u8.into();
        let b: GarbledUint8 = 7_u8.into();

        let mut builder = CircuitBuilder::default();
        builder.add_input(&a);

        builder.xor_op();
        builder.add_input(&b);

        builder.and_op();
        let c: GarbledUint8 = 5_u8.into();
        builder.add_input(&c);

        // Simulate the circuit
        let result = builder
            .execute_new()
            .expect("Failed to execute XOR circuit");

        println!("XOR result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, 4_u8 ^ 7_u8 & 5_u8);
    }

    #[test]
    fn test_and_8_flat_new() {
        let a: GarbledUint8 = 8_u8.into();
        let b: GarbledUint8 = 7_u8.into();

        let mut builder = CircuitBuilder::default();
        builder.add_input(&a);

        builder.xor_op();
        builder.add_input(&b);

        let c: GarbledUint8 = 9_u8.into();
        builder.and_op();
        builder.add_input(&c);

        let d: GarbledUint8 = 2_u8.into();
        builder.xor_op();
        builder.add_input(&d);

        // Simulate the circuit
        let result = builder
            .execute_new()
            .expect("Failed to execute XOR circuit");

        println!("XOR result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, 8_u8 ^ 7_u8 & 9_u8 ^ 2_u8);
    }

    #[test]
    fn test_and_8_flat_new2() {
        let a: GarbledUint8 = 8_u8.into();
        let b: GarbledUint8 = 4_u8.into();
        let c: GarbledUint8 = 9_u8.into();
        let d: GarbledUint8 = 2_u8.into();

        let mut builder = CircuitBuilder::default();

        // Step 1: Perform AND: 4 & 9
        builder.add_input(&b);
        builder.add_input(&c);
        builder.and_op();

        // Step 2: XOR result of AND with 8: 8 ^ (4 & 9)
        builder.add_input(&a);
        builder.xor_op();

        // Step 3: XOR result with 2: (8 ^ (4 & 9)) ^ 2
        builder.add_input(&d);
        builder.xor_op();

        // Simulate the circuit
        let result = builder
            .execute_new()
            .expect("Failed to execute XOR circuit");

        println!("XOR result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, (8_u8 ^ (4_u8 & 9_u8)) ^ 2_u8); // Explicit precedence
    }

    #[test]
    fn test_boolean_xor_and() {
        let a: GarbledBoolean = true.into(); // true
        let b: GarbledBoolean = false.into(); // false
        let c: GarbledBoolean = false.into(); // false

        let mut builder = CircuitBuilder::default();

        // Step 1: AND operation between b and c: false & false
        builder.add_input(&a);
        builder.add_input(&b);
        builder.xor_op();

        // Step 2: XOR the result of AND with a (true ^ (false & false))
        builder.add_input(&c); // Add 'a' as input (true)
        builder.and_op(); // XOR operation with the result of the AND

        // Simulate the circuit
        let result = builder
            .execute_new()
            .expect("Failed to execute XOR circuit");

        println!("Result: {}", result);
        let result: bool = result.into();

        // this works, because parentheses are used to enforce precedence
        assert_eq!(result, (true ^ false) & false); // Expected result: true

        // this fails, because & has higher precedence than ^
        // assert_eq!(result, true ^ false & false); // Expected result: true
    }

    #[test]
    fn test_boolean_and_xor() {
        let a: GarbledBoolean = true.into(); // true
        let b: GarbledBoolean = false.into(); // false
        let c: GarbledBoolean = false.into(); // false

        let mut builder = CircuitBuilder::default();

        // Step 1: AND operation between b and c: false & false
        builder.add_input(&a);
        builder.add_input(&b);
        builder.and_op();

        // Step 2: XOR the result of AND with a (true ^ (false & false))
        builder.add_input(&c); // Add 'a' as input (true)
        builder.xor_op(); // XOR operation with the result of the AND

        // Simulate the circuit
        let result = builder
            .execute_new()
            .expect("Failed to execute XOR circuit");

        println!("Result: {}", result);
        let result: bool = result.into();
        assert_eq!(result, true & false ^ false); // Expected result: true
    }

    // Define an enum that can hold different types
    enum StackElement<const N: usize> {
        Uint(GarbledUint<N>),
        Op(GateOp),
    }

    struct Stack<const N: usize> {
        elements: Vec<StackElement<N>>,
    }

    impl<const N: usize> Stack<N> {
        fn new() -> Self {
            Stack {
                elements: Vec::new(),
            }
        }

        fn push(&mut self, item: StackElement<N>) {
            self.elements.push(item);
        }

        fn pop(&mut self) -> Option<StackElement<N>> {
            self.elements.pop()
        }
    }

    #[test]
    fn test_u8_and_xor() {
        let a: GarbledUint8 = 8_u8.into();
        let b: GarbledUint8 = 42_u8.into();
        let c: GarbledUint8 = 17_u8.into();

        let inputs = vec![a.clone(), b.clone(), c.clone()];
        //let ops = vec![GateOp::And, GateOp::Xor];

        let combined_input: Vec<bool> =
            inputs.iter().flat_map(|input| input.bits.clone()).collect();

        let mut gates = Vec::new();

        let N = 8; // Number of bits in the input
        let input_count = 3; // Number of inputs
        for _ in 0..N * input_count {
            gates.push(Gate::InContrib);
        }

        let mut output_indices = Vec::new();
        for i in 0..N {
            gates.push(Gate::And(i, i + N));
            gates.push(Gate::Xor(gates.len() as u32 - 1, i + N * 2));
            output_indices.push(gates.len() as u32 - 1);
        }

        // print gates for debugging
        for gate in gates.iter() {
            match gate {
                Gate::InEval => println!("InEval (Invalid)"),
                Gate::InContrib => println!("InContrib"),
                Gate::Xor(a, b) => println!("Xor({}, {})", a, b),
                Gate::And(a, b) => println!("And({}, {})", a, b),
                Gate::Not(a) => println!("Not({})", a),
            }
        }

        println!("Output indices: {:?}", output_indices);

        // Execute the circuit with the concatenated inputs
        let program = Circuit::new(gates, output_indices);
        let result = get_executor()
            .execute(&program, &combined_input, &[])
            .unwrap();
        let result: u8 = GarbledUint::<8>::new(result).into();

        println!("result: {}", result);
        //let result: u8 = result.into();
        assert_eq!(result, 8_u8 & 42_u8 ^ 17_u8); // Explicit precedence
    }

    /*
    #[test]
    fn test_mux() {
        const N: usize = 32;

        let a: GarbledUint32 = 1900142_u32.into(); // if s is false, output should be a
        let b: GarbledUint32 = 771843900_u32.into(); // if s is true, output should be b
        let s: GarbledBit = true.into();

        let mut builder: CircuitBuilder<N> = CircuitBuilder::new(vec![&a, &b, &s]);

        // Add MUX gates for each bit
        let mut output_indices = Vec::with_capacity(N);
        for i in 0..N {
            let mux_gate = builder.push_mux(i as u32, (N + i) as u32, (2 * N) as u32);
            output_indices.push(mux_gate);
        }

        // combine the three inputs into a single value
        let input = [a.bits.clone(), b.bits.clone(), s.bits].concat();

        // Simulate the circuit
        let result = builder
            .execute_with_input(&input, output_indices.clone())
            .expect("Failed to execute MUX circuit");

        println!("MUX result: {}", result);
        assert_eq!(result, b);

        let s: GarbledBit = false.into();
        // combine the three inputs into a single value
        let input = [a.bits.clone(), b.bits.clone(), s.bits].concat();

        // Simulate the circuit
        let result = builder
            .execute_with_input(&input, output_indices)
            .expect("Failed to execute MUX circuit");

        println!("MUX result: {}", result);
        assert_eq!(result, a);
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
        let s: GarbledUint32 = 0b11111111_11111111_11111111_11111111_u32.into();
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
        let s: GarbledUint64 =
            0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_u64.into();
        let a: GarbledUint64 = 23948323290804923_u64.into();
        let b: GarbledUint64 = 834289823983634323_u64.into();

        let result = build_and_execute_mux(&s, &a, &b);
        assert_eq!(result, a);

        let s: GarbledUint64 = 0_u64.into();
        let result = build_and_execute_mux(&s, &a, &b);
        assert_eq!(result, b);
    }
    */
}
