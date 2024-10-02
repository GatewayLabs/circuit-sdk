use std::collections::HashMap;

pub type GateIndex = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum BuilderGate {
    Xor(GateIndex, GateIndex),
    And(GateIndex, GateIndex),
}

pub struct CircuitBuilder {
    pub gates: Vec<BuilderGate>, // List of gates in the circuit.
    pub gate_counter: usize,     // Counter for generating gate indices.
    pub cache: HashMap<BuilderGate, GateIndex>, // Cache for sub-expression sharing.
    pub negated: HashMap<GateIndex, GateIndex>, // Cache for negated values.
}

impl CircuitBuilder {
    /// Creates a new CircuitBuilder with an initial state.
    pub fn new() -> Self {
        CircuitBuilder {
            gates: Vec::new(),
            gate_counter: 0,
            cache: HashMap::new(),
            negated: HashMap::new(),
        }
    }

    /// Adds a new XOR gate to the circuit and returns its wire index.
    pub fn push_xor(&mut self, x: GateIndex, y: GateIndex) -> GateIndex {
        let gate = BuilderGate::Xor(x, y);
        self.gate_counter += 1;
        self.gates.push(gate);
        let gate_index = self.gate_counter - 1;
        self.cache.insert(gate, gate_index);

        // Sub-expression sharing: storing negated results for future use.
        if x == 1 {
            self.negated.insert(y, gate_index);
            self.negated.insert(gate_index, y);
        }
        if y == 1 {
            self.negated.insert(x, gate_index);
            self.negated.insert(gate_index, x);
        }
        gate_index
    }

    /// Adds a new AND gate to the circuit and returns its wire index.
    pub fn push_and(&mut self, x: GateIndex, y: GateIndex) -> GateIndex {
        let gate = BuilderGate::And(x, y);
        self.gate_counter += 1;
        self.gates.push(gate);
        self.cache.insert(gate, self.gate_counter - 1);
        self.gate_counter - 1
    }

    pub fn push_not(&mut self, x: GateIndex) -> GateIndex {
        self.push_xor(x, 1)
    }

    pub fn push_or(&mut self, x: GateIndex, y: GateIndex) -> GateIndex {
        let xor = self.push_xor(x, y);
        let and = self.push_and(x, y);
        self.push_xor(xor, and)
    }

    pub fn push_eq(&mut self, x: GateIndex, y: GateIndex) -> GateIndex {
        let xor = self.push_xor(x, y);
        self.push_xor(xor, 1)
    }

