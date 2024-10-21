use crate::executor::get_executor;
use crate::operations::circuits::builder::build_and_execute_addition;
use crate::uint::GarbledUint;
use std::ops::Add;
use tandem::{Circuit, Gate, GateIndex};

#[derive(Clone)]
pub struct CircuitBuilder<const N: usize> {
    gates: Vec<Gate>,
    input_offset: GateIndex,
}

impl<const N: usize> Default for CircuitBuilder<N> {
    fn default() -> Self {
        Self {
            gates: Vec::new(),
            input_offset: 0,
        }
    }
}

impl<const N: usize> CircuitBuilder<N> {
    pub fn push_input<const R: usize>(&mut self, input: &GarbledUint<R>) -> Vec<GateIndex> {
        let mut indices = Vec::with_capacity(R);
        for bit in &input.bits {
            self.gates.push(Gate::InContrib);
            indices.push(self.gates.len() as GateIndex);
        }
        indices
    }

    pub fn push_xor(&mut self, a: GateIndex, b: GateIndex) -> GateIndex {
        let xor_index = self.gates.len() as u32;
        self.gates.push(Gate::Xor(a, b));
        xor_index
    }

    pub fn push_and(&mut self, a: GateIndex, b: GateIndex) -> GateIndex {
        let and_index = self.gates.len() as u32;
        self.gates.push(Gate::And(a, b));
        and_index
    }

    pub fn push_not(&mut self, a: GateIndex) -> GateIndex {
        let not_index = self.gates.len() as u32;
        self.gates.push(Gate::Not(a));
        not_index
    }

    pub fn build(self, output_indices: Vec<GateIndex>) -> Circuit {
        Circuit::new(self.gates, output_indices)
    }

    pub fn execute_with_input(
        &self,
        input: &[bool],
        output_indices: Vec<u32>,
    ) -> anyhow::Result<GarbledUint<N>> {
        let program = Circuit::new(self.gates.clone(), output_indices);
        let result = get_executor().execute(&program, input, &[])?;

        // debug print big-endian and little-endian results as u32
        let res = result
            .clone()
            .iter()
            .rev()
            .fold(0, |acc, &x| acc * 2 + x as u32);
        println!("big-endian: {}", res);

        let res = result.clone().iter().fold(0, |acc, &x| acc * 2 + x as u32);
        println!("little-endian: {}", res);

        Ok(GarbledUint::new(result))
    }
}

// Modify GarbledUint to track its position in the circuit
#[derive(Clone)]
pub struct GarbledUintWithBuilder<const N: usize> {
    pub builder: CircuitBuilder<N>,
    pub bits: Vec<GateIndex>,
}

// Implement the Add operation for Uint<N>
impl<const N: usize> Add for GarbledUintWithBuilder<N> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        let mut result_bits = Vec::with_capacity(N);
        let mut carry = None;

        for i in 0..N {
            let (sum, new_carry) = self.builder.full_adder(self.bits[i], rhs.bits[i], carry);
            result_bits.push(sum);
            carry = new_carry;
        }

        self.bits = result_bits;
        self
    }
}

impl<const N: usize> CircuitBuilder<N> {
    // Full adder logic for addition
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

    pub fn len(&self) -> GateIndex {
        self.gates.len() as u32
    }
}

// Convert GarbledUint into a GarbledUintWithBuilder, linking the builder with the bits
impl<const N: usize> From<GarbledUint<N>> for GarbledUintWithBuilder<N> {
    fn from(value: GarbledUint<N>) -> Self {
        let mut builder = CircuitBuilder::default();
        let bits = builder.push_input(&value);
        GarbledUintWithBuilder { builder, bits }
    }
}

// Test for multi-addition in a single circuit
#[cfg(test)]
mod tests {
    use super::*;
    use crate::uint::GarbledUint8;

    #[test]
    fn test_uint_add_multi() {
        let a: GarbledUint8 = 70_u8.into();
        let b: GarbledUint8 = 85_u8.into();
        let c: GarbledUint8 = 8_u8.into();

        let a_with_builder: GarbledUintWithBuilder<8> = a.into();
        let b_with_builder: GarbledUintWithBuilder<8> = b.into();
        let c_with_builder: GarbledUintWithBuilder<8> = c.into();

        let result = a_with_builder + b_with_builder + c_with_builder;
        let result_value: u8 = result
            .builder
            .execute_with_input(&[true; 8], result.bits)
            .unwrap()
            .into();

        assert_eq!(result_value, 70_u8 + 85_u8 + 8_u8);
    }
}
