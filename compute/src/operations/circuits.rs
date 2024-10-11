use crate::simulator::simulate;
use crate::uint::GarbledUint;
use std::cmp::Ordering;
use tandem::GateIndex;
use tandem::{Circuit, Gate};

pub struct CircuitBuilder<const N: usize> {
    gates: Vec<Gate>,
}

impl<const N: usize> Default for CircuitBuilder<N> {
    fn default() -> Self {
        let gates = Vec::new();
        Self { gates }
    }
}

impl<const N: usize> CircuitBuilder<N> {
    pub fn add_input<const R: usize>(&mut self, input: &GarbledUint<R>) {
        for _ in &input.bits {
            self.gates.push(Gate::InContrib);
        }
    }

    pub fn len(&self) -> GateIndex {
        self.gates.len() as u32
    }

    // Add a XOR gate between two inputs and return the index
    pub fn add_xor(&mut self, a: GateIndex, b: GateIndex) -> GateIndex {
        let xor_index = self.gates.len() as u32;
        self.gates.push(Gate::Xor(a, b));
        xor_index
    }

    // Add an AND gate between two inputs and return the index
    pub fn add_and(&mut self, a: GateIndex, b: GateIndex) -> GateIndex {
        let and_index = self.gates.len() as u32;
        self.gates.push(Gate::And(a, b));
        and_index
    }

    // Add a NOT gate for a single input and return the index
    pub fn add_not(&mut self, a: GateIndex) -> GateIndex {
        let not_index = self.gates.len() as u32;
        self.gates.push(Gate::Not(a));
        not_index
    }

    // Add a gate for OR operation: OR(a, b) = (a ⊕ b) ⊕ (a & b)
    pub fn add_or(&mut self, a: GateIndex, b: GateIndex) -> GateIndex {
        let xor_gate = self.add_xor(a, b);
        let and_gate = self.add_and(a, b);
        self.add_xor(xor_gate, and_gate)
    }

    // Add a NAND gate: NAND(a, b) = NOT(a & b)
    pub fn add_nand(&mut self, a: GateIndex, b: GateIndex) -> GateIndex {
        let and_gate = self.add_and(a, b);
        self.add_not(and_gate)
    }

    // Add a NOR gate: NOR(a, b) = NOT(OR(a, b))
    pub fn add_nor(&mut self, a: GateIndex, b: GateIndex) -> GateIndex {
        let or_gate = self.add_or(a, b);
        self.add_not(or_gate)
    }

    // Add an XNOR gate: XNOR(a, b) = NOT(a ⊕ b)
    pub fn add_xnor(&mut self, a: GateIndex, b: GateIndex) -> GateIndex {
        let xor_gate = self.add_xor(a, b);
        self.add_not(xor_gate)
    }

    #[allow(dead_code)]
    // Add a MUX gate: MUX(a, b, s) = (a & !s) | (b & s)
    pub fn add_mux(&mut self, a: GateIndex, b: GateIndex, s: GateIndex) -> GateIndex {
        let not_s = self.add_xor(s, 1); //self.add_not(s);
        let a_selected = self.add_and(a, s);
        let b_selected = self.add_and(b, not_s);
        self.add_xor(a_selected, b_selected)
    }

    // Build and return a Circuit from the current gates with given output indices
    pub fn build(self, output_indices: Vec<GateIndex>) -> Circuit {
        Circuit::new(self.gates, output_indices)
    }

    fn add_garbled_uints(&mut self, a: &[GateIndex], b: &[GateIndex]) -> Vec<GateIndex> {
        let mut result = Vec::with_capacity(a.len());
        let mut carry = None;

        for i in 0..a.len() {
            let sum = self.full_adder(a[i], b[i], carry);
            result.push(sum.0);
            carry = sum.1;
        }

        result
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

    // Simulate the circuit using the provided input values
    pub fn execute(
        &self,
        lhs: &GarbledUint<N>,
        rhs: &GarbledUint<N>,
        output_indices: Vec<u32>,
    ) -> anyhow::Result<GarbledUint<N>> {
        let input = [lhs.bits.clone(), rhs.bits.clone()].concat();
        self.execute_with_input(&input, output_indices)
    }

    pub fn execute_with_input(
        &self,
        input: &[bool],
        output_indices: Vec<u32>,
    ) -> anyhow::Result<GarbledUint<N>> {
        let program = Circuit::new(self.gates.clone(), output_indices);
        let result = simulate(&program, input, &[])?;
        Ok(GarbledUint::new(result))
    }
}

pub(super) fn build_and_execute_xor<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.add_input(lhs);
    builder.add_input(rhs);

