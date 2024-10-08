use crate::uint::GarbledUint;
use std::ops::{BitAnd, BitXor, Not, Shl, Shr};
use tandem::{Circuit, Gate};

// Helper function to build and simulate a circuit for binary operations
fn build_and_simulate<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: Option<&GarbledUint<N>>,
    gate_fn: fn(u32, u32) -> Gate,
) -> GarbledUint<N> {
    let mut gates = Vec::new();

    // Push input gates for both Uint<N> objects
    for _ in 0..N {
        gates.push(Gate::InContrib); // From first Uint<N> (lhs)
    }

    for _ in 0..N {
        gates.push(Gate::InEval); // From second Uint<N> (rhs)
    }

    // Define gates for each bit in lhs and rhs
    for i in 0..N {
        let gate = gate_fn(i as u32, (N + i) as u32);
        gates.push(gate);
    }

    // Define the output indices (for N-bit operation)
    let output_indices: Vec<u32> = (2 * N as u32..2 * N as u32 + N as u32).collect();

    // Create the circuit
    let program = Circuit::new(gates, output_indices);

    // Simulate the circuit
    let bits_rhs = rhs.map_or(lhs.bits.clone(), |r| r.bits.clone());
    let result = lhs.simulate(&program, &lhs.bits, &bits_rhs).unwrap();

    // Return the resulting Uint<N>
    GarbledUint::new(result)
}

// Implement the XOR operation for Uint<N>
impl<const N: usize> BitXor for GarbledUint<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        build_and_simulate(&self, Some(&rhs), Gate::Xor)
    }
}

// Implement the XOR operation for &Uint<N>
impl<const N: usize> BitXor for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        build_and_simulate(self, Some(rhs), Gate::Xor)
    }
}

// Implement the AND operation for Uint<N>
impl<const N: usize> BitAnd for GarbledUint<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        build_and_simulate(&self, Some(&rhs), Gate::And)
    }
}

// Implement the AND operation for &Uint<N>
impl<const N: usize> BitAnd for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn bitand(self, rhs: Self) -> Self::Output {
        build_and_simulate(self, Some(rhs), Gate::And)
    }
}

// Helper function to handle NOT operation (unary)
fn build_and_simulate_not<const N: usize>(input: &GarbledUint<N>) -> GarbledUint<N> {
    let mut gates = Vec::new();

    // Push input gates for Uint<N> object
    for _ in 0..N {
        gates.push(Gate::InContrib); // From first Uint<N> (lhs)
    }

    for _ in 0..N {
        gates.push(Gate::InEval); // From second Uint<N> (rhs)
    }

    // Define NOT gates for each bit in the Uint<N>
    for i in 0..N * 2 {
        gates.push(Gate::Not(i.try_into().unwrap())); // NOT gate for each bit
    }

    // Define the output indices (for N-bit NOT)
    let n = N as u32;
    let output_indices: Vec<u32> = (2 * n..2 * n + n).collect();

    // Create the circuit
    let program = Circuit::new(gates, output_indices);

    // Simulate the circuit
    let result = input.simulate(&program, &input.bits, &input.bits).unwrap();

    // Return the resulting Uint<N>
    GarbledUint::new(result)
}

// Implement the NOT operation for Uint<N>
impl<const N: usize> Not for GarbledUint<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        build_and_simulate_not(&self)
    }
}

// Implement the NOT operation for &Uint<N>
impl<const N: usize> Not for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn not(self) -> Self::Output {
        build_and_simulate_not(self)
    }
}

// Helper function for shift operations
fn shift_bits_left<const N: usize>(bits: &mut Vec<bool>, shift: usize) {
    for _ in 0..shift {
        bits.remove(N - 1); // Remove the most significant bit
        bits.insert(0, false); // Insert a 0 to the least significant bit
    }
}

fn shift_bits_right<const N: usize>(bits: &mut Vec<bool>, shift: usize) {
    for _ in 0..shift {
        bits.remove(0); // Remove the least significant bit
        bits.push(false); // Insert a 0 to the most significant bit
    }
}

// Implement Shift Left operation for Uint<N>
impl<const N: usize> Shl<usize> for GarbledUint<N> {
    type Output = Self;

    fn shl(mut self, shift: usize) -> Self::Output {
        shift_bits_left::<N>(&mut self.bits, shift);
        self
    }
}

// Implement Shift Left operation for &Uint<N>
impl<const N: usize> Shl<usize> for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn shl(self, shift: usize) -> Self::Output {
        let mut bits = self.bits.clone();
        shift_bits_left::<N>(&mut bits, shift);
        GarbledUint::new(bits)
    }
}

