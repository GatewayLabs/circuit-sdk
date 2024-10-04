use crate::uint::Uint;
use std::ops::{BitAnd, BitXor, Not, Shl, Shr};
use tandem::{Circuit, Gate};

// Implement the XOR operation for Uint<N>
impl<const N: usize> BitXor for Uint<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        // Build a circuit that performs an N-bit XOR operation
        let mut gates = Vec::new();

        // Push the input gates for both Uint<N> objects
        for _ in 0..N {
            gates.push(Gate::InContrib); // From first Uint<N> (self)
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // From second Uint<N> (rhs)
        }

        // Define XOR gates for each corresponding bit in self and rhs
        for i in 0..N {
            gates.push(Gate::Xor(
                i.try_into().unwrap(),
                (N + i).try_into().unwrap(),
            )); // XOR gate between corresponding bits
        }

        // Define the output indices (for N-bit XOR)
        let n = N as u32;
        let output_indices: Vec<u32> = (2 * n..2 * n + n).collect();

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &rhs.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

// Implement the XOR operation for &Uint<N>
impl<const N: usize> BitXor for &Uint<N> {
    type Output = Uint<N>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        // Build a circuit that performs an N-bit XOR operation
        let mut gates = Vec::new();

        // Push the input gates for both Uint<N> objects
        for _ in 0..N {
            gates.push(Gate::InContrib); // From first Uint<N> (self)
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // From second Uint<N> (rhs)
        }

        // Define XOR gates for each corresponding bit in self and rhs
        for i in 0..N {
            gates.push(Gate::Xor(
                i.try_into().unwrap(),
                (N + i).try_into().unwrap(),
            )); // XOR gate between corresponding bits
        }

        // Define the output indices (for N-bit XOR)
        let n = N as u32;
        let output_indices: Vec<u32> = (2 * n..2 * n + n).collect();

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &rhs.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

// Implement the AND operation for Uint<N>
impl<const N: usize> BitAnd for Uint<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        // Build a circuit that performs an N-bit AND operation
        let mut gates = Vec::new();

        // Push the input gates for both Uint<N> objects
        for _ in 0..N {
            gates.push(Gate::InContrib); // From first Uint<N> (self)
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // From second Uint<N> (rhs)
        }

        // Define AND gates for each corresponding bit in self and rhs
        for i in 0..N {
            gates.push(Gate::And(
                i.try_into().unwrap(),
                (N + i).try_into().unwrap(),
            )); // AND gate between corresponding bits
        }

        // Define the output indices (for N-bit AND)
        let n = N as u32;
        let output_indices: Vec<u32> = (2 * n..2 * n + n).collect();

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &rhs.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

// Implement the AND operation for &Uint<N>
impl<const N: usize> BitAnd for &Uint<N> {
    type Output = Uint<N>;

