use crate::int::GarbledInt;
use crate::operations::helpers::{
    build_and_simulate, build_and_simulate_nand, build_and_simulate_nor, build_and_simulate_not,
    build_and_simulate_xnor,
};
use crate::simulator::simulate;
use crate::uint::GarbledUint;
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};
use tandem::{Circuit, Gate};

// Implement the XOR operation for Uint<N>
impl<const N: usize> BitXor for GarbledUint<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        build_and_simulate(&self, Some(&rhs), Gate::Xor)
    }
}

// Implement the XOR operation for &GarbledUint<N>
impl<const N: usize> BitXor for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        build_and_simulate(self, Some(rhs), Gate::Xor)
    }
}

// Implement the XOR operation for GarbledInt<N>
impl<const N: usize> BitXor for GarbledInt<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        build_and_simulate(&self.into(), Some(&rhs.into()), Gate::Xor).into()
    }
}

// Implement the XOR operation for &GarbledInt<N>
impl<const N: usize> BitXor for &GarbledInt<N> {
    type Output = GarbledInt<N>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        build_and_simulate(&self.into(), Some(&rhs.into()), Gate::Xor).into()
    }
}

// Implement the AND operation for Uint<N>
impl<const N: usize> BitAnd for GarbledUint<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        build_and_simulate(&self, Some(&rhs), Gate::And)
    }
}

// Implement the AND operation for &GarbledUint<N>
impl<const N: usize> BitAnd for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn bitand(self, rhs: Self) -> Self::Output {
        build_and_simulate(self, Some(rhs), Gate::And)
    }
}

// Implement the AND operation for GarbledInt<N>
impl<const N: usize> BitAnd for GarbledInt<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        build_and_simulate(&self.into(), Some(&rhs.into()), Gate::And).into()
    }
}

// Implement the AND operation for &GarbledInt<N>
impl<const N: usize> BitAnd for &GarbledInt<N> {
    type Output = GarbledInt<N>;

    fn bitand(self, rhs: Self) -> Self::Output {
        build_and_simulate(&self.into(), Some(&rhs.into()), Gate::And).into()
    }
}

// Implement the NOT operation for Uint<N>
impl<const N: usize> Not for GarbledUint<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        build_and_simulate_not(&self)
    }
}

// Implement the NOT operation for &GarbledUint<N>
impl<const N: usize> Not for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn not(self) -> Self::Output {
        build_and_simulate_not(self)
    }
}

// Helper function to build and simulate a circuit for OR operation
fn build_and_simulate_or<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: Option<&GarbledUint<N>>,
) -> GarbledUint<N> {
    let mut gates = Vec::new();

    // Push input gates for both Uint<N> objects (lhs and rhs)
    for _ in 0..N {
        gates.push(Gate::InContrib); // From first Uint<N> (lhs)
    }

    for _ in 0..N {
        gates.push(Gate::InEval); // From second Uint<N> (rhs)
    }

    // Define gates for each bit in lhs and rhs
    let mut output_indices = Vec::with_capacity(N);

    for i in 0..N {
        // OR(a, b) = (a ⊕ b) ⊕ (a & b)

        // Step 1: XOR gate for (a ⊕ b)
        let xor_gate = Gate::Xor(i as u32, (N + i) as u32);
        let xor_gate_idx = gates.len() as u32;
        gates.push(xor_gate);

        // Step 2: AND gate for (a & b)
        let and_gate = Gate::And(i as u32, (N + i) as u32);
        let and_gate_idx = gates.len() as u32;
        gates.push(and_gate);

        // Step 3: XOR gate for final OR result (a ⊕ b) ⊕ (a & b)
        let final_or_gate = Gate::Xor(xor_gate_idx, and_gate_idx);
        gates.push(final_or_gate);

        // Step 4: Store the output index of this bit's OR result
        output_indices.push(gates.len() as u32 - 1);
    }

    // Create the circuit
    let program = Circuit::new(gates, output_indices);

    // Simulate the circuit
    let bits_rhs = rhs.map_or(lhs.bits.clone(), |r| r.bits.clone());
    let result = simulate(&program, &lhs.bits, &bits_rhs).unwrap();

    // Return the resulting Uint<N>
    GarbledUint::new(result)
}

// Implement the OR operation for GarbledUint<N>
impl<const N: usize> BitOr for GarbledUint<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        build_and_simulate_or(&self, Some(&rhs))
    }
}

// Implement the OR operation for &GarbledUint<N>
impl<const N: usize> BitOr for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn bitor(self, rhs: Self) -> Self::Output {
        build_and_simulate_or(self, Some(rhs))
    }
}

