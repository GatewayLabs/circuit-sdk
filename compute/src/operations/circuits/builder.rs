use crate::executor::get_executor;
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
    pub fn push_input<const R: usize>(&mut self, input: &GarbledUint<R>) {
        for _ in &input.bits {
            self.gates.push(Gate::InContrib);
        }
    }

    pub fn len(&self) -> GateIndex {
        self.gates.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.gates.is_empty()
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
        let result = get_executor().execute(&program, input, &[])?;
        Ok(GarbledUint::new(result))
    }
}

pub(crate) fn build_and_execute_xor<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.push_input(lhs);
    builder.push_input(rhs);

    // Add XOR gates for each bit
    let mut output_indices = Vec::with_capacity(N);
    for i in 0..N {
        let xor_gate = builder.push_xor(i as u32, (N + i) as u32);
        output_indices.push(xor_gate);
    }

    // Simulate the circuit
    builder
        .execute(lhs, rhs, output_indices)
        .expect("Failed to execute XOR circuit")
}

pub(crate) fn build_and_execute_and<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.push_input(lhs);
    builder.push_input(rhs);

    // Add AND gates for each bit
    let mut output_indices = Vec::with_capacity(N);
    for i in 0..N {
        let and_gate = builder.push_and(i as u32, (N + i) as u32);
        output_indices.push(and_gate);
    }

    // Simulate the circuit
    builder
        .execute(lhs, rhs, output_indices)
        .expect("Failed to execute AND circuit")
}

pub(crate) fn build_and_execute_or<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.push_input(lhs);
    builder.push_input(rhs);

    // Add OR gates for each bit
    let mut output_indices = Vec::with_capacity(N);
    for i in 0..N {
        let or_gate = builder.push_or(i as u32, (N + i) as u32);
        output_indices.push(or_gate);
    }

    // Simulate the circuit
    builder
        .execute(lhs, rhs, output_indices)
        .expect("Failed to execute OR circuit")
}

pub(crate) fn build_and_execute_addition<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.push_input(lhs);
    builder.push_input(rhs);

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
        .execute(lhs, rhs, output_indices)
        .expect("Failed to execute addition circuit")
}

pub(crate) fn build_and_execute_subtraction<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.push_input(lhs);
    builder.push_input(rhs);

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
    let mut builder = CircuitBuilder::default();
    builder.push_input(lhs);
    builder.push_input(rhs);

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
        .execute(lhs, rhs, result.to_vec())
        .expect("Failed to execute multiplication circuit")
}

pub(crate) fn build_and_execute_division<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.push_input(lhs);
    builder.push_input(rhs);

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
        .execute(lhs, rhs, result.to_vec())
        .expect("Failed to execute division circuit")
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
    let mut builder = CircuitBuilder::default();
    builder.push_input(lhs);
    builder.push_input(rhs);

    let mut output_indices = Vec::with_capacity(N);

    for i in 0..N {
        let nand_gate = builder.push_nand(i as u32, (N + i) as u32);
        output_indices.push(nand_gate);
    }

    builder
        .execute(lhs, rhs, output_indices)
        .expect("Failed to execute NAND circuit")
}

pub(crate) fn build_and_execute_nor<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.push_input(lhs);
    builder.push_input(rhs);

    let mut output_indices = Vec::with_capacity(N);

    for i in 0..N {
        let nor_gate = builder.push_nor(i as u32, (N + i) as u32);
        output_indices.push(nor_gate);
    }

    builder
        .execute(lhs, rhs, output_indices)
        .expect("Failed to execute NOR circuit")
}

pub(crate) fn build_and_execute_xnor<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.push_input(lhs);
    builder.push_input(rhs);

    let mut output_indices = Vec::with_capacity(N);

    for i in 0..N {
        let xnor_gate = builder.push_xnor(i as u32, (N + i) as u32);
        output_indices.push(xnor_gate);
    }

    builder
        .execute(lhs, rhs, output_indices)
        .expect("Failed to execute XNOR circuit")
}

pub(crate) fn build_and_execute_equality<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> bool {
    let mut builder: CircuitBuilder<N> = CircuitBuilder::default();
    builder.push_input(lhs);
    builder.push_input(rhs);

    let mut result = builder.push_xnor(0, N as u32);

    for i in 1..N {
        let current_comparison = builder.push_xnor(i as u32, (N + i) as u32);
        result = builder.push_and(result, current_comparison);
    }
    let result = builder.execute(lhs, rhs, vec![result]).unwrap();
    result.bits[0]
}

pub(crate) fn build_and_execute_comparator<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> Ordering {
    let mut builder: CircuitBuilder<N> = CircuitBuilder::default();
    builder.push_input(lhs);
    builder.push_input(rhs);

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
    let mut builder = CircuitBuilder::default();
    builder.push_input(input);

    let mut output_indices = Vec::with_capacity(N);

    for i in 0..N {
        let not_gate = builder.push_not(i as u32);
        output_indices.push(not_gate);
    }

    builder
        .execute_with_input(&input.bits, output_indices)
        .expect("Failed to execute NOT circuit")
}

#[allow(dead_code)]
pub(crate) fn build_and_execute_mux<const N: usize, const S: usize>(
    condition: &GarbledUint<S>,
    if_true: &GarbledUint<N>,
    if_false: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.push_input(if_false);
    builder.push_input(if_true);
    builder.push_input(condition);

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

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::uint::GarbledBit;
    use crate::uint::GarbledUint32;
    use crate::uint::GarbledUint64;
    use crate::uint::GarbledUint8;

    #[test]
    fn test_mux() {
        const N: usize = 32;

        let mut builder: CircuitBuilder<N> = CircuitBuilder::default();
        let a: GarbledUint32 = 1900142_u32.into(); // if s is false, output should be a
        let b: GarbledUint32 = 771843900_u32.into(); // if s is true, output should be b
        let s: GarbledBit = true.into();

        builder.push_input(&a);
        builder.push_input(&b);
        builder.push_input(&s);

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
}
