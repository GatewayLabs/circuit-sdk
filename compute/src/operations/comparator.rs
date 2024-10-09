use crate::uint::GarbledUint;
use std::cmp::Ordering;
use tandem::{Circuit, Gate};

// Helper function to build and simulate a circuit for comparison operations
fn build_and_simulate_comparison<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
    gate_fn: fn(u32, u32, &mut Vec<Gate>) -> u32,
) -> bool {
    let mut gates = Vec::new();

    // Push input gates for both Uint<N> objects
    for _ in 0..N {
        gates.push(Gate::InContrib); // From first Uint<N> (lhs)
    }
    for _ in 0..N {
        gates.push(Gate::InEval); // From second Uint<N> (rhs)
    }

    // Build the comparison circuit
    let mut result = gate_fn(0, N as u32, &mut gates);
    for i in 1..N {
        let current_comparison = gate_fn(i as u32, (N + i) as u32, &mut gates);
        let new_result = gates.len() as u32;
        gates.push(Gate::Xor(result, current_comparison));
        result = new_result;
    }

    // The final gate is our output
    let output_indices = vec![result];

    // Create the circuit
    let program = Circuit::new(gates, output_indices);

    // Simulate the circuit
    let result = lhs.simulate(&program, &lhs.bits, &rhs.bits).unwrap();

    // Return the boolean result
    result[0]
}

impl<const N: usize> GarbledUint<N> {
    // Helper method for equality comparison
    fn eq_helper(&self, other: &Self) -> bool {
        !build_and_simulate_comparison(self, other, |a, b, gates| {
            let xor = gates.len() as u32;
            gates.push(Gate::Xor(a, b));
            xor
        })
    }

    // Helper method for ordering comparison
    fn cmp_helper(&self, other: &Self) -> Ordering {
        let mut lt = false;
        let mut gt = false;

        for i in (0..N).rev() {
            let a = self.bits[i];
            let b = other.bits[i];

            if a && !b {
                gt = true;
                break;
            } else if !a && b {
                lt = true;
                break;
            }
        }

        if lt {
            Ordering::Less
        } else if gt {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl<const N: usize> PartialEq for GarbledUint<N> {
    fn eq(&self, other: &Self) -> bool {
        self.eq_helper(other)
    }
}

impl<const N: usize> Eq for GarbledUint<N> {}

impl<const N: usize> PartialOrd for GarbledUint<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp_helper(other))
    }
}

impl<const N: usize> Ord for GarbledUint<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_helper(other)
    }
}

impl<const N: usize> PartialEq<&GarbledUint<N>> for GarbledUint<N> {
    fn eq(&self, other: &&Self) -> bool {
        self.eq_helper(*other)
    }
}

impl<const N: usize> PartialOrd<&GarbledUint<N>> for GarbledUint<N> {
    fn partial_cmp(&self, other: &&Self) -> Option<Ordering> {
        Some(self.cmp_helper(*other))
    }
}

#[cfg(test)]
mod tests {
    use crate::uint::{GarbledUint128, GarbledUint16, GarbledUint32, GarbledUint64, GarbledUint8};

    #[test]
    fn test_equality() {
        let a = GarbledUint8::from_u8(123);
        let b = GarbledUint8::from_u8(123);
        let c = GarbledUint8::from_u8(124);

        assert_eq!(&a, &b);
        assert_ne!(&a, &c);
    }

    #[test]
    fn test_ordering() {
        let a = GarbledUint8::from_u8(123);
        let b = GarbledUint8::from_u8(124);
        let c = GarbledUint8::from_u8(122);

        assert!(&a < &b);
        assert!(&a > &c);
        assert!(&a <= &a);
        assert!(&a >= &a);
        assert!(&b > &a);
        assert!(&c < &a);
    }

    #[test]
    fn test_comparison_larger_types() {
        let a16 = GarbledUint16::from_u16(1000);
        let b16 = GarbledUint16::from_u16(2000);
        assert!(&a16 < &b16);

        let a32 = GarbledUint32::from_u32(100000);
        let b32 = GarbledUint32::from_u32(200000);
        assert!(&a32 < &b32);

        let a64 = GarbledUint64::from_u64(10000000000);
        let b64 = GarbledUint64::from_u64(20000000000);
        assert!(&a64 < &b64);

        let a128 = GarbledUint128::from_u128(100000000000000000000);
        let b128 = GarbledUint128::from_u128(200000000000000000000);
        assert!(&a128 < &b128);
    }
}
