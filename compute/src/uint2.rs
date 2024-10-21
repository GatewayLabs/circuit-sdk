use crate::executor::get_executor;
use crate::int::GarbledInt;
use crate::operations::circuits::builder::get_circuit_builder_8;
use crate::operations::circuits::builder::CIRCUIT_BUILDER_8;
use crate::uint::GarbledUintBits;
use std::borrow::BorrowMut;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::BitXor;
use std::sync::Arc;
use tandem::Circuit;
use tandem::Gate;
use tandem::GateIndex;

pub type GarbledBoolean = TestGarbledUint<1>;
pub type GarbledBit = TestGarbledUint<1>;
pub type TestGarbledUint2 = TestGarbledUint<2>;
pub type TestGarbledUint4 = TestGarbledUint<4>;
pub type TestGarbledUint8 = TestGarbledUint<8>;
pub type TestGarbledUint16 = TestGarbledUint<16>;
pub type TestGarbledUint32 = TestGarbledUint<32>;
pub type TestGarbledUint64 = TestGarbledUint<64>;
pub type TestGarbledUint128 = TestGarbledUint<128>;

impl<const N: usize> GarbledUintBits for TestGarbledUint<N> {
    fn bits(&self) -> &[bool] {
        &self.bits
    }
}

// Define a new type Uint<N>
#[derive(Debug, Clone)]
pub struct TestGarbledUint<const N: usize> {
    pub bits: Vec<bool>,              // Store the bits of the unsigned integer
    _phantom: PhantomData<[bool; N]>, // PhantomData to ensure the N bit size
}

impl<const N: usize> TestGarbledUint<N> {
    pub fn zero() -> Self {
        TestGarbledUint::new(vec![false])
    }

    pub fn one() -> Self {
        TestGarbledUint::new(vec![true])
    }
}

impl<const N: usize> Display for TestGarbledUint<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u128::from(self.clone()))
    }
}

// Implement Uint<N>
impl<const N: usize> TestGarbledUint<N> {
    // Constructor for TestGarbledUint<N> from a boolean vector
    pub fn new(bits: Vec<bool>) -> Self {
        assert_eq!(bits.len(), N, "The number of bits must be {}", N);
        TestGarbledUint {
            bits,
            _phantom: PhantomData,
        }
    }
}

impl<const N: usize> From<GarbledInt<N>> for TestGarbledUint<N> {
    fn from(uint: GarbledInt<N>) -> Self {
        // Directly copy the bits from the unsigned Uint<N> to the signed GarbledInt<N>
        TestGarbledUint {
            bits: uint.bits,
            _phantom: PhantomData,
        }
    }
}

impl<const N: usize> From<&GarbledInt<N>> for TestGarbledUint<N> {
    fn from(int: &GarbledInt<N>) -> Self {
        TestGarbledUint {
            bits: int.bits.clone(),
            _phantom: PhantomData,
        }
    }
}

impl From<bool> for GarbledBit {
    fn from(value: bool) -> Self {
        TestGarbledUint::new(vec![value])
    }
}

impl<const N: usize> From<u8> for TestGarbledUint<N> {
    fn from(value: u8) -> Self {
        assert!(N <= 8, "Uint<N> can only support up to 8 bits for u8");

        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        TestGarbledUint::new(bits)
    }
}

impl<const N: usize> From<u16> for TestGarbledUint<N> {
    fn from(value: u16) -> Self {
        assert!(N <= 16, "Uint<N> can only support up to 16 bits for u16");

        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        TestGarbledUint::new(bits)
    }
}

impl<const N: usize> From<u32> for TestGarbledUint<N> {
    fn from(value: u32) -> Self {
        assert!(N <= 32, "Uint<N> can only support up to 32 bits for u32");

        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        TestGarbledUint::new(bits)
    }
}

impl<const N: usize> From<u64> for TestGarbledUint<N> {
    fn from(value: u64) -> Self {
        assert!(N <= 64, "Uint<N> can only support up to 64 bits for u64");

        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        TestGarbledUint::new(bits)
    }
}

impl<const N: usize> From<u128> for TestGarbledUint<N> {
    fn from(value: u128) -> Self {
        assert!(N <= 128, "Uint<N> can only support up to 128 bits for u128");

        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        TestGarbledUint::new(bits)
    }
}

impl From<GarbledBit> for bool {
    fn from(guint: TestGarbledUint<1>) -> Self {
        guint.bits[0]
    }
}

