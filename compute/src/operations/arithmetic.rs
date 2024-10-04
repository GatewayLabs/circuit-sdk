use crate::uint::Uint;
use tandem::{Circuit, Gate};

// Implement the Add operation for Uint<N> and &Uint<N>
impl<const N: usize> std::ops::Add for Uint<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // Build a circuit that performs an N-bit ADD operation
        let mut gates = Vec::new();

        // Push the input gates for the Uint<N> object
        for _ in 0..N {
            gates.push(Gate::InContrib); // From the Uint<N> object
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // From second Uint<N> (rhs)
        }

        // Define the carry bit
        let mut carry_index = None; // No carry initially

        // Define sum bit indices
        let mut sum_bit_indices = Vec::with_capacity(N);

        // Generate gates for N-bit addition
        for i in 0..N {
            let a = i; // Index for bit i of self
            let b = N + i; // Index for bit i of rhs

            // XOR gate for sum bit (a ⊕ b)
            let sum_xor_index = gates.len();
            gates.push(Gate::Xor(a.try_into().unwrap(), b.try_into().unwrap()));

            // If carry exists, XOR the result of the previous XOR with the carry
            let final_sum_index = if let Some(carry) = carry_index {
                let sum_with_carry_index = gates.len();
                gates.push(Gate::Xor(sum_xor_index.try_into().unwrap(), carry));
                sum_with_carry_index
            } else {
                sum_xor_index
            };

            sum_bit_indices.push(final_sum_index);

            // Compute the new carry: (a & b) | (a & carry) | (b & carry)
            let and_ab = gates.len();
            gates.push(Gate::And(a.try_into().unwrap(), b.try_into().unwrap())); // a & b

            if let Some(carry) = carry_index {
                let and_a_carry = gates.len();
                gates.push(Gate::And(a.try_into().unwrap(), carry)); // a & carry

                let and_b_carry = gates.len();
                gates.push(Gate::And(b.try_into().unwrap(), carry)); // b & carry

                // Combine the carry parts using XOR and AND to simulate OR
                let xor_ab_carry = gates.len();
                gates.push(Gate::Xor(
                    and_ab.try_into().unwrap(),
                    and_a_carry.try_into().unwrap(),
                )); // XOR part 1
                gates.push(Gate::Xor(
                    xor_ab_carry.try_into().unwrap(),
                    and_b_carry.try_into().unwrap(),
                )); // Final carry (simulated OR)
                carry_index = Some((gates.len() - 1).try_into().unwrap());
            } else {
                // If there is no previous carry, the carry is just a & b
                carry_index = Some(and_ab.try_into().unwrap());
            }
        }

        // Define output indices (sum bits from the addition)
        let output_indices: Vec<u32> = sum_bit_indices.iter().map(|&index| index as u32).collect();

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &rhs.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

impl<const N: usize> std::ops::Add for &Uint<N> {
    type Output = Uint<N>;