    // Add XOR gates for each bit
    let mut output_indices = Vec::with_capacity(N);
    for i in 0..N {
        let xor_gate = builder.add_xor(i as u32, (N + i) as u32);
        output_indices.push(xor_gate);
    }

    // Simulate the circuit
    builder
        .execute(lhs, rhs, output_indices)
        .expect("Failed to execute XOR circuit")
}

pub(super) fn build_and_execute_and<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.add_input(lhs);
    builder.add_input(rhs);

    // Add AND gates for each bit
    let mut output_indices = Vec::with_capacity(N);
    for i in 0..N {
        let and_gate = builder.add_and(i as u32, (N + i) as u32);
        output_indices.push(and_gate);
    }

    // Simulate the circuit
    builder
        .execute(lhs, rhs, output_indices)
        .expect("Failed to execute AND circuit")
}

pub(super) fn build_and_execute_or<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.add_input(lhs);
    builder.add_input(rhs);

    // Add OR gates for each bit
    let mut output_indices = Vec::with_capacity(N);
    for i in 0..N {
        let or_gate = builder.add_or(i as u32, (N + i) as u32);
        output_indices.push(or_gate);
    }

    // Simulate the circuit
    builder
        .execute(lhs, rhs, output_indices)
        .expect("Failed to execute OR circuit")
}

pub(super) fn build_and_execute_addition<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.add_input(lhs);
    builder.add_input(rhs);

    let mut carry = None;

    // Create a full adder for each bit
    let mut output_indices = Vec::with_capacity(N);
    for i in 0..N {
        // let (sum, new_carry) = full_adder(&mut builder, i as u32, (N + i) as u32, carry);

        let (sum, new_carry) = builder.full_adder(i as GateIndex, (N + i) as GateIndex, carry);
        output_indices.push(sum);
        carry = new_carry;
    }

    // Simulate the circuit
    builder
        .execute(lhs, rhs, output_indices)
        .expect("Failed to execute addition circuit")
}

pub(super) fn build_and_execute_subtraction<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.add_input(lhs);
    builder.add_input(rhs);

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
        .execute(lhs, rhs, output_indices)
        .expect("Failed to execute subtraction circuit")
}

fn full_subtractor<const N: usize>(
    builder: &mut CircuitBuilder<N>,
    a: u32,
    b: u32,
    borrow: Option<u32>,
) -> (u32, Option<u32>) {
    // XOR gate for difference bit (a ⊕ b)
    let xor_ab = builder.add_xor(a, b);

    // If borrow exists, XOR the result of the previous XOR with the borrow
    let diff = if let Some(borrow) = borrow {
        builder.add_xor(xor_ab, borrow)
    } else {
        xor_ab
    };

    // Compute the new borrow: (!a & b) | (a & borrow) | (!b & borrow)
    let not_a = builder.add_not(a);
    let and_not_a_b = builder.add_and(not_a, b);

    let new_borrow = if let Some(borrow) = borrow {
        let and_a_borrow = builder.add_and(a, borrow);
        let not_b = builder.add_not(b);
        let and_not_b_borrow = builder.add_and(not_b, borrow);

        // Combine borrow parts using XOR and AND to simulate OR
        let xor_borrow_parts = builder.add_xor(and_not_a_b, and_a_borrow);
        builder.add_xor(xor_borrow_parts, and_not_b_borrow)
    } else {
        and_not_a_b
    };

    (diff, Some(new_borrow))
}

pub(super) fn build_and_execute_multiplication<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.add_input(lhs);
    builder.add_input(rhs);

    let mut partial_products = Vec::with_capacity(N);

    // Generate partial products
    for i in 0..N {
        let shifted_product = generate_partial_product(&mut builder, 0, N as GateIndex, i);
        partial_products.push(shifted_product);
    }

    // Sum up all partial products
    let mut result = partial_products[0].clone();
    for partial_product in partial_products.iter().take(N).skip(1) {
        result = builder.add_garbled_uints(&result, partial_product);
    }

    // Simulate the circuit
    builder
        .execute(lhs, rhs, result.to_vec())
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
            builder.add_not(rhs_start);
            builder.add_and(rhs_start, zero_bit); // Constant 0
            partial_product.push(builder.len() - 1);
        } else {
            let lhs_bit = lhs_start + (i - shift) as u32;
            let and_gate = builder.len();
            builder.add_and(lhs_bit, rhs_start + shift as u32);
            partial_product.push(and_gate);
        }
    }

    partial_product
}

pub(super) fn build_and_execute_nand<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.add_input(lhs);
    builder.add_input(rhs);

    let mut output_indices = Vec::with_capacity(N);

    for i in 0..N {
        let nand_gate = builder.add_nand(i as u32, (N + i) as u32);
        output_indices.push(nand_gate);
    }

    builder
        .execute(lhs, rhs, output_indices)
        .expect("Failed to execute NAND circuit")
}

