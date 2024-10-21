use crate::executor::get_executor;
use crate::uint::GarbledUint;

use tandem::Circuit;
use tandem::Gate;
use tandem::GateIndex;

#[derive(Debug, Clone)]
pub(crate) enum GateOp {
    Xor(GateIndex), // xor input with top of stack
    And(GateIndex),
    Or(GateIndex),
    Mux(GateIndex, GateIndex),
    Not,
    Xnor(GateIndex),
    Nor(GateIndex),
    Nand(GateIndex),
    Add(GateIndex),
    Sub(GateIndex),
}

pub(crate) struct CircuitBuilder<const N: usize> {
    inputs: Vec<GarbledUint<N>>,
    gates: Vec<Gate>,
    output_indices: Vec<u32>,
    ops: Vec<GateOp>,
}

impl<const N: usize> Default for CircuitBuilder<N> {
    fn default() -> Self {
        Self {
            inputs: Vec::new(),
            gates: Vec::new(),
            output_indices: Vec::new(),
            ops: Vec::new(),
        }
    }
}

impl<const N: usize> CircuitBuilder<N> {
    pub fn new() -> Self {
        CircuitBuilder {
            inputs: Vec::new(),
            gates: Vec::new(),
            output_indices: Vec::new(),
            ops: Vec::new(),
        }
    }

    pub fn add_inputs(&mut self, inputs: Vec<&GarbledUint<N>>) {
        self.inputs = inputs.iter().cloned().cloned().collect();
        for _ in 0..N * inputs.len() {
            self.gates.push(Gate::InContrib); // Each bit of the input contributes
        }
    }

    // Add a new input to the builder
    pub fn add_input(&mut self, input: GarbledUint<N>) {
        self.inputs.push(input);
        for _ in 0..N {
            self.gates.push(Gate::InContrib); // Each bit of the input contributes
        }
    }

    pub fn len(&self) -> GateIndex {
        self.gates.len() as u32
    }

    pub fn push_eq(&mut self, x: GateIndex, y: GateIndex) -> GateIndex {
        let xor = self.push_xor(x, y);
        self.push_xor(xor, 1)
    }