// Implement the OR operation for GarbledInt<N>
impl<const N: usize> BitOr for GarbledInt<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        build_and_simulate_or(&self.into(), Some(&rhs.into())).into()
    }
}

// Implement the OR operation for &GarbledInt<N>
impl<const N: usize> BitOr for &GarbledInt<N> {
    type Output = GarbledInt<N>;

    fn bitor(self, rhs: Self) -> Self::Output {
        build_and_simulate_or(&self.into(), Some(&rhs.into())).into()
    }
}

// Implement the NOT operation for GarbledInt<N>
impl<const N: usize> Not for GarbledInt<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        build_and_simulate_not(&self.into()).into()
    }
}

// Implement the NOT operation for &GarbledInt<N>
impl<const N: usize> Not for &GarbledInt<N> {
    type Output = GarbledInt<N>;

    fn not(self) -> Self::Output {
        build_and_simulate_not(&self.into()).into()
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

// Implement Shift Left operation for &GarbledUint<N>
impl<const N: usize> Shl<usize> for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn shl(self, shift: usize) -> Self::Output {
        let mut bits = self.bits.clone();
        shift_bits_left::<N>(&mut bits, shift);
        GarbledUint::new(bits)
    }
}

// Implement Shift Left operation for GarbledInt<N>
impl<const N: usize> Shl<usize> for GarbledInt<N> {
    type Output = Self;

    fn shl(self, shift: usize) -> Self::Output {
        let mut bits = self.bits;
        shift_bits_left::<N>(&mut bits, shift);
        GarbledInt::new(bits)
    }
}

// Implement Shift Left operation for &GarbledInt<N>
impl<const N: usize> Shl<usize> for &GarbledInt<N> {
    type Output = GarbledInt<N>;

    fn shl(self, shift: usize) -> Self::Output {
        let mut bits = self.bits.clone();
        shift_bits_left::<N>(&mut bits, shift);
        GarbledInt::new(bits)
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

// Implement Shift Right operation for &GarbledUint<N>
impl<const N: usize> Shr<usize> for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn shr(self, shift: usize) -> Self::Output {
        let mut bits = self.bits.clone();
        shift_bits_right::<N>(&mut bits, shift);
        GarbledUint::new(bits)
    }
}

// Implement Shift Right operation for GarbledInt<N>
impl<const N: usize> Shr<usize> for GarbledInt<N> {
    type Output = Self;

    fn shr(self, shift: usize) -> Self::Output {
        let mut bits = self.bits;
        shift_bits_right::<N>(&mut bits, shift);
        GarbledInt::new(bits)
    }
}

// Implement Shift Right operation for &GarbledInt<N>
impl<const N: usize> Shr<usize> for &GarbledInt<N> {
    type Output = GarbledInt<N>;

    fn shr(self, shift: usize) -> Self::Output {
        let mut bits = self.bits.clone();
        shift_bits_right::<N>(&mut bits, shift);
        GarbledInt::new(bits)
    }
}

// Implement the NAND, NOR, XNOR operators for GarbledUint<N>
impl<const N: usize> GarbledUint<N> {
    pub fn nand(self, rhs: Self) -> Self {
        build_and_simulate_nand(&self, Some(&rhs))
    }

    pub fn nor(self, rhs: Self) -> Self {
        build_and_simulate_nor(&self, Some(&rhs))
    }

    pub fn xnor(self, rhs: Self) -> Self {
        build_and_simulate_xnor(&self, Some(&rhs))
    }
}

// Implement the NAND, NOR, XNOR operators for GarbledInt<N>
impl<const N: usize> GarbledInt<N> {
    pub fn nand(self, rhs: Self) -> Self {
        build_and_simulate_nand(&self.into(), Some(&rhs.into())).into()
    }

    pub fn nor(self, rhs: Self) -> Self {
        build_and_simulate_nor(&self.into(), Some(&rhs.into())).into()
    }