pub(super) fn build_and_execute_nor<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.add_input(lhs);
    builder.add_input(rhs);

    let mut output_indices = Vec::with_capacity(N);

    for i in 0..N {
        let nor_gate = builder.add_nor(i as u32, (N + i) as u32);
        output_indices.push(nor_gate);
    }

    builder
        .execute(lhs, rhs, output_indices)
        .expect("Failed to execute NOR circuit")
}

pub(super) fn build_and_execute_xnor<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.add_input(lhs);
    builder.add_input(rhs);

    let mut output_indices = Vec::with_capacity(N);

    for i in 0..N {
        let xnor_gate = builder.add_xnor(i as u32, (N + i) as u32);
        output_indices.push(xnor_gate);
    }

    builder
        .execute(lhs, rhs, output_indices)
        .expect("Failed to execute XNOR circuit")
}

pub(super) fn build_and_execute_equality<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> bool {
    let mut builder: CircuitBuilder<N> = CircuitBuilder::default();
    builder.add_input(lhs);
    builder.add_input(rhs);

    let mut result = builder.add_xnor(0, N as u32);

    for i in 1..N {
        let current_comparison = builder.add_xnor(i as u32, (N + i) as u32);
        result = builder.add_and(result, current_comparison);
    }
    let result = builder.execute(lhs, rhs, vec![result]).unwrap();
    result.bits[0]
}

pub(super) fn build_and_execute_comparator<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> Ordering {
    let mut builder: CircuitBuilder<N> = CircuitBuilder::default();
    builder.add_input(lhs);
    builder.add_input(rhs);

    let (lt_output, eq_output) = comparator_circuit::<N>(&mut builder);

    let program = builder.build(vec![lt_output, eq_output]);
    let input = [lhs.bits.clone(), rhs.bits.clone()].concat();
    let result = simulate(&program, &input, &[]).unwrap();

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
    let eq_i = builder.add_xnor(i as u32, (N + i) as u32);
    eq_list[i] = eq_i;

    let nt = builder.add_not(i as u32);
    let lt_i = builder.add_and(nt, (N + i) as u32);
    lt_list[i] = lt_i;

    for idx in (0..i).rev() {
        let xn = builder.add_xnor(idx as u32, (N + idx) as u32);
        let eq_i = builder.add_and(eq_list[idx + 1], xn);
        eq_list[idx] = eq_i;

        let nt = builder.add_not(idx as u32);
        let aa = builder.add_and(nt, (N + idx) as u32);
        let temp_lt = builder.add_and(eq_list[idx + 1], aa);
        lt_list[idx] = builder.add_or(lt_list[idx + 1], temp_lt);
    }

    (lt_list[0], eq_list[0])
}

pub(super) fn build_and_execute_not<const N: usize>(input: &GarbledUint<N>) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.add_input(input);

    let mut output_indices = Vec::with_capacity(N);

    for i in 0..N {
        let not_gate = builder.add_not(i as u32);
        output_indices.push(not_gate);
    }

    builder
        .execute_with_input(&input.bits, output_indices)
        .expect("Failed to execute NOT circuit")
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::uint::GarbledUint1;
    use crate::uint::GarbledUint8;

    #[test]
    fn test_mux() {
        const N: usize = 8;

        let mut builder: CircuitBuilder<N> = CircuitBuilder::default();
        let a: GarbledUint8 = 182_u8.into(); // if true, output should be 35
        let b: GarbledUint8 = 42_u8.into(); // if false, output should be 42
        let s: GarbledUint1 = true.into();

        builder.add_input(&a);
        builder.add_input(&b);
        builder.add_input(&s);

        // Add MUX gates for each bit
        let mut output_indices = Vec::with_capacity(N);
        for i in 0..N {
            let mux_gate = builder.add_mux(i as u32, (N + i) as u32, (2 * N) as u32);
            output_indices.push(mux_gate);
        }

        // combine the three inputs into a single value
        let input = [a.bits.clone(), b.bits.clone(), s.bits].concat();

        // Simulate the circuit
        let result = builder
            .execute_with_input(&input, output_indices.clone())
            .expect("Failed to execute MUX circuit");

        println!("MUX result: {}", result);
        assert_eq!(result, a);

        let s: GarbledUint1 = false.into();
        // combine the three inputs into a single value
        let input = [a.bits.clone(), b.bits.clone(), s.bits].concat();

        // Simulate the circuit
        let result = builder
            .execute_with_input(&input, output_indices)
            .expect("Failed to execute MUX circuit");

        println!("MUX result: {}", result);
        assert_eq!(result, b);
    }
}
