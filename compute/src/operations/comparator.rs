use crate::uint::GarbledUint;
use std::cmp::Ordering;
use tandem::{Circuit, Gate};

fn comparator_circuit<const N: usize>(
    a_indices: &[u32],
    b_indices: &[u32],
    gates: &mut Vec<Gate>,
) -> (u32, u32) {
    let mut eq_list = vec![0; N];
    let mut lt_list = vec![0; N];

    let n = N;

    // Start from the most significant bit (MSB)
    let i = n - 1;

    // Compute initial eq and lt for MSB
    // eq[i] = ¬(A[i] ⊻ B[i])
    let a_xor_b = gates.len() as u32;
    gates.push(Gate::Xor(a_indices[i], b_indices[i]));

    let eq_i = gates.len() as u32;
    gates.push(Gate::Not(a_xor_b));

    eq_list[i] = eq_i;

    // lt[i] = ¬A[i] ∧ B[i]
    let not_a = gates.len() as u32;
    gates.push(Gate::Not(a_indices[i]));

    let lt_i = gates.len() as u32;
    gates.push(Gate::And(not_a, b_indices[i]));

    lt_list[i] = lt_i;

    // Iterate from MSB-1 down to LSB
    for idx in (0..i).rev() {
        // Compute eq[i] = eq[i+1] ∧ ¬(A[i] ⊻ B[i])
        let a_xor_b = gates.len() as u32;
        gates.push(Gate::Xor(a_indices[idx], b_indices[idx]));

        let not_a_xor_b = gates.len() as u32;
        gates.push(Gate::Not(a_xor_b));

        let eq_i = gates.len() as u32;
        gates.push(Gate::And(eq_list[idx + 1], not_a_xor_b));

        eq_list[idx] = eq_i;

        // Compute lt[i]
        // temp_lt = ¬A[i] ∧ B[i]
        let not_a = gates.len() as u32;
        gates.push(Gate::Not(a_indices[idx]));

        let not_a_and_b = gates.len() as u32;
        gates.push(Gate::And(not_a, b_indices[idx]));

        // temp_lt = eq[i+1] ∧ not_a_and_b
        let temp_lt = gates.len() as u32;
        gates.push(Gate::And(eq_list[idx + 1], not_a_and_b));

        // lt[i] = lt[i+1] ∨ temp_lt
        // Since we don't have an OR gate, use lt_i = (lt_prev ⊻ temp_lt) ⊻ (lt_prev ∧ temp_lt)
        let lt_prev = lt_list[idx + 1];

        let lt_xor_temp = gates.len() as u32;
        gates.push(Gate::Xor(lt_prev, temp_lt));

        let lt_and_temp = gates.len() as u32;
        gates.push(Gate::And(lt_prev, temp_lt));

        let lt_i = gates.len() as u32;
        gates.push(Gate::Xor(lt_xor_temp, lt_and_temp));

        lt_list[idx] = lt_i;
    }

    // Return the final lt and eq outputs
    (lt_list[0], eq_list[0])
}

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
        let mut gates = Vec::new();

        // Prepare input indices for both operands
        let mut a_indices = Vec::with_capacity(N);
        let mut b_indices = Vec::with_capacity(N);
        for _ in 0..N {
            a_indices.push(gates.len() as u32);
            gates.push(Gate::InContrib); // Inputs from 'self'
        }
        for _ in 0..N {
            b_indices.push(gates.len() as u32);
            gates.push(Gate::InEval); // Inputs from 'other'
        }

        // Build the comparator circuit
        let (lt_output, eq_output) = comparator_circuit::<N>(&a_indices, &b_indices, &mut gates);

        // Define outputs
        let output_indices = vec![lt_output, eq_output];

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &other.bits).unwrap();

        // Interpret the result
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
}

impl<const N: usize> PartialEq for GarbledUint<N> {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp_helper(other), Ordering::Equal)
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
    fn test_uint_equality() {
        let a = GarbledUint8::from_u8(123);
        let b = GarbledUint8::from_u8(123);
        let c = GarbledUint8::from_u8(124);

        assert_eq!(&a, &b);
        assert_ne!(&a, &c);
    }

    #[test]
    fn test_unsigned_comparison() {
        let a = GarbledUint8::from_u8(100);
        let b = GarbledUint8::from_u8(150);

        assert!(a < b);
        assert!(b > a);
        assert!(a != b);

        let c = GarbledUint8::from_u8(200);
        let d = GarbledUint8::from_u8(200);

        assert!(c == d);
        assert!(c <= d);
        assert!(c >= d);
    }

    #[test]
    fn test_uint_edge_cases() {
        let zero = GarbledUint8::from_u8(0);
        let max = GarbledUint8::from_u8(u8::MAX);

        assert!(zero < max);
        assert!(max > zero);
        assert!(zero != max);
    }

    #[test]
    fn test_uint_larger_comparison() {
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