    // Add a new operation to the builder
    pub fn add_op(&mut self, op: GateOp) {
        self.ops.push(op);
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

    // Add a MUX gate: MUX(a, b, s) = (a & !s) | (b & s)
    pub fn push_mux(&mut self, a: GateIndex, b: GateIndex, s: GateIndex) -> GateIndex {
        if a == b {
            return a;
        }
        let not_s = self.push_not(s);
        let and_a_not_s = self.push_and(a, not_s);
        let and_b_s = self.push_and(b, s);
        self.push_or(and_a_not_s, and_b_s)
    }

    pub fn push_adder(
        &mut self,
        x: GateIndex,
        y: GateIndex,
        carry: GateIndex,
    ) -> (GateIndex, GateIndex) {
        // first half-adder:
        let wire_u = self.push_xor(x, y);
        let wire_v = self.push_and(x, y);
        // second half-adder:
        let wire_s = self.push_xor(wire_u, carry);
        let wire_w = self.push_and(wire_u, carry);

        let carry = self.push_or(wire_v, wire_w);
        (wire_s, carry)
    }

    fn full_subtractor(&mut self, a: u32, b: u32, borrow: Option<u32>) -> (u32, Option<u32>) {
        // XOR gate for difference bit (a ⊕ b)
        let xor_ab = self.push_xor(a, b);

        // If borrow exists, XOR the result of the previous XOR with the borrow
        let diff = if let Some(borrow) = borrow {
            self.push_xor(xor_ab, borrow)
        } else {
            xor_ab
        };

        // Compute the new borrow: (!a & b) | (a & borrow) | (!b & borrow)
        let not_a = self.push_not(a);
        let and_not_a_b = self.push_and(not_a, b);

        let new_borrow = if let Some(borrow) = borrow {
            let and_a_borrow = self.push_and(a, borrow);
            let not_b = self.push_not(b);
            let and_not_b_borrow = self.push_and(not_b, borrow);

            // Combine borrow parts using XOR and AND to simulate OR
            let xor_borrow_parts = self.push_xor(and_not_a_b, and_a_borrow);
            self.push_xor(xor_borrow_parts, and_not_b_borrow)
        } else {
            and_not_a_b
        };

        (diff, Some(new_borrow))
    }

    // Build the circuit using a stack of operations
    pub fn build_circuit(&mut self) {
        let n = N as u32;

        // Clone the operations into a temporary vector to avoid borrow conflicts
        let ops: Vec<_> = self.ops.clone(); // Clone `self.ops` to avoid immutable borrow

        // optional carry_borrow for addition/subtraction
        let mut carry_borrow = 0;

        // Loop through each bit of the inputs
        for i in 0..n {
            let mut current_index = i;

            // Loop through the provided operations
            for op in ops.iter() {
                match op {
                    GateOp::And(val) => {
                        let value_index = i + n * (val);
                        current_index = self.push_and(current_index, value_index);
                    }
                    GateOp::Xor(val) => {
                        let value_index = i + n * (val);
                        current_index = self.push_xor(current_index, value_index);
                    }
                    GateOp::Not => {
                        // Apply NOT to the current bit
                        current_index = self.push_not(current_index);
                    }
                    GateOp::Or(val) => {
                        let value_index = i + n * (val);
                        current_index = self.push_or(current_index, value_index);
                    }
                    GateOp::Mux(a, b) => {
                        let value_index_a = i + n * (a);
                        let value_index_b = i + n * (b);
                        current_index = self.push_mux(current_index, value_index_a, value_index_b);
                    }
                    GateOp::Xnor(val) => {
                        let value_index = i + n * (val);
                        current_index = self.push_xnor(current_index, value_index);
                    }
                    GateOp::Nor(val) => {
                        let value_index = i + n * (val);
                        current_index = self.push_nor(current_index, value_index);
                    }
                    GateOp::Nand(val) => {
                        let value_index = i + n * (val);
                        current_index = self.push_nand(current_index, value_index);
                    }
                    GateOp::Add(val) => {
                        // Perform addition using the full adder
                        let value_index = i + n * (val);

                        let (sum, new_carry) =
                            self.push_adder(current_index, value_index, carry_borrow);
                        //let (sum, new_carry) =
                        //    self.full_adder(current_index, value_index, carry_borrow);
                        carry_borrow = new_carry; // Propagate the carry to the next bit
                        current_index = sum;
                    }
                    GateOp::Sub(val) => {
                        //let value_index = i + n * (val);
                        //let (diff, new_borrow) =
                        //    self.full_subtractor(current_index, value_index, carry_borrow);
                        // carry_borrow = new_borrow;
                        // current_index = diff;
                    }
                }
            }

            // Store the index of the final gate result for each bit
            self.output_indices.push(current_index);
        }
    }

    // Execute the accumulated circuit
    pub fn execute(&self) -> anyhow::Result<GarbledUint<N>> {
        // Debug print gates
        for gate in self.gates.iter() {
            match gate {
                Gate::InEval => println!("InEval (Invalid)"),
                Gate::InContrib => println!("InContrib"),
                Gate::Xor(a, b) => println!("Xor({}, {})", a, b),
                Gate::And(a, b) => println!("And({}, {})", a, b),
                Gate::Not(a) => println!("Not({})", a),
            }
        }
        // Debug print output indices
        println!("output_indices: {:?}", self.output_indices.clone());

        let program = Circuit::new(self.gates.clone(), self.output_indices.clone());

        let combined_input: Vec<bool> = self
            .inputs
            .iter()
            .flat_map(|input| input.bits.clone())
            .collect();

        let result = get_executor().execute(&program, &combined_input, &[])?;

        println!("boolean result: {:?}", result);

        // Convert the boolean result to a u64
        let debug_result = result
            .iter()
            .rev()
            .map(|b| if *b { 1 } else { 0 })
            .fold(0, |acc, x| acc * 2 + x);

        println!("debug result: {}", debug_result);

        Ok(GarbledUint::new(result))
    }
}

mod tests {
    use super::*;
    use crate::uint::GarbledUint32;
    use crate::uint::GarbledUint8;