impl<const N: usize> From<TestGarbledUint<N>> for u8 {
    fn from(guint: TestGarbledUint<N>) -> Self {
        assert!(N <= 8, "Uint<N> can only be converted to u8 if N <= 8");

        let mut value: u8 = 0;
        for (i, &bit) in guint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

impl<const N: usize> From<TestGarbledUint<N>> for u16 {
    fn from(guint: TestGarbledUint<N>) -> Self {
        assert!(N <= 16, "Uint<N> can only be converted to u16 if N <= 16");

        let mut value: u16 = 0;
        for (i, &bit) in guint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

impl<const N: usize> From<TestGarbledUint<N>> for u32 {
    fn from(guint: TestGarbledUint<N>) -> Self {
        assert!(N <= 32, "Uint<N> can only be converted to u32 if N <= 32");

        let mut value: u32 = 0;
        for (i, &bit) in guint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

impl<const N: usize> From<TestGarbledUint<N>> for u64 {
    fn from(guint: TestGarbledUint<N>) -> Self {
        assert!(N <= 64, "Uint<N> can only be converted to u64 if N <= 64");

        let mut value: u64 = 0;
        for (i, &bit) in guint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

impl<const N: usize> From<TestGarbledUint<N>> for u128 {
    fn from(guint: TestGarbledUint<N>) -> Self {
        assert!(
            N <= 128,
            "Uint<N> can only be converted to u128 if N <= 128"
        );

        let mut value: u128 = 0;
        for (i, &bit) in guint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

//// operations

// Implement the XOR operation for Uint<N>
impl<const N: usize> BitXor for TestGarbledUint<N> {
    type Output = Self;

    fn bitxor(self, _rhs: Self) -> Self::Output {
        accumulate_xor::<N>();
        self
    }
}

// Implement the XOR operation for &TestGarbledUint<N>
impl<const N: usize> BitXor for &TestGarbledUint<N> {
    type Output = TestGarbledUint<N>;

    fn bitxor(self, _rhs: Self) -> Self::Output {
        accumulate_xor::<N>();
        self.clone()
    }
}

pub struct CircuitBuilder<const N: usize> {
    gates: Vec<Gate>,
}

impl<const N: usize> CircuitBuilder<N> {
    pub fn new(inputs: Vec<&dyn GarbledUintBits>) -> Self {
        let mut gates = Vec::new();

        // Iterate over the inputs and push a `Gate::InContrib` for each bit
        gates.extend(
            inputs
                .iter()
                .flat_map(|input| input.bits().iter().map(|_| Gate::InContrib)),
        );

        Self { gates }
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

    pub fn execute(
        &mut self,
        inputs: Vec<&TestGarbledUint<N>>,
        output_indices: Vec<u32>,
    ) -> anyhow::Result<TestGarbledUint<N>> {
        // Combine all input bits into a single concatenated vector
        let combined_input: Vec<bool> =
            inputs.iter().flat_map(|input| input.bits.clone()).collect();

        // Execute the circuit with the concatenated inputs
        self.execute_with_input(&combined_input, output_indices)
    }

    pub fn execute_with_input(
        &mut self,
        input: &[bool],
        output_indices: Vec<u32>,
    ) -> anyhow::Result<TestGarbledUint<N>> {
        let program = Circuit::new(self.gates.clone(), output_indices);
        let result = get_executor().execute(&program, input, &[])?;
        Ok(TestGarbledUint::new(result))
    }
}

pub(crate) fn accumulate_xor<const N: usize>() {
    //let mut builder = CircuitBuilder::new(vec![lhs, rhs]);

    //let builder = CIRCUIT_BUILDER_8.borrow_mut();
    let builder = get_circuit_builder_8();
    //let builder = Arc::clone(&builder);
    let mut builder = builder.write().unwrap();

    // Add XOR gates for each bit
    for i in 0..N {
        let xor_gate = builder.push_xor(i as u32, (N + i) as u32);
        builder.add_output(xor_gate);
    }

    // Simulate the circuit
    //builder
    //    .execute(vec![lhs, rhs], output_indices)
    //    .expect("Failed to execute XOR circuit")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_xor_8() {
        let lhs: TestGarbledUint8 = 5_u8.into();
        let rhs: TestGarbledUint8 = 8_u8.into();

        {
            let builder = get_circuit_builder_8();
            //let builder = Arc::clone(&builder);
            let mut builder = builder.write().unwrap();
            builder.inputs(vec![&lhs, &rhs]);
        }

        let _result = (&lhs ^ &rhs) ^ (&rhs ^ &lhs);

        {
            let builder = get_circuit_builder_8();
            //let builder = Arc::clone(&builder);
            let mut builder = builder.write().unwrap();
            let result = builder.execute2().unwrap();
            let res: u8 = result.into();
            assert_eq!(res, (5_u8 ^ 8_u8) ^ (8_u8 ^ 5_u8));
        }
    }
}
