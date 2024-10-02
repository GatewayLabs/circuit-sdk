use crate::operations::Uint;
use tandem::states::{Contributor, Evaluator};
use tandem::GateIndex;
use tandem::{Circuit, Error, Gate};

impl<const N: usize> std::ops::Mul for Uint<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut gates = Vec::new();

        // Push input gates for both `self` and `rhs`
        for _ in 0..N {
            gates.push(Gate::InContrib); // For self bits
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // For rhs bits
        }

        // Initialize product and carry arrays
        let mut product_bits = vec![None; 2 * N]; // Up to 2 * N bits
        let mut carry_bits = vec![None; 2 * N]; // Carry propagation array

        // Full-adder logic: Iterate through each bit of `self` and `rhs`
        for i in 0..N {
            for j in 0..N {
                let partial_product = gates.len();
                gates.push(Gate::And(i as u32, (N + j) as u32)); // AND gate for partial product

                let pos = i + j;

                if let Some(existing_product) = product_bits[pos] {
                    // Sum the current product with the new partial product
                    let sum = gates.len();
                    gates.push(Gate::Xor(existing_product as u32, partial_product as u32));

                    let carry = gates.len();
                    gates.push(Gate::And(existing_product as u32, partial_product as u32));

                    product_bits[pos] = Some(sum);

                    // Manage carry propagation with debugging
                    println!("Integrating carry at pos {} with carry {}", pos, carry);
                    integrate_carry(
                        &mut gates,
                        &mut carry_bits,
                        &mut product_bits,
                        carry,
                        pos + 1,
                        2 * N,
                    );
                } else {
                    product_bits[pos] = Some(partial_product);
                }
            }
        }

        // Collect the output indices (the first N bits of the product)
        let output_indices: Vec<u32> = product_bits
            .iter()
            .take(N)
            .filter_map(|&bit| bit)
            .map(|index| index as u32)
            .collect();

        // Debug the output indices and gates before running simulation
        println!(
            "--- End of multiplication, output indices: {:?} ---",
            output_indices
        );
        println!("--- Gates: {:?} ---", gates);

        // Create the circuit with gates and outputs
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit using the bits from `self` and `rhs`
        let result = self.simulate(&program, &self.bits, &rhs.bits).unwrap();

        // Return the final Uint result with the N-bit output
        Uint::<N>::new(result)
    }
}

// Helper function to integrate carry into the next product bit
fn integrate_carry(
    gates: &mut Vec<Gate>,
    carry_bits: &mut Vec<Option<usize>>,
    product_bits: &mut Vec<Option<usize>>,
    carry: usize,
    pos: usize,
    max_bits: usize,
) {
    if pos >= max_bits {
        return; // No more positions to propagate the carry to
    }

    if let Some(existing_carry) = carry_bits[pos] {
        // There is already a carry bit at this position; propagate it

        // Sum the carries
        let carry_sum = gates.len();
        gates.push(Gate::Xor(existing_carry as u32, carry as u32));

        // Generate a new carry
        let new_carry = gates.len();
        gates.push(Gate::And(existing_carry as u32, carry as u32));

        // Store the sum at the current position
        product_bits[pos] = Some(carry_sum);

        // Continue propagating the new carry to the next position
        integrate_carry(
            gates,
            carry_bits,
            product_bits,
            new_carry,
            pos + 1,
            max_bits,
        );
    } else {
        // No carry exists at this position, so place the current carry
        product_bits[pos] = Some(carry);
        carry_bits[pos] = Some(carry);
    }
}

#[cfg(test)]
mod tests {
    use crate::{operations::Uint, u256::U256};

    #[test]
    fn test_mul_small_works() {
        let a = Uint::<128>::from_u128(2); // Binary 10
        let b = Uint::<128>::from_u128(3); // Binary 11
        let c = a * b;
        assert_eq!(c.to_u128(), 2 * 3); // Expect 6 (110b)
    }

    #[test]
    fn test_mul_u8_works() {
        let a = Uint::<8>::from_u8(3);
        let b = Uint::<8>::from_u8(3);
        let c = a * b;
        assert_eq!(c.to_u8(), 3 * 3); // Expect 9 (1001b)
    }

    #[test]
    fn test_mul_u8_fails() {
        let a = Uint::<8>::from_u8(3);
        let b = Uint::<8>::from_u8(4);
        let c = a * b;
        assert_eq!(c.to_u8(), 3 * 4);
    }

    #[test]
    fn test_mul_big_works() {
        let a = Uint::<128>::from_u128(2); // Binary 10
        let b = Uint::<128>::from_u128(3); // Binary 11
        let c = a * b;
        assert_eq!(c.to_u128(), 2 * 3); // Expect 6 (110b)
    }

    #[test]
    fn test_add_big_works() {
        let a = Uint::<128>::from_u128(2); // Binary 10
        let b = Uint::<128>::from_u128(3); // Binary 11
        let c = a + b;
        assert_eq!(c.to_u128(), 2 + 3); // Expect 6 (110b)
    }
}