    pub fn xnor(self, rhs: Self) -> Self {
        build_and_simulate_xnor(&self.into(), Some(&rhs.into())).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::int::{GarbledInt128, GarbledInt16, GarbledInt32, GarbledInt64, GarbledInt8};
    use crate::uint::{GarbledUint128, GarbledUint16, GarbledUint32, GarbledUint64, GarbledUint8};

    #[test]
    fn test_from_u8_xor() {
        let a: GarbledUint8 = 170_u8.into(); // Binary 10101010
        let b: GarbledUint8 = 85_u8.into(); // Binary 01010101

        let result: u8 = (&a ^ &b).into();
        assert_eq!(result, 255); // Expected result of XOR between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_xor() {
        let a: GarbledUint16 = 43690_u16.into(); // Binary 1010101010101010
        let b: GarbledUint16 = 21845_u16.into(); // Binary 0101010101010101

        let result: u16 = (&a ^ &b).into();
        assert_eq!(result, 65535); // Expected result of XOR between 1010101010101010 and 0101010101010101
    }

    #[test]
    fn test_from_u32_xor() {
        let a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
        let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

        let result: u32 = (&a ^ &b).into();
        assert_eq!(result, 4294967295); // Expected result of XOR between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_xor() {
        let a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result: u64 = (&a ^ &b).into();
        assert_eq!(result, 18446744073709551615); // Expected result of XOR between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_xor() {
        let a: GarbledUint128 = 170_u128.into(); // Binary 10101010
        let b: GarbledUint128 = 85_u128.into(); // Binary 01010101

        let result: u128 = (&a ^ &b).into();
        assert_eq!(result, 255); // Expected result of XOR between 10101010 and 01010101
    }

    #[test]
    fn test_from_i8_xor() {
        let a: GarbledInt8 = (-86_i8).into(); // Two's complement binary for -86 is 10101010
        let b: GarbledInt8 = (-43).into(); // Two's complement binary for -43 is 11010101

        let result: i8 = (a ^ b).into();
        assert_eq!(result, -86_i8 ^ -43_i8); // Expected result of XOR between 10101010 and 11010101
    }

    #[test]
    fn test_from_i16_xor() {
        let a: GarbledInt<16> = (-21846_i16).into(); // Two's complement binary for -21846 is 1010101010101010
        let b: GarbledInt<16> = (-10923_i16).into(); // Two's complement binary for -10923 is 1101010101010101

        let result: i16 = (a ^ b).into();
        assert_eq!(result, -21846_i16 ^ -10923_i16); // Expected result of XOR between 1010101010101010 and 1101010101010101
    }

    #[test]
    fn test_from_u8_and() {
        let a: GarbledUint8 = 170_u8.into(); // Binary 10101010
        let b: GarbledUint8 = 85_u8.into(); // Binary 01010101

        let result: u8 = (a & b).into();
        assert_eq!(result, 170 & 85); // Expected result of AND between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_and() {
        let a: GarbledUint16 = 43690_u16.into(); // Binary 1010101010101010
        let b: GarbledUint16 = 21845_u16.into(); // Binary 0101010101010101

        let result: u16 = (a & b).into();
        assert_eq!(result, 43690 & 21845); // Expected result of AND between 1010101010101010 and 0101010101010101
    }

    #[test]
    fn test_from_u32_and() {
        let a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
        let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

        let result: u32 = (a & b).into();
        assert_eq!(result, 2863311530 & 1431655765); // Expected result of AND between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_and() {
        let a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result: u64 = (a & b).into();
        assert_eq!(result, 12297829382473034410 & 6148914691236517205);
        // Expected result of AND between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_and() {
        let a: GarbledUint128 = 170_u128.into(); // Binary 10101010
        let b: GarbledUint128 = 85_u128.into(); // Binary 01010101

        let result: u128 = (a & b).into();
        assert_eq!(result, 170 & 85); // Expected result of AND between 10101010 and 01010101
    }

    #[test]
    fn test_from_u8_or() {
        let a: GarbledUint8 = 170_u8.into(); // Binary 10101010
        let b: GarbledUint8 = 85_u8.into(); // Binary 01010101

        let result: u8 = (a | b).into();
        assert_eq!(result, 170 | 85); // Expected result of OR between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_or() {
        let a: GarbledUint16 = 43707_u16.into(); // Binary 1010101010101011
        let b: GarbledUint16 = 21845_u16.into(); // Binary 0101010101010101

        let result: u16 = (a | b).into();
        assert_eq!(result, 43707 | 21845); // Expected result of OR between 1010101010101011 and 0101010101010101
    }

    #[test]
    fn test_from_u32_or() {
        let a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
        let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

        let result: u32 = (a | b).into();
        assert_eq!(result, 2863311530 | 1431655765); // Expected result of OR between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_or() {
        let a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result: u64 = (a | b).into();
        assert_eq!(result, 12297829382473034410 | 6148914691236517205);
        // Expected result of OR between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_or() {
        let a: GarbledUint128 = 170_u128.into(); // Binary 10101010
        let b: GarbledUint128 = 85_u128.into(); // Binary 01010101

        let result: u128 = (a | b).into();
        assert_eq!(result, 170 | 85); // Expected result of OR between 10101010 and 01010101
    }

    #[test]
    fn test_from_i8_or() {
        let a: GarbledInt8 = (-86).into(); // Two's complement binary for -86 is 10101010
        let b: GarbledInt8 = (-43).into(); // Two's complement binary for -43 is 11010101

        let result: i8 = (a | b).into();
        assert_eq!(result, -86_i8 | -43_i8); // Expected result of OR between 10101010 and 11010101
    }

    #[test]
    fn test_from_i16_or() {
        let a: GarbledInt<16> = (-21846).into(); // Two's complement binary for -21846 is 1010101010101010
        let b: GarbledInt<16> = (-10923).into(); // Two's complement binary for -10923 is 1101010101010101

        let result: i16 = (a | b).into();
        assert_eq!(result, -21846_i16 | -10923_i16); // Expected result of OR between 1010101010101010 and 1101010101010101
    }

    #[test]
    fn test_from_i32_or() {
        let a: GarbledInt<32> = (-1431655766).into(); // Two's complement binary for -1431655766 is 10101010101010101010101010101010
        let b: GarbledInt<32> = (-715827883).into(); // Two's complement binary for -715827883 is 11010101010101010101010101010101

        let result: i32 = (a | b).into();
        assert_eq!(result, -1431655766_i32 | -715827883_i32);
        // Expected result of OR between 10101010101010101010101010101010 and 11010101010101010101010101010101
    }

    #[test]
    fn test_from_i64_or() {
        let a: GarbledInt<64> = (-6148914691236517206_i64).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
        let b: GarbledInt<64> = (-3074457345618258603_i64).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

        let result: i64 = (a | b).into();
        assert_eq!(result, -6148914691236517206_i64 | -3074457345618258603_i64);
    }

    #[test]
    fn test_from_i128_or() {
        let a: GarbledInt<128> = (-6148914691236517206_i128).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
        let b: GarbledInt<128> = (-3074457345618258603_i128).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

        let result: i128 = (a | b).into();
        assert_eq!(
            result,
            -6148914691236517206_i128 | -3074457345618258603_i128
        );
    }

    #[test]
    fn test_int_and() {
        let a: GarbledInt8 = (-86).into(); // Two's complement binary for -86 is 10101010
        let b: GarbledInt8 = (-43).into(); // Two's complement binary for -43 is 11010101

        let result: i8 = (a & b).into();
        assert_eq!(result, -86_i8 & -43_i8); // Expected result of AND between 10101010 and 11010101
    }

    #[test]
    fn test_from_i16_and() {
        let a: GarbledInt16 = (-21846).into(); // Two's complement binary for -21846 is 1010101010101010
        let b: GarbledInt16 = (-10923).into(); // Two's complement binary for -10923 is 1101010101010101

        let result: i16 = (a & b).into();
        assert_eq!(result, -21846_i16 & -10923_i16); // Expected result of AND between 1010101010101010 and 1101010101010101
    }

    #[test]
    fn test_from_i32_and() {
        let a: GarbledInt32 = (-1431655766).into(); // Two's complement binary for -1431655766 is 10101010101010101010101010101010
        let b: GarbledInt32 = (-715827883).into(); // Two's complement binary for -715827883 is 11010101010101010101010101010101

        let result: i32 = (a & b).into();
        assert_eq!(result, -1431655766_i32 & -715827883_i32);
        // Expected result of AND between 10101010101010101010101010101010 and 11010101010101010101010101010101
    }

    #[test]
    fn test_from_i64_and() {
        let a: GarbledInt64 = (-6148914691236517206_i64).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
        let b: GarbledInt64 = (-3074457345618258603_i64).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

        let result: i64 = (a & b).into();
        assert_eq!(result, -6148914691236517206_i64 & -3074457345618258603_i64);
        // Expected result of AND between 1010101010101010101010101010101010101010101010101010101010101010 and 1101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_i128_and() {
        let a: GarbledInt128 = (-6148914691236517206_i128).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
        let b: GarbledInt128 = (-3074457345618258603_i128).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

        let result: i128 = (a & b).into();
        assert_eq!(
            result,
            -6148914691236517206_i128 & -3074457345618258603_i128
        );
        // Expected result of AND between 1010101010101010101010101010101010101010101010101010101010101010 and 1101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u8_not() {
        let a: GarbledUint8 = 170_u8.into(); // Binary 10101010

        let result: u8 = (!a).into();
        assert_eq!(result, !170); // Expected result of NOT on 10101010
    }

    #[test]
    fn test_from_u16_not() {
        let a: GarbledUint16 = 43690_u16.into(); // Binary 1010101010101010

        let result: u16 = (!a).into();
        assert_eq!(result, !43690); // Expected result of NOT on 1010101010101010
    }

    #[test]
    fn test_from_u32_not() {
        let a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010

        let result: u32 = (!a).into();
        assert_eq!(result, !2863311530); // Expected result of NOT on 10101010101010101010101010101010
    }

    #[test]
    fn test_from_u64_not() {
        let a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010

        let result: u64 = (!a).into();
        assert_eq!(result, !12297829382473034410); // Expected result of NOT on 1010101010101010101010101010101010101010101010101010101010101010
    }

    #[test]
    fn test_from_u128_not() {
        let a: GarbledUint128 = 170_u128.into(); // Binary 10101010

        let result: u128 = (!a).into();
        assert_eq!(result, !170); // Expected result of NOT on 10101010
    }

    #[test]
    fn test_from_i8_not() {
        let a: GarbledInt8 = (-86).into(); // Two's complement binary for -86 is 10101010

        let result: i8 = (!a).into();
        assert_eq!(result, !-86_i8); // Expected result of NOT on 10101010
    }

    #[test]
    fn test_from_i16_not() {
        let a: GarbledInt16 = (-21846).into(); // Two's complement binary for -21846 is 1010101010101010

        let result: i16 = (!a).into();
        assert_eq!(result, !-21846_i16); // Expected result of NOT on 1010101010101010
    }

    #[test]
    fn test_from_i32_not() {
        let a: GarbledInt32 = (-1431655766).into(); // Two's complement binary for -1431655766 is 10101010101010101010101010101010

        let result: i32 = (!a).into();
        assert_eq!(result, !-1431655766_i32); // Expected result of NOT on 10101010101010101010101010101010
    }

    #[test]
    fn test_from_i64_not() {
        let a: GarbledInt64 = (-6148914691236517206_i64).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010

        let result: i64 = (!a).into();
        assert_eq!(result, !-6148914691236517206_i64); // Expected result of NOT on 1010101010101010101010101010101010101010101010101010101010101010
    }

    #[test]
    fn test_from_i128_not() {
        let a: GarbledInt128 = (-6148914691236517206_i128).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010

        let result: i128 = (!a).into();
        assert_eq!(result, !-6148914691236517206_i128); // Expected result of NOT on 1010101010101010101010101010101010101010101010101010101010101010
    }

    #[test]
    fn test_left_shift_uint() {
        let a: GarbledUint8 = 0b0001_u8.into(); // Binary 0001
        let result: u8 = (a << 1).into(); // Perform left shift by 1
        assert_eq!(result, 0b0010_u8);

        let a: GarbledUint8 = 0b0001_u8.into(); // Binary 0001
        let result: u8 = (a << 2).into(); // Perform left shift by 2
        assert_eq!(result, 0b0100_u8);

        let a: GarbledUint8 = 0b0001_u8.into(); // Binary 0001
        let result: u8 = (a << 3).into(); // Perform left shift by 3
        assert_eq!(result, 0b1000);

        let a = GarbledUint::<4>::new(vec![true, false, false, false]); // Binary 0001
        let result: u8 = (a << 2).into(); // Perform left shift by 2
        assert_eq!(result, 0b0100); // Binary 0100 (Left shift result of 0001)

        let a = GarbledUint::<4>::new(vec![true, false, false, false]); // Binary 0001

        let result: u8 = (a << 3).into(); // Perform left shift by 3
        assert_eq!(result, 0b1000); // Binary 1000 (Left shift result of 0001)
    }

    #[test]
    fn test_left_shift_int() {
        let a: GarbledInt8 = 0b1000_i8.into(); // Binary 1000

        let result: i8 = (a << 1).into(); // Perform left shift by 1
        assert_eq!(result, 0b10000_i8); // Binary 0000 (Left shift result of 1000)

        let a: GarbledInt8 = 0b1000_i8.into(); // Binary 1000

        let result: i8 = (a << 2).into(); // Perform left shift by 2
        assert_eq!(result, 0b100000_i8); // Binary 0000 (Left shift result of 1000)

        let a: GarbledInt8 = 0b1000_i8.into(); // Binary 1000

        let result: i8 = (a << 3).into(); // Perform left shift by 3
        assert_eq!(result, 0b1000000_i8); // Binary 0000 (Left shift result of 1000)

        let a: GarbledInt8 = 1_i8.into(); // Binary 1000

        let result: i8 = (a << 1).into(); // Perform left shift by 1
        assert_eq!(result, 0b0010_i8); // Binary 0010 (Left shift result of 0001)

        let a = GarbledInt::<4>::new(vec![true, false, false, false]); // Binary 0001

        let result: i8 = (a << 2).into(); // Perform left shift by 2
        assert_eq!(result, 0b0100_i8); // Binary 0100 (Left shift result of 0001)

        let a = GarbledInt::<4>::new(vec![true, false, false, false]); // Binary 0001

        let result: i8 = (a << 3).into(); // Perform left shift by 3
        assert_eq!(result, 0b1000_i8); // Binary 1000 (Left shift result of 0001)
    }

    #[test]
    fn test_right_shift_uint() {
        let a = GarbledUint::<4>::new(vec![false, false, false, true]); // Binary 1000

        let result: u8 = (a >> 1).into(); // Perform right shift by 1
        assert_eq!(result, 0b0100); // Binary 0100 (Right shift result of 1000)

        let a = GarbledUint::<4>::new(vec![false, false, false, true]); // Binary 1000

        let result: u8 = (a >> 2).into(); // Perform right shift by 2
        assert_eq!(result, 0b0010); // Binary 0010 (Right shift result of 1000)

        let a = GarbledUint::<4>::new(vec![false, false, false, true]); // Binary 1000

        let result: u8 = (a >> 3).into(); // Perform right shift by 3
        assert_eq!(result, 0b0001); // Binary 0001 (Right shift result of 1000)
    }

    #[test]
    fn test_from_u8_nand() {
        let a: GarbledUint8 = 170_u8.into(); // Binary 10101010
        let b: GarbledUint8 = 85_u8.into(); // Binary 01010101

        let result: u8 = a.nand(b).into();
        assert_eq!(result, !(170 & 85)); // Expected result of NAND between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_nand() {
        let a: GarbledUint16 = 43690_u16.into(); // Binary 1010101010101010
        let b: GarbledUint16 = 21845_u16.into(); // Binary 0101010101010101

        let result: u16 = a.nand(b).into();
        assert_eq!(result, !(43690 & 21845)); // Expected result of NAND between 1010101010101010 and 0101010101010101
    }

    #[test]
    fn test_from_u32_nand() {
        let a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
        let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

        let result: u32 = a.nand(b).into();
        assert_eq!(result, !(2863311530 & 1431655765)); // Expected result of NAND between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_nand() {
        let a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result: u64 = a.nand(b).into();
        assert_eq!(result, !(12297829382473034410 & 6148914691236517205));
        // Expected result of NAND between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_nand() {
        let a: GarbledUint128 = 170_u128.into(); // Binary 10101010
        let b: GarbledUint128 = 85_u128.into(); // Binary 01010101

        let result: u128 = a.nand(b).into();
        assert_eq!(result, !(170 & 85)); // Expected result of NAND between 10101010 and 01010101
    }

    #[test]
    fn test_from_i8_nand() {
        let a: GarbledInt8 = (-86).into(); // Two's complement binary for -86 is 10101010
        let b: GarbledInt8 = (-43).into(); // Two's complement binary for -43 is 11010101

        let result: i8 = a.nand(b).into();
        assert_eq!(result, !(-86_i8 & -43_i8)); // Expected result of NAND between 10101010 and 11010101
    }

    #[test]
    fn test_from_i16_nand() {
        let a: GarbledInt<16> = (-21846).into(); // Two's complement binary for -21846 is 1010101010101010
        let b: GarbledInt<16> = (-10923).into(); // Two's complement binary for -10923 is 1101010101010101

        let result: i16 = a.nand(b).into();
        assert_eq!(result, !(-21846_i16 & -10923_i16)); // Expected result of NAND between 1010101010101010 and 1101010101010101
    }

    #[test]
    fn test_from_i32_nand() {
        let a: GarbledInt<32> = (-1431655766).into(); // Two's complement binary for -1431655766 is 10101010101010101010101010101010
        let b: GarbledInt<32> = (-715827883).into(); // Two's complement binary for -715827883 is 11010101010101010101010101010101

        let result: i32 = a.nand(b).into();
        assert_eq!(result, !(-1431655766_i32 & -715827883_i32));
        // Expected result of NAND between 10101010101010101010101010101010 and 11010101010101010101010101010101
    }

    #[test]
    fn test_from_i64_nand() {
        let a: GarbledInt64 = (-6148914691236517206_i64).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
        let b: GarbledInt64 = (-3074457345618258603_i64).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

        let result: i64 = a.nand(b).into();
        assert_eq!(
            result,
            !(-6148914691236517206_i64 & -3074457345618258603_i64)
        );
    }

    #[test]
    fn test_from_i128_nand() {
        let a: GarbledInt<128> = (-6148914691236517206_i128).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
        let b: GarbledInt<128> = (-3074457345618258603_i128).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

        let result: i128 = a.nand(b).into();
        assert_eq!(
            result,
            !(-6148914691236517206_i128 & -3074457345618258603_i128)
        );
    }

    #[test]
    fn test_from_u8_nor() {
        let a: GarbledUint8 = 170_u8.into(); // Binary 10101010
        let b: GarbledUint8 = 85_u8.into(); // Binary 01010101

        let result: u8 = a.nor(b).into();
        assert_eq!(result, !(170 | 85)); // Expected result of NOR between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_nor() {
        let a: GarbledUint16 = 43707_u16.into(); // Binary 1010101010101011
        let b: GarbledUint16 = 21845_u16.into(); // Binary 0101010101010101

        let result: u16 = a.nor(b).into();
        assert_eq!(result, !(43707 | 21845)); // Expected result of NOR between 1010101010101011 and 0101010101010101
    }

    #[test]
    fn test_from_u32_nor() {
        let a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
        let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

        let result: u32 = a.nor(b).into();
        assert_eq!(result, !(2863311530 | 1431655765)); // Expected result of NOR between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_nor() {
        let a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result: u64 = a.nor(b).into();
        assert_eq!(result, !(12297829382473034410 | 6148914691236517205));
        // Expected result of NOR between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_nor() {
        let a: GarbledUint128 = 170_u128.into(); // Binary 10101010
        let b: GarbledUint128 = 85_u128.into(); // Binary 01010101

        let result: u128 = a.nor(b).into();
        assert_eq!(result, !(170 | 85)); // Expected result of NOR between 10101010 and 01010101
    }

    #[test]
    fn test_from_i8_nor() {
        let a: GarbledInt8 = (-86).into(); // Two's complement binary for -86 is 10101010
        let b: GarbledInt8 = (-43).into(); // Two's complement binary for -43 is 11010101

        let result: i8 = a.nor(b).into();
        assert_eq!(result, !(-86_i8 | -43_i8)); // Expected result of NOR between 10101010 and 11010101
    }

    #[test]
    fn test_from_i16_nor() {
        let a: GarbledInt<16> = (-21846).into(); // Two's complement binary for -21846 is 1010101010101010
        let b: GarbledInt<16> = (-10923).into(); // Two's complement binary for -10923 is 1101010101010101

        let result: i16 = a.nor(b).into();
        assert_eq!(result, !(-21846_i16 | -10923_i16)); // Expected result of NOR between 1010101010101010 and 1101010101010101
    }

    #[test]
    fn test_from_i32_nor() {
        let a: GarbledInt<32> = (-1431655766).into(); // Two's complement binary for -1431655766 is 10101010101010101010101010101010
        let b: GarbledInt<32> = (-715827883).into(); // Two's complement binary for -715827883 is 11010101010101010101010101010101

        let result: i32 = a.nor(b).into();
        assert_eq!(result, !(-1431655766_i32 | -715827883_i32));
        // Expected result of NOR between 10101010101010101010101010101010 and 11010101010101010101010101010101
    }

    #[test]
    fn test_from_i64_nor() {
        let a: GarbledInt<64> = (-6148914691236517206_i64).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
        let b: GarbledInt<64> = (-3074457345618258603_i64).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

        let result: i64 = a.nor(b).into();
        assert_eq!(
            result,
            !(-6148914691236517206_i64 | -3074457345618258603_i64)
        );
        // Expected result of NOR between 1010101010101010101010101010101010101010101010101010101010101010 and 1101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_i128_nor() {
        let a: GarbledInt<128> = (-6148914691236517206_i128).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
        let b: GarbledInt<128> = (-3074457345618258603_i128).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

        let result: i128 = a.nor(b).into();
        assert_eq!(
            result,
            !(-6148914691236517206_i128 | -3074457345618258603_i128)
        );
        // Expected result of NOR between 1010101010101010101010101010101010101010101010101010101010101010 and 1101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u8_xnor() {
        let a: GarbledUint8 = 170_u8.into(); // Binary 10101010
        let b: GarbledUint8 = 85_u8.into(); // Binary 01010101

        let result: u8 = a.xnor(b).into();
        assert_eq!(result, !(170 ^ 85)); // Expected result of XNOR between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_xnor() {
        let a: GarbledUint16 = 43690_u16.into(); // Binary 1010101010101010
        let b: GarbledUint16 = 21845_u16.into(); // Binary 0101010101010101

        let result: u16 = a.xnor(b).into();
        assert_eq!(result, !(43690 ^ 21845)); // Expected result of XNOR between 1010101010101010 and 0101010101010101
    }

    #[test]
    fn test_from_u32_xnor() {
        let a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
        let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

        let result: u32 = a.xnor(b).into();
        assert_eq!(result, !(2863311530 ^ 1431655765)); // Expected result of XNOR between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_xnor() {
        let a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result: u64 = a.xnor(b).into();
        assert_eq!(result, !(12297829382473034410 ^ 6148914691236517205));
        // Expected result of XNOR between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_xnor() {
        let a: GarbledUint128 = 170_u128.into(); // Binary 10101010
        let b: GarbledUint128 = 85_u128.into(); // Binary 01010101

        let result: u128 = a.xnor(b).into();
        assert_eq!(result, !(170 ^ 85)); // Expected result of XNOR between 10101010 and 01010101
    }

    #[test]
    fn test_from_i8_xnor() {
        let a: GarbledInt8 = (-86).into(); // Two's complement binary for -86 is 10101010
        let b: GarbledInt8 = (-43).into(); // Two's complement binary for -43 is 11010101

        let result: i8 = a.xnor(b).into();
        assert_eq!(result, !(-86_i8 ^ -43_i8)); // Expected result of XNOR between 10101010 and 11010101
    }

    #[test]
    fn test_from_i16_xnor() {
        let a: GarbledInt16 = (-21846).into(); // Two's complement binary for -21846 is 1010101010101010
        let b: GarbledInt16 = (-10923).into(); // Two's complement binary for -10923 is 1101010101010101

        let result: i16 = a.xnor(b).into();
        assert_eq!(result, !(-21846_i16 ^ -10923_i16)); // Expected result of XNOR between 1010101010101010 and 1101010101010101
    }

    #[test]
    fn test_from_i32_xnor() {
        let a: GarbledInt32 = (-1431655766).into(); // Two's complement binary for -1431655766 is 10101010101010101010101010101010
        let b: GarbledInt32 = (-715827883).into(); // Two's complement binary for -715827883 is 11010101010101010101010101010101

        let result: i32 = a.xnor(b).into();
        assert_eq!(result, !(-1431655766_i32 ^ -715827883_i32));
        // Expected result of XNOR between 10101010101010101010101010101010 and 11010101010101010101010101010101
    }

    #[test]
    fn test_from_i64_xnor() {
        let a: GarbledInt64 = (-6148914691236517206_i64).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
        let b: GarbledInt64 = (-3074457345618258603_i64).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

        let result: i64 = a.xnor(b).into();
        assert_eq!(
            result,
            !(-6148914691236517206_i64 ^ -3074457345618258603_i64)
        );
        // Expected result of XNOR between 1010101010101010101010101010101010101010101010101010101010101010 and 1101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_i128_xnor() {
        let a: GarbledInt128 = (-6148914691236517206_i128).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
        let b: GarbledInt128 = (-3074457345618258603_i128).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

        let result: i128 = a.xnor(b).into();
        assert_eq!(
            result,
            !(-6148914691236517206_i128 ^ -3074457345618258603_i128)
        );
        // Expected result of XNOR between 1010101010101010101010101010101010101010101010101010101010101010 and 1101010101010101010101010101010101010101010101010101010101010101
    }

    #[ignore = "still testing bitwise right shift int"]
    #[test]
    fn test_right_shift_int() {
        let a: GarbledInt8 = 0b1000_i8.into(); // Binary 1000

        let result: i8 = (a >> 1).into(); // Perform right shift by 1
        assert_eq!(result, 0b0100_i8); // Binary 0100 (Right shift result of 1000)

        let a: GarbledInt8 = 0b1000_i8.into(); // Binary 1000

        let result: i8 = (a >> 2).into(); // Perform right shift by 2
        assert_eq!(result, 0b0010_i8); // Binary 0010 (Right shift result of 1000)

        let a: GarbledInt8 = 0b1000_i8.into(); // Binary 1000

        let result: i8 = (a >> 3).into(); // Perform right shift by 3
        assert_eq!(result, 0b0001_i8); // Binary 0001 (Right shift result of 1000)

        let a: GarbledInt8 = 1_i8.into(); // Binary 0001

        let result: i8 = (a >> 1).into(); // Perform right shift by 1
        assert_eq!(result, 0b0000_i8); // Binary 0000 (Right shift result of 0001)

        let a = GarbledInt::<4>::new(vec![true, false, false, false]); // Binary 0001

        let result: i8 = (a >> 2).into(); // Perform right shift by 2
        assert_eq!(result, 0b0000_i8); // Binary 0000 (Right shift result of 0001)

        let a = GarbledInt::<4>::new(vec![true, false, false, false]); // Binary 0001

        let result: i8 = (a >> 3).into(); // Perform right shift by 3
        assert_eq!(result, 0b0000_i8); // Binary 0000 (Right shift result of 0001)
    }
}