    fn add(self, rhs: Self) -> Self::Output {
        // Build a circuit that performs an N-bit ADD operation
        let mut gates = Vec::new();

        // Push the input gates for the Uint<N> object
        for _ in 0..N {
            gates.push(Gate::InContrib); // From the Uint<N> object
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // From second Uint<N> (rhs)
        }

        // Define the carry bit
        let mut carry_index = None; // No carry initially

        // Define sum bit indices
        let mut sum_bit_indices = Vec::with_capacity(N);

        // Generate gates for N-bit addition
        for i in 0..N {
            let a = i; // Index for bit i of self
            let b = N + i; // Index for bit i of rhs

            // XOR gate for sum bit (a ⊕ b)
            let sum_xor_index = gates.len();
            gates.push(Gate::Xor(a.try_into().unwrap(), b.try_into().unwrap()));

            // If carry exists, XOR the result of the previous XOR with the carry
            let final_sum_index = if let Some(carry) = carry_index {
                let sum_with_carry_index = gates.len();
                gates.push(Gate::Xor(sum_xor_index.try_into().unwrap(), carry));
                sum_with_carry_index
            } else {
                sum_xor_index
            };

            sum_bit_indices.push(final_sum_index);

            // Compute the new carry: (a & b) | (a & carry) | (b & carry)
            let and_ab = gates.len();
            gates.push(Gate::And(a.try_into().unwrap(), b.try_into().unwrap())); // a & b

            if let Some(carry) = carry_index {
                let and_a_carry = gates.len();
                gates.push(Gate::And(a.try_into().unwrap(), carry)); // a & carry

                let and_b_carry = gates.len();
                gates.push(Gate::And(b.try_into().unwrap(), carry)); // b & carry

                // Combine the carry parts using XOR and AND to simulate OR
                let xor_ab_carry = gates.len();
                gates.push(Gate::Xor(
                    and_ab.try_into().unwrap(),
                    and_a_carry.try_into().unwrap(),
                )); // XOR part 1
                gates.push(Gate::Xor(
                    xor_ab_carry.try_into().unwrap(),
                    and_b_carry.try_into().unwrap(),
                )); // Final carry (simulated OR)
                carry_index = Some((gates.len() - 1).try_into().unwrap());
            } else {
                // If there is no previous carry, the carry is just a & b
                carry_index = Some(and_ab.try_into().unwrap());
            }
        }

        // Define output indices (sum bits from the addition)
        let output_indices: Vec<u32> = sum_bit_indices.iter().map(|&index| index as u32).collect();

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &rhs.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

// Implement the Sub operation for Uint<N> and &Uint<N>
impl<const N: usize> std::ops::Sub for Uint<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // Build a circuit that performs an N-bit SUB operation
        let mut gates = Vec::new();

        // Push the input gates for the Uint<N> object
        for _ in 0..N {
            gates.push(Gate::InContrib); // From the Uint<N> object
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // From second Uint<N> (rhs)
        }

        // Define the borrow bit
        let mut borrow_index = None; // No borrow initially

        // Define difference bit indices
        let mut diff_bit_indices = Vec::with_capacity(N);

        // Generate gates for N-bit subtraction
        for i in 0..N {
            let a = i; // Index for bit i of self
            let b = N + i; // Index for bit i of rhs

            // XOR gate for difference bit (a ⊕ b)
            let diff_xor_index = gates.len();
            gates.push(Gate::Xor(a.try_into().unwrap(), b.try_into().unwrap()));

            // If borrow exists, XOR the result of the previous XOR with the borrow
            let final_diff_index = if let Some(borrow) = borrow_index {
                let diff_with_borrow_index = gates.len();
                gates.push(Gate::Xor(diff_xor_index.try_into().unwrap(), borrow));
                diff_with_borrow_index
            } else {
                diff_xor_index
            };

            diff_bit_indices.push(final_diff_index);

            // Compute the new borrow: (!a & b) | (a & borrow) | (!b & borrow)
            let not_a = gates.len();
            gates.push(Gate::Not(a.try_into().unwrap())); // !a

            let and_not_a_b = gates.len();
            gates.push(Gate::And(not_a.try_into().unwrap(), b.try_into().unwrap())); // !a & b

            if let Some(borrow) = borrow_index {
                let and_a_borrow = gates.len();
                gates.push(Gate::And(a.try_into().unwrap(), borrow)); // a & borrow

                let not_b = gates.len();
                gates.push(Gate::Not(b.try_into().unwrap())); // !b

                let and_not_borrow = gates.len();
                gates.push(Gate::And(not_b.try_into().unwrap(), borrow)); // !b & borrow

                // Combine the borrow parts using XOR and AND to simulate OR
                let xor_borrow_parts = gates.len();
                gates.push(Gate::Xor(
                    and_not_a_b.try_into().unwrap(),
                    and_a_borrow.try_into().unwrap(),
                )); // XOR part 1
                gates.push(Gate::Xor(
                    xor_borrow_parts.try_into().unwrap(),
                    and_not_borrow.try_into().unwrap(),
                )); // Final borrow (simulated OR)
                borrow_index = Some((gates.len() - 1).try_into().unwrap());
            } else {
                // If there is no previous borrow, the borrow is just !a & b
                borrow_index = Some(and_not_a_b.try_into().unwrap());
            }
        }

        // Define output indices (difference bits from the subtraction)
        let output_indices: Vec<u32> = diff_bit_indices.iter().map(|&index| index as u32).collect();

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &rhs.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

// Implement the Sub operation for &Uint<N>
impl<const N: usize> std::ops::Sub for &Uint<N> {
    type Output = Uint<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        // Build a circuit that performs an N-bit SUB operation
        let mut gates = Vec::new();

        // Push the input gates for the Uint<N> object
        for _ in 0..N {
            gates.push(Gate::InContrib); // From the Uint<N> object
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // From second Uint<N> (rhs)
        }

        // Define the borrow bit
        let mut borrow_index = None; // No borrow initially

        // Define difference bit indices
        let mut diff_bit_indices = Vec::with_capacity(N);

        // Generate gates for N-bit subtraction
        for i in 0..N {
            let a = i; // Index for bit i of self
            let b = N + i; // Index for bit i of rhs

            // XOR gate for difference bit (a ⊕ b)
            let diff_xor_index = gates.len();
            gates.push(Gate::Xor(a.try_into().unwrap(), b.try_into().unwrap()));

            // If borrow exists, XOR the result of the previous XOR with the borrow
            let final_diff_index = if let Some(borrow) = borrow_index {
                let diff_with_borrow_index = gates.len();
                gates.push(Gate::Xor(diff_xor_index.try_into().unwrap(), borrow));
                diff_with_borrow_index
            } else {
                diff_xor_index
            };

            diff_bit_indices.push(final_diff_index);

            // Compute the new borrow: (!a & b) | (a & borrow) | (!b & borrow)
            let not_a = gates.len();
            gates.push(Gate::Not(a.try_into().unwrap())); // !a

            let and_not_a_b = gates.len();
            gates.push(Gate::And(not_a.try_into().unwrap(), b.try_into().unwrap())); // !a & b

            if let Some(borrow) = borrow_index {
                let and_a_borrow = gates.len();
                gates.push(Gate::And(a.try_into().unwrap(), borrow)); // a & borrow

                let not_b = gates.len();
                gates.push(Gate::Not(b.try_into().unwrap())); // !b

                let and_not_borrow = gates.len();
                gates.push(Gate::And(not_b.try_into().unwrap(), borrow)); // !b & borrow

                // Combine the borrow parts using XOR and AND to simulate OR
                let xor_borrow_parts = gates.len();
                gates.push(Gate::Xor(
                    and_not_a_b.try_into().unwrap(),
                    and_a_borrow.try_into().unwrap(),
                )); // XOR part 1
                gates.push(Gate::Xor(
                    xor_borrow_parts.try_into().unwrap(),
                    and_not_borrow.try_into().unwrap(),
                )); // Final borrow (simulated OR)
                borrow_index = Some((gates.len() - 1).try_into().unwrap());
            } else {
                // If there is no previous borrow, the borrow is just !a & b
                borrow_index = Some(and_not_a_b.try_into().unwrap());
            }
        }

        // Define output indices (difference bits from the subtraction)
        let output_indices: Vec<u32> = diff_bit_indices.iter().map(|&index| index as u32).collect();

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &rhs.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uint_add() {
        let a = Uint::<4>::new(vec![true, true, false, false]); // Binary 1100
        let b = Uint::<4>::new(vec![false, false, true, true]); // Binary 0011

        let result = a + b; // Perform addition on the 4-bit values
        assert_eq!(result.to_u8(), 0b1111); // Binary 1111 (Addition result of 1100 + 0011)
    }

    #[test]
    fn test_from_u8_add() {
        let a = Uint::<8>::from_u8(170); // Binary 10101010
        let b = Uint::<8>::from_u8(85); // Binary 01010101

        let result = a + b; // Perform addition on the 4-bit values
        assert_eq!(result.to_u8(), 170 + 85); // Expected result of addition between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_add() {
        let a = Uint::<16>::from_u16(43690); // Binary 1010101010101010
        let b = Uint::<16>::from_u16(21845); // Binary 0101010101010101

        let result = a + b;
        assert_eq!(result.to_u16(), 43690 + 21845); // Expected result of addition between 1010101010101010 and 0101010101010101
    }

    #[test]
    fn test_from_u32_add() {
        let a = Uint::<32>::from_u32(2863311530); // Binary 10101010101010101010101010101010
        let b = Uint::<32>::from_u32(1431655765); // Binary 01010101010101010101010101010101

        let result = a + b;
        assert_eq!(result.to_u32(), 2863311530 + 1431655765); // Expected result of addition between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_add() {
        let a = Uint::<64>::from_u64(12297829382473034410); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b = Uint::<64>::from_u64(6148914691236517205); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result = a + b;
        assert_eq!(result.to_u64(), 12297829382473034410 + 6148914691236517205);
        // Expected result of addition between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_add() {
        let a = Uint::<128>::from_u128(12297829382473034410); // Binary 10101010
        let b = Uint::<128>::from_u128(6148914691236517205); // Binary 01010101

        let result = a + b;
        assert_eq!(result.to_u128(), 12297829382473034410 + 6148914691236517205);

        println!("{}", result.to_u128());
        // Expected result of addition between 10101010 and 01010101
    }

    #[test]
    fn test_uint_sub() {
        let a = Uint::<4>::from_u8(3);
        let b = Uint::<4>::from_u8(2);

        let result = a - b; // Perform subtraction on the 4-bit values
        assert_eq!(result.to_u8(), 3 - 2);
    }

    #[test]
    fn test_from_u8_sub() {
        let a = Uint::<8>::from_u8(170); // Binary 10101010
        let b = Uint::<8>::from_u8(100); // Binary 01100100

        let result = a - b;
        assert_eq!(result.to_u8(), 170 - 100); // Expected result of subtraction between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_sub() {
        let a = Uint::<16>::from_u16(43690); // Binary 1010101010101010
        let b = Uint::<16>::from_u16(21845); // Binary 0101010101010101

        let result = a - b;
        assert_eq!(result.to_u16(), 43690 - 21845); // Expected result of subtraction between 1010101010101010 and 0101010101010101
    }

    #[test]
    fn test_from_u32_sub() {
        let a = Uint::<32>::from_u32(2863311530); // Binary 10101010101010101010101010101010
        let b = Uint::<32>::from_u32(1431655765); // Binary 01010101010101010101010101010101

        let result = a - b;
        assert_eq!(result.to_u32(), 2863311530 - 1431655765); // Expected result of subtraction between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_sub() {
        let a = Uint::<64>::from_u64(12297829382473034410); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b = Uint::<64>::from_u64(6148914691236517205); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result = a - b;
        assert_eq!(result.to_u64(), 12297829382473034410 - 6148914691236517205);
        // Expected result of subtraction between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_sub() {
        let a = Uint::<128>::from_u128(170); // Binary 10101010
        let b = Uint::<128>::from_u128(85); // Binary 01010101

        let result = a - b;
        assert_eq!(result.to_u128(), 170 - 85); // Expected result of subtraction between 10101010 and 01010101
    }
}