    pub fn push_mux(&mut self, s: GateIndex, x0: GateIndex, x1: GateIndex) -> GateIndex {
        if x0 == x1 {
            return x0;
        }
        let not_s = self.push_not(s);
        let x0_selected = self.push_and(x0, s);
        let x1_selected = self.push_and(x1, not_s);
        self.push_xor(x0_selected, x1_selected)
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

    pub fn push_multiplier(
        &mut self,
        x: GateIndex,
        y: GateIndex,
        z: GateIndex,
        carry: GateIndex,
    ) -> (GateIndex, GateIndex) {
        let x_and_y = self.push_and(x, y);
        self.push_adder(x_and_y, z, carry)
    }

    pub fn push_addition_circuit(
        &mut self,
        x: &[GateIndex],
        y: &[GateIndex],
    ) -> (Vec<GateIndex>, GateIndex, GateIndex) {
        assert_eq!(x.len(), y.len());
        let bits = x.len();

        let mut carry_prev = 0;
        let mut carry = 0;
        let mut sum = vec![0; bits];
        // sequence of full adders:
        for i in (0..bits).rev() {
            let (s, c) = self.push_adder(x[i], y[i], carry);
            sum[i] = s;
            carry_prev = carry;
            carry = c;
        }
        (sum, carry, carry_prev)
    }

    pub fn push_negation_circuit(&mut self, x: &[GateIndex]) -> Vec<GateIndex> {
        // flip bits and increment to get negate:
        let mut carry = 1;
        let mut neg = vec![0; x.len()];
        for i in (0..x.len()).rev() {
            let x = self.push_not(x[i]);
            // half-adder:
            neg[i] = self.push_xor(carry, x);
            carry = self.push_and(carry, x);
        }
        neg
    }

    pub fn push_subtraction_circuit(
        &mut self,
        x: &[GateIndex],
        y: &[GateIndex],
        is_signed: bool,
    ) -> (Vec<GateIndex>, GateIndex) {
        assert_eq!(x.len(), y.len());
        let bits = x.len();

        let mut x_extended = vec![0; bits + 1];
        x_extended[1..].copy_from_slice(x);
        if is_signed {
            x_extended[0] = x_extended[1];
        }
        let mut y_extended = vec![0; bits + 1];
        y_extended[1..].copy_from_slice(y);
        if is_signed {
            y_extended[0] = y_extended[1];
        }
        let y_negated = self.push_negation_circuit(&y_extended);

        let (mut sum_extended, _, _) = self.push_addition_circuit(&x_extended, &y_negated);
        let sign = sum_extended[0];
        let sum = sum_extended.split_off(1);
        let overflow = if is_signed {
            self.push_xor(sign, sum[0])
        } else {
            sign
        };
        (sum, overflow)
    }

    pub fn push_unsigned_division_circuit(
        &mut self,
        x: &[GateIndex],
        y: &[GateIndex],
    ) -> (Vec<GateIndex>, Vec<GateIndex>) {
        assert_eq!(x.len(), y.len());
        let bits = x.len();

        let mut quotient = vec![0; bits];
        let mut remainder = x.to_vec();
        for shift_amount in (0..bits).rev() {
            let mut overflow = 0;
            let mut y_shifted = vec![0; bits];
            for y in y.iter().copied().take(shift_amount) {
                overflow = self.push_or(overflow, y);
            }
            y_shifted[..(bits - shift_amount)]
                .clone_from_slice(&y[shift_amount..((bits - shift_amount) + shift_amount)]);

            let (x_sub, carry) = self.push_subtraction_circuit(&remainder, &y_shifted, false);
            let carry_or_overflow = self.push_or(carry, overflow);
            for i in 0..bits {
                remainder[i] = self.push_mux(carry_or_overflow, remainder[i], x_sub[i]);
            }
            let quotient_bit = self.push_not(carry);
            quotient[bits - shift_amount - 1] = self.push_mux(overflow, 0, quotient_bit);
        }
        (quotient, remainder)
    }

    pub fn push_comparator_circuit(
        &mut self,
        bits: usize,
        x: &[GateIndex],
        is_x_signed: bool,
        y: &[GateIndex],
        is_y_signed: bool,
    ) -> (GateIndex, GateIndex) {
        let mut acc_gt = 0;
        let mut acc_lt = 0;
        for i in 0..bits {
            let xor = self.push_xor(x[i], y[i]);

            let xor_and_x = self.push_and(xor, x[i]);
            let xor_and_y = self.push_and(xor, y[i]);
            let (gt, lt) = if i == 0 && (is_x_signed || is_y_signed) {
                (xor_and_y, xor_and_x)
            } else {
                (xor_and_x, xor_and_y)
            };

            let gt = self.push_or(gt, acc_gt);
            let lt = self.push_or(lt, acc_lt);

            let not_acc_gt = self.push_not(acc_gt);
            let not_acc_lt = self.push_not(acc_lt);

            acc_gt = self.push_and(gt, not_acc_lt);
            acc_lt = self.push_and(lt, not_acc_gt)
        }
        (acc_lt, acc_gt)
    }

    pub fn push_condswap(
        &mut self,
        s: GateIndex,
        x: GateIndex,
        y: GateIndex,
    ) -> (GateIndex, GateIndex) {
        if x == y {
            return (x, y);
        }
        let x_xor_y = self.push_xor(x, y);
        let swap = self.push_and(x_xor_y, s);
        let x_swapped = self.push_xor(x, swap);
        let y_swapped = self.push_xor(y, swap);
        (x_swapped, y_swapped)
    }

    pub fn push_sorter(
        &mut self,
        bits: usize,
        x: &[GateIndex],
        y: &[GateIndex],
    ) -> (Vec<GateIndex>, Vec<GateIndex>) {
        let (_, gt) = self.push_comparator_circuit(bits, x, false, y, false);
        let mut min = vec![];
        let mut max = vec![];
        for (x, y) in x.iter().zip(y.iter()) {
            let (a, b) = self.push_condswap(gt, *x, *y);
            min.push(a);
            max.push(b);
        }
        (min, max)
    }

    /// Constructs a basic circuit with both XOR and AND gates.
    pub fn build_basic_circuit(&mut self, input_x: GateIndex, input_y: GateIndex) -> GateIndex {
        let xor_output = self.push_xor(input_x, input_y); // First XOR operation
        let and_output = self.push_and(input_x, input_y); // First AND operation
        self.push_xor(xor_output, and_output) // XOR result with AND result
    }
}

#[cfg(test)]
mod tests {
    use garble_lang::circuit::Gate;