    fn bitand(self, rhs: Self) -> Self::Output {
        // Build a circuit that performs an N-bit AND operation
        let mut gates = Vec::new();

        // Push the input gates for both Uint<N> objects
        for _ in 0..N {
            gates.push(Gate::InContrib); // From first Uint<N> (self)
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // From second Uint<N> (rhs)
        }

        // Define AND gates for each corresponding bit in self and rhs
        for i in 0..N {
            gates.push(Gate::And(
                i.try_into().unwrap(),
                (N + i).try_into().unwrap(),
            )); // AND gate between corresponding bits
        }

        // Define the output indices (for N-bit AND)
        let n = N as u32;
        let output_indices: Vec<u32> = (2 * n..2 * n + n).collect();

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &rhs.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

// Implement the NOT operation for Uint<N>
impl<const N: usize> Not for Uint<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        // Build a circuit that performs an N-bit NOT operation
        let mut gates = Vec::new();

        // Push the input gates for the Uint<N> object
        for _ in 0..N {
            gates.push(Gate::InContrib); // From the Uint<N> object
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

        //let output_indices = vec![2];

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &self.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

// Implement the NOT operation for &Uint<N>
impl<const N: usize> Not for &Uint<N> {
    type Output = Uint<N>;

    fn not(self) -> Self::Output {
        // Build a circuit that performs an N-bit NOT operation
        let mut gates = Vec::new();

        // Push the input gates for the Uint<N> object
        for _ in 0..N {
            gates.push(Gate::InContrib); // From the Uint<N> object
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

        //let output_indices = vec![2];

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &self.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

// Implement Shift Left operation for Uint<N> and &Uint<N>
impl<const N: usize> Shl<usize> for Uint<N> {
    type Output = Self;

    fn shl(self, shift: usize) -> Self::Output {
        let mut bits = self.bits.clone();

        // Shift the bits to the left by the specified amount
        for _ in 0..shift {
            bits.remove(N - 1); // Remove the most significant bit in little-endian
            bits.insert(0, false); // Add a 0 to the least significant bit in little-endian
        }

        Uint::new(bits)
    }
}

// Implement Shift Right operation for Uint<N> and &Uint<N>
impl<const N: usize> Shr<usize> for Uint<N> {
    type Output = Self;

    fn shr(self, shift: usize) -> Self::Output {
        let mut bits = self.bits.clone();

        // Shift the bits to the right by the specified amount
        for _ in 0..shift {
            bits.remove(0); // Remove the least significant bit in little-endian
            bits.push(false); // Add a 0 to the most significant bit in little-endian
        }

        Uint::new(bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uint_xor() {
        let a = Uint::<2>::new(vec![true, false]); // Binary 10
        let b = Uint::<2>::new(vec![false, true]); // Binary 01

        let result = a ^ b; // Perform XOR on the 2-bit values
        assert_eq!(result.to_u8(), 3); // Expected result of XOR between 10 and 01

        let a = Uint::<4>::new(vec![true, true, false, false]); // Binary 1100
        let b = Uint::<4>::new(vec![false, false, true, true]); // Binary 0011

        let result = a ^ b; // Perform XOR on the 4-bit values
        assert_eq!(result.to_u8(), 15); // Expected result of XOR between 1100 and 0011
    }

    #[test]
    fn test_from_u8_xor() {
        let a = Uint::<8>::from_u8(170); // Binary 10101010
        let b = Uint::<8>::from_u8(85); // Binary 01010101

        let result = &a ^ &b;
        assert_eq!(result.to_u8(), 255); // Expected result of XOR between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_xor() {
        let a = Uint::<16>::from_u16(43690); // Binary 1010101010101010
        let b = Uint::<16>::from_u16(21845); // Binary 0101010101010101

        let result = a ^ b;
        assert_eq!(result.to_u16(), 65535); // Expected result of XOR between 1010101010101010 and 0101010101010101
    }

    #[test]
    fn test_from_u32_xor() {
        let a = Uint::<32>::from_u32(2863311530); // Binary 10101010101010101010101010101010
        let b = Uint::<32>::from_u32(1431655765); // Binary 01010101010101010101010101010101

        let result = a ^ b;
        assert_eq!(result.to_u32(), 4294967295); // Expected result of XOR between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_xor() {
        let a = Uint::<64>::from_u64(12297829382473034410); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b = Uint::<64>::from_u64(6148914691236517205); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result = a ^ b;
        assert_eq!(result.to_u64(), 18446744073709551615); // Expected result of XOR between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_xor() {
        let a = Uint::<128>::from_u128(170); // Binary 10101010
        let b = Uint::<128>::from_u128(85); // Binary 01010101

        let result = a ^ b;
        assert_eq!(result.to_u128(), 255); // Expected result of XOR between 10101010 and 01010101
    }

    #[test]
    fn test_uint_and() {
        let a = Uint::<2>::new(vec![true, false]); // Binary 10
        let b = Uint::<2>::new(vec![false, true]); // Binary 01

        let result = a & b; // Perform AND on the 2-bit values
        assert_eq!(result.to_u8(), 0); // Expected result of AND between 10 and 01

        let a = Uint::<4>::new(vec![true, true, false, false]); // Binary 1100
        let b = Uint::<4>::new(vec![false, false, true, true]); // Binary 0011

        let result = a & b; // Perform AND on the 4-bit values
        assert_eq!(result.to_u8(), 0); // Expected result of AND between 1100 and 0011

        let a = Uint::<4>::new(vec![true, false, false, true]); // Binary 1001
        let b = Uint::<4>::new(vec![false, false, false, false]); // Binary 0000

        let result = a & b; // Perform AND on the 4-bit values
        assert_eq!(result.to_u8(), 0); // Expected result of AND between 1001 and 0000
    }

    #[test]
    fn test_from_u8_and() {
        let a = Uint::<8>::from_u8(170); // Binary 10101010
        let b = Uint::<8>::from_u8(85); // Binary 01010101

        let result = a & b;
        assert_eq!(result.to_u8(), 170 & 85); // Expected result of AND between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_and() {
        let a = Uint::<16>::from_u16(43690); // Binary 1010101010101010
        let b = Uint::<16>::from_u16(21845); // Binary 0101010101010101

        let result = a & b;
        assert_eq!(result.to_u16(), 43690 & 21845); // Expected result of AND between 1010101010101010 and 0101010101010101
    }

    #[test]
    fn test_from_u32_and() {
        let a = Uint::<32>::from_u32(2863311530); // Binary 10101010101010101010101010101010
        let b = Uint::<32>::from_u32(1431655765); // Binary 01010101010101010101010101010101

        let result = a & b;
        assert_eq!(result.to_u32(), 2863311530 & 1431655765); // Expected result of AND between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_and() {
        let a = Uint::<64>::from_u64(12297829382473034410); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b = Uint::<64>::from_u64(6148914691236517205); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result = a & b;
        assert_eq!(result.to_u64(), 12297829382473034410 & 6148914691236517205);
        // Expected result of AND between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_and() {
        let a = Uint::<128>::from_u128(170); // Binary 10101010
        let b = Uint::<128>::from_u128(85); // Binary 01010101

        let result = a & b;
        assert_eq!(result.to_u128(), 170 & 85); // Expected result of AND between 10101010 and 01010101
    }

    #[test]
    fn test_from_u8_not() {
        let a = Uint::<8>::from_u8(170); // Binary 10101010

        let result = !a;
        assert_eq!(result.to_u8(), !170); // Expected result of NOT on 10101010
    }

    #[test]
    fn test_from_u16_not() {
        let a = Uint::<16>::from_u16(43690); // Binary 1010101010101010

        let result = !a;
        assert_eq!(result.to_u16(), !43690); // Expected result of NOT on 1010101010101010
    }

    #[test]
    fn test_from_u32_not() {
        let a = Uint::<32>::from_u32(2863311530); // Binary 10101010101010101010101010101010

        let result = !a;
        assert_eq!(result.to_u32(), !2863311530); // Expected result of NOT on 10101010101010101010101010101010
    }

    #[test]
    fn test_from_u64_not() {
        let a = Uint::<64>::from_u64(12297829382473034410); // Binary 1010101010101010101010101010101010101010101010101010101010101010

        let result = !a;
        assert_eq!(result.to_u64(), !12297829382473034410);
        // Expected result of NOT on 1010101010101010101010101010101010101010101010101010101010101010
    }

    #[test]
    fn test_from_u128_not() {
        let a = Uint::<128>::from_u128(170); // Binary 10101010

        let result = !a;
        assert_eq!(result.to_u128(), !170); // Expected result of NOT on 10101010
    }

    #[test]
    fn test_left_shift() {
        let a = Uint::<4>::new(vec![false, false, false, true]); // Binary 1000

        let result = a << 1; // Perform left shift by 1
        assert_eq!(result.to_u8(), 0b0000 as u8); // Binary 0000 (Left shift result of 1000)

        // binary literal of 0000

        let a = Uint::<4>::new(vec![false, false, false, true]); // Binary 1000

        let result = a << 2; // Perform left shift by 2
        assert_eq!(result.to_u8(), 0b0000); // Binary 0000 (Left shift result of 1000)

        let a = Uint::<4>::new(vec![false, false, false, true]); // Binary 1000

        let result = a << 3; // Perform left shift by 3
        assert_eq!(result.to_u8(), 0b0000); // Binary 0000 (Left shift result of 1000)

        //let a = Uint::<4>::new(vec![false, false, false, true]); // Binary 0001

        let a = Uint::<8>::from_u8(1); // Binary 0001

        let result = a << 1; // Perform left shift by 1
        assert_eq!(result.to_u8(), 0b0010); // Binary 0010 (Left shift result of 0001)

        let a = Uint::<4>::new(vec![true, false, false, false]); // Binary 0001

        let result = a << 2; // Perform left shift by 2
        assert_eq!(result.to_u8(), 0b0100); // Binary 0100 (Left shift result of 0001)

        let a = Uint::<4>::new(vec![true, false, false, false]); // Binary 0001

        let result = a << 3; // Perform left shift by 3
        assert_eq!(result.to_u8(), 0b1000); // Binary 1000 (Left shift result of 0001)
    }

    #[test]
    fn test_right_shift() {
        let a = Uint::<4>::new(vec![false, false, false, true]); // Binary 1000

        let result = a >> 1; // Perform right shift by 1
        assert_eq!(result.to_u8(), 0b0100); // Binary 0100 (Right shift result of 1000)

        let a = Uint::<4>::new(vec![false, false, false, true]); // Binary 1000

        let result = a >> 2; // Perform right shift by 2
        assert_eq!(result.to_u8(), 0b0010); // Binary 0010 (Right shift result of 1000)

        let a = Uint::<4>::new(vec![false, false, false, true]); // Binary 1000

        let result = a >> 3; // Perform right shift by 3
        assert_eq!(result.to_u8(), 0b0001); // Binary 0001 (Right shift result of 1000)
    }
}