// Implement Shift Right operation for Uint<N>
impl<const N: usize> Shr<usize> for GarbledUint<N> {
    type Output = Self;

    fn shr(mut self, shift: usize) -> Self::Output {
        shift_bits_right::<N>(&mut self.bits, shift);
        self
    }
}

// Implement Shift Right operation for &Uint<N>
impl<const N: usize> Shr<usize> for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn shr(self, shift: usize) -> Self::Output {
        let mut bits = self.bits.clone();
        shift_bits_right::<N>(&mut bits, shift);
        GarbledUint::new(bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uint::{GarbledUint16, GarbledUint32, GarbledUint64, GarbledUint8, GarbledUint128};

    #[test]
    fn test_uint_xor() {
        let a = GarbledUint::<2>::new(vec![true, false]); // Binary 10
        let b = GarbledUint::<2>::new(vec![false, true]); // Binary 01

        let result = a ^ b; // Perform XOR on the 2-bit values
        assert_eq!(result.to_u8(), 3); // Expected result of XOR between 10 and 01

        let a = GarbledUint::<4>::new(vec![true, true, false, false]); // Binary 1100
        let b = GarbledUint::<4>::new(vec![false, false, true, true]); // Binary 0011

        let result = a ^ b; // Perform XOR on the 4-bit values
        assert_eq!(result.to_u8(), 15); // Expected result of XOR between 1100 and 0011
    }

    #[test]
    fn test_from_u8_xor() {
        let a = GarbledUint8::from_u8(170); // Binary 10101010
        let b = GarbledUint8::from_u8(85); // Binary 01010101

        let result = &a ^ &b;
        assert_eq!(result.to_u8(), 255); // Expected result of XOR between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_xor() {
        let a = GarbledUint16::from_u16(43690); // Binary 1010101010101010
        let b = GarbledUint16::from_u16(21845); // Binary 0101010101010101

        let result = a ^ b;
        assert_eq!(result.to_u16(), 65535); // Expected result of XOR between 1010101010101010 and 0101010101010101
    }

    #[test]
    fn test_from_u32_xor() {
        let a = GarbledUint32::from_u32(2863311530); // Binary 10101010101010101010101010101010
        let b = GarbledUint32::from_u32(1431655765); // Binary 01010101010101010101010101010101

        let result = a ^ b;
        assert_eq!(result.to_u32(), 4294967295); // Expected result of XOR between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_xor() {
        let a = GarbledUint64::from_u64(12297829382473034410); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b = GarbledUint64::from_u64(6148914691236517205); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result = a ^ b;
        assert_eq!(result.to_u64(), 18446744073709551615); // Expected result of XOR between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_xor() {
        let a = GarbledUint128::from_u128(170); // Binary 10101010
        let b = GarbledUint128::from_u128(85); // Binary 01010101

        let result = a ^ b;
        assert_eq!(result.to_u128(), 255); // Expected result of XOR between 10101010 and 01010101
    }

    #[test]
    fn test_uint_and() {
        let a = GarbledUint::<2>::new(vec![true, false]); // Binary 10
        let b = GarbledUint::<2>::new(vec![false, true]); // Binary 01

        let result = a & b; // Perform AND on the 2-bit values
        assert_eq!(result.to_u8(), 0); // Expected result of AND between 10 and 01

        let a = GarbledUint::<4>::new(vec![true, true, false, false]); // Binary 1100
        let b = GarbledUint::<4>::new(vec![false, false, true, true]); // Binary 0011

        let result = a & b; // Perform AND on the 4-bit values
        assert_eq!(result.to_u8(), 0); // Expected result of AND between 1100 and 0011

        let a = GarbledUint::<4>::new(vec![true, false, false, true]); // Binary 1001
        let b = GarbledUint::<4>::new(vec![false, false, false, false]); // Binary 0000

        let result = a & b; // Perform AND on the 4-bit values
        assert_eq!(result.to_u8(), 0); // Expected result of AND between 1001 and 0000
    }

    #[test]
    fn test_from_u8_and() {
        let a = GarbledUint8::from_u8(170); // Binary 10101010
        let b = GarbledUint8::from_u8(85); // Binary 01010101

        let result = a & b;
        assert_eq!(result.to_u8(), 170 & 85); // Expected result of AND between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_and() {
        let a = GarbledUint16::from_u16(43690); // Binary 1010101010101010
        let b = GarbledUint16::from_u16(21845); // Binary 0101010101010101

        let result = a & b;
        assert_eq!(result.to_u16(), 43690 & 21845); // Expected result of AND between 1010101010101010 and 0101010101010101
    }

    #[test]
    fn test_from_u32_and() {
        let a = GarbledUint32::from_u32(2863311530); // Binary 10101010101010101010101010101010
        let b = GarbledUint32::from_u32(1431655765); // Binary 01010101010101010101010101010101

        let result = a & b;
        assert_eq!(result.to_u32(), 2863311530 & 1431655765); // Expected result of AND between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_and() {
        let a = GarbledUint64::from_u64(12297829382473034410); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b = GarbledUint64::from_u64(6148914691236517205); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result = a & b;
        assert_eq!(result.to_u64(), 12297829382473034410 & 6148914691236517205);
        // Expected result of AND between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_and() {
        let a = GarbledUint128::from_u128(170); // Binary 10101010
        let b = GarbledUint128::from_u128(85); // Binary 01010101

        let result = a & b;
        assert_eq!(result.to_u128(), 170 & 85); // Expected result of AND between 10101010 and 01010101
    }

    #[test]
    fn test_from_u8_not() {
        let a = GarbledUint8::from_u8(170); // Binary 10101010

        let result = !a;
        assert_eq!(result.to_u8(), !170); // Expected result of NOT on 10101010
    }

    #[test]
    fn test_from_u16_not() {
        let a = GarbledUint16::from_u16(43690); // Binary 1010101010101010

        let result = !a;
        assert_eq!(result.to_u16(), !43690); // Expected result of NOT on 1010101010101010
    }

    #[test]
    fn test_from_u32_not() {
        let a = GarbledUint32::from_u32(2863311530); // Binary 10101010101010101010101010101010

        let result = !a;
        assert_eq!(result.to_u32(), !2863311530); // Expected result of NOT on 10101010101010101010101010101010
    }

    #[test]
    fn test_from_u64_not() {
        let a = GarbledUint64::from_u64(12297829382473034410); // Binary 1010101010101010101010101010101010101010101010101010101010101010

        let result = !a;
        assert_eq!(result.to_u64(), !12297829382473034410);
        // Expected result of NOT on 1010101010101010101010101010101010101010101010101010101010101010
    }

    #[test]
    fn test_from_u128_not() {
        let a = GarbledUint128::from_u128(170); // Binary 10101010

        let result = !a;
        assert_eq!(result.to_u128(), !170); // Expected result of NOT on 10101010
    }

    #[test]
    fn test_left_shift() {
        let a = GarbledUint::<4>::new(vec![false, false, false, true]); // Binary 1000

        let result = a << 1; // Perform left shift by 1
        assert_eq!(result.to_u8(), 0b0000_u8); // Binary 0000 (Left shift result of 1000)

        // binary literal of 0000

        let a = GarbledUint::<4>::new(vec![false, false, false, true]); // Binary 1000

        let result = a << 2; // Perform left shift by 2
        assert_eq!(result.to_u8(), 0b0000_u8); // Binary 0000 (Left shift result of 1000)

        let a = GarbledUint::<4>::new(vec![false, false, false, true]); // Binary 1000

        let result = a << 3; // Perform left shift by 3
        assert_eq!(result.to_u8(), 0b0000); // Binary 0000 (Left shift result of 1000)

        //let a = Uint::<4>::new(vec![false, false, false, true]); // Binary 0001

        let a = GarbledUint8::from_u8(1); // Binary 0001

        let result = a << 1; // Perform left shift by 1
        assert_eq!(result.to_u8(), 0b0010); // Binary 0010 (Left shift result of 0001)

        let a = GarbledUint::<4>::new(vec![true, false, false, false]); // Binary 0001

        let result = a << 2; // Perform left shift by 2
        assert_eq!(result.to_u8(), 0b0100); // Binary 0100 (Left shift result of 0001)

        let a = GarbledUint::<4>::new(vec![true, false, false, false]); // Binary 0001

        let result = a << 3; // Perform left shift by 3
        assert_eq!(result.to_u8(), 0b1000); // Binary 1000 (Left shift result of 0001)
    }

    #[test]
    fn test_right_shift() {
        let a = GarbledUint::<4>::new(vec![false, false, false, true]); // Binary 1000

        let result = a >> 1; // Perform right shift by 1
        assert_eq!(result.to_u8(), 0b0100); // Binary 0100 (Right shift result of 1000)

        let a = GarbledUint::<4>::new(vec![false, false, false, true]); // Binary 1000

        let result = a >> 2; // Perform right shift by 2
        assert_eq!(result.to_u8(), 0b0010); // Binary 0010 (Right shift result of 1000)

        let a = GarbledUint::<4>::new(vec![false, false, false, true]); // Binary 1000

        let result = a >> 3; // Perform right shift by 3
        assert_eq!(result.to_u8(), 0b0001); // Binary 0001 (Right shift result of 1000)
    }
}