    #[test]
    fn test_xor() {
        let a: GarbledUint8 = 8_u8.into();
        let b: GarbledUint8 = 42_u8.into();

        let mut builder: CircuitBuilder<8> = CircuitBuilder::new(); // 8-bit inputs (u8)

        // Add inputs
        builder.add_input(a.clone());
        builder.add_input(b.clone());
        builder.add_op(GateOp::Xor(1));

        // Build the circuit using the stack of operations
        builder.build_circuit();

        // Execute the circuit
        let result = builder.execute().expect("Failed to execute circuit");

        println!("result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, 8_u8 ^ 42_u8); // Explicit precedence
    }

    #[test]
    fn test_and() {
        let a: GarbledUint8 = 8_u8.into();
        let b: GarbledUint8 = 42_u8.into();

        let mut builder: CircuitBuilder<8> = CircuitBuilder::new(); // 8-bit inputs (u8)

        // Add inputs
        builder.add_input(a.clone());
        builder.add_input(b.clone());
        builder.add_op(GateOp::And(1));

        // Build the circuit using the stack of operations
        builder.build_circuit();

        // Execute the circuit
        let result = builder.execute().expect("Failed to execute circuit");

        println!("result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, 8_u8 & 42_u8); // Explicit precedence
    }

    #[test]
    fn test_not() {
        let a: GarbledUint8 = 8_u8.into();

        let mut builder: CircuitBuilder<8> = CircuitBuilder::new(); // 8-bit inputs (u8)

        // reorder the inputs based on order of operations
        // Add inputs
        builder.add_input(a.clone());
        builder.add_op(GateOp::Not);

        // Build the circuit using the stack of operations
        builder.build_circuit();

        // Execute the circuit
        let result = builder.execute().expect("Failed to execute circuit");

        println!("result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, !8_u8);
    }

    #[test]
    fn test_and_xor_or_not() {
        let a: GarbledUint8 = 8_u8.into();
        let b: GarbledUint8 = 42_u8.into();
        let c: GarbledUint8 = 17_u8.into();
        let d: GarbledUint8 = 3_u8.into();

        let mut builder: CircuitBuilder<8> = CircuitBuilder::new(); // 8-bit inputs (u8)
        builder.add_inputs(vec![&a, &b, &c, &d]);

        // Add inputs
        builder.add_op(GateOp::And(1));
        builder.add_op(GateOp::Xor(2));
        builder.add_op(GateOp::Or(3));
        builder.add_op(GateOp::Not);

        // Build the circuit using the stack of operations
        builder.build_circuit();

        // Execute the circuit
        let result = builder.execute().expect("Failed to execute circuit");

        println!("result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, !(8_u8 & 42_u8 ^ 17_u8 | 3_u8)); // Explicit precedence
    }

    #[test]
    fn test_and_or() {
        let a: GarbledUint32 = 8_u32.into();
        let b: GarbledUint32 = 42_u32.into();
        let c: GarbledUint32 = 17_u32.into();

        let mut builder: CircuitBuilder<32> = CircuitBuilder::new(); // 8-bit inputs (u8)
        builder.add_inputs(vec![&a, &b, &c]);

        // Add inputs
        builder.add_op(GateOp::And(1));
        builder.add_op(GateOp::Or(2));

        // Build the circuit using the stack of operations
        builder.build_circuit();

        // Execute the circuit
        let result = builder.execute().expect("Failed to execute circuit");

        println!("result: {}", result);
        let result: u32 = result.into();
        assert_eq!(result, 8_u32 & 42_u32 | 17_u32); // Explicit precedence
    }

    #[test]
    fn test_and_xor() {
        let a: GarbledUint8 = 8_u8.into();
        let b: GarbledUint8 = 42_u8.into();
        let c: GarbledUint8 = 17_u8.into();

        let mut builder: CircuitBuilder<8> = CircuitBuilder::new(); // 8-bit inputs (u8)

        // Add inputs
        builder.add_input(a.clone());
        builder.add_input(b.clone());
        builder.add_op(GateOp::And(1));
        builder.add_input(c.clone());
        builder.add_op(GateOp::Xor(2));

        // Build the circuit using the stack of operations
        builder.build_circuit();

        // Execute the circuit
        let result = builder.execute().expect("Failed to execute circuit");

        println!("result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, 8_u8 & 42_u8 ^ 17_u8); // Explicit precedence
    }

    #[test]
    fn test_xor_and() {
        let a: GarbledUint8 = 8_u8.into();
        let b: GarbledUint8 = 42_u8.into();
        let c: GarbledUint8 = 17_u8.into();

        let mut builder: CircuitBuilder<8> = CircuitBuilder::new(); // 8-bit inputs (u8)

        // reorder the inputs based on order of operations
        // Add inputs
        builder.add_input(a.clone());
        builder.add_input(b.clone());
        builder.add_op(GateOp::Xor(1));
        builder.add_input(c.clone());
        builder.add_op(GateOp::And(2));

        // Build the circuit using the stack of operations
        builder.build_circuit();

        // Execute the circuit
        let result = builder.execute().expect("Failed to execute circuit");

        println!("result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, (8_u8 ^ 42_u8) & 17_u8); // Explicit precedence
    }

    #[test]
    fn test_xnor() {
        let a: GarbledUint8 = 8_u8.into();
        let b: GarbledUint8 = 42_u8.into();

        let mut builder: CircuitBuilder<8> = CircuitBuilder::new(); // 8-bit inputs (u8)

        // Add inputs
        builder.add_input(a.clone());
        builder.add_input(b.clone());

        // Define the operations: AND first two, XOR with the third input
        builder.add_op(GateOp::Xor(1));
        builder.add_op(GateOp::Not);

        // Build the circuit using the stack of operations
        builder.build_circuit();

        // Execute the circuit
        let result = builder.execute().expect("Failed to execute circuit");

        println!("result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, !(8_u8 ^ 42_u8)); // Explicit precedence
    }

    #[test]
    fn test_nor() {
        let a: GarbledUint8 = 8_u8.into();
        let b: GarbledUint8 = 42_u8.into();

        let mut builder: CircuitBuilder<8> = CircuitBuilder::new(); // 8-bit inputs (u8)

        // Add inputs
        builder.add_input(a.clone());
        builder.add_input(b.clone());

        // Define the operations: AND first two, XOR with the third input
        builder.add_op(GateOp::Or(1));
        builder.add_op(GateOp::Not);

        // Build the circuit using the stack of operations
        builder.build_circuit();

        // Execute the circuit
        let result = builder.execute().expect("Failed to execute circuit");

        println!("result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, !(8_u8 | 42_u8)); // Explicit precedence
    }

    #[test]
    fn test_or() {
        let a: GarbledUint8 = 8_u8.into();
        let b: GarbledUint8 = 42_u8.into();

        let mut builder: CircuitBuilder<8> = CircuitBuilder::new(); // 8-bit inputs (u8)

        // Add inputs
        builder.add_input(a.clone());
        builder.add_input(b.clone());
        builder.add_op(GateOp::Or(1));

        // Build the circuit using the stack of operations
        builder.build_circuit();

        // Execute the circuit
        let result = builder.execute().expect("Failed to execute circuit");

        println!("result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, 8_u8 | 42_u8); // Explicit precedence
    }

    #[test]
    fn test_nand() {
        let a: GarbledUint8 = 8_u8.into();
        let b: GarbledUint8 = 42_u8.into();

        let mut builder: CircuitBuilder<8> = CircuitBuilder::new(); // 8-bit inputs (u8)

        // Add inputs
        builder.add_input(a.clone()); // 0th input
        builder.add_input(b.clone()); // 1st input

        // Define the operations: AND first two, XOR with the third input
        builder.add_op(GateOp::And(1));
        builder.add_op(GateOp::Not);

        // Build the circuit using the stack of operations
        builder.build_circuit();

        // Execute the circuit
        let result = builder.execute().expect("Failed to execute circuit");

        println!("result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, !(8_u8 & 42_u8)); // Explicit precedence
    }

    #[test]
    fn test_not_and() {
        let a: GarbledUint8 = 8_u8.into();
        let b: GarbledUint8 = 42_u8.into();

        let mut builder: CircuitBuilder<8> = CircuitBuilder::new(); // 8-bit inputs (u8)

        // Add inputs
        builder.add_input(a.clone());
        builder.add_input(b.clone());

        // Define the operations: AND first two, XOR with the third input
        builder.add_op(GateOp::Not);
        builder.add_op(GateOp::And(1));

        // Build the circuit using the stack of operations
        builder.build_circuit();

        // Execute the circuit
        let result = builder.execute().expect("Failed to execute circuit");

        println!("result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, !8_u8 & 42_u8); // Explicit precedence
    }

    #[test]
    fn test_mux() {
        let a: GarbledUint8 = 8_u8.into();
        let b: GarbledUint8 = 42_u8.into();
        let s: GarbledUint8 = 1_u8.into();

        let mut builder: CircuitBuilder<8> = CircuitBuilder::new(); // 8-bit inputs (u8)

        // Add inputs
        builder.add_input(a.clone());
        builder.add_input(b.clone());
        builder.add_input(s.clone());
        builder.add_op(GateOp::Mux(1, 2));

        // Build the circuit using the stack of operations
        builder.build_circuit();

        // Execute the circuit
        let result = builder.execute().expect("Failed to execute circuit");

        println!("result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, (8_u8 & !1_u8) | (42_u8 & 1_u8)); // Explicit precedence
    }

    #[test]
    fn test_addition() {
        let a: GarbledUint8 = 100_u8.into();
        let b: GarbledUint8 = 50_u8.into();

        let mut builder: CircuitBuilder<8> = CircuitBuilder::new(); // 8-bit inputs (u8)

        // Add inputs
        builder.add_input(a.clone());
        builder.add_input(b.clone());
        builder.add_op(GateOp::Add(1));

        // Build the circuit using the stack of operations
        builder.build_circuit();

        // Execute the circuit
        let result = builder.execute().expect("Failed to execute circuit");

        println!("result: {}", result);
        let result: u8 = result.into();
        assert_eq!(result, 100 + 50); // Explicit precedence
    }

    #[test]
    fn test_addition_3() {
        let a: GarbledUint8 = 8_u8.into();
        let b: GarbledUint8 = 2_u8.into();
        let c: GarbledUint8 = 10_u8.into();

        let mut builder: CircuitBuilder<8> = CircuitBuilder::new(); // 8-bit inputs (u8)

        // Add inputs
        builder.add_input(a.clone());
        builder.add_input(b.clone());
        builder.add_input(c.clone());
        builder.add_op(GateOp::Add(1)); // Adds a and b
        builder.add_op(GateOp::Add(2)); // Adds c to the result of a + b

        // Build the circuit using the stack of operations
        builder.build_circuit();

        // Execute the circuit
        let result = builder.execute().expect("Failed to execute circuit");

        println!("result: {}", result);
        let result: u8 = result.into();
        //let result: u8 = result.try_into().unwrap();
        assert_eq!(result, 8 + 2 + 1); // Explicit precedence for multi-input addition
    }
}