    use super::*;

    #[test]
    fn test_basic_circuit() {
        let mut builder = CircuitBuilder::new();
        let input_x = 0;
        let input_y = 1;
        let output = builder.build_basic_circuit(input_x, input_y);
        assert_eq!(builder.gates.len(), 3);
        assert_eq!(output, 2);
        assert_eq!(builder.gates[0], BuilderGate::Xor(input_x, input_y));
        assert_eq!(builder.gates[1], BuilderGate::And(input_x, input_y));
        assert_eq!(builder.gates[2], BuilderGate::Xor(0, 1));
    }

    #[test]
    fn test_adder() {
        let mut builder = CircuitBuilder::new();

        // Define input values (single-bit values for simplicity).
        let input_x: GateIndex = 0; // Representing a single bit (0 or 1)
        let input_y: GateIndex = 1; // Representing a single bit (0 or 1)
        let carry_in: GateIndex = 2; // Initial carry-in bit (can be 0 or 1)

        // Add a full adder to the circuit (x + y + carry_in).
        let (sum, carry_out) = builder.push_adder(input_x, input_y, carry_in);

        // Expected number of gates: 7 gates.
        assert_eq!(builder.gates.len(), 7);

        // The output wires (sum and carry_out) should be at the correct gate indices.
        assert_eq!(sum, 2); // Sum is at index 2 (result of the first XOR and carry_in)
        assert_eq!(carry_out, 6); // Carry-out from the OR of the two AND gates

        // Verify that the gates created are correct.
        assert_eq!(builder.gates[0], BuilderGate::Xor(input_x, input_y)); // XOR of x and y
        assert_eq!(builder.gates[1], BuilderGate::And(input_x, input_y)); // AND of x and y
        assert_eq!(builder.gates[2], BuilderGate::Xor(0, carry_in)); // XOR of the XOR result and carry_in
        assert_eq!(builder.gates[3], BuilderGate::And(0, carry_in)); // AND of the XOR result and carry_in

        // The carry out is computed using the push_or method, so the final carry is at gate 6.
        assert_eq!(builder.gates[6], BuilderGate::Xor(4, 5)); // OR result using XOR of two ANDs
    }

    #[test]
    fn test_full_adder_chain() {
        let mut builder = CircuitBuilder::new();

        // Create a chain of adders with carry propagation.
        let input_a = vec![0, 1]; // First 2-bit input
        let input_b = vec![2, 3]; // Second 2-bit input
        let mut carry = 0; // Initial carry-in is 0

        let mut sum = vec![];

        // Perform bitwise addition for each pair of bits, starting from the least significant bit.
        for i in (0..2).rev() {
            let (s, c) = builder.push_adder(input_a[i], input_b[i], carry);
            sum.push(s);
            carry = c;
        }

        // Expected number of gates after two full adders.
        // Each adder produces 7 gates, so 14 gates total.
        assert_eq!(builder.gates.len(), 14);

        // Verify the sum output.
        assert_eq!(sum.len(), 2);
        assert_eq!(sum[0], 6); // First sum from the last XOR
        assert_eq!(sum[1], 13); // Second sum from the second XOR in the chain

        // Verify the carry-out from the chain.
        assert_eq!(carry, 12); // The final carry-out after both adders
    }
}
